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
pub mod fid;
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
        let spliced = SplicerV3.splice(content, &scan.regions, &exps);
        out_text = spliced.text.clone();

        // If debug_output is requested, emit a structured JSON envelope instead of plain code.
        if _debug_output {
            // Minimal structural validation (collect issues) for inclusion in JSON
            let issues = ValidatorV3
                .validate_terminal_last_native(content, &scan.regions, &mir, _target_language.unwrap_or(TargetLanguage::Python3));

            // Build a compact JSON envelope: { targetLanguage, code, <langAlias>, sourceMap, errors, schemaVersion }
            fn json_escape(s: &str) -> String {
                let mut out = String::with_capacity(s.len() + 16);
                for ch in s.chars() {
                    match ch {
                        '\\' => out.push_str("\\\\"),
                        '"' => out.push_str("\\\""),
                        '\n' => out.push_str("\\n"),
                        '\r' => out.push_str("\\r"),
                        '\t' => out.push_str("\\t"),
                        c if c.is_control() => {
                            use std::fmt::Write as _;
                            let _ = write!(&mut out, "\\u{:04x}", c as u32);
                        }
                        c => out.push(c),
                    }
                }
                out
            }

            let lang = _target_language.unwrap_or(TargetLanguage::Python3);
            let code_escaped = json_escape(&spliced.text);
            let map_json = spliced.build_trailer_json();
            let errors_json = build_errors_json(&issues);
            let lang_alias_key = match lang {
                TargetLanguage::Python3 => "python",
                TargetLanguage::TypeScript => "typescript",
                TargetLanguage::CSharp => "csharp",
                TargetLanguage::C => "c",
                TargetLanguage::Cpp => "cpp",
                TargetLanguage::Java => "java",
                TargetLanguage::Rust => "rust",
                _ => "target",
            };
            let lang_value = match lang {
                TargetLanguage::Python3 => "python_3",
                TargetLanguage::TypeScript => "typescript",
                TargetLanguage::CSharp => "csharp",
                TargetLanguage::C => "c",
                TargetLanguage::Cpp => "cpp",
                TargetLanguage::Java => "java",
                TargetLanguage::Rust => "rust",
                _ => "unknown",
            };
            let mut json = String::new();
            json.push_str("{\"targetLanguage\":\"");
            json.push_str(lang_value);
            json.push_str("\",\"code\":\"");
            json.push_str(&code_escaped);
            json.push_str("\",");
            // language-specific alias for code (backward-compat)
            json.push('"'); json.push_str(lang_alias_key); json.push_str("\":\"");
            json.push_str(&code_escaped);
            json.push_str("\",");
            // sourceMap (already a JSON object)
            json.push_str("\"sourceMap\":");
            json.push_str(&map_json);
            json.push_str(",");
            // errors (already a JSON object: { errors: [...], schemaVersion: 1 })
            // normalize to an array field for top-level by extracting the inner structure
            // We keep the object shape as-is under key "errorsEnvelope" and also expose an array under "errors" for convenience.
            json.push_str("\"errorsEnvelope\":");
            json.push_str(&errors_json);
            // Build a flat errors array by simple extraction (cheap parsing avoided): leave as alias to envelope.errors for now
            json.push_str(",\"errors\":");
            json.push_str(&{
                // naive slice: find errors array
                if let Some(start) = errors_json.find("[") {
                    if let Some(end) = errors_json.rfind("]") {
                        errors_json[start..=end].to_string()
                    } else { "[]".to_string() }
                } else { "[]".to_string() }
            });
            json.push_str(",\"schemaVersion\":1}");
            return Ok(json);
        }

        // Structured error JSON trailer for tools (always emitted in V3 demo paths)
        // Note: Kept unconditional for V3 demo compiles so test infrastructure can assert shape deterministically.
        let issues = ValidatorV3
            .validate_terminal_last_native(content, &scan.regions, &mir, _target_language.unwrap_or(TargetLanguage::Python3));
        let json = build_errors_json(&issues);
        out_text.push_str("\n/*#errors-json#\n");
        out_text.push_str(&json);
        out_text.push_str("\n#errors-json#*/\n");
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
            // Add visitor-style line map trailer (targetLine/sourceLine) for convenience
            let lmap = sp.build_line_map_json(content);
            out_text.push_str("\n/*#visitor-map#\n");
            out_text.push_str(&lmap);
            out_text.push_str("\n#visitor-map#*/\n");
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

fn build_errors_json(issues: &[crate::frame_c::v3::validator::ValidationIssueV3]) -> String {
    // Build { "errors": [ { "code": "E400", "message": "..." }, ... ], "schemaVersion": 1 }
    let mut s = String::from("{\"errors\":[");
    for (i, iss) in issues.iter().enumerate() {
        if i > 0 { s.push(','); }
        let msg = &iss.message;
        // Extract leading code if present: "E###: ..."
        let mut code = "".to_string();
        if let Some((head, _rest)) = msg.split_once(':') {
            if head.starts_with('E') && head.len() >= 4 && head[1..].chars().take(3).all(|c| c.is_ascii_digit()) {
                code = head.to_string();
            }
        }
        s.push_str("{\"code\":");
        if code.is_empty() { s.push_str("null"); } else { s.push('"'); s.push_str(&code); s.push('"'); }
        s.push_str(",\"message\":");
        // naive JSON escape for quotes and backslashes
        let esc = msg.replace('\\', "\\\\").replace('"', "\\\"");
        s.push('"'); s.push_str(&esc); s.push('"');
        s.push('}');
    }
    s.push_str("],\"schemaVersion\":1}");
    s
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
    // Stage 10: Prefer Arcanum-derived system name for expansion context
    let arc_for_ctx = crate::frame_c::v3::arcanum::build_arcanum_from_outline_bytes(bytes, 0);
    let system_name = {
        // pick the first declared system if present; otherwise fallback to textual scan
        if let Some((name, _)) = arc_for_ctx.systems.iter().next() { Some(name.clone()) } else { find_system_name(bytes, 0) }
    };
    let emit_body_only = std::env::var("FRAME_EMIT_BODY_ONLY").ok().as_deref() == Some("1");
    let emit_exec = std::env::var("FRAME_EMIT_EXEC").ok().as_deref() == Some("1");
    let mut out = String::new();
    let mut body_chunks: Vec<String> = Vec::new();
    let mut frameful_chunks: Vec<(bool, String)> = Vec::new();
    let mut exec_body_src: Option<String> = None;
    let mut exec_mir: Option<Vec<crate::frame_c::v3::mir::MirItemV3>> = None;
    let mut cursor = 0usize;
    for b in &parts.bodies {
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
        // Optional mapping and visitor-map trailers for module demo path
        if std::env::var("FRAME_MAP_TRAILER").ok().as_deref() == Some("1") {
            use crate::frame_c::v3::splice::{SplicerV3 as _SplicerV3, OriginV3};
            use crate::frame_c::v3::native_region_scanner::RegionSpan;
            // Rebuild splice maps per-body and merge into module-target offsets
            let mut module_map: Vec<(RegionSpan, OriginV3)> = Vec::new();
            let mut visitor: Vec<(usize, usize, &'static str)> = Vec::new();
            // Helper to compute line number from byte offset
            fn offset_to_line(s: &str, off: usize) -> usize {
                let bytes = s.as_bytes(); let mut i=0usize; let mut line=1usize; while i < bytes.len() && i < off { if bytes[i]==b'\n' { line+=1; } i+=1; } line
            }
            let mut out_offset = 0usize;
            let mut cur = 0usize;
            for b in &parts.bodies {
                // account for pre-body literal text copied verbatim
                if b.open_byte > cur { out_offset += content_str[cur..b.open_byte].len(); }
                let body_bytes = &bytes[b.open_byte..=b.close_byte];
                // Build expansions (production path) and splice for mapping
                let scan = match lang {
                    TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_bytes, 0),
                    TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_bytes, 0),
                    TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_bytes, 0),
                    TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_bytes, 0),
                    TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_bytes, 0),
                    TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_bytes, 0),
                    TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_bytes, 0),
                    _ => nscan::python::NativeRegionScannerPyV3.scan(body_bytes, 0)
                }.unwrap_or(crate::frame_c::v3::native_region_scanner::ScanResultV3 { close_byte: body_bytes.len().saturating_sub(1), regions: Vec::new() });
                let sys_ctx = system_name.as_deref();
                let (mir, _parse_issues) = MirAssemblerV3.assemble_collect(body_bytes, &scan.regions);
                let exps: Vec<String> = {
                    use crate::frame_c::v3::expander::*; let mut v=Vec::new(); let mut mi=0usize; for r in &scan.regions { if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r { if mi>=mir.len(){break;} let m=&mir[mi]; mi+=1; let s = match lang { TargetLanguage::Python3 => PyExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::TypeScript => TsExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::CSharp => CExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::C => CExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::Cpp => CppExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::Java => JavaExpanderV3.expand(m, *indent, sys_ctx), TargetLanguage::Rust => RustExpanderV3.expand(m, *indent, sys_ctx), _ => String::new(), }; v.push(s); } } v };
                let sp = SplicerV3.splice(body_bytes, &scan.regions, &exps);
                // Merge mapping with module offset
                for (tgt, origin) in &sp.splice_map {
                    module_map.push((RegionSpan{ start: out_offset + tgt.start, end: out_offset + tgt.end }, origin.clone()));
                    if matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript) {
                        // Visitor mapping: targetLine/sourceLine
                        let target_line = offset_to_line(&out, out_offset + tgt.start);
                        let source_line = match origin { OriginV3::Frame{ source } | OriginV3::Native{ source } => { let body_str = std::str::from_utf8(body_bytes).unwrap_or(""); offset_to_line(body_str, source.start) } };
                        let origin_str: &'static str = match origin { OriginV3::Frame{..} => "frame", OriginV3::Native{..} => "native" };
                        visitor.push((target_line, source_line, origin_str));
                    }
                }
                out_offset += sp.text.len();
                cur = b.close_byte + 1;
            }
            // Build trailers
            let mut map_json = String::from("{\"map\":[");
            let mut first=true; for (tgt, origin) in &module_map { if !first { map_json.push(','); } else { first=false; } map_json.push_str(&format!("{{\"targetStart\":{},\"targetEnd\":{},", tgt.start, tgt.end)); match origin { OriginV3::Frame{ source } => map_json.push_str(&format!("\"origin\":\"frame\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end)), OriginV3::Native{ source } => map_json.push_str(&format!("\"origin\":\"native\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end)), } }
            map_json.push_str("] ,\"version\":1,\"schemaVersion\":1}");
            out.push_str("\n/*#frame-map#\n"); out.push_str(&map_json); out.push_str("\n#frame-map#*/\n");
            if matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript) {
                let mut vjson = String::from("{\"mappings\":["); let mut f=true; for (tline, sline, origin) in &visitor { if !f { vjson.push(','); } else { f=false; } vjson.push_str(&format!("{{\"targetLine\":{},\"sourceLine\":{},\"origin\":\"{}\"}}", tline, sline, origin)); } vjson.push_str("] ,\"schemaVersion\":1}");
                out.push_str("\n/*#visitor-map#\n"); out.push_str(&vjson); out.push_str("\n#visitor-map#*/\n");
            }
        }
        // Structured errors JSON trailer for module compile (always for V3 demo)
        {
            // Run validation to collect issues akin to validate_module_demo_with_mode
            let mut issues = Vec::new();
            let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
            let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
            for is in outline_issues { issues.push(is); }
            let validator = ValidatorV3;
            for b in &parts.bodies {
                let body_bytes = &bytes[b.open_byte..=b.close_byte];
                let scan = match lang {
                    TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_bytes, 0),
                    TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_bytes, 0),
                    TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_bytes, 0),
                    TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_bytes, 0),
                    TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_bytes, 0),
                    TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_bytes, 0),
                    TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_bytes, 0),
                    _ => Ok(crate::frame_c::v3::native_region_scanner::ScanResultV3 { close_byte: body_bytes.len().saturating_sub(1), regions: Vec::new() })
                }.unwrap_or(crate::frame_c::v3::native_region_scanner::ScanResultV3 { close_byte: body_bytes.len().saturating_sub(1), regions: Vec::new() });
                let (mir, parse_issues) = MirAssemblerV3.assemble_collect(body_bytes, &scan.regions);
                for is in parse_issues { issues.push(is); }
                let extra = validator.validate_terminal_last_native(body_bytes, &scan.regions, &mir, lang);
                for is in extra { issues.push(is); }

                // Include native parser diagnostics (facade mode) in errors-json for Py/TS/Rust by default
                let enable_native = matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust);
                if enable_native {
                    let exps: Vec<String> = {
                        use crate::frame_c::v3::expander::*;
                        let mut v = Vec::new();
                        let mut mi = 0usize;
                        for r in &scan.regions {
                            if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                                if mi >= mir.len() { break; }
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
                                v.push(s);
                            }
                        }
                        v
                    };
                    let spliced = SplicerV3.splice(body_bytes, &scan.regions, &exps);
                    if let Some(facade) = crate::frame_c::v3::facade::NativeFacadeRegistryV3::get(lang) {
                        if let Ok(diags) = facade.parse(&spliced.text) {
                            for d in diags {
                                if let Some((_origin, src)) = spliced.map_spliced_range_to_origin(d.start, d.end) {
                                    issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("E500: native syntax (frame:{}-{}): {}", src.start, src.end, d.message) });
                                } else {
                                    issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("E500: native syntax: {}", d.message) });
                                }
                            }
                        }
                    }
                }
            }
            let json = build_errors_json(&issues);
            out.push_str("\n/*#errors-json#\n");
            out.push_str(&json);
            out.push_str("\n#errors-json#*/\n");
        }
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

        // Stage 07 (native facade parsing):
        // Enable by default for Python/TypeScript/Rust (hermetic parsers), or when strict_native is requested.
        let enable_native = strict_native || matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust);
        if enable_native {
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
            // Optional native parsing via facades (adapter may no-op if feature not enabled)
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

/// Validate a module file using a pre-built project Arcanum (cross-file symbol table).
/// Mirrors validate_module_demo_with_mode but uses the provided Arcanum for
/// transition target and parent-forward checks.
pub fn validate_module_with_arcanum(
    content_str: &str,
    lang: TargetLanguage,
    arc: &crate::frame_c::v3::arcanum::Arcanum,
    strict_native: bool,
) -> Result<ValidationResultV3, RunError> {
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitionerV3::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let validator = ValidatorV3;
    let mut all_issues = Vec::new();
    // include import scanning issues
    all_issues.extend(parts.import_issues.into_iter());
    // Outline grammar and section checks
    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
    let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
    all_issues.extend(outline_issues);
    let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
    all_issues.extend(outer_issues);
    all_issues.extend(validator.validate_machine_state_headers(bytes, outline_start));
    all_issues.extend(validator.validate_handlers_in_state(&items));

    let system_name = find_system_name(bytes, 0);

    for b in parts.bodies {
        let body_bytes = &bytes[b.open_byte..=b.close_byte];
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
        // Cross-file transition targets
        let sys = system_name.as_deref();
        res.issues.extend(validator.validate_transition_targets_arcanum(&mir, arc, sys));
        // Parent-forward availability via Arcanum
        if validator.has_machine_section(bytes, outline_start) {
            if matches!(b.kind, BodyKindV3::Handler | BodyKindV3::Unknown) {
                if mir.iter().any(|m| matches!(m, crate::frame_c::v3::mir::MirItemV3::Forward { .. })) {
                    let enclosing_state = b.state_id.as_deref();
                    let mut ok_parent = false;
                    if let Some(state_name) = enclosing_state {
                        let sys_name = system_name.as_deref().unwrap_or("_");
                        ok_parent = arc.has_parent(sys_name, state_name) || arc.has_parent("_", state_name);
                    }
                    if !ok_parent {
                        res.issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E403: Cannot forward to parent: no parent available".into() });
                    }
                }
            }
        }
        // No native after terminal MIR
        let extra = validator.validate_terminal_last_native(body_bytes, &scan.regions, &mir, lang);
        res.issues.extend(extra);
        res.ok = res.issues.is_empty();
        all_issues.extend(res.issues);

        // Stage 07 (native facade parsing): enable by default for Python/TS/Rust, or when strict_native is set
        let enable_native = strict_native || matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust);
        if enable_native {
            let exps: Vec<String> = {
                use crate::frame_c::v3::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{ indent, .. } = r {
                        if mi >= mir.len() { break; }
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
                        v.push(s);
                    }
                }
                v
            };
            let spliced = SplicerV3.splice(body_bytes, &scan.regions, &exps);
            if let Some(facade) = crate::frame_c::v3::facade::NativeFacadeRegistryV3::get(lang) {
                if let Ok(diags) = facade.parse(&spliced.text) {
                    for d in diags {
                        if let Some((_origin, src)) = spliced.map_spliced_range_to_origin(d.start, d.end) {
                            all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("native syntax (frame:{}-{}): {}", src.start, src.end, d.message) });
                        } else {
                            all_issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: format!("native syntax: {}", d.message) });
                        }
                    }
                }
            }
        }
    }
    let ok = all_issues.is_empty();
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
