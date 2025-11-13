use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::native_region_scanner as nscan;
use crate::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;
use crate::frame_c::v3::mir_assembler::MirAssemblerV3;
use crate::frame_c::v3::expander::{FrameStatementExpanderV3, PyExpanderV3, TsExpanderV3, CExpanderV3, CppExpanderV3, JavaExpanderV3, RustExpanderV3};
use crate::frame_c::v3::splice::SplicerV3;
use crate::frame_c::v3::validator::{ValidatorV3, ValidationResultV3, ValidatorPolicyV3, BodyKindV3};

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
pub mod ast;
pub mod arcanum;
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
    let scan = match scan_res {
        Ok(s) => s,
        Err(e) => {
            // Map protected-region close errors to structured validation issues for single-body demo
            let mut issues: Vec<crate::frame_c::v3::validator::ValidationIssueV3> = Vec::new();
            let msg = e.message.to_lowercase();
            if msg.contains("unterminated comment") {
                issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E106: unterminated comment".into() });
            } else if (msg.contains("unterminated") && msg.contains("string")) || msg.contains("unterminated raw") || msg.contains("unterminated verbatim") || msg.contains("unterminated interp") {
                issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E100: unterminated string".into() });
            } else if msg.contains("body not closed") {
                issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E103: unterminated body".into() });
            }
            if !issues.is_empty() {
                return Ok(crate::frame_c::v3::validator::ValidationResultV3 { ok: false, issues });
            }
            return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e)));
        }
    };
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
    let mut frameful_chunks: Vec<(bool, String)> = Vec::new();
    let mut exec_body_src: Option<String> = None;
    let mut exec_mir: Option<Vec<crate::frame_c::v3::mir::MirItemV3>> = None;
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
                            TargetLanguage::CSharp => CExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::C => CExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::Cpp => CppExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::Java => JavaExpanderV3.expand(m, *indent, sys_ctx),
                            TargetLanguage::Rust => RustExpanderV3.expand(m, *indent, sys_ctx),
                            _ => String::new(),
                        };
                        v.push(s);
                    }
                }
                v
            };
            let spliced = SplicerV3.splice(body_src.as_bytes(), &scan.regions, &exps).text;
            let has_frames = !exps.is_empty();
            if has_frames && exec_body_src.is_none() {
                exec_body_src = Some(body_src.to_string());
            }
            if has_frames && exec_mir.is_none() {
                exec_mir = Some(mir.clone());
            }
            frameful_chunks.push((has_frames, spliced.clone()));
            spliced
        };
        if emit_body_only || emit_exec {
            body_chunks.push(body_out);
        } else {
            out.push_str(&body_out);
        }
        cursor = b.close_byte + 1;
    }
    if emit_exec {
        // Build a minimal executable wrapper for Python/TypeScript using the first frameful body
        let body = frameful_chunks.iter().find(|(has, _)| *has).map(|(_, s)| s.clone()).or_else(|| body_chunks.get(0).cloned()).unwrap_or_default();
        let program = match lang {
            TargetLanguage::Python3 => {
                let mut p = String::new();
                // Use repository runtime rather than inlining primitives
                p.push_str("from frame_runtime_py import FrameEvent, FrameCompartment\n\n");
                p.push_str("class M:\n    def __init__(self):\n        self._compartment = FrameCompartment('__S_state_A')\n    def _frame_transition(self, next_compartment):\n        self._compartment = next_compartment\n        print(f'TRANSITION:{next_compartment.state}')\n    def _frame_router(self, __e, compartment=None):\n        print('FORWARD:PARENT')\n    def _frame_stack_push(self):\n        print('STACK:PUSH')\n    def _frame_stack_pop(self):\n        print('STACK:POP')\n");
                p.push_str("def native():\n    pass\n\n");
                p.push_str("def handler(self, __e, compartment):\n");
                // Re-emit only production glue with a consistent function indent
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::PyExpanderV3;
                    for m in mirv { let s = PyExpanderV3.expand(m, 4, system_name.as_deref()); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::python::NativeRegionScannerPyV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::PyExpanderV3;
                    for m in &mir { let s = PyExpanderV3.expand(m, 4, system_name.as_deref()); p.push_str(&s); }
                }
                p.push_str("\nif __name__ == '__main__':\n    m=M()\n    handler(m, FrameEvent('e', None), m._compartment)\n");
                p
            }
            TargetLanguage::TypeScript => {
                let mut p = String::new();
                // Use runtime import; allow runner to override with FRAME_TS_EXEC_IMPORT to ensure a resolvable path
                let ts_exec_import = std::env::var("FRAME_TS_EXEC_IMPORT").ok().unwrap_or_else(|| String::from("../../../frame_runtime_ts/index"));
                p.push_str(&format!("import {{ FrameEvent, FrameCompartment }} from '{}'\n", ts_exec_import));
                p.push_str("class M { public _compartment: FrameCompartment = new FrameCompartment('__S_state_A'); _frame_transition(n: FrameCompartment){ this._compartment=n; console.log('TRANSITION:'+n.state); } _frame_router(__e: FrameEvent, c?: FrameCompartment){ console.log('FORWARD:PARENT'); } _frame_stack_push(){ console.log('STACK:PUSH'); } _frame_stack_pop(){ console.log('STACK:POP'); } }\n");
                p.push_str("function native(): void {}\n\n");
                // In exec mode, relax the handler's compartment type to avoid TS type errors on parentCompartment
                p.push_str("function handler(self: M, __e: FrameEvent, compartment: any) {\n");
                for line in body.lines() {
                    let mut s = line.to_string();
                    if !(s.ends_with(';') || s.ends_with('{') || s.ends_with('}')) { s.push(';'); }
                    p.push_str("    "); p.push_str(&s); p.push('\n');
                }
                p.push_str("}\n(async function(){ const m=new M(); handler.call(m, m, new FrameEvent('e', null), m._compartment); })();\n");
                p
            }
            TargetLanguage::Rust => {
                let mut p = String::new();
                p.push_str("#[derive(Default)] struct FrameCompartment<'a>{ state: &'a str, forward_event: Option<()>, exit_args: Option<()>, enter_args: Option<()>, parent_compartment: Option<&'a FrameCompartment<'a>>, state_args: Option<()>, }\n");
                p.push_str("fn __frame_transition(state: &str){ println!(\"TRANSITION:{}\", state); }\n");
                p.push_str("fn __frame_forward(){ println!(\"FORWARD:PARENT\"); }\n");
                p.push_str("fn __frame_stack_push(){ println!(\"STACK:PUSH\"); }\n");
                p.push_str("fn __frame_stack_pop(){ println!(\"STACK:POP\"); }\n");
                p.push_str("fn handler() {\n");
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::RustFacadeExpanderV3;
                    for m in mirv { let s = RustFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::rust::NativeRegionScannerRustV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::RustFacadeExpanderV3;
                    for m in &mir { let s = RustFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                }
                p.push_str("}\nfn main(){ handler(); }\n");
                p
            }
            TargetLanguage::C => {
                let mut p = String::new();
                p.push_str("#include <stddef.h>\n#include <stdio.h>\n\n");
                p.push_str("typedef struct FrameCompartment { const char* state; void* forwardEvent; void* exitArgs; void* enterArgs; struct FrameCompartment* parentCompartment; void* stateArgs; } FrameCompartment;\n");
                p.push_str("static inline FrameCompartment frame_compartment_new(const char* state){ FrameCompartment c = { state, 0, 0, 0, 0, 0 }; return c; }\n");
                p.push_str("void __frame_transition(const char* state) { printf(\"TRANSITION:%s\\n\", state); }\n");
                p.push_str("void __frame_forward(void) { printf(\"FORWARD:PARENT\\n\"); }\n");
                p.push_str("void __frame_stack_push(void) { printf(\"STACK:PUSH\\n\"); }\n");
                p.push_str("void __frame_stack_pop(void) { printf(\"STACK:POP\\n\"); }\n\n");
                p.push_str("void handler(void) {\n");
                p.push_str("    FrameCompartment compartment = frame_compartment_new(\"__S_state_A\");\n");
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::CFacadeExpanderV3;
                    for m in mirv { let s = CFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::c::NativeRegionScannerCV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::CFacadeExpanderV3;
                    for m in &mir { let s = CFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                }
                p.push_str("}\nint main(void){ handler(); return 0; }\n");
                p
            }
            TargetLanguage::Cpp => {
                let mut p = String::new();
                p.push_str("#include <cstddef>\n#include <cstdio>\n\n");
                p.push_str("struct FrameCompartment { const char* state; void* forwardEvent; void* exitArgs; void* enterArgs; FrameCompartment* parentCompartment; void* stateArgs; };\n");
                p.push_str("inline FrameCompartment frame_compartment_new(const char* state){ return FrameCompartment{ state, nullptr, nullptr, nullptr, nullptr, nullptr }; }\n");
                p.push_str("void __frame_transition(const char* state) { std::printf(\"TRANSITION:%s\\n\", state); }\n");
                p.push_str("void __frame_forward() { std::printf(\"FORWARD:PARENT\\n\"); }\n");
                p.push_str("void __frame_stack_push() { std::printf(\"STACK:PUSH\\n\"); }\n");
                p.push_str("void __frame_stack_pop() { std::printf(\"STACK:POP\\n\"); }\n\n");
                p.push_str("void handler() {\n");
                p.push_str("    FrameCompartment compartment = frame_compartment_new(\"__S_state_A\");\n");
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::CppFacadeExpanderV3;
                    for m in mirv { let s = CppFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::cpp::NativeRegionScannerCppV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::CppFacadeExpanderV3;
                    for m in &mir { let s = CppFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                }
                p.push_str("}\nint main(){ handler(); return 0; }\n");
                p
            }
            TargetLanguage::Java => {
                let mut p = String::new();
                p.push_str("public class ExecMain {\n");
                p.push_str("  static class FrameCompartment { String state; Object forwardEvent, exitArgs, enterArgs, stateArgs; FrameCompartment parentCompartment; FrameCompartment(String s){ this.state=s; } }\n");
                p.push_str("  static FrameCompartment compartment = new FrameCompartment(\"__S_state_A\");\n");
                p.push_str("  static void __frame_transition(String state){ System.out.println(\"TRANSITION:\"+state); }\n");
                p.push_str("  static void __frame_forward(){ System.out.println(\"FORWARD:PARENT\"); }\n");
                p.push_str("  static void __frame_stack_push(){ System.out.println(\"STACK:PUSH\"); }\n");
                p.push_str("  static void __frame_stack_pop(){ System.out.println(\"STACK:POP\"); }\n");
                p.push_str("  static void handler(){\n");
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::JavaFacadeExpanderV3;
                    for m in mirv { let s = JavaFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::java::NativeRegionScannerJavaV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::JavaFacadeExpanderV3;
                    for m in &mir { let s = JavaFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                }
                p.push_str("  }\n  public static void main(String[] args){ handler(); }\n}\n");
                p
            }
            TargetLanguage::CSharp => {
                let mut p = String::new();
                p.push_str("using System;\nclass ExecMain {\n");
                p.push_str("  class FrameCompartment { public string state; public object forwardEvent, exitArgs, enterArgs, stateArgs; public FrameCompartment parentCompartment; public FrameCompartment(string s){ state=s; } }\n");
                p.push_str("  static FrameCompartment compartment = new FrameCompartment(\"__S_state_A\");\n");
                p.push_str("  static void __frame_transition(string state){ Console.WriteLine(\"TRANSITION:\"+state); }\n");
                p.push_str("  static void __frame_forward(){ Console.WriteLine(\"FORWARD:PARENT\"); }\n");
                p.push_str("  static void __frame_stack_push(){ Console.WriteLine(\"STACK:PUSH\"); }\n");
                p.push_str("  static void __frame_stack_pop(){ Console.WriteLine(\"STACK:POP\"); }\n");
                p.push_str("  static void handler(){\n");
                if let Some(ref mirv) = exec_mir {
                    use crate::frame_c::v3::expander::CFacadeExpanderV3;
                    for m in mirv { let s = CFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                } else if let Some(src) = exec_body_src.as_ref() {
                    let scan = match nscan::csharp::NativeRegionScannerCsV3.scan(src.as_bytes(), 0) { Ok(s) => s, Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3{ close_byte: src.len().saturating_sub(1), regions: Vec::new() } };
                    let mir = MirAssemblerV3.assemble(src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                    use crate::frame_c::v3::expander::CFacadeExpanderV3;
                    for m in &mir { let s = CFacadeExpanderV3.expand(m, 4, None); p.push_str(&s); }
                }
                p.push_str("  }\n  static void Main(string[] args){ handler(); }\n}\n");
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
    // Partition the module. If partitioning fails due to outline issues (e.g., missing '{' after a header),
    // fall back to a tolerant outline scan to surface structured diagnostics (E-codes) instead of a hard error.
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => {
            // Map body close and prolog errors from the partitioner into structured E-codes where possible.
            let emsg = e.0;
            if emsg.starts_with("prolog error:") {
                // E105: Missing/invalid prolog; ensure a proper validator failure rather than a hard error
                let mut issues = Vec::new();
                let msg = if emsg.contains("NotFirstNonWhitespace") {
                    "E105: expected @target prolog as first non-whitespace token"
                } else if emsg.contains("Missing") {
                    "E105: expected @target <lang> at start of file"
                } else {
                    "E105: invalid @target prolog"
                };
                issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: msg.into() });
                return Ok(ValidationResultV3 { ok: false, issues });
            }
            if emsg.starts_with("body close error:") {
                let mapped = if emsg.contains("UnterminatedComment") || emsg.to_lowercase().contains("unterminated comment") {
                    vec![crate::frame_c::v3::validator::ValidationIssueV3{ message: "E106: unterminated comment".into() }]
                } else if emsg.contains("UnterminatedString") || emsg.to_lowercase().contains("unterminated string") {
                    vec![crate::frame_c::v3::validator::ValidationIssueV3{ message: "E100: unterminated string".into() }]
                } else if emsg.contains("UnmatchedBraces") || emsg.to_lowercase().contains("body not closed") {
                    vec![crate::frame_c::v3::validator::ValidationIssueV3{ message: "E103: unterminated body".into() }]
                } else {
                    Vec::new()
                };
                if !mapped.is_empty() {
                    return Ok(ValidationResultV3 { ok: false, issues: mapped });
                }
            }
            // Tolerant outline scan will collect E111 and similar diagnostics.
            let outline_start = 0usize; // tolerant scan will walk whole file
            let (_items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
            if !outline_issues.is_empty() {
                return Ok(ValidationResultV3 { ok: false, issues: outline_issues });
            } else {
                // If we couldn't recover any diagnostics, return the original partition error
                return Err(RunError::new(frame_exitcode::PARSE_ERR, "module partition error"));
            }
        }
    };
    let validator = ValidatorV3;
    let mut all_issues = Vec::new();
    // include import scanning issues
    all_issues.extend(parts.import_issues.into_iter());
    // Outer grammar: re-scan outline and enforce section placement
    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
    let mut known_states = std::collections::HashSet::new();
    let mut system_name: Option<String> = None;
    // Build Arcanum symbol table from outline
    let mut arcanum_symtab: Option<crate::frame_c::v3::arcanum::Arcanum> = None;
    {
        let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
        all_issues.extend(outline_issues);
        let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
        all_issues.extend(outer_issues);
        // machine section: simple state header check for '{'
        let state_issues = validator.validate_machine_state_headers(bytes, outline_start);
        all_issues.extend(state_issues);
        // handlers must be nested inside a state block in machine:
        let handler_scope_issues = validator.validate_handlers_in_state(&items);
        all_issues.extend(handler_scope_issues);
        // Collect known state names (coarse) and build Arcanum for symbol-precision
        known_states = validator.collect_machine_state_names(bytes, outline_start);
        arcanum_symtab = Some(crate::frame_c::v3::arcanum::build_arcanum_from_outline_bytes(bytes, outline_start));
        // Best-effort scan for system name
        system_name = find_system_name(bytes, 0);
        // Debug hook removed: known_states reporting was temporary for triage
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
        // Validate that transition targets refer to known states (symbol-table first, fallback to coarse set)
        if let Some(ref arc) = arcanum_symtab {
            if !known_states.is_empty() {
                let sys = system_name.as_deref();
                res.issues.extend(validator.validate_transition_targets_arcanum(&mir, arc, sys));
            }
        } else if !known_states.is_empty() {
            res.issues.extend(validator.validate_transition_targets(&mir, &known_states));
        }
        // Parent-forward rule (module demos only): require a parent for the enclosing state
        if validator.has_machine_section(bytes, outline_start) {
            if matches!(b.kind, BodyKindV3::Handler | BodyKindV3::Unknown) {
                if mir.iter().any(|m| matches!(m, crate::frame_c::v3::mir::MirItemV3::Forward { .. })) {
                    let enclosing_state = b.state_id.as_deref();
                    let mut ok_parent = false;
                    if let Some(state_name) = enclosing_state {
                        if let Some(ref arc) = arcanum_symtab {
                            let sys = system_name.as_deref().unwrap_or("_");
                            ok_parent = arc.has_parent(sys, state_name) || arc.has_parent("_", state_name);
                        }
                    }
                    if !ok_parent {
                        all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E403: Cannot forward to parent: no parent available".into() });
                    }
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
