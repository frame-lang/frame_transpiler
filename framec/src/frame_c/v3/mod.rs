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
pub mod outline_scanner;
pub mod facade;
// future: pub mod import_validator;

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
        // Build expansions aligned with region indents
        let mut out_text = String::new();
        let exps: Vec<String> = {
            let mut v = Vec::new();
            let mut mi = 0usize;
            for r in &scan.regions {
                if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                    let m = &mir[mi];
                    mi += 1;
                    let s = match lang {
                        TargetLanguage::Python3 => PyExpanderV3.expand(m, *indent, None),
                        TargetLanguage::TypeScript => TsExpanderV3.expand(m, *indent, None),
                        TargetLanguage::CSharp => CExpanderV3.expand(m, *indent, None),
                        TargetLanguage::C => CExpanderV3.expand(m, *indent, None),
                        TargetLanguage::Cpp => CppExpanderV3.expand(m, *indent, None),
                        TargetLanguage::Java => JavaExpanderV3.expand(m, *indent, None),
                        TargetLanguage::Rust => RustExpanderV3.expand(m, *indent, None),
                        _ => String::new(),
                    };
                    v.push(s);
                }
            }
            v
        };
        out_text = SplicerV3.splice(content, &scan.regions, &exps).text;
        if std::env::var("FRAME_MAP_TRAILER").ok().as_deref() == Some("1") {
            // rebuild splice to include map
            let exps: Vec<String> = match lang {
                TargetLanguage::Python3 => mir.iter().map(|m| PyExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::TypeScript => mir.iter().map(|m| TsExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::CSharp => mir.iter().map(|m| CExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::C => mir.iter().map(|m| CExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::Cpp => mir.iter().map(|m| CppExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::Java => mir.iter().map(|m| JavaExpanderV3.expand(m, 0, None)).collect(),
                TargetLanguage::Rust => mir.iter().map(|m| RustExpanderV3.expand(m, 0, None)).collect(),
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
    let mut res = ValidatorV3.validate_regions_mir(&scan.regions, &mir);
    // Also enforce no native text after terminal MIR
    let extra = ValidatorV3.validate_terminal_last_native(content, &scan.regions, &mir, target_language.unwrap_or(TargetLanguage::Python3));
    res.issues.extend(extra);
    res.ok = res.issues.is_empty();
    Ok(res)
}

pub fn compile_module_demo(content_str: &str, lang: TargetLanguage) -> Result<String, RunError> {
    // Partition file into bodies and rewrite each body via single-body pipeline
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let system_name = find_system_name(bytes, 0);
    let emit_body_only = std::env::var("FRAME_EMIT_BODY_ONLY").ok().as_deref() == Some("1");
    let emit_exec = std::env::var("FRAME_EMIT_EXEC").ok().as_deref() == Some("1");
    let mut out = String::new();
    let mut body_chunks: Vec<String> = Vec::new();
    let mut cursor = 0usize;
    for b in parts.bodies {
        if b.open_byte > cursor { out.push_str(&content_str[cursor..b.open_byte]); }
        let body_src = &content_str[b.open_byte..b.close_byte+1];
        // Allow facade-mode expansions in compile path for smoke testing via env flag
        let facade_mode = std::env::var("FRAME_FACADE_EXPANSION").ok().as_deref() == Some("1");
        let body_out = if facade_mode {
            if std::env::var("FRAME_DEBUG_FACADE").ok().as_deref() == Some("1") { eprintln!("[facade-compile] lang={:?}", lang); }
            // Re-scan/assemble to build wrapper-call expansions and splice
            let scan = match lang {
                TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_src.as_bytes(), 0),
                _ => nscan::python::NativeRegionScannerPyV3.scan(body_src.as_bytes(), 0)
            }.map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e)))?;
            let mir = MirAssemblerV3.assemble(body_src.as_bytes(), &scan.regions).map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Parse error: {:?}", e)))?;
            let exps: Vec<String> = {
                use crate::frame_c::v3::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                        let m = &mir[mi]; mi += 1;
                        let s = match lang {
                            TargetLanguage::Python3 => PyFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::TypeScript => TsFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::CSharp => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::C => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Cpp => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Java => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Rust => RustFacadeExpanderV3.expand(m, *indent, None),
                            _ => String::new(),
                        };
                        if std::env::var("FRAME_DEBUG_FACADE").ok().as_deref() == Some("1") {
                            let kind = match m { crate::frame_c::v3::mir::MirItemV3::Transition{..} => "Transition", crate::frame_c::v3::mir::MirItemV3::Forward{..} => "Forward", crate::frame_c::v3::mir::MirItemV3::StackPush{..} => "StackPush", crate::frame_c::v3::mir::MirItemV3::StackPop{..} => "StackPop" };
                            eprintln!("[facade-compile] MIR -> {} exp_len={} preview={:?}", kind, s.len(), if s.len()>60 { &s[..60] } else { &s });
                        }
                        v.push(s);
                    }
                }
                if std::env::var("FRAME_DEBUG_FACADE").ok().as_deref() == Some("1") {
                    eprintln!("[facade-compile] regions={} mir={} exps={}", scan.regions.len(), mir.len(), v.len());
                    for (idx, r) in scan.regions.iter().enumerate() {
                        match r { crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ kind, .. } => eprintln!("[facade-compile] region[{idx}] = Frame({:?})", kind), _ => {} }
                    }
                    for (i, s) in v.iter().enumerate() {
                        let preview = if s.len() > 60 { &s[..60] } else { &s }; eprintln!("[facade-compile] exp[{i}] = {:?}", preview);
                    }
                }
                v
            };
            let spliced = SplicerV3.splice(body_src.as_bytes(), &scan.regions, &exps);
            if std::env::var("FRAME_DEBUG_FACADE").ok().as_deref() == Some("1") {
                eprintln!(
                    "[facade-compile] spliced_len={} has_transition={} has_forward={} has_stack={}",
                    spliced.text.len(),
                    spliced.text.contains("__frame_transition"),
                    spliced.text.contains("__frame_forward"),
                    spliced.text.contains("__frame_stack_")
                );
            }
            spliced.text
        } else {
            // Production-style expansions for Python/TypeScript; comment-only for others
            let scan = match lang {
                TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_src.as_bytes(), 0),
                TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_src.as_bytes(), 0),
                _ => nscan::python::NativeRegionScannerPyV3.scan(body_src.as_bytes(), 0)
            }.map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e)))?;
            let mir = MirAssemblerV3.assemble(body_src.as_bytes(), &scan.regions).map_err(|e| RunError::new(frame_exitcode::PARSE_ERR, &format!("Parse error: {:?}", e)))?;
            let sys_ctx = system_name.as_deref();
            let exps: Vec<String> = {
                use crate::frame_c::v3::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                        if mi >= mir.len() { break; }
                        let m = &mir[mi]; mi += 1;
                        let s = match lang {
                            TargetLanguage::Python3 => PyExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::TypeScript => TsExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::CSharp => CExpanderV3.expand(m, *indent, None),
                            TargetLanguage::C => CExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Cpp => CppExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Java => JavaExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Rust => RustExpanderV3.expand(m, *indent, None),
                            _ => String::new(),
                        };
                        v.push(s);
                    }
                }
                v
            };
            SplicerV3.splice(body_src.as_bytes(), &scan.regions, &exps).text
        };
        if emit_body_only || emit_exec {
            body_chunks.push(body_out);
        } else {
            out.push_str(&body_out);
        }
        cursor = b.close_byte + 1;
    }
    if emit_exec {
        // Build a minimal executable wrapper for Python/TypeScript using the first body
        let body = body_chunks.get(0).cloned().unwrap_or_default();
        let program = match lang {
            TargetLanguage::Python3 => {
                let mut p = String::new();
                // Use repository runtime rather than inlining primitives
                p.push_str("from frame_runtime_py import FrameEvent, FrameCompartment\n\n");
                p.push_str("class M:\n    def __init__(self):\n        self._compartment = FrameCompartment('__S_state_A')\n    def _frame_transition(self, next_compartment):\n        self._compartment = next_compartment\n    def _frame_router(self, __e, compartment=None):\n        pass\n    def _frame_stack_push(self):\n        pass\n    def _frame_stack_pop(self):\n        pass\n");
                p.push_str("def native():\n    pass\n\n");
                p.push_str("def handler(self, __e, compartment):\n");
                for line in body.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.is_empty() { continue; }
                    p.push_str("    "); p.push_str(trimmed); p.push('\n');
                }
                p.push_str("\nif __name__ == '__main__':\n    m=M()\n    handler(m, FrameEvent('e'), m._compartment)\n");
                p
            }
            TargetLanguage::TypeScript => {
                let mut p = String::new();
                p.push_str("class FrameEvent { constructor(public message: string, public parameters: any|null) {} }\n");
                p.push_str("class FrameCompartment { constructor(public state: string) {} public forwardEvent: FrameEvent|null=null; public exitArgs: any=null; public enterArgs: any=null; public parentCompartment: FrameCompartment|null=null; public stateArgs: any=null; }\n");
                p.push_str("class M { public _compartment: FrameCompartment = new FrameCompartment('__S_state_A'); _frame_transition(n: FrameCompartment){ this._compartment=n; } _frame_router(__e: FrameEvent, c?: FrameCompartment){ } _frame_stack_push(){} _frame_stack_pop(){} }\n");
                p.push_str("function native(): void {}\n\n");
                p.push_str("function handler(self: M, __e: FrameEvent, compartment: FrameCompartment) {\n");
                for line in body.lines() {
                    let mut s = line.to_string();
                    if !(s.ends_with(';') || s.ends_with('{') || s.ends_with('}')) { s.push(';'); }
                    p.push_str("    "); p.push_str(&s); p.push('\n');
                }
                p.push_str("}\n(function(){ const m=new M(); handler(m, new FrameEvent('e', null), m._compartment); })();\n");
                p
            }
            _ => body,
        };
        return Ok(program);
    }
    if emit_body_only {
        // Concatenate only the spliced/expanded bodies
        let joined = body_chunks.join("\n");
        Ok(joined)
    } else {
        if cursor < bytes.len() { out.push_str(&content_str[cursor..]); }
        Ok(out)
    }
}

pub fn validate_module_demo(content_str: &str, lang: TargetLanguage) -> Result<ValidationResultV3, RunError> {
    validate_module_demo_with_mode(content_str, lang, false)
}

pub fn validate_module_demo_with_mode(content_str: &str, lang: TargetLanguage, strict_native: bool) -> Result<ValidationResultV3, RunError> {
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let validator = ValidatorV3;
    let mut all_issues = Vec::new();
    // include import scanning issues
    all_issues.extend(parts.import_issues.into_iter());
    // Outer grammar: re-scan outline and enforce section placement
    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
    let mut known_states = std::collections::HashSet::new();
    let mut system_name: Option<String> = None;
    {
        let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
        all_issues.extend(outline_issues);
        let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
        all_issues.extend(outer_issues);
        // machine section: simple state header check for '{'
        let state_issues = validator.validate_machine_state_headers(bytes, outline_start);
        all_issues.extend(state_issues);
        // Collect known state names from machine sections
        known_states = validator.collect_machine_state_names(bytes, outline_start);
        // Best-effort scan for system name
        system_name = find_system_name(bytes, 0);
    }
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
        let (mir, parse_issues) = MirAssemblerV3.assemble_collect(body_bytes, &scan.regions);
        if !parse_issues.is_empty() { all_issues.extend(parse_issues); }
        let policy = ValidatorPolicyV3 { body_kind: Some(b.kind) };
        let mut res = validator.validate_regions_mir_with_policy(&scan.regions, &mir, policy);
        // Validate that transition targets refer to known states (coarse file-level set)
        if !known_states.is_empty() {
            res.issues.extend(validator.validate_transition_targets(&mir, &known_states));
        }
        // If no parent relationship exists anywhere in the machine, flag any use of parent forward.
        // Only enforce when a machine: section exists (single-body demos are exempt).
        if validator.has_machine_section(bytes, outline_start) {
            let has_parent = validator.has_any_parent_relationship(bytes, outline_start);
            if !has_parent {
                if mir.iter().any(|m| matches!(m, crate::frame_c::v3::mir::MirItemV3::Forward { .. })) {
                    all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E403: Cannot forward to parent: no parent available".into() });
                }
            }
        }
        // Enforce no native after terminal MIR at body level
        let extra = validator.validate_terminal_last_native(body_bytes, &scan.regions, &mir, lang);
        res.issues.extend(extra);
        res.ok = res.issues.is_empty();
        all_issues.extend(res.issues);

        // Stage 07 (strict facade mode): build spliced body with wrapper-call expansions
        if strict_native {
            let exps: Vec<String> = {
                use crate::frame_c::v3::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                        if mi >= mir.len() { break; }
                        let m = &mir[mi];
                        mi += 1;
                        let s = match lang {
                            TargetLanguage::Python3 => PyFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::TypeScript => TsFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::CSharp => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::C => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Cpp => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Java => CFacadeExpanderV3.expand(m, *indent, None),
                            TargetLanguage::Rust => RustFacadeExpanderV3.expand(m, *indent, None),
                            _ => String::new(),
                        };
                        v.push(s);
                    }
                }
                v
            };
            let spliced = SplicerV3.splice(body_bytes, &scan.regions, &exps);
            // Optional native parsing via facades (none registered by default)
            if let Some(facade) = crate::frame_c::v3::facade::NativeFacadeRegistryV3::get(lang) {
                if let Ok(diags) = facade.parse(&spliced.text) {
                    for d in diags {
                        if let Some((origin, src)) = spliced.map_spliced_range_to_origin(d.start, d.end) {
                            let origin_str = match origin { crate::frame_c::v3::splice::OriginV3::Frame{..} => "frame", crate::frame_c::v3::splice::OriginV3::Native{..} => "native" };
                            all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("native syntax ({}:{}-{}): {}", origin_str, src.start, src.end, d.message) });
                        } else {
                            all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("native syntax: {}", d.message) });
                        }
                    }
                }
            }
        }
    }
    let ok = all_issues.is_empty();
    if strict_native && !ok {
        // In strict/native mode, surface native diagnostics as a failing status for callers that want to gate on facades
        return Err(RunError::new(exitcode::DATAERR, "native facade validation failed"));
    }
    Ok(ValidationResultV3 { ok, issues: all_issues })
}

// SOL-anchored scan for `system <Ident> {` ignoring common comments
fn find_system_name(bytes: &[u8], start: usize) -> Option<String> {
    let n = bytes.len();
    let mut i = start;
    while i < n {
        // skip whitespace
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
        if i >= n { break; }
        // skip line comments
        if bytes[i] == b'/' && i+1 < n && bytes[i+1] == b'/' { while i < n && bytes[i] != b'\n' { i += 1; } continue; }
        if bytes[i] == b'#' { while i < n && bytes[i] != b'\n' { i += 1; } continue; }
        // skip block comments
        if bytes[i] == b'/' && i+1 < n && bytes[i+1] == b'*' {
            i += 2; while i+1 < n && !(bytes[i] == b'*' && bytes[i+1] == b'/') { i += 1; } if i+1 < n { i += 2; } continue;
        }
        // read ident
        let mut j = i;
        while j < n && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
        if j > i {
            let kw = String::from_utf8_lossy(&bytes[i..j]).to_ascii_lowercase();
            if kw == "system" {
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                let name_start = j;
                while j < n && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
                if j > name_start {
                    return Some(String::from_utf8_lossy(&bytes[name_start..j]).to_string());
                }
            }
        }
        while i < n && bytes[i] != b'\n' { i += 1; }
    }
    None
}
