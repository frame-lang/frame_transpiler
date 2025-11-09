use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::native_region_scanner as nscan;
use crate::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;
use crate::frame_c::v3::mir_assembler::MirAssemblerV3;
use crate::frame_c::v3::expander::{FrameStatementExpanderV3, PyExpanderV3, TsExpanderV3, CExpanderV3, CppExpanderV3, JavaExpanderV3, RustExpanderV3};
use crate::frame_c::v3::splice::SplicerV3;
use crate::frame_c::v3::validator::{ValidatorV3, ValidationResultV3, ValidatorPolicyV3};

pub mod body_closer;
pub mod native_region_scanner;
pub mod frame_statement_parser;
pub mod mir;
pub mod mir_assembler;
pub mod expander;
pub mod splice;
pub mod validator;
pub mod multifile_demo;
pub mod module_partitioner;
pub mod prolog_scanner;
pub mod import_scanner;

/// V3 compiler entrypoint (MVP scaffold).
///
/// This will replace the legacy pipeline incrementally. For now it returns a
/// deterministic error so the CLI remains usable while we bring up stages.
pub struct CompilerV3;

impl CompilerV3 {
    pub fn compile_single_file(
        _input_path: Option<&str>,
        _content: &str,
        _target_language: Option<TargetLanguage>,
        _debug_output: bool,
    ) -> Result<String, RunError> {
        // MVP demo: treat whole content as a single native body starting with '{'
        let content = _content.as_bytes();
        if content.first().copied() != Some(b'{') {
            return Err(RunError::new(
                frame_exitcode::PARSE_ERR,
                "V3 demo expects body starting at '{' (single-body debug mode)",
            ));
        }
        let lang = _target_language.unwrap_or(TargetLanguage::Python3);
        // Select scanner
        let scan_res = match lang {
            TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(content, 0),
            TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(content, 0),
            TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(content, 0),
            TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(content, 0),
            TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(content, 0),
            TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(content, 0),
            TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(content, 0),
            _ => {
                return Err(RunError::new(
                    frame_exitcode::PARSE_ERR,
                    "V3 demo only supports python_3, typescript, csharp, c, cpp, java, rust",
                ))
            }
        };
        let scan = match scan_res { Ok(s) => s, Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e))) };
        // Assemble MIR
        let asm = MirAssemblerV3;
        let mir = asm.assemble(content, &scan.regions).map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Parse error: {:?}", e)))?;
        // Select expander
        let mut out_text = match lang {
            TargetLanguage::Python3 => {
                let exp = PyExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::TypeScript => {
                let exp = TsExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::CSharp => {
                let exp = CExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::C => {
                let exp = CExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::Cpp => {
                let exp = CppExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::Java => {
                let exp = JavaExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            TargetLanguage::Rust => {
                let exp = RustExpanderV3;
                let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
                SplicerV3.splice(content, &scan.regions, &exps)
                    .text
            }
            _ => unreachable!(),
        };
        if std::env::var("FRAME_MAP_TRAILER").ok().as_deref() == Some("1") {
            // rebuild splice to include map
            let exps: Vec<String> = match lang {
                TargetLanguage::Python3 => mir.iter().map(|m| PyExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::TypeScript => mir.iter().map(|m| TsExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::CSharp => mir.iter().map(|m| CExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::C => mir.iter().map(|m| CExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::Cpp => mir.iter().map(|m| CppExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::Java => mir.iter().map(|m| JavaExpanderV3.expand(m, 0)).collect(),
                TargetLanguage::Rust => mir.iter().map(|m| RustExpanderV3.expand(m, 0)).collect(),
                _ => vec![],
            };
            let sp = SplicerV3.splice(content, &scan.regions, &exps);
            let trailer = sp.build_trailer_json();
            out_text.push_str("\n/*#frame-map#\n");
            out_text.push_str(&trailer);
            out_text.push_str("\n#frame-map#*/\n");
        }
        Ok(out_text)
    }

    pub fn compile_multifile_unsupported() -> Result<String, RunError> {
        Err(RunError::new(
            frame_exitcode::PARSE_ERR,
            "Multi-file build is temporarily unavailable during V3 rebuild",
        ))
    }
}

// keep single import set at top of file

pub fn validate_single_body(content_str: &str, target_language: Option<TargetLanguage>) -> Result<ValidationResultV3, RunError> {
    let content = content_str.as_bytes();
    if content.first().copied() != Some(b'{') { return Err(RunError::new(frame_exitcode::PARSE_ERR, "V3 demo expects body starting at '{'")); }
    let lang = target_language.unwrap_or(TargetLanguage::Python3);
    let scan_res = match lang {
        TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(content, 0),
        TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(content, 0),
        TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(content, 0),
        TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(content, 0),
        TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(content, 0),
        TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(content, 0),
        TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(content, 0),
        _ => return Err(RunError::new(frame_exitcode::PARSE_ERR, "target not supported in V3 demo")),
    };
    let scan = match scan_res { Ok(s) => s, Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e))) };
    let asm = MirAssemblerV3; let mir = asm.assemble(content, &scan.regions).map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Parse error: {:?}", e)))?;
    let res = ValidatorV3.validate_regions_mir(&scan.regions, &mir);
    Ok(res)
}

pub fn compile_module_demo(content_str: &str, lang: TargetLanguage) -> Result<String, RunError> {
    // Partition file into bodies and rewrite each body via single-body pipeline
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let mut out = String::new();
    let mut cursor = 0usize;
    for b in parts.bodies {
        if b.open_byte > cursor { out.push_str(&content_str[cursor..b.open_byte]); }
        let body_src = &content_str[b.open_byte..b.close_byte+1];
        let body_out = CompilerV3::compile_single_file(None, body_src, Some(lang), false)?;
        out.push_str(&body_out);
        cursor = b.close_byte + 1;
    }
    if cursor < bytes.len() { out.push_str(&content_str[cursor..]); }
    Ok(out)
}

pub fn validate_module_demo(content_str: &str, lang: TargetLanguage) -> Result<ValidationResultV3, RunError> {
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let validator = ValidatorV3;
    let mut all_issues = Vec::new();
    for b in parts.bodies {
        let body_bytes = &bytes[b.open_byte..=b.close_byte];
        // scan and assemble
        let scan_res = match lang {
            TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_bytes, 0),
            TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_bytes, 0),
            TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_bytes, 0),
            TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_bytes, 0),
            TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_bytes, 0),
            TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_bytes, 0),
            TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_bytes, 0),
            _ => return Err(RunError::new(frame_exitcode::PARSE_ERR, "target not supported in V3 demo")),
        };
        let scan = match scan_res { Ok(s) => s, Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e))) };
        let mir = MirAssemblerV3.assemble(body_bytes, &scan.regions).map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Parse error: {:?}", e)))?;
        let policy = ValidatorPolicyV3 { body_kind: Some(b.kind) };
        let res = validator.validate_regions_mir_with_policy(&scan.regions, &mir, policy);
        all_issues.extend(res.issues);
    }
    Ok(ValidationResultV3 { ok: all_issues.is_empty(), issues: all_issues })
}
