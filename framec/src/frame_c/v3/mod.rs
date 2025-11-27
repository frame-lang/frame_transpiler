use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::native_region_scanner as nscan;
use crate::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use crate::frame_c::v3::mir_assembler::MirAssemblerV3;
use crate::frame_c::v3::expander::{FrameStatementExpanderV3, PyExpanderV3, TsExpanderV3, CExpanderV3, CppExpanderV3, JavaExpanderV3, RustExpanderV3};
use crate::frame_c::v3::splice::SplicerV3;
use crate::frame_c::v3::validator::{ValidatorV3, ValidationResultV3, ValidatorPolicyV3, BodyKindV3};
use crate::frame_c::v3::system_parser::SystemParserV3;
use crate::frame_c::v3::interface_parser::{InterfaceParserV3, InterfaceMethodMeta};

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
pub mod domain_scanner;
pub mod interface_parser;
pub mod system_parser;
pub mod machine_parser;
pub mod native_symbol_snapshot;
pub mod system_param_semantics;
pub mod rust_domain_scanner;
// future: pub mod import_validator;

fn ts_param_idents(params: &str) -> String {
    let mut names: Vec<String> = Vec::new();
    for part in params.split(',') {
        let mut p = part.trim();
        if p.is_empty() {
            continue;
        }
        if p.starts_with("...") {
            p = &p[3..];
        }
        let mut name = String::new();
        for ch in p.chars() {
            if ch.is_whitespace() || ch == ':' || ch == '=' {
                break;
            }
            name.push(ch);
        }
        if !name.is_empty() {
            names.push(name);
        }
    }
    names.join(", ")
}

/// Best-effort scanner for TypeScript class instance fields used in native bodies.
/// Looks for `this.<ident>` patterns inside native regions of handlers/actions/operations
/// and returns the set of candidate field names. This is used to synthesize class
/// field declarations so `tsc` does not report TS2339 for state stored on `this`.
fn collect_ts_field_candidates(
    content: &str,
    parts: &crate::frame_c::v3::module_partitioner::ModulePartitionsV3,
) -> std::collections::HashSet<String> {
    use crate::frame_c::v3::native_region_scanner::RegionV3;
    let mut fields: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in &parts.bodies {
        // Only scan handler/action/operation bodies; ignore functions/unknown for now.
        if !matches!(
            b.kind,
            BodyKindV3::Handler | BodyKindV3::Action | BodyKindV3::Operation
        ) {
            continue;
        }
        if b.close_byte <= b.open_byte || b.close_byte >= content.len() {
            continue;
        }
        let body_src = &content[b.open_byte..=b.close_byte];
        let scan = match nscan::typescript::NativeRegionScannerTsV3.scan(body_src.as_bytes(), 0)
        {
            Ok(s) => s,
            Err(_) => crate::frame_c::v3::native_region_scanner::ScanResultV3 {
                close_byte: body_src.len().saturating_sub(1),
                regions: Vec::new(),
            },
        };
        for r in &scan.regions {
            if let RegionV3::NativeText { span } = r {
                if span.end <= span.start || span.end > body_src.len() {
                    continue;
                }
                let seg = &body_src.as_bytes()[span.start..span.end];
                let mut i = 0usize;
                while i + 5 <= seg.len() {
                    // Look for "this." followed by an identifier.
                    if &seg[i..i + 5] == b"this." {
                        let mut j = i + 5;
                        let mut name = String::new();
                        while j < seg.len() {
                            let ch = seg[j] as char;
                            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                                name.push(ch);
                                j += 1;
                            } else {
                                break;
                            }
                        }
                        if !name.is_empty() {
                            fields.insert(name);
                        }
                        i = j;
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    fields
}

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
        let mut out_text = spliced.text.clone();

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
        if let TargetLanguage::Python3 = lang {
            out_text.push_str("\n'''/*#errors-json#\n");
            out_text.push_str(&json);
            out_text.push_str("\n#errors-json#*/'''\n");
        } else {
            out_text.push_str("\n/*#errors-json#\n");
            out_text.push_str(&json);
            out_text.push_str("\n#errors-json#*/\n");
        }
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
            if let TargetLanguage::Python3 = lang {
                out_text.push_str("\n'''/*#frame-map#\n");
                out_text.push_str(&trailer);
                out_text.push_str("\n#frame-map#*/'''\n");
            } else {
                out_text.push_str("\n/*#frame-map#\n");
                out_text.push_str(&trailer);
                out_text.push_str("\n#frame-map#*/\n");
            }
            // Add visitor-style line map trailer (targetLine/sourceLine) for convenience
            let lmap = sp.build_line_map_json(content);
            if let TargetLanguage::Python3 = lang {
                out_text.push_str("\n'''/*#visitor-map#\n");
                out_text.push_str(&lmap);
                out_text.push_str("\n#visitor-map#*/'''\n");
            } else {
                out_text.push_str("\n/*#visitor-map#\n");
                out_text.push_str(&lmap);
                out_text.push_str("\n#visitor-map#*/\n");
            }
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

pub(crate) fn parse_system_params(bytes: &[u8], sys_name: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    // Returns (start_state_params, enter_event_params, domain_params)
    fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' || b == b'\r' || b == b'\n' }
    fn is_ident_start(b: u8) -> bool { (b as char).is_ascii_alphabetic() || b == b'_' }
    fn is_ident(b: u8) -> bool { (b as char).is_ascii_alphanumeric() || b == b'_' }

    let n = bytes.len();
    let mut i = 0usize;
    while i < n {
        // skip whitespace
        while i < n && is_space(bytes[i]) { i += 1; }
        if i >= n { break; }
        // skip comments
        if bytes[i] == b'#' {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        if i + 1 < n && bytes[i] == b'/' {
            let c2 = bytes[i + 1];
            if c2 == b'/' {
                while i < n && bytes[i] != b'\n' { i += 1; }
                continue;
            } else if c2 == b'*' {
                i += 2;
                while i + 1 < n {
                    if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
                continue;
            }
        }
        // read potential "system" keyword
        let mut j = i;
        while j < n && !is_space(bytes[j]) { j += 1; }
        let token = String::from_utf8_lossy(&bytes[i..j]).to_ascii_lowercase();
        if token != "system" {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        // Found "system"; read name
        i = j;
        while i < n && is_space(bytes[i]) { i += 1; }
        let name_start = i;
        while i < n && is_ident(bytes[i]) { i += 1; }
        if name_start == i {
            continue;
        }
        let name = String::from_utf8_lossy(&bytes[name_start..i]).to_string();
        if name != sys_name {
            // Different system; skip this line and continue
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        // Skip spaces; expect optional '(' for system_params
        while i < n && is_space(bytes[i]) { i += 1; }
        if i >= n || bytes[i] != b'(' {
            return (Vec::new(), Vec::new(), Vec::new());
        }
        i += 1; // after '('
        let mut start_params: Vec<String> = Vec::new();
        let mut enter_params: Vec<String> = Vec::new();
        let mut domain_params: Vec<String> = Vec::new();
        while i < n {
            while i < n && is_space(bytes[i]) { i += 1; }
            if i >= n { break; }
            if bytes[i] == b')' {
                break;
            }
            // Start-state parameter list: "$(p1, p2)"
            if bytes[i] == b'$' && i + 1 < n && bytes[i + 1] == b'(' {
                i += 2; // skip "$("
                let mut k = i;
                while k < n && bytes[k] != b')' {
                    if is_ident_start(bytes[k]) {
                        let ident_start = k;
                        k += 1;
                        while k < n && is_ident(bytes[k]) { k += 1; }
                        let ident = String::from_utf8_lossy(&bytes[ident_start..k]).to_string();
                        start_params.push(ident);
                    } else {
                        k += 1;
                    }
                }
                i = k;
                if i < n && bytes[i] == b')' { i += 1; }
            }
            // Enter-event parameter list: "$>(p1, p2)"
            else if bytes[i] == b'$' && i + 1 < n && bytes[i + 1] == b'>' {
                i += 2;
                while i < n && is_space(bytes[i]) { i += 1; }
                if i < n && bytes[i] == b'(' {
                    i += 1;
                    let mut k = i;
                    while k < n && bytes[k] != b')' {
                        if is_ident_start(bytes[k]) {
                            let ident_start = k;
                            k += 1;
                            while k < n && is_ident(bytes[k]) { k += 1; }
                            let ident = String::from_utf8_lossy(&bytes[ident_start..k]).to_string();
                            enter_params.push(ident);
                        } else {
                            k += 1;
                        }
                    }
                    i = k;
                    if i < n && bytes[i] == b')' { i += 1; }
                }
            }
            // Domain parameter: IDENT at top level
            else if is_ident_start(bytes[i]) {
                let ident_start = i;
                i += 1;
                while i < n && is_ident(bytes[i]) { i += 1; }
                let ident = String::from_utf8_lossy(&bytes[ident_start..i]).to_string();
                domain_params.push(ident);
            } else {
                i += 1;
            }
            // skip to next ',' or ')'
            while i < n && is_space(bytes[i]) { i += 1; }
            if i < n && bytes[i] == b',' {
                i += 1;
                continue;
            }
            if i < n && bytes[i] == b')' {
                break;
            }
        }
        return (start_params, enter_params, domain_params);
    }
    (Vec::new(), Vec::new(), Vec::new())
}

/// Best-effort selection of the textual first state in the given system.
/// Uses Arcanum state spans to choose the earliest state header for a stable
/// start state name instead of hard-coding "A".
fn find_start_state_name(
    arc: &crate::frame_c::v3::arcanum::Arcanum,
    sys_name: &str,
) -> Option<String> {
    let sys = arc.systems.get(sys_name)?;
    let mut best_name: Option<String> = None;
    let mut best_start: Option<usize> = None;
    for mach in sys.machines.values() {
        for st in mach.states.values() {
            match best_start {
                None => {
                    best_start = Some(st.span.start);
                    best_name = Some(st.name.clone());
                }
                Some(cur) => {
                    if st.span.start < cur {
                        best_start = Some(st.span.start);
                        best_name = Some(st.name.clone());
                    }
                }
            }
        }
    }
    best_name
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
        // Fallback: if we didn't capture a frameful MIR earlier (e.g., some non-Py/TS langs),
        // attempt to locate the first body that contains Frame statements and assemble a MIR
        // so exec harnesses can emit wrapper markers consistently across languages.
        if exec_mir.is_none() {
            for b in &parts.bodies {
                let body_src = &content_str[b.open_byte..=b.close_byte];
                let scan = match lang {
                    TargetLanguage::Python3 => nscan::python::NativeRegionScannerPyV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTsV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCsV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::C => nscan::c::NativeRegionScannerCV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCppV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::Java => nscan::java::NativeRegionScannerJavaV3.scan(body_src.as_bytes(), 0),
                    TargetLanguage::Rust => nscan::rust::NativeRegionScannerRustV3.scan(body_src.as_bytes(), 0),
                    _ => Ok(crate::frame_c::v3::native_region_scanner::ScanResultV3 { close_byte: body_src.len().saturating_sub(1), regions: Vec::new() })
                }.unwrap_or(crate::frame_c::v3::native_region_scanner::ScanResultV3 { close_byte: body_src.len().saturating_sub(1), regions: Vec::new() });
                let mir = MirAssemblerV3.assemble(body_src.as_bytes(), &scan.regions).unwrap_or_else(|_| Vec::new());
                if !mir.is_empty() {
                    exec_mir = Some(mir);
                    break;
                }
            }
        }
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
        // Build runnable module code for Python/TypeScript/Rust unless explicitly disabled
        let compile_runtimable = std::env::var("FRAME_COMPILE_RUNTIMABLE").ok().map(|v| v != "0").unwrap_or(true);
        if compile_runtimable {
            match lang {
                TargetLanguage::Python3 => {
                    use std::collections::BTreeMap;
                    let sys_name = system_name.clone().unwrap_or_else(|| String::from("S"));
                    let (start_params, enter_params, domain_params) = parse_system_params(bytes, &sys_name);
                     // Prefer the first declared state as the start state when available.
                    let start_state = find_start_state_name(&arc_for_ctx, &sys_name).unwrap_or_else(|| String::from("A"));
                    let mut module = String::new();
                    module.push_str("from frame_runtime_py import FrameEvent, FrameCompartment\n\n");
                    // Build per-system interface metadata (return initializers) for Python.
                    let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, TargetLanguage::Python3);
                    let iface_parser = crate::frame_c::v3::interface_parser::InterfaceParserV3;
                    let iface_meta_map = iface_parser.collect_method_metadata(bytes, &module_ast, TargetLanguage::Python3);
                    let iface_meta_for_sys = iface_meta_map.get(&sys_name);

                    module.push_str(&format!("class {}:\n", sys_name));
                    // For now, accept arbitrary system parameters but keep semantics simple:
                    // we seed the initial compartment for the first declared state and
                    // thread system parameters into state_args/enter_args/domain fields.
                    module.push_str("    def __init__(self, *sys_params):\n");
                    module.push_str("        start_count = "); module.push_str(&start_params.len().to_string()); module.push_str("\n");
                    module.push_str("        enter_count = "); module.push_str(&enter_params.len().to_string()); module.push_str("\n");
                    module.push_str("        start_args = list(sys_params[0:start_count])\n");
                    module.push_str("        enter_args = list(sys_params[start_count:start_count+enter_count])\n");
                    module.push_str("        domain_args = list(sys_params[start_count+enter_count:])\n");
                    // Build state_args dict keyed by parameter names
                    module.push_str("        state_args = {}\n");
                    for (idx, name) in start_params.iter().enumerate() {
                        module.push_str(&format!("        if len(start_args) > {}: state_args[\"{}\"] = start_args[{}]\n", idx, name, idx));
                    }
                    // Apply domain parameters as attributes when present
                    for (idx, name) in domain_params.iter().enumerate() {
                        module.push_str(&format!("        if len(domain_args) > {}: self.{} = domain_args[{}]\n", idx, name, idx));
                    }
                    module.push_str(&format!("        self._compartment = FrameCompartment(\"__{}_state_{}\", enter_args=enter_args, state_args=state_args)\n", sys_name, start_state));
                    module.push_str("        self._stack = []\n");
                    module.push_str("        self._system_return_stack = []\n");
                    module.push_str("        enter_event = FrameEvent(\"$enter\", enter_args)\n");
                    module.push_str("        self._frame_router(enter_event, self._compartment)\n");
                    module.push_str("    def _frame_transition(self, next_compartment: FrameCompartment):\n");
                    module.push_str("        self._compartment = next_compartment\n");
                    module.push_str("        enter_event = FrameEvent(\"$enter\", getattr(next_compartment, \"enter_args\", None))\n");
                    module.push_str("        self._frame_router(enter_event, next_compartment)\n");
                    module.push_str("    def _frame_router(self, __e: FrameEvent, c: FrameCompartment=None):\n");
                    module.push_str("        compartment = c or self._compartment\n");
                    module.push_str("        msg = getattr(__e, \"_message\", None)\n");
                    module.push_str("        if msg is None:\n");
                    module.push_str("            return\n");
                    module.push_str("        handler = getattr(self, f\"_event_{msg}\", None)\n");
                    module.push_str("        if handler is None:\n");
                    module.push_str("            return\n");
                    module.push_str("        return handler(__e, compartment)\n");
                    module.push_str("    def _frame_stack_push(self):\n");
                    module.push_str("        self._stack.append(self._compartment)\n");
                    module.push_str("    def _frame_stack_pop(self):\n");
                    module.push_str("        if self._stack:\n");
                    module.push_str("            prev = self._stack.pop()\n");
                    module.push_str("            self._frame_transition(prev)\n");
                    // Group handlers by event name (owner_id) and collect actions/operations/functions.
                    let mut handler_groups: BTreeMap<String, Vec<(Option<String>, String, bool)>> = BTreeMap::new();
                    let mut function_defs: Vec<String> = Vec::new();
                    for (idx, b) in parts.bodies.iter().enumerate() {
                        let spliced_full = frameful_chunks.get(idx).map(|(_, s)| s.as_str()).unwrap_or("");
                        let spliced = {
                            let bytes = spliced_full.as_bytes();
                            let mut li = 0usize; let mut ri = bytes.len();
                            while li < ri && bytes[li].is_ascii_whitespace() { li += 1; }
                            while ri > li && bytes[ri-1].is_ascii_whitespace() { ri -= 1; }
                            if li < ri && bytes[li] == b'{' && ri > 0 && bytes[ri-1] == b'}' && li+1 < ri-1 { &spliced_full[li+1..ri-1] } else { spliced_full }
                        };
                        match b.kind {
                            crate::frame_c::v3::validator::BodyKindV3::Handler => {
                                let hname = b.owner_id.as_deref().unwrap_or("handler").to_string();
                                // Detect async handlers from header text (best-effort).
                                let is_async = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    hdr.trim_start().starts_with("async ")
                                } else {
                                    false
                                };
                                handler_groups.entry(hname).or_default().push((b.state_id.clone(), spliced.to_string(), is_async));
                            }
                            crate::frame_c::v3::validator::BodyKindV3::Action => {
                                let aname = b.owner_id.as_deref().unwrap_or("action");
                                // Extract parameter list and async flag from header (best-effort)
                                let (is_async, params) = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    let async_flag = hdr.trim_start().starts_with("async ");
                                    let param_text = if let Some(lp) = hdr.find('(') {
                                        if let Some(rp_rel) = hdr[lp+1..].find(')') {
                                            hdr[lp+1..lp+1+rp_rel].trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() };
                                    (async_flag, param_text)
                                } else { (false, String::new()) };
                                // Helper to build call argument list from params (strip annotations/defaults)
                                fn build_call_args(param_text: &str) -> String {
                                    let mut names: Vec<String> = Vec::new();
                                    for raw in param_text.split(',') {
                                        let t = raw.trim();
                                        if t.is_empty() { continue; }
                                        let mut end = t.len();
                                        if let Some(idx) = t.find('=') { end = end.min(idx); }
                                        if let Some(idx) = t.find(':') { end = end.min(idx); }
                                        let name = t[..end].trim();
                                        if !name.is_empty() {
                                            names.push(name.to_string());
                                        }
                                    }
                                    names.join(", ")
                                }
                                let call_args = build_call_args(&params);

                                // Internal implementation
                                let sig = if params.is_empty() {
                                    if is_async {
                                        format!("    async def _action_{}(self):\n", aname)
                                    } else {
                                        format!("    def _action_{}(self):\n", aname)
                                    }
                                } else {
                                    if is_async {
                                        format!("    async def _action_{}(self, {}):\n", aname, params)
                                    } else {
                                        format!("    def _action_{}(self, {}):\n", aname, params)
                                    }
                                };
                                module.push_str(&sig);
                                let mut min_indent: Option<usize> = None;
                                for ln in spliced.lines() {
                                    if ln.trim().is_empty() { continue; }
                                    let indent = ln.as_bytes().iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
                                }
                                let base = min_indent.unwrap_or(0);
                                let mut emitted_code = false;
                                for ln in spliced.lines() {
                                    let raw = ln.trim_end();
                                    if raw.trim().is_empty() {
                                        module.push_str("        \n");
                                        continue;
                                    }
                                    if raw.trim_start().starts_with('#') {
                                        module.push_str("        ");
                                        module.push_str(raw.trim_start());
                                        module.push('\n');
                                        continue;
                                    }
                                    let mut t = raw.to_string();
                                    if t.contains("system.return") {
                                        t = t.replace(
                                            "system.return",
                                            "self._system_return_stack[-1]",
                                        );
                                    }
                                    let bytes_ln = t.as_bytes();
                                    let indent = bytes_ln.iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    let offset = if indent >= base { base } else { indent };
                                    let content = &t[offset..];
                                    // Preserve relative indentation beyond the base.
                                    let extra = if indent > base {
                                        let extra_bytes = &bytes_ln[base..indent];
                                        std::str::from_utf8(extra_bytes).unwrap_or("")
                                    } else {
                                        ""
                                    };
                                    emitted_code = true;
                                    module.push_str("        ");
                                    module.push_str(extra);
                                    module.push_str(content);
                                    module.push('\n');
                                }
                                if !emitted_code { module.push_str("        pass\n"); }

                                // Public wrapper with FRM action name so call sites like self.log(...)
                                // and self.handle(...) continue to work.
                                let wrapper_sig = if params.is_empty() {
                                    if is_async {
                                        format!("    async def {}(self):\n", aname)
                                    } else {
                                        format!("    def {}(self):\n", aname)
                                    }
                                } else {
                                    if is_async {
                                        format!("    async def {}(self, {}):\n", aname, params)
                                    } else {
                                        format!("    def {}(self, {}):\n", aname, params)
                                    }
                                };
                                module.push_str(&wrapper_sig);
                                if is_async {
                                    if call_args.is_empty() {
                                        module.push_str(&format!("        return await self._action_{}()\n", aname));
                                    } else {
                                        module.push_str(&format!("        return await self._action_{}({})\n", aname, call_args));
                                    }
                                } else {
                                    if call_args.is_empty() {
                                        module.push_str(&format!("        return self._action_{}()\n", aname));
                                    } else {
                                        module.push_str(&format!("        return self._action_{}({})\n", aname, call_args));
                                    }
                                }
                            }
                            crate::frame_c::v3::validator::BodyKindV3::Operation => {
                                let oname = b.owner_id.as_deref().unwrap_or("operation");
                                let (is_async, params) = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    let async_flag = hdr.trim_start().starts_with("async ");
                                    let param_text = if let Some(lp) = hdr.find('(') {
                                        if let Some(rp_rel) = hdr[lp+1..].find(')') {
                                            hdr[lp+1..lp+1+rp_rel].trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() };
                                    (async_flag, param_text)
                                } else { (false, String::new()) };
                                let sig = if params.is_empty() {
                                    if is_async {
                                        format!("    async def _operation_{}(self):\n", oname)
                                    } else {
                                        format!("    def _operation_{}(self):\n", oname)
                                    }
                                } else {
                                    if is_async {
                                        format!("    async def _operation_{}(self, {}):\n", oname, params)
                                    } else {
                                        format!("    def _operation_{}(self, {}):\n", oname, params)
                                    }
                                };
                                module.push_str(&sig);
                                let mut min_indent: Option<usize> = None;
                                for ln in spliced.lines() {
                                    if ln.trim().is_empty() { continue; }
                                    let indent = ln.as_bytes().iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
                                }
                                let base = min_indent.unwrap_or(0);
                                let mut emitted_code = false;
                                for ln in spliced.lines() {
                                    let raw = ln.trim_end();
                                    if raw.trim().is_empty() {
                                        module.push_str("        \n");
                                        continue;
                                    }
                                    if raw.trim_start().starts_with('#') {
                                        module.push_str("        ");
                                        module.push_str(raw.trim_start());
                                        module.push('\n');
                                        continue;
                                    }
                                    let mut t = raw.to_string();
                                    if t.contains("system.return") {
                                        t = t.replace(
                                            "system.return",
                                            "self._system_return_stack[-1]",
                                        );
                                    }
                                    let bytes_ln = t.as_bytes();
                                    let indent = bytes_ln.iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    let offset = if indent >= base { base } else { indent };
                                    let content = &t[offset..];
                                    let extra = if indent > base {
                                        let extra_bytes = &bytes_ln[base..indent];
                                        std::str::from_utf8(extra_bytes).unwrap_or("")
                                    } else {
                                        ""
                                    };
                                    emitted_code = true;
                                    module.push_str("        ");
                                    module.push_str(extra);
                                    module.push_str(content);
                                    module.push('\n');
                                }
                                if !emitted_code { module.push_str("        pass\n"); }
                            }
                            crate::frame_c::v3::validator::BodyKindV3::Function => {
                                let fname = b.owner_id.as_deref().unwrap_or("fn");
                                // Extract async flag and parameter list from header (best-effort).
                                let (is_async, params) = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    let async_flag = hdr.trim_start().starts_with("async ");
                                    let param_text = if let Some(lp) = hdr.find('(') {
                                        if let Some(rp_rel) = hdr[lp+1..].find(')') {
                                            hdr[lp+1..lp+1+rp_rel].trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() };
                                    (async_flag, param_text)
                                } else { (false, String::new()) };
                                let sig = if params.is_empty() {
                                    if is_async {
                                        format!("async def {}():\n", fname)
                                    } else {
                                        format!("def {}():\n", fname)
                                    }
                                } else {
                                    if is_async {
                                        format!("async def {}({}):\n", fname, params)
                                    } else {
                                        format!("def {}({}):\n", fname, params)
                                    }
                                };
                                let mut fun = String::new();
                                fun.push_str(&sig);
                                let mut min_indent: Option<usize> = None;
                                for ln in spliced.lines() {
                                    if ln.trim().is_empty() { continue; }
                                    let indent = ln.as_bytes().iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
                                }
                                let base = min_indent.unwrap_or(0);
                                let mut emitted = false;
                                for ln in spliced.lines() {
                                    let raw = ln.trim_end();
                                    if raw.trim().is_empty() {
                                        fun.push_str("    \n");
                                        continue;
                                    }
                                    if raw.trim_start().starts_with('#') {
                                        fun.push_str("    ");
                                        fun.push_str(raw.trim_start());
                                        fun.push('\n');
                                        continue;
                                    }
                                    let bytes_ln = raw.as_bytes();
                                    let indent = bytes_ln.iter().take_while(|b| **b == b' ' || **b == b'\t').count();
                                    let offset = if indent >= base { base } else { indent };
                                    let content = &raw[offset..];
                                    let extra = if indent > base {
                                        let extra_bytes = &bytes_ln[base..indent];
                                        std::str::from_utf8(extra_bytes).unwrap_or("")
                                    } else {
                                        ""
                                    };
                                    emitted = true;
                                    fun.push_str("    ");
                                    fun.push_str(extra);
                                    fun.push_str(content);
                                    fun.push('\n');
                                }
                                if !emitted {
                                    fun.push_str("    pass\n");
                                }
                                function_defs.push(fun);
                            }
                            _ => {}
                        }
                    }
                    // Helper: emit a Python handler body with normalized indentation while
                    // preserving relative nesting (including nested defs / blocks) and
                    // aligning Frame expansion lines (transitions, forwards, stack ops)
                    // to the logical block they belong to.
                    fn emit_py_handler_body(module: &mut String, body: &str, pad: &str) {
                        // Compute the minimal indent across non-empty lines to serve as
                        // the normalization base for this handler body.
                        let mut min_indent: Option<usize> = None;
                        for ln in body.lines() {
                            if ln.trim().is_empty() {
                                continue;
                            }
                            let indent = ln
                                .as_bytes()
                                .iter()
                                .take_while(|b| **b == b' ' || **b == b'\t')
                                .count();
                            min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
                        }
                        let base = min_indent.unwrap_or(0);
                        let mut has_non_comment = false;
                        // Track the last logical indent so that Frame expansion lines
                        // (e.g., transitions) can align with their surrounding native
                        // statements instead of inheriting any extra indent coming from
                        // the original Frame source. Also track whether the previous
                        // logical line ended with a colon so we can indent the first
                        // statement in that block one level deeper.
                        let mut prev_indent_norm: Option<usize> = None;
                        let mut last_line_ended_with_colon = false;

                        for ln in body.lines() {
                            let raw = ln.trim_end();
                            if raw.trim().is_empty() {
                                module.push_str(pad);
                                module.push('\n');
                                continue;
                            }
                            let mut t = raw.to_string();
                            // Rewrite system.return to top-of-stack access.
                            if t.contains("system.return") {
                                t = t.replace("system.return", "self._system_return_stack[-1]");
                            }
                            let bytes_ln = t.as_bytes();
                            let indent_orig = bytes_ln
                                .iter()
                                .take_while(|b| **b == b' ' || **b == b'\t')
                                .count();
                            let content = &t[indent_orig..];
                            let trimmed = content.trim_start();
                            if trimmed.is_empty() {
                                module.push_str(pad);
                                module.push('\n');
                                continue;
                            }
                            let is_comment = trimmed.starts_with('#');
                            if !is_comment {
                                has_non_comment = true;
                            }
                            // Frame expansion lines emitted by the expander should align
                            // with the surrounding block rather than keep any extra
                            // indentation from the original Frame file.
                            let is_expander = trimmed.starts_with("next_compartment = FrameCompartment(")
                                || trimmed.starts_with("next_compartment.exit_args =")
                                || trimmed.starts_with("next_compartment.enter_args =")
                                || trimmed.starts_with("next_compartment.state_args =")
                                || trimmed.starts_with("self._frame_transition(")
                                || trimmed.starts_with("self._frame_stack_push(")
                                || trimmed.starts_with("self._frame_stack_pop(")
                                || trimmed.starts_with(
                                    "self._frame_router(__e, compartment.parent_compartment)",
                                );
                            // Choose a normalized indent:
                            // - If the previous logical line ended with a colon, the first
                            //   statement in that block must be indented one level deeper
                            //   than the colon line.
                            // - Otherwise, for expansion lines, align to the previous
                            //   logical indent when available; for native lines, keep the
                            //   original indent.
                            // Clamp to at least `base` so expansion lines do not end up
                            // less indented than their block.
                            let mut indent_norm = if last_line_ended_with_colon {
                                prev_indent_norm.unwrap_or(base).saturating_add(4)
                            } else if is_expander {
                                prev_indent_norm.unwrap_or(indent_orig)
                            } else {
                                indent_orig
                            };
                            if indent_norm < base {
                                indent_norm = base;
                            }
                            let extra_width = indent_norm.saturating_sub(base);
                            let extra = " ".repeat(extra_width);

                            // Handler-only sugar: `return expr` => `system.return = expr; return`.
                            if trimmed.starts_with("return ")
                                && !trimmed.starts_with("return:")
                                && trimmed != "return"
                                && trimmed != "return:"
                            {
                                let expr = trimmed["return ".len()..]
                                    .trim_end_matches(':')
                                    .trim_end();
                                if !expr.is_empty() {
                                    module.push_str(pad);
                                    module.push_str(&extra);
                                    module.push_str("self._system_return_stack[-1] = ");
                                    module.push_str(expr);
                                    module.push('\n');
                                    module.push_str(pad);
                                    module.push_str(&extra);
                                    module.push_str("return\n");
                                    prev_indent_norm = Some(indent_norm);
                                    last_line_ended_with_colon = false;
                                    continue;
                                }
                            }

                            module.push_str(pad);
                            module.push_str(&extra);
                            module.push_str(trimmed);
                            module.push('\n');
                            prev_indent_norm = Some(indent_norm);
                            last_line_ended_with_colon = trimmed.ends_with(':');
                        }
                        // If the handler body was comment-only, Python still
                        // requires a statement; emit a pass after the comments.
                        if !has_non_comment {
                            module.push_str(pad);
                            module.push_str("pass\n");
                        }
                    }

                    // Generate event-specific handler methods grouped by state.
                    // Internal handlers are named `_event_<name>` so that the public
                    // interface methods can use the bare name (tick(), status(), …)
                    // without colliding with the router entrypoints.
                    for (hname, entries) in handler_groups.iter() {
                        let any_async = entries.iter().any(|(_, _, is_async)| *is_async);
                        if any_async {
                            module.push_str(&format!("    async def _event_{}(self, __e: FrameEvent, compartment: FrameCompartment):\n", hname));
                        } else {
                            module.push_str(&format!("    def _event_{}(self, __e: FrameEvent, compartment: FrameCompartment):\n", hname));
                        }
                        module.push_str("        c = compartment or self._compartment\n");
                        if entries.len() == 1 && entries[0].0.is_none() {
                            // Single, state-less handler: emit directly under the router without
                            // an explicit state guard.
                            let body = &entries[0].1;
                            emit_py_handler_body(&mut module, body, "        ");
                        } else if entries.len() == 1 {
                            // Single state-qualified handler: emit a single guarded block and
                            // normalize indentation inside that block while preserving relative
                            // nesting (e.g., nested defs / blocks).
                            let (state_id_opt, body, _) = &entries[0];
                            if let Some(state_name) = state_id_opt.as_deref() {
                                let compiled_id = if let Some(sys) = system_name.as_deref() {
                                    format!("__{}_state_{}", sys, state_name)
                                } else {
                                    state_name.to_string()
                                };
                                module.push_str(&format!(
                                    "        if c.state == \"{}\":\n",
                                    compiled_id
                                ));
                                emit_py_handler_body(&mut module, body, "            ");
                            } else {
                                // Defensive fallback: treat as state-less handler if the outline
                                // did not record a state id.
                                let body = &entries[0].1;
                                emit_py_handler_body(&mut module, body, "        ");
                            }
                        } else {
                            let mut first_case = true;
                            for (state_id_opt, body, _) in entries {
                                if let Some(state_name) = state_id_opt.as_deref() {
                                    let compiled_id = if let Some(sys) = system_name.as_deref() {
                                        format!("__{}_state_{}", sys, state_name)
                                    } else {
                                        state_name.to_string()
                                    };
                                    if first_case {
                                        module.push_str(&format!("        if c.state == \"{}\":\n", compiled_id));
                                        first_case = false;
                                    } else {
                                        module.push_str(&format!("        elif c.state == \"{}\":\n", compiled_id));
                                    }
                                } else {
                                    if first_case {
                                        module.push_str("        if True:\n");
                                        first_case = false;
                                    } else {
                                        module.push_str("        else:\n");
                                    }
                                }
                                // Normalize the body under the state guard while
                                // preserving relative nesting and aligning Frame
                                // expansion lines to the logical block.
                                emit_py_handler_body(&mut module, body, "            ");
                            }
                        }
                    }
                    // Public interface wrappers: for each event name we expose a method
                    // that constructs a FrameEvent and routes it through the kernel.
                    for hname in handler_groups.keys() {
                        let meta = iface_meta_for_sys.and_then(|m| m.get(hname));
                        let init_expr_opt = meta.and_then(|m| m.return_init.as_deref());
                        module.push_str(&format!("    def {}(self, *args, **kwargs):\n", hname));
                        if let Some(init_expr) = init_expr_opt {
                            module.push_str(&format!("        __initial = {}\n", init_expr));
                        } else {
                            module.push_str("        __initial = None\n");
                        }
                        module.push_str("        self._system_return_stack.append(__initial)\n");
                        module.push_str(&format!(
                            "        __e = FrameEvent(\"{}\", list(args) if args else None)\n",
                            hname
                        ));
                        module.push_str("        try:\n");
                        module.push_str("            self._frame_router(__e, self._compartment)\n");
                        module.push_str("            return self._system_return_stack[-1]\n");
                        module.push_str("        finally:\n");
                        module.push_str("            self._system_return_stack.pop()\n");
                    }

                    // Append top-level functions (including fn main) after the class.
                    for fun in function_defs {
                        module.push('\n');
                        module.push_str(&fun);
                    }

                    // Post-process to normalize bare `return` indentation immediately
                    // following a Frame transition so that Python does not see an
                    // unexpected indent. We align `return` to the preceding
                    // `self._frame_transition(next_compartment)` line.
                    let mut fixed = String::new();
                    let lines: Vec<&str> = module.lines().collect();
                    for (i, line) in lines.iter().enumerate() {
                        let mut out_line = (*line).to_string();
                        if i > 0 {
                            let trimmed = out_line.trim_start();
                            if trimmed == "return" || trimmed == "return:" {
                                let prev = lines[i - 1];
                                let prev_trimmed = prev.trim_start();
                                if prev_trimmed.starts_with("self._frame_transition(next_compartment)") {
                                    let prev_indent = prev.len().saturating_sub(prev_trimmed.len());
                                    out_line = format!("{}{}", " ".repeat(prev_indent), trimmed);
                                }
                            }
                        }
                        fixed.push_str(&out_line);
                        fixed.push('\n');
                    }
                    out = fixed;
                }
                TargetLanguage::TypeScript => {
                    let sys_name = system_name.clone().unwrap_or_else(|| String::from("S"));
                    // Build per-system interface metadata (return types and initializers)
                    let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, TargetLanguage::TypeScript);
                    let iface_parser = crate::frame_c::v3::interface_parser::InterfaceParserV3;
                    let iface_meta_map = iface_parser.collect_method_metadata(bytes, &module_ast, TargetLanguage::TypeScript);
                    let iface_meta_for_sys = iface_meta_map.get(&sys_name);
                    // Scan native bodies for `this.<field>` usages so we can synthesize
                    // class field declarations for state stored on `this`.
                    let field_candidates = collect_ts_field_candidates(content_str, &parts);
                    let ts_import = std::env::var("FRAME_TS_EXEC_IMPORT").ok().unwrap_or_else(|| String::from("frame_runtime_ts"));
                    let mut module = String::new();
                    module.push_str(&format!("import {{ FrameEvent, FrameCompartment }} from '{}'\n\n", ts_import));
                    module.push_str("export class "); module.push_str(&sys_name); module.push_str(" {\n");
                    // Domain fields (if any) from domain: block
                    let domain_fields = scan_ts_domain_fields(bytes);
                    for (name, ty_opt, init_opt) in domain_fields.iter() {
                        module.push_str("  public ");
                        module.push_str(name);
                        if let Some(ty) = ty_opt.as_ref() {
                            module.push_str(": ");
                            module.push_str(ty);
                        } else {
                            module.push_str(": any");
                        }
                        if let Some(init) = init_opt.as_ref() {
                            if !init.is_empty() {
                                module.push_str(" = ");
                                module.push_str(init);
                            }
                        }
                        module.push_str(";\n");
                    }
                    // System/state parameter metadata from outline: $(...), $>(...), and domain param list.
                    let (start_params, enter_params, domain_params) = parse_system_params(bytes, &sys_name);
                    let start_state = find_start_state_name(&arc_for_ctx, &sys_name).unwrap_or_else(|| String::from("A"));
                    module.push_str(&format!("  public _compartment: FrameCompartment = new FrameCompartment('__{}_state_{}');\n", sys_name, start_state));
                    module.push_str("  private _stack: FrameCompartment[] = [];\n");
                    module.push_str("  private _systemReturnStack: any[] = [];\n");
                    // Constructor: partition system params into start / enter / domain and seed initial compartment.
                    module.push_str("  constructor(...sysParams: any[]) {\n");
                    module.push_str("    const startCount = "); module.push_str(&start_params.len().to_string()); module.push_str(";\n");
                    module.push_str("    const enterCount = "); module.push_str(&enter_params.len().to_string()); module.push_str(";\n");
                    module.push_str("    const startArgs = sysParams.slice(0, startCount);\n");
                    module.push_str("    const enterArgs = sysParams.slice(startCount, startCount + enterCount);\n");
                    module.push_str("    const domainArgs = sysParams.slice(startCount + enterCount);\n");
                    module.push_str("    const stateArgs: any = {};\n");
                    for (idx, name) in start_params.iter().enumerate() {
                        module.push_str(&format!("    if (startArgs.length > {}) stateArgs['{}'] = startArgs[{}];\n", idx, name, idx));
                    }
                    // Apply domain parameters as overrides on matching fields when present.
                    for (idx, name) in domain_params.iter().enumerate() {
                        module.push_str(&format!("    if (domainArgs.length > {}) (this as any).{} = domainArgs[{}];\n", idx, name, idx));
                    }
                    module.push_str(&format!("    this._compartment = new FrameCompartment('__{}_state_{}', enterArgs, undefined, stateArgs);\n", sys_name, start_state));
                    module.push_str("    const enterEvent = new FrameEvent(\"$enter\", enterArgs);\n");
                    module.push_str("    this._frame_router(enterEvent, this._compartment);\n");
                    module.push_str("  }\n");
                    module.push_str("  _frame_transition(n: FrameCompartment){ this._compartment = n; const enterEvent = new FrameEvent(\"$enter\", n.enterArgs); this._frame_router(enterEvent, n); }\n");
                    module.push_str("  _frame_stack_push(){ this._stack.push(this._compartment); }\n");
                    module.push_str("  _frame_stack_pop(){ const prev = this._stack.pop(); if (prev) this._frame_transition(prev); }\n");
                    // Group handlers by interface method name so we emit a single
                    // public method per interface function and dispatch on state.
                    // Tuple: (state_id, body_text, params_text, handler_init_expr)
                    let mut handler_groups: std::collections::BTreeMap<String, Vec<(Option<String>, String, String, Option<String>)>> = std::collections::BTreeMap::new();
                    // Collect actions/operations for later emission.
                    // Tuple: (name, is_async, params_text, body_text, return_type_opt)
                    let mut actions: Vec<(String, bool, String, String, Option<String>)> = Vec::new();
                    let mut operations: Vec<(String, bool, String, String, Option<String>)> = Vec::new();
                    for (idx, b) in parts.bodies.iter().enumerate() {
                        let spliced_full = frameful_chunks.get(idx).map(|(_, s)| s.as_str()).unwrap_or("");
                        let spliced_trimmed = {
                            let bytes = spliced_full.as_bytes();
                            let mut li = 0usize; let mut ri = bytes.len();
                            while li < ri && bytes[li].is_ascii_whitespace() { li += 1; }
                            while ri > li && bytes[ri-1].is_ascii_whitespace() { ri -= 1; }
                            if li < ri && bytes[li] == b'{' && ri > 0 && bytes[ri-1] == b'}' && li+1 < ri-1 {
                                &spliced_full[li+1..ri-1]
                            } else { spliced_full }
                        };
                        match b.kind {
                            crate::frame_c::v3::validator::BodyKindV3::Handler => {
                                let hname = b.owner_id.as_deref().unwrap_or("handler").to_string();
                                let mut params = String::new();
                                let mut handler_init: Option<String> = None;
                                if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    // Reuse interface header parser to extract initializer (and ignore any type).
                                    if let Some((_, meta)) =
                                        crate::frame_c::v3::interface_parser::parse_interface_header_meta(hdr)
                                    {
                                        if let Some(init) = meta.return_init {
                                            if !init.trim().is_empty() {
                                                handler_init = Some(init);
                                            }
                                        }
                                    }
                                    if let Some(lp) = hdr.find('(') {
                                        if let Some(rp_rel) = hdr[lp+1..].find(')') {
                                            params = hdr[lp+1..lp+1+rp_rel].trim().to_string();
                                        }
                                    }
                                }
                                handler_groups
                                    .entry(hname)
                                    .or_default()
                                    .push((b.state_id.clone(), spliced_trimmed.to_string(), params, handler_init));
                            }
                            crate::frame_c::v3::validator::BodyKindV3::Action => {
                                let aname = b.owner_id.as_deref().unwrap_or("action").to_string();
                                let (is_async, params, ret_type_opt) = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    let mut core = hdr.trim_start();
                                    let async_flag = core.starts_with("async ");
                                    if async_flag {
                                        core = core["async ".len()..].trim_start();
                                    }
                                    let param_text = if let Some(lp) = core.find('(') {
                                        if let Some(rp_rel) = core[lp+1..].find(')') {
                                            core[lp+1..lp+1+rp_rel].trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() };
                                    // Reuse interface header parser to extract an optional return type.
                                    let ret_ty = crate::frame_c::v3::interface_parser::parse_interface_header_meta(core)
                                        .and_then(|(_, m)| m.return_type)
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty());
                                    (async_flag, param_text, ret_ty)
                                } else { (false, String::new(), None) };
                                actions.push((aname, is_async, params, spliced_trimmed.to_string(), ret_type_opt));
                            }
                            crate::frame_c::v3::validator::BodyKindV3::Operation => {
                                let oname = b.owner_id.as_deref().unwrap_or("operation").to_string();
                                let (is_async, params, ret_type_opt) = if let Some(hs) = b.header_span {
                                    let hdr = std::str::from_utf8(&bytes[hs.start..hs.end]).unwrap_or("");
                                    let mut core = hdr.trim_start();
                                    let async_flag = core.starts_with("async ");
                                    if async_flag {
                                        core = core["async ".len()..].trim_start();
                                    }
                                    let param_text = if let Some(lp) = core.find('(') {
                                        if let Some(rp_rel) = core[lp+1..].find(')') {
                                            core[lp+1..lp+1+rp_rel].trim().to_string()
                                        } else { String::new() }
                                    } else { String::new() };
                                    let ret_ty = crate::frame_c::v3::interface_parser::parse_interface_header_meta(core)
                                        .and_then(|(_, m)| m.return_type)
                                        .map(|s| s.trim().to_string())
                                        .filter(|s| !s.is_empty());
                                    (async_flag, param_text, ret_ty)
                                } else { (false, String::new(), None) };
                                operations.push((oname, is_async, params, spliced_trimmed.to_string(), ret_type_opt));
                            }
                            _ => {}
                        }
                    }
                    // Track parameter arity per interface method for router calls.
                    let mut router_cases: Vec<(String, usize)> = Vec::new();
                    for (hname, entries) in handler_groups {
                        // Choose a parameter list from any header that declared one.
                        let mut params = String::new();
                        for (_, _, ptxt, _) in &entries {
                            if !ptxt.is_empty() {
                                params = ptxt.clone();
                                break;
                            }
                        }
                        let param_idents = if params.is_empty() { String::new() } else { ts_param_idents(&params) };
                        // Public interface wrapper: Frame-friendly signature (no FrameEvent/FrameCompartment).
                        // Initialize a per-call system.return slot and forward to the router.
                        let meta = iface_meta_for_sys.and_then(|m| m.get(&hname));
                        let ret_type = meta
                            .and_then(|m| m.return_type.as_deref())
                            .unwrap_or("any");
                        let init_expr_opt = meta.and_then(|m| m.return_init.as_deref());

                        if params.is_empty() {
                            module.push_str(&format!("  public {}(): {} {{\n", hname, ret_type));
                        } else {
                            module.push_str(&format!("  public {}({}): {} {{\n", hname, params, ret_type));
                        }
                        if let Some(init_expr) = init_expr_opt {
                            module.push_str(&format!("    const __initial = {};\n", init_expr));
                        } else {
                            module.push_str("    const __initial = undefined;\n");
                        }
                        module.push_str("    this._systemReturnStack.push(__initial);\n");
                        module.push_str("    try {\n");
                        module.push_str(&format!("      const __e = new FrameEvent(\"{}\", null);\n", hname));
                        if param_idents.is_empty() {
                            module.push_str("      this._frame_router(__e, this._compartment);\n");
                        } else {
                            module.push_str(&format!("      this._frame_router(__e, this._compartment, {});\n", param_idents));
                        }
                        module.push_str("      return this._systemReturnStack[this._systemReturnStack.length - 1];\n");
                        module.push_str("    } finally {\n");
                        module.push_str("      this._systemReturnStack.pop();\n");
                        module.push_str("    }\n");
                        module.push_str("  }\n");
                        // Internal event handler: router target with full signature.
                        if params.is_empty() {
                            module.push_str(&format!("  private _event_{}(__e: FrameEvent, compartment: FrameCompartment): void {{\n", hname));
                        } else {
                            module.push_str(&format!("  private _event_{}(__e: FrameEvent, compartment: FrameCompartment, {}): void {{\n", hname, params));
                        }
                        module.push_str("    const c = compartment || this._compartment;\n");
                        // If any handler variant declared a header initializer, apply it to the current system.return slot.
                        let handler_init_expr = entries.iter().find_map(|(_, _, _, init)| init.as_ref());
                        if let Some(init) = handler_init_expr {
                            module.push_str(&format!("    this._systemReturnStack[this._systemReturnStack.length - 1] = {};\n", init));
                        }
                        if entries.len() == 1 && entries[0].0.is_none() {
                            // Single, non-state-qualified handler: inline body directly.
                            let body = &entries[0].1;
                            for line in body.lines() {
                                let raw = line.trim_end();
                                if raw.trim().is_empty() {
                                    module.push_str("    \n");
                                    continue;
                                }
                                let mut t = raw.to_string();
                                // Rewrite system.return to top-of-stack access.
                                if t.contains("system.return") {
                                    t = t.replace(
                                        "system.return",
                                        "this._systemReturnStack[this._systemReturnStack.length - 1]",
                                    );
                                }
                                let trimmed = t.trim_start();
                                // Handler-only sugar: return expr; => system.return = expr; return;
                                if trimmed.starts_with("return ") && !trimmed.starts_with("return;") {
                                    let expr = trimmed["return ".len()..].trim_end_matches(';').trim();
                                    if !expr.is_empty() {
                                        module.push_str("    this._systemReturnStack[this._systemReturnStack.length - 1] = ");
                                        module.push_str(expr);
                                        module.push_str(";\n");
                                        module.push_str("    return;\n");
                                        continue;
                                    }
                                }
                                let needs_sc = !(trimmed.ends_with(';') || trimmed.ends_with('{') || trimmed.ends_with('}'));
                                module.push_str("    ");
                                module.push_str(trimmed);
                                if needs_sc { module.push_str(";"); }
                                module.push('\n');
                            }
                        } else {
                            module.push_str("    switch (c.state) {\n");
                            for (state_id_opt, body, _, _) in entries {
                                if let Some(state_name) = state_id_opt.as_deref() {
                                    let compiled_id = if let Some(sys) = system_name.as_deref() {
                                        format!("__{}_state_{}", sys, state_name)
                                    } else {
                                        state_name.to_string()
                                    };
                                    module.push_str(&format!("      case '{}':\n", compiled_id));
                                } else {
                                    module.push_str("      default:\n");
                                }
                                for line in body.lines() {
                                    let raw = line.trim_end();
                                    if raw.trim().is_empty() {
                                        module.push_str("        \n");
                                        continue;
                                    }
                                    let mut t = raw.to_string();
                                    if t.contains("system.return") {
                                        t = t.replace(
                                            "system.return",
                                            "this._systemReturnStack[this._systemReturnStack.length - 1]",
                                        );
                                    }
                                    let trimmed = t.trim_start();
                                    if trimmed.starts_with("return ") && !trimmed.starts_with("return;") {
                                        let expr = trimmed["return ".len()..].trim_end_matches(';').trim();
                                        if !expr.is_empty() {
                                            module.push_str("        this._systemReturnStack[this._systemReturnStack.length - 1] = ");
                                            module.push_str(expr);
                                            module.push_str(";\n");
                                            module.push_str("        return;\n");
                                            continue;
                                        }
                                    }
                                    let needs_sc = !(trimmed.ends_with(';') || trimmed.ends_with('{') || trimmed.ends_with('}'));
                                    module.push_str("        ");
                                    module.push_str(trimmed);
                                    if needs_sc { module.push_str(";"); }
                                    module.push('\n');
                                }
                                    module.push_str("        break;\n");
                            }
                            module.push_str("    }\n");
                        }
                        module.push_str("  }\n");
                        let arity = if param_idents.is_empty() {
                            0
                        } else {
                            param_idents
                                .split(',')
                                .filter(|s| !s.trim().is_empty())
                                .count()
                        };
                        router_cases.push((hname, arity));
                    }
                    // Router: dispatch on event name to internal handlers and propagate
                    // any return value back to the interface wrapper.
                    module.push_str("  _frame_router(__e: FrameEvent, c?: FrameCompartment, ...args: any[]): any {\n");
                    module.push_str("    const _c = c || this._compartment;\n");
                    module.push_str("    switch (__e.message) {\n");
                    for (hname, arity) in router_cases {
                        if arity == 0 {
                            module.push_str(&format!("      case \"{}\": return this._event_{}(__e, _c);\n", hname, hname));
                        } else {
                            let mut call = format!("      case \"{}\": return this._event_{}(__e, _c", hname, hname);
                            for idx in 0..arity {
                                call.push_str(", args[");
                                call.push_str(&idx.to_string());
                                call.push(']');
                            }
                            call.push_str(");\n");
                            module.push_str(&call);
                        }
                    }
                    module.push_str("      default: return;\n");
                    module.push_str("    }\n");
                    module.push_str("  }\n");
                    // Emit actions as class methods so state bodies can call them.
                    for (aname, is_async, params, body, ret_type_opt) in &actions {
                        let ty = ret_type_opt.as_deref().unwrap_or("any");
                        if *is_async {
                            if params.is_empty() {
                                module.push_str(&format!("  public async {}(): Promise<{}> {{\n", aname, ty));
                            } else {
                                module.push_str(&format!("  public async {}({}): Promise<{}> {{\n", aname, params, ty));
                            }
                        } else {
                            let rty = if ret_type_opt.is_some() { ty } else { "void" };
                            if params.is_empty() {
                                module.push_str(&format!("  public {}(): {} {{\n", aname, rty));
                            } else {
                                module.push_str(&format!("  public {}({}): {} {{\n", aname, params, rty));
                            }
                        }
                        for line in body.lines() {
                            let raw = line.trim_end();
                            if raw.is_empty() {
                                module.push_str("    \n");
                                continue;
                            }
                            let mut t = raw.to_string();
                            if t.contains("system.return") {
                                t = t.replace(
                                    "system.return",
                                    "this._systemReturnStack[this._systemReturnStack.length - 1]",
                                );
                            }
                            let trimmed = t.trim_start();
                            let needs_sc = !(trimmed.ends_with(';') || trimmed.ends_with('{') || trimmed.ends_with('}'));
                            module.push_str("    ");
                            module.push_str(trimmed);
                            if needs_sc { module.push_str(";"); }
                            module.push('\n');
                        }
                        module.push_str("  }\n");
                    }
                    // Emit operations similarly (internal helpers)
                    for (oname, is_async, params, body, ret_type_opt) in &operations {
                        let ty = ret_type_opt.as_deref().unwrap_or("any");
                        if *is_async {
                            if params.is_empty() {
                                module.push_str(&format!("  public async {}(): Promise<{}> {{\n", oname, ty));
                            } else {
                                module.push_str(&format!("  public async {}({}): Promise<{}> {{\n", oname, params, ty));
                            }
                        } else {
                            let rty = if ret_type_opt.is_some() { ty } else { "void" };
                            if params.is_empty() {
                                module.push_str(&format!("  public {}(): {} {{\n", oname, rty));
                            } else {
                                module.push_str(&format!("  public {}({}): {} {{\n", oname, params, rty));
                            }
                        }
                        for line in body.lines() {
                            let raw = line.trim_end();
                            if raw.is_empty() {
                                module.push_str("    \n");
                                continue;
                            }
                            let mut t = raw.to_string();
                            if t.contains("system.return") {
                                t = t.replace(
                                    "system.return",
                                    "this._systemReturnStack[this._systemReturnStack.length - 1]",
                                );
                            }
                            let trimmed = t.trim_start();
                            let needs_sc = !(trimmed.ends_with(';') || trimmed.ends_with('{') || trimmed.ends_with('}'));
                            module.push_str("    ");
                            module.push_str(trimmed);
                            if needs_sc { module.push_str(";"); }
                            module.push('\n');
                        }
                        module.push_str("  }\n");
                    }
                    // Synthesize class fields for native-body state stored on `this.`.
                    let mut reserved: std::collections::HashSet<String> = std::collections::HashSet::new();
                    // Domain fields
                    for (name, _, _) in &domain_fields {
                        reserved.insert(name.clone());
                    }
                    // Internal runtime fields
                    reserved.insert("_compartment".to_string());
                    reserved.insert("_stack".to_string());
                    reserved.insert("_systemReturnStack".to_string());
                    // Interface methods
                    if let Some(iface) = iface_meta_for_sys {
                        for k in iface.keys() {
                            reserved.insert(k.clone());
                        }
                    }
                    // Actions and operations
                    for (aname, _, _, _, _) in &actions {
                        reserved.insert(aname.clone());
                    }
                    for (oname, _, _, _, _) in &operations {
                        reserved.insert(oname.clone());
                    }
                    let mut inferred: Vec<String> = field_candidates
                        .into_iter()
                        .filter(|name| {
                            !reserved.contains(name)
                                && !name.starts_with("_frame_")
                                && !name.starts_with("_event_")
                        })
                        .collect();
                    inferred.sort();
                    inferred.dedup();
                    for name in inferred {
                        module.push_str("  public ");
                        module.push_str(&name);
                        module.push_str(": any;\n");
                    }
                    module.push_str("}\n");
                    out = module;
                }
                TargetLanguage::Rust => {
                    let sys_name = system_name.clone().unwrap_or_else(|| String::from("S"));
                    // Collect per-system interface method metadata so we can derive
                    // a typed return enum and (later) interface wrappers.
                    let module_ast_rust = SystemParserV3::parse_module(bytes, TargetLanguage::Rust);
                    let iface_parser_rust = InterfaceParserV3;
                    let iface_meta_map_rust =
                        iface_parser_rust.collect_method_metadata(bytes, &module_ast_rust, TargetLanguage::Rust);
                    let iface_meta_for_sys_rust = iface_meta_map_rust.get(&sys_name);
                    let has_returns = iface_meta_for_sys_rust.map_or(false, |m| !m.is_empty());
                    let return_enum_name = format!("{}Return", sys_name);
                    // Prefer the first declared state as the start state when available.
                    let start_state = find_start_state_name(&arc_for_ctx, &sys_name).unwrap_or_else(|| String::from("A"));
                    let mut module = String::new();
                    // Minimal event and compartment structs for mapping/debug output and
                    // basic runtime semantics. Derive Default on the compartment so
                    // RustExpanderV3 can use `..Default::default()`.
                    module.push_str("#[derive(Debug, Clone)] struct FrameEvent{ message: String }\n");
                    module.push_str("#[derive(Debug, Clone, Default)] struct FrameCompartment{ state: StateId, forward_event: Option<FrameEvent>, exit_args: Option<()>, enter_args: Option<()>, parent_compartment: Option<*const FrameCompartment>, state_args: Option<()>, }\n");
                    // Domain fields (if any) from a top-level domain: block. For Rust
                    // demo modules we assume a single system per file and emit domain
                    // variables as struct fields with their declared type when present.
                    let domain_fields_rs = crate::frame_c::v3::rust_domain_scanner::scan_rs_domain_fields(bytes);
                    // Helper: emit a Rust handler body, optionally rewriting `system.return`
                    // usage for interface handlers into calls on the per-method setter.
                    fn emit_rs_handler_body(
                        module: &mut String,
                        body: &str,
                        pad: &str,
                        setter_name: Option<&str>,
                    ) {
                        for ln in body.lines() {
                            let raw = ln.trim_end();
                            if raw.trim().is_empty() {
                                module.push_str(pad);
                                module.push('\n');
                                continue;
                            }
                            let t = raw.to_string();
                            let trimmed = t.trim_start();
                            if let Some(method) = setter_name {
                                // Handler-only sugar: `return expr;` => setter call + `return;`.
                                if trimmed.starts_with("return ")
                                    && trimmed.trim_end() != "return;"
                                    && trimmed != "return"
                                {
                                    let expr = trimmed["return ".len()..]
                                        .trim_end_matches(';')
                                        .trim();
                                    if !expr.is_empty() {
                                        module.push_str(pad);
                                        module.push_str("self._set_system_return_for_");
                                        module.push_str(method);
                                        module.push('(');
                                        module.push_str(expr);
                                        module.push_str(");\n");
                                        module.push_str(pad);
                                        module.push_str("return;\n");
                                        continue;
                                    }
                                }
                                // Direct assignment form: `system.return = expr;`.
                                if trimmed.starts_with("system.return") {
                                    if let Some(eq_idx) = trimmed.find('=') {
                                        let expr = trimmed[eq_idx + 1..]
                                            .trim_end_matches(';')
                                            .trim();
                                        if !expr.is_empty() {
                                            module.push_str(pad);
                                            module.push_str("self._set_system_return_for_");
                                            module.push_str(method);
                                            module.push('(');
                                            module.push_str(expr);
                                            module.push_str(");\n");
                                            continue;
                                        }
                                    }
                                }
                            }
                            module.push_str(pad);
                            module.push_str(trimmed);
                            module.push('\n');
                        }
                    }
                    // System struct scaffold: one per Frame system.
                    module.push_str(&format!("struct {} {{\n    compartment: FrameCompartment,\n    _stack: Vec<FrameCompartment>,\n", sys_name));
                    if has_returns {
                        module.push_str(&format!("    _system_return_stack: Vec<{}>,\n", return_enum_name));
                    }
                    for (name, ty_opt, _) in &domain_fields_rs {
                        module.push_str("    ");
                        module.push_str(name);
                        module.push_str(": ");
                        if let Some(ty) = ty_opt.as_ref() {
                            module.push_str(ty);
                        } else {
                            module.push_str("()");
                        }
                        module.push_str(",\n");
                    }
                    module.push_str("}\n\n");
                    // Minimal runtime methods on the system. These provide basic
                    // transition/stack semantics for V3 Rust while remaining compatible
                    // with the existing exec-smoke harnesses (which still use facade
                    // wrapper calls).
                    module.push_str(&format!("impl {} {{\n", sys_name));
                    // Basic constructor: seed the compartment with the start state and an empty stack.
                    module.push_str("    fn new() -> Self {\n");
                    module.push_str("        Self {\n");
                    module.push_str(&format!(
                        "            compartment: FrameCompartment{{ state: StateId::{}, ..Default::default() }},\n",
                        start_state
                    ));
                    module.push_str("            _stack: Vec::new(),\n");
                    if has_returns {
                        module.push_str(&format!(
                            "            _system_return_stack: Vec::<{}>::new(),\n",
                            return_enum_name
                        ));
                    }
                    for (name, _, init_opt) in &domain_fields_rs {
                        module.push_str("            ");
                        module.push_str(name);
                        module.push_str(": ");
                        if let Some(init) = init_opt.as_ref() {
                            if !init.is_empty() {
                                module.push_str(init);
                            } else {
                                module.push_str("Default::default()");
                            }
                        } else {
                            module.push_str("Default::default()");
                        }
                        module.push_str(",\n");
                    }
                    module.push_str("        }\n");
                    module.push_str("    }\n");
                    module.push_str("    fn _frame_transition(&mut self, next: &FrameCompartment){\n");
                    module.push_str("        // Basic transition: update the active state id; other fields remain unchanged for now.\n");
                    module.push_str("        self.compartment.state = next.state;\n");
                    module.push_str("    }\n");
                    module.push_str("    fn _frame_stack_push(&mut self){\n");
                    module.push_str("        self._stack.push(self.compartment.clone());\n");
                    module.push_str("    }\n");
                    module.push_str("    fn _frame_stack_pop(&mut self){\n");
                    module.push_str("        if let Some(prev) = self._stack.pop() {\n");
                    module.push_str("            self._frame_transition(&prev);\n");
                    module.push_str("        }\n");
                    module.push_str("    }\n");
                    module.push_str("}\n\n");
                    // Group handlers by interface name and state so we can emit a single
                    // internal method per interface that dispatches on `self.compartment.state`.
                    use std::collections::BTreeMap;
                    let mut handler_map: BTreeMap<String, Vec<(Option<String>, String)>> = BTreeMap::new();
                    for (idx, b) in parts.bodies.iter().enumerate() {
                        if let crate::frame_c::v3::validator::BodyKindV3::Handler = b.kind {
                            let hname = b.owner_id.as_deref().unwrap_or("handler").to_string();
                            let spliced_full = frameful_chunks
                                .get(idx)
                                .map(|(_, s)| s.as_str())
                                .unwrap_or("");
                            let spliced_slice = {
                                let bytes = spliced_full.as_bytes();
                                let mut li = 0usize;
                                let mut ri = bytes.len();
                                while li < ri && bytes[li].is_ascii_whitespace() {
                                    li += 1;
                                }
                                while ri > li && bytes[ri - 1].is_ascii_whitespace() {
                                    ri -= 1;
                                }
                                if li < ri
                                    && bytes[li] == b'{'
                                    && ri > 0
                                    && bytes[ri - 1] == b'}'
                                    && li + 1 < ri - 1
                                {
                                    &spliced_full[li + 1..ri - 1]
                                } else {
                                    spliced_full
                                }
                            };
                            let spliced = spliced_slice.to_string();
                            handler_map
                                .entry(hname)
                                .or_default()
                                .push((b.state_id.clone(), spliced));
                        }
                    }
                    if !handler_map.is_empty() {
                        // Helper: decide whether a handler owner id can be used as a
                        // Rust identifier. We keep the original message name for
                        // FrameEvent but skip router arms for names that are not valid
                        // Rust identifiers (e.g., entry handlers like `$>()`).
                        fn is_valid_rust_ident(name: &str) -> bool {
                            let mut chars = name.chars();
                            match chars.next() {
                                Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
                                _ => return false,
                            }
                            for c in chars {
                                if !(c.is_ascii_alphanumeric() || c == '_') {
                                    return false;
                                }
                            }
                            true
                        }
                        module.push_str(&format!("impl {} {{\n", sys_name));
                        // Router: dispatch based on message name and current StateId by
                        // calling the corresponding internal handler method. This provides a
                        // minimal runtime entrypoint for interface wrappers and future
                        // event plumbing.
                        module.push_str("    fn _frame_router(&mut self, e: Option<FrameEvent>) {\n");
                        module.push_str("        if let Some(ev) = e {\n");
                        module.push_str("            match ev.message.as_str() {\n");
                        for hname in handler_map.keys() {
                            if !is_valid_rust_ident(hname) {
                                continue;
                            }
                            module.push_str(&format!("                \"{}\" => self._event_{}(),\n", hname, hname));
                        }
                        module.push_str("                _ => { }\n");
                        module.push_str("            }\n");
                        module.push_str("        }\n");
                        module.push_str("    }\n");
                        // Emit methods that implement each handler, dispatching on StateId
                        // when a handler is implemented in multiple states.
                        for (hname, entries) in handler_map {
                            let use_name_in_code = is_valid_rust_ident(&hname);
                            let setter_name = if has_returns {
                                if let Some(meta_map) = iface_meta_for_sys_rust {
                                    if meta_map.contains_key(&hname) {
                                        Some(hname.as_str())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            // State-less handler: emit a simple internal method with the body as-is.
                            if use_name_in_code {
                                if entries.len() == 1 && entries[0].0.is_none() {
                                    module.push_str(&format!("    fn _event_{}(&mut self) {{\n", hname));
                                    emit_rs_handler_body(&mut module, &entries[0].1, "        ", setter_name);
                                    module.push_str("    }\n");
                                } else {
                                    // Multi-state handler: dispatch on StateId and splice per-state bodies.
                                    module.push_str(&format!("    fn _event_{}(&mut self) {{\n", hname));
                                    module.push_str("        match self.compartment.state {\n");
                                    for (state_opt, body) in &entries {
                                        if let Some(ref sid) = state_opt {
                                            module.push_str(&format!("            StateId::{} => {{\n", sid));
                                            emit_rs_handler_body(&mut module, body, "                ", setter_name);
                                            module.push_str("            }\n");
                                        } else {
                                            // Fallback for handlers not tied to a specific state when
                                            // others are state-qualified.
                                            module.push_str("            _ => {\n");
                                            emit_rs_handler_body(&mut module, body, "                ", setter_name);
                                            module.push_str("            }\n");
                                        }
                                    }
                                    // Ensure exhaustive match in case there are states without handlers.
                                    module.push_str("            _ => { }\n");
                                    module.push_str("        }\n");
                                    module.push_str("    }\n");
                                }
                            }
                        }
                        module.push_str("}\n\n");
                    }
                    // If this system declares interface methods, synthesize per-method
                    // setters for `system.return` and public interface wrappers that
                    // allocate a per-call return slot, route an event, and then pop
                    // the slot and return the payload.
                    if has_returns {
                        if let Some(iface_meta) = iface_meta_for_sys_rust {
                            use std::collections::BTreeMap;
                            let mut ordered: BTreeMap<&String, &InterfaceMethodMeta> = BTreeMap::new();
                            for (name, meta) in iface_meta {
                                ordered.insert(name, meta);
                            }
                            module.push_str(&format!("impl {} {{\n", sys_name));
                            for (name, meta) in ordered {
                                let ty = meta.return_type.as_deref().unwrap_or("()");
                                // Per-method setter: update the payload in the top-of-stack
                                // slot when the current call belongs to this interface.
                                module.push_str(&format!(
                                    "    fn _set_system_return_for_{}(&mut self, value: {}) {{\n",
                                    name, ty
                                ));
                                module.push_str("        if let Some(");
                                module.push_str(&return_enum_name);
                                module.push_str("::");
                                module.push_str(name);
                                module.push_str("(ref mut v)) = self._system_return_stack.last_mut() {\n");
                                module.push_str("            *v = value;\n");
                                module.push_str("        }\n");
                                module.push_str("    }\n");
                                // Public interface wrapper: initialize the per-call slot,
                                // route an event, then pop and return the payload.
                                module.push_str(&format!(
                                    "    pub fn {}(&mut self) -> {} {{\n",
                                    name, ty
                                ));
                                if let Some(init) = meta.return_init.as_deref() {
                                    module.push_str("        let __initial: ");
                                    module.push_str(ty);
                                    module.push_str(" = ");
                                    module.push_str(init);
                                    module.push_str(";\n");
                                } else if ty == "()" {
                                    module.push_str("        let __initial: () = ();\n");
                                } else {
                                    module.push_str("        let __initial: ");
                                    module.push_str(ty);
                                    module.push_str(" = ::std::default::Default::default();\n");
                                }
                                module.push_str("        self._system_return_stack.push(");
                                module.push_str(&return_enum_name);
                                module.push_str("::");
                                module.push_str(name);
                                module.push_str("(__initial));\n");
                                module.push_str("        let __event = FrameEvent{ message: \"");
                                module.push_str(name);
                                module.push_str("\".to_string() };\n");
                                module.push_str("        self._frame_router(Some(__event));\n");
                                module.push_str("        let __result = match self._system_return_stack.pop() {\n");
                                module.push_str("            Some(");
                                module.push_str(&return_enum_name);
                                module.push_str("::");
                                module.push_str(name);
                                module.push_str("(value)) => value,\n");
                                module.push_str("            _ => {\n");
                                if ty == "()" {
                                    module.push_str("                ()\n");
                                } else {
                                    module.push_str("                ::std::default::Default::default()\n");
                                }
                                module.push_str("            }\n");
                                module.push_str("        };\n");
                                module.push_str("        __result\n");
                                module.push_str("    }\n");
                            }
                            module.push_str("}\n\n");
                        }
                    }
                    // Enum StateId
                    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
                    let validator = crate::frame_c::v3::validator::ValidatorV3;
                    let states = validator.collect_machine_state_names(bytes, outline_start);
                    if !states.is_empty() {
                        let mut enum_src = String::new();
                        enum_src.push_str("#[allow(dead_code)]\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
                        enum_src.push_str("enum StateId { ");
                        let mut first = true;
                        for s in states.iter() {
                            if !first {
                                enum_src.push_str(", ");
                            }
                            enum_src.push_str(s);
                            first = false;
                        }
                        enum_src.push_str(" }\n");
                        // Default implementation: prefer the start state when present,
                        // otherwise fall back to the first collected state.
                        let default_state: &str = if states.iter().any(|s| s == &start_state) {
                            start_state.as_str()
                        } else {
                            states.iter().next().map(|s| s.as_str()).unwrap_or(start_state.as_str())
                        };
                        enum_src.push_str(&format!("\nimpl Default for StateId {{ fn default() -> Self {{ StateId::{} }} }}\n\n", default_state));
                        module = enum_src + &module;
                    }
                    // Scaffold: per-system typed return enum (not yet wired into semantics).
                    if has_returns {
                        if let Some(iface) = iface_meta_for_sys_rust {
                            let mut ret_enum = String::new();
                            ret_enum.push_str("#[allow(dead_code)]\n#[derive(Debug, Clone)]\n");
                            ret_enum.push_str(&format!("enum {} {{ ", return_enum_name));
                            let mut first = true;
                            for (mname, meta) in iface {
                                if !first {
                                    ret_enum.push_str(", ");
                                }
                                first = false;
                                let vname = mname; // method name as variant id (Rust allows lower_snake, though unusual)
                                let ty = meta.return_type.as_deref().unwrap_or("()");
                                ret_enum.push_str(vname);
                                ret_enum.push('(');
                                ret_enum.push_str(ty);
                                ret_enum.push(')');
                            }
                            ret_enum.push_str(" }\n\n");
                            module = ret_enum + &module;
                        }
                    }
                    out = module;
                }
                _ => {}
            }
        }
        // Rust enum StateId is always emitted above within the Rust branch
        // Optional mapping and visitor-map trailers for module demo path
        if std::env::var("FRAME_MAP_TRAILER").ok().as_deref() == Some("1") {
            use crate::frame_c::v3::splice::OriginV3;
            use crate::frame_c::v3::native_region_scanner::RegionSpan;
            // Rebuild splice maps per-body and merge into module-target offsets
            let mut module_map: Vec<(RegionSpan, OriginV3)> = Vec::new();
            let mut visitor: Vec<(usize, usize, usize, usize, &'static str)> = Vec::new();
            // Helper to compute line number from byte offset
            fn offset_to_line(s: &str, off: usize) -> usize {
                let bytes = s.as_bytes(); let mut i=0usize; let mut line=1usize; while i < bytes.len() && i < off { if bytes[i]==b'\n' { line+=1; } i+=1; } line
            }
            // Helper to compute 1-based column from byte offset
            fn offset_to_col(s: &str, off: usize) -> usize {
                let bytes = s.as_bytes(); let mut i = if off > bytes.len() { bytes.len() } else { off };
                while i > 0 { if bytes[i-1]==b'\n' { break; } i-=1; }
                (off.saturating_sub(i)) + 1
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
                    if matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust) {
                        // Visitor mapping: target/source lines + columns
                        let t_off = out_offset + tgt.start;
                        let target_line = offset_to_line(&out, t_off);
                        let target_col = offset_to_col(&out, t_off);
                        let body_str = std::str::from_utf8(body_bytes).unwrap_or("");
                        let (source_line, source_col) = match origin {
                            OriginV3::Frame{ source } | OriginV3::Native{ source } => (offset_to_line(body_str, source.start), offset_to_col(body_str, source.start)),
                        };
                        let origin_str: &'static str = match origin { OriginV3::Frame{..} => "frame", OriginV3::Native{..} => "native" };
                        visitor.push((target_line, target_col, source_line, source_col, origin_str));
                    }
                }
                out_offset += sp.text.len();
                cur = b.close_byte + 1;
            }
            // Build trailers
            let mut map_json = String::from("{\"map\":[");
            let mut first=true; for (tgt, origin) in &module_map { if !first { map_json.push(','); } else { first=false; } map_json.push_str(&format!("{{\"targetStart\":{},\"targetEnd\":{},", tgt.start, tgt.end)); match origin { OriginV3::Frame{ source } => map_json.push_str(&format!("\"origin\":\"frame\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end)), OriginV3::Native{ source } => map_json.push_str(&format!("\"origin\":\"native\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end)), } }
            map_json.push_str("] ,\"version\":1,\"schemaVersion\":1}");
            if let TargetLanguage::Python3 = lang {
                out.push_str("\n'''/*#frame-map#\n"); out.push_str(&map_json); out.push_str("\n#frame-map#*/'''\n");
            } else {
                out.push_str("\n/*#frame-map#\n"); out.push_str(&map_json); out.push_str("\n#frame-map#*/\n");
            }
            if matches!(lang, TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust) {
                let mut vjson = String::from("{\"mappings\":[");
                let mut f=true; for (tline, tcol, sline, scol, origin) in &visitor { if !f { vjson.push(','); } else { f=false; } vjson.push_str(&format!("{{\"targetLine\":{},\"targetColumn\":{},\"sourceLine\":{},\"sourceColumn\":{},\"origin\":\"{}\"}}", tline, tcol, sline, scol, origin)); }
                vjson.push_str("] ,\"schemaVersion\":2}");
                if let TargetLanguage::Python3 = lang {
                    out.push_str("\n'''/*#visitor-map#\n"); out.push_str(&vjson); out.push_str("\n#visitor-map#*/'''\n");
                } else {
                    out.push_str("\n/*#visitor-map#\n"); out.push_str(&vjson); out.push_str("\n#visitor-map#*/\n");
                }
            }
            // Optional debug manifest trailer for debugger tooling
            if std::env::var("FRAME_DEBUG_MANIFEST").ok().as_deref() == Some("1") {
                // Build a minimal manifest: system name and state compiled IDs
                let mut manifest = String::from("{");
                if let Some((sys_name, _)) = arc_for_ctx.systems.iter().next() {
                    manifest.push_str("\"system\":\""); manifest.push_str(sys_name); manifest.push_str("\",");
                    // Collect states for first system only (demo manifest)
                    manifest.push_str("\"states\":[");
                    let mut first = true;
                    if let Some(sys) = arc_for_ctx.systems.get(sys_name) {
                        for (_mname, mach) in &sys.machines {
                            for (sname, _sdecl) in &mach.states {
                                if !first { manifest.push(','); } else { first = false; }
                                let cid = format!("__{}_state_{}", sys_name, sname);
                                manifest.push_str(&format!("{{\"name\":\"{}\",\"compiledId\":\"{}\"}}", sname, cid));
                            }
                        }
                    }
                    manifest.push(']');
                    // Collect handlers with compiled IDs and params from outline (best-effort)
                    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
                    let (items, _issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
                    fn extract_params(hdr: &str) -> Vec<String> {
                        if let Some(lp) = hdr.find('(') { if let Some(rp_rel) = hdr[lp+1..].find(')') {
                            let rp = lp+1+rp_rel; let inside = &hdr[lp+1..rp];
                            return inside.split(',').map(|t| t.trim()).filter(|t| !t.is_empty()).map(|t| t.split(|c| c=='='||c==':').next().unwrap_or("").trim().to_string()).filter(|s| !s.is_empty()).collect();
                        } }
                        Vec::new()
                    }
                    manifest.push_str(",\"handlers\":[");
                    let mut hf = true;
                    for it in &items {
                        if let crate::frame_c::v3::validator::BodyKindV3::Handler = it.kind {
                            if let (Some(st), Some(hn)) = (it.state_id.as_ref(), it.owner_id.as_ref()) {
                                let hdr = std::str::from_utf8(&bytes[it.header_span.start..it.header_span.end]).unwrap_or("");
                                let params = extract_params(hdr);
                                let hcid = format!("__{}_state_{}__handler_{}", sys_name, st, hn);
                                if !hf { manifest.push(','); } else { hf=false; }
                                manifest.push_str("{");
                                manifest.push_str(&format!("\"state\":\"{}\",\"name\":\"{}\",\"compiledId\":\"{}\"", st, hn, hcid));
                                if !params.is_empty() {
                                    manifest.push_str(",\"params\":[");
                                    for (i,p) in params.iter().enumerate() { if i>0 { manifest.push(','); } manifest.push('"'); manifest.push_str(p); manifest.push('"'); }
                                    manifest.push(']');
                                }
                                manifest.push_str("}");
                            }
                        }
                    }
                    manifest.push(']');
                } else {
                    manifest.push_str("\"system\":null,\"states\":[],\"handlers\":[]");
                }
                manifest.push_str(",\"schemaVersion\":2}");
                if let TargetLanguage::Python3 = lang {
                    out.push_str("\n'''/*#debug-manifest#\n"); out.push_str(&manifest); out.push_str("\n#debug-manifest#*/'''\n");
                } else {
                    out.push_str("\n/*#debug-manifest#\n"); out.push_str(&manifest); out.push_str("\n#debug-manifest#*/\n");
                }
            }
        }
        // Optional native symbol snapshot trailer (Stage 10A)
        if std::env::var("FRAME_NATIVE_SYMBOL_SNAPSHOT").ok().as_deref() == Some("1") {
            let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
            let (items, _issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
                fn extract_params_with_spans(hdr: &str) -> (Vec<String>, Vec<(usize, usize)>) {
                if let Some(lp) = hdr.find('(') {
                    if let Some(rp_rel) = hdr[lp+1..].find(')') {
                        let rp = lp + 1 + rp_rel;
                        let inside = &hdr[lp+1..rp];
                        let mut names = Vec::new();
                        let mut spans = Vec::new();
                        let bytes = inside.as_bytes();
                    let mut i = 0usize;
                    while i < bytes.len() {
                            while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b',') { i += 1; }
                            if i >= bytes.len() { break; }
                            let tok_start = i;
                            while i < bytes.len() && bytes[i] != b',' { i += 1; }
                            let tok_end = i; // exclusive
                            let token = &inside[tok_start..tok_end];
                            let trimmed = token.trim();
                            if !trimmed.is_empty() {
                                let base = trimmed.split(|c| c=='=' || c==':').next().unwrap_or("").trim();
                                if !base.is_empty() {
                                    names.push(base.to_string());
                                    // compute absolute span within hdr by trimming leading/trailing spaces of token
                                    let lead = token.len() - token.trim_start().len();
                                    let trail = token.len() - token.trim_end().len();
                                    let abs_s = lp + 1 + tok_start + lead;
                                    let abs_e = lp + 1 + tok_end - trail;
                                    spans.push((abs_s, abs_e));
                                }
                            }
                        }
                        return (names, spans);
                    }
                }
                (Vec::new(), Vec::new())
            }
            let mut entries_json = String::from("{\"entries\":[");
            let mut first = true;
            for it in &items {
                if matches!(it.kind, crate::frame_c::v3::validator::BodyKindV3::Handler) {
                    let start = it.header_span.start; let end = it.header_span.end.min(bytes.len());
                    let hdr = std::str::from_utf8(&bytes[start..end]).unwrap_or("");
                    let (params, pspans) = extract_params_with_spans(hdr);
                    // Stage 10C: parser-backed extraction for Python (RustPython) when available; fallback to header-based
                    #[cfg(feature = "native-py-rp")]
                    {
                        fn py_parse_params_from_inside(inside: &str) -> Option<Vec<String>> {
                            use rustpython_parser::mode::Mode;
                            use rustpython_parser::parser;
                            // Build a minimal def to parse
                            let src = format!("def __f({}):\n    pass\n", inside);
                            if let Ok(module) = parser::parse(&src, Mode::Module) {
                                // rustpython_parser returns a located AST; traverse top-level for FunctionDef
                                // The API exposes module!().body or .statements depending on version; use Debug fallback
                                // Try to access via AST helper traits
                                // We conservatively regex fallback if AST layout differs
                                // But keep AST branch best-effort when available
                                #[allow(unused_imports)]
                                use rustpython_parser::ast as pyast;
                                let mut names: Vec<String> = Vec::new();
                                // The Display/Debug form is not ideal; attempt to downcast via pattern matching
                                // Use Into<Vec<_>> approach via helper if present; else return None to keep header params
                                // Since rustpython AST API can shift, guard with a simple string-based fallback
                                let text = format!("{:?}", module);
                                // naive extract identifiers between '(' and ')' of __f from the pretty AST text
                                if let Some(start) = text.find("__f(") {
                                    if let Some(rest) = text.get(start+4..) {
                                        if let Some(end) = rest.find(')') { let inner = &rest[..end];
                                            for tok in inner.split(',') {
                                                let t = tok.trim();
                                                if t.is_empty() { continue; }
                                                // tokens like arg=..., or name: type, or *args/**kwargs patterns; keep base ident chars
                                                let base = t.split(|c| c=='=' || c==':').next().unwrap_or("").trim().trim_start_matches('*');
                                                if !base.is_empty() && base.chars().all(|c| c.is_ascii_alphanumeric() || c=='_') {
                                                    names.push(base.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                                if !names.is_empty() { return Some(names); }
                            }
                            None
                        }
                        if let Some(lp) = hdr.find('(') { if let Some(rp) = hdr[lp+1..].find(')') {
                            let inside = &hdr[lp+1..lp+1+rp];
                            if let Some(nv) = py_parse_params_from_inside(inside) { if !nv.is_empty() { params = nv; } }
                        } }
                    }
                    // Stage 10C: parser-backed param extraction (TypeScript) when available; fallback to header-based
                    #[cfg(feature = "native-ts")]
                    {
                        fn ts_collect_idents(pat: &swc_ecma_ast::Pat, out: &mut Vec<String>) {
                            use swc_ecma_ast::*;
                            match pat {
                                Pat::Ident(BindingIdent { id, .. }) => out.push(id.sym.to_string()),
                                Pat::Array(ArrayPat { elems, .. }) => {
                                    for e in elems {
                                        if let Some(p) = e { ts_collect_idents(p, out); }
                                    }
                                }
                                Pat::Assign(AssignPat { left, .. }) => {
                                    ts_collect_idents(left, out);
                                }
                                Pat::Rest(RestPat { arg, .. }) => {
                                    ts_collect_idents(arg, out);
                                }
                                Pat::Object(ObjectPat { props, .. }) => {
                                    for _p in props { /* skip destructuring names for now */ }
                                }
                                _ => {}
                            }
                        }
                        fn parse_ts_params_from_inside(inside: &str) -> Option<Vec<String>> {
                            use swc_common::{sync::Lrc, FileName, SourceMap};
                            use swc_ecma_ast::EsVersion;
                            use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
                            let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
                            let src = format!("function __f({}){{}}", inside);
                            let fm = cm.new_source_file(FileName::Custom("snippet.ts".into()).into(), src);
                            let lexer = Lexer::new(
                                Syntax::Typescript(Default::default()),
                                EsVersion::Es2020,
                                StringInput::from(&*fm),
                                None,
                            );
                            let mut parser = Parser::new_from(lexer);
                            if let Ok(module) = parser.parse_script() {
                                for stmt in module.body {
                                    if let swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Fn(fd)) = stmt {
                                        let mut names = Vec::new();
                                        for p in &fd.function.params {
                                            ts_collect_idents(&p.pat, &mut names);
                                        }
                                        return Some(names);
                                    }
                                }
                            }
                            None
                        }
                        // Attempt to parse the header param list via SWC on the substring between '(' and ')'
                        if let Some(lp) = hdr.find('(') {
                            if let Some(rp) = hdr[lp+1..].find(')') { 
                                let inside = &hdr[lp+1..lp+1+rp];
                                if let Some(names) = parse_ts_params_from_inside(inside) {
                                    if !names.is_empty() { params = names; }
                                }
                            }
                        }
                    }
                    if !first { entries_json.push(','); } else { first = false; }
                    entries_json.push_str("{\"state\":");
                    if let Some(ref s) = it.state_id { entries_json.push('"'); entries_json.push_str(s); entries_json.push('"'); } else { entries_json.push_str("null"); }
                    entries_json.push_str(",\"owner\":");
                    if let Some(ref o) = it.owner_id { entries_json.push('"'); entries_json.push_str(o); entries_json.push('"'); } else { entries_json.push_str("null"); }
                    entries_json.push_str(",\"params\":[");
                    for (i, p) in params.iter().enumerate() { if i>0 { entries_json.push(','); } entries_json.push('"'); entries_json.push_str(p); entries_json.push('"'); }
                    entries_json.push_str("]");
                    if !pspans.is_empty() {
                        entries_json.push_str(",\"paramSpans\":"); // emit array directly below
                        // build spans JSON
                        let mut s = String::from("[");
                        for (i, (ss, ee)) in pspans.iter().enumerate() {
                            if i>0 { s.push(','); }
                            s.push_str(&format!("{{\"start\":{},\"end\":{}}}", start + ss, start + ee));
                        }
                        s.push(']');
                        // replace placeholder
                        entries_json.push_str(&s);
                    }
                    entries_json.push_str("}");
                }
            }
            entries_json.push_str("],\"schemaVersion\":1}");
            out.push_str("\n/*#native-symbols#\n"); out.push_str(&entries_json); out.push_str("\n#native-symbols#*/\n");
        }
        // Structured errors JSON trailer for module compile (always for V3 demo)
        {
            // Run validation to collect issues akin to validate_module_demo_with_mode
            let mut issues = Vec::new();
            let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
            let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
            for is in outline_issues { issues.push(is); }
            let validator = ValidatorV3;
            // Build module AST, known-states, and Arcanum (symbol table) for E402/E403 parity
            let module_ast = SystemParserV3::parse_module(bytes, lang);
            let known_states = validator.collect_machine_state_names(bytes, outline_start);
            let arcanum = crate::frame_c::v3::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
            let sys_param_issues = validator.validate_system_param_semantics(bytes, outline_start, lang, &arcanum, &items);
            for is in sys_param_issues { issues.push(is); }
            // Collect interface method names for system.method validation.
            let interface_methods = InterfaceParserV3.collect_all_interface_method_names(bytes, &module_ast, lang);
            let system_name = find_system_name(bytes, 0);
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
                // E401: frame statements disallowed in actions/operations
                if matches!(b.kind, crate::frame_c::v3::validator::BodyKindV3::Action | crate::frame_c::v3::validator::BodyKindV3::Operation) {
                    let pol = crate::frame_c::v3::validator::ValidatorPolicyV3 { body_kind: Some(b.kind) };
                    let res = validator.validate_regions_mir_with_policy(&scan.regions, &mir, pol);
                    for is in res.issues { issues.push(is); }
                }
                // E400: terminal-last policy
                let extra = validator.validate_terminal_last_native(body_bytes, &scan.regions, &mir, lang);
                for is in extra { issues.push(is); }
                // E402: unknown state (prefer Arcanum resolution)
                if !known_states.is_empty() {
                    let sys = system_name.as_deref();
                    let e402 = validator.validate_transition_targets_arcanum(&mir, &arcanum, sys);
                    for is in e402 { issues.push(is); }
                }
                // E405: advisory state param arity (flag-gated)
                if std::env::var("FRAME_VALIDATE_NATIVE_POLICY").ok().as_deref() == Some("1") {
                    let sys = system_name.as_deref();
                    let adv = validator.validate_transition_state_arity_arcanum(&mir, &arcanum, sys);
                    for is in adv { issues.push(is); }
                }
                // E403: parent-forward requires a declared parent when forwarding from a handler
                if validator.has_machine_section(bytes, outline_start) {
                    if matches!(b.kind, BodyKindV3::Handler | BodyKindV3::Function | BodyKindV3::Unknown) {
                        if mir.iter().any(|m| matches!(m, crate::frame_c::v3::mir::MirItemV3::Forward { .. })) {
                            let enclosing_state = b.state_id.as_deref();
                            let mut ok_parent = false;
                            if let Some(state_name) = enclosing_state {
                                let sys = system_name.as_deref().unwrap_or("_");
                                ok_parent = arcanum.has_parent(sys, state_name) || arcanum.has_parent("_", state_name);
                            }
                            if !ok_parent {
                                issues.push(crate::frame_c::v3::validator::ValidationIssueV3{ message: "E403: Cannot forward to parent: no parent available".into() });
                            }
                        }
                    }
                }
                // E406/E407: system.method/system.return usage
                if matches!(b.kind, crate::frame_c::v3::validator::BodyKindV3::Handler | crate::frame_c::v3::validator::BodyKindV3::Action | crate::frame_c::v3::validator::BodyKindV3::Operation) {
                    let call_issues = validator.validate_system_calls_interface(body_bytes, &scan.regions, &interface_methods);
                    for is in call_issues { issues.push(is); }
                    let ret_issues = validator.validate_system_return_usage(body_bytes, &scan.regions, b.kind);
                    for is in ret_issues { issues.push(is); }
                }

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
            if std::env::var("FRAME_ERROR_JSON").ok().as_deref() == Some("1") {
                let json = build_errors_json(&issues);
                if let TargetLanguage::Python3 = lang {
                    out.push_str("\n'''/*#errors-json#\n");
                    out.push_str(&json);
                    out.push_str("\n#errors-json#*/'''\n");
                } else {
                    out.push_str("\n/*#errors-json#\n");
                    out.push_str(&json);
                    out.push_str("\n#errors-json#*/\n");
                }
            }
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
    let outline_start = parts
        .imports
        .last()
        .map(|s| s.end)
        .or(parts.prolog.as_ref().map(|p| p.end))
        .unwrap_or(0);
    // Collect known state names and per-module context for validations that
    // depend on Arcanum or system-wide information.
    let (known_states, system_name, interface_methods, arcanum_symtab) = {
        let (items, outline_issues) = crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
        all_issues.extend(outline_issues);
        let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
        all_issues.extend(outer_issues);
        // Enforce per-system block ordering and uniqueness using ModuleAst (operations:, interface:, machine:, actions:, domain:)
        // and validate machine state headers from the same AST.
        let module_ast = SystemParserV3::parse_module(bytes, lang);
        let block_order_issues = validator.validate_system_block_order_ast(&module_ast);
        all_issues.extend(block_order_issues);
        // Enforce single `fn main` per module.
        let main_issues = validator.validate_main_functions(bytes, &items);
        all_issues.extend(main_issues);
        // machine section: simple state header check for '{' driven from ModuleAst.
        let state_issues = validator.validate_machine_state_headers_ast(bytes, &module_ast);
        all_issues.extend(state_issues);
        // handlers must be nested inside a state block in machine:, validated against Arcanum/AST.
        let arc_for_ctx =
            crate::frame_c::v3::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
        let handler_scope_issues =
            validator.validate_handlers_in_state_ast(bytes, &items, &module_ast, &arc_for_ctx);
        all_issues.extend(handler_scope_issues);
        // Collect known state names (coarse) and build Arcanum for symbol-precision.
        // For PRT languages we rely on the ModuleAst-backed Arcanum; non-PRT languages
        // continue to use a coarse known-state set for E402.
        let known_states = validator.collect_machine_state_names(bytes, outline_start);
        let arcanum_symtab = Some(arc_for_ctx.clone());
        let sys_param_issues =
            validator.validate_system_param_semantics(bytes, outline_start, lang, &arc_for_ctx, &items);
        all_issues.extend(sys_param_issues);
        // Collect interface method names for system.method(...) validation using the system parser.
        let interface_methods =
            InterfaceParserV3.collect_all_interface_method_names(bytes, &module_ast, lang);
        // Best-effort scan for system name
        let system_name = find_system_name(bytes, 0);
        // Debug hook removed: known_states reporting was temporary for triage
        (known_states, system_name, interface_methods, arcanum_symtab)
    };
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
        // Validate that transition targets refer to known states.
        match lang {
            TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust => {
                if let Some(ref arc) = arcanum_symtab {
                    let sys = system_name.as_deref();
                    res.issues.extend(
                        validator.validate_transition_targets_arcanum(&mir, arc, sys)
                    );
                }
            }
            _ => {
                if !known_states.is_empty() {
                    res.issues.extend(
                        validator.validate_transition_targets(&mir, &known_states)
                    );
                }
            }
        }
        // Optional advisory policy: state parameter arity (Stage 10B).
        if std::env::var("FRAME_VALIDATE_NATIVE_POLICY").ok().as_deref() == Some("1") {
            if let Some(ref arc) = arcanum_symtab {
                let sys = system_name.as_deref();
                res.issues.extend(validator.validate_transition_state_arity_arcanum(&mir, arc, sys));
            }
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
        // Enforce that system.return is not used in operations.
        let ret_issues = validator.validate_system_return_usage(body_bytes, &scan.regions, b.kind);
        res.issues.extend(ret_issues);
        // Enforce that system.method(...) calls target interface methods.
        if matches!(b.kind, BodyKindV3::Handler | BodyKindV3::Action | BodyKindV3::Operation) {
            let sys_issues = validator.validate_system_calls_interface(body_bytes, &scan.regions, &interface_methods);
            res.issues.extend(sys_issues);
        }
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
    // Per-system block ordering and uniqueness (operations:, interface:, machine:, actions:, domain:) using ModuleAst.
    let module_ast = SystemParserV3::parse_module(bytes, lang);
    let block_order_issues = validator.validate_system_block_order_ast(&module_ast);
    all_issues.extend(block_order_issues);
    // Enforce single `fn main` per module.
    let main_issues = validator.validate_main_functions(bytes, &items);
    all_issues.extend(main_issues);
    // Machine state headers and handler placement must be validated via ModuleAst/Arcanum.
    let state_issues = validator.validate_machine_state_headers_ast(bytes, &module_ast);
    all_issues.extend(state_issues);
    let arc_for_ctx = crate::frame_c::v3::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
    let handler_scope_issues =
        validator.validate_handlers_in_state_ast(bytes, &items, &module_ast, &arc_for_ctx);
    all_issues.extend(handler_scope_issues);

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
        // Optional advisory policy: state parameter arity (Stage 10B).
        if std::env::var("FRAME_VALIDATE_NATIVE_POLICY").ok().as_deref() == Some("1") {
            res.issues.extend(validator.validate_transition_state_arity_arcanum(&mir, arc, sys));
        }
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

// Minimal domain: scanner for TypeScript runnable modules.
// Scans for a top-level `domain:` block and extracts Frame-style domain
// variables into (name, type, initializer) triples.
fn scan_ts_domain_fields(bytes: &[u8]) -> Vec<(String, Option<String>, Option<String>)> {
    let n = bytes.len();
    let mut i = 0usize;
    let mut domain_start: Option<usize> = None;
    while i < n {
        // skip leading whitespace
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
        if i >= n { break; }
        let line_start = i;
        // skip comments
        if bytes[i] == b'#' {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        if bytes[i] == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            while i < n && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        // read identifier at SOL
        let mut j = i;
        while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
        let kw_start = j;
        while j < n && (bytes[j] as char).is_ascii_alphanumeric() { j += 1; }
        if kw_start < j && j < n && bytes[j] == b':' {
            let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
            if kw.as_str() == "domain" {
                // domain: header detected; domain block begins after this line
                let mut k = line_start;
                while k < n && bytes[k] != b'\n' { k += 1; }
                domain_start = Some(if k < n { k + 1 } else { n });
                break;
            }
        }
        while i < n && bytes[i] != b'\n' { i += 1; }
        if i < n { i += 1; }
    }
    let mut out: Vec<(String, Option<String>, Option<String>)> = Vec::new();
    let mut p = match domain_start { Some(s) => s, None => return out };
    while p < n {
        let line_start = p;
        while p < n && bytes[p] != b'\n' { p += 1; }
        let line_end = p;
        if p < n { p += 1; }
        let line = &bytes[line_start..line_end];
        // trim
        let mut s = 0usize; let mut e = line.len();
        while s < e && (line[s] == b' ' || line[s] == b'\t') { s += 1; }
        while e > s && (line[e-1] == b' ' || line[e-1] == b'\t' || line[e-1] == b'\r') { e -= 1; }
        if s >= e { continue; }
        let slice = &line[s..e];
        // comments
        if slice[0] == b'#' { continue; }
        if slice.len() >= 2 && slice[0] == b'/' && slice[1] == b'/' { continue; }
        // helper to trim trailing ';'
        let trim_semicolon = |s: &str| {
            let t = s.trim_end();
            if t.ends_with(';') { t[..t.len()-1].trim().to_string() } else { t.to_string() }
        };
        // Case 1: "var name[: Type] = expr" or "var name[: Type]"
        if slice.len() >= 4 && &slice[..4].to_ascii_lowercase() == b"var " {
            let rest = &slice[4..];
            let rest_str = String::from_utf8_lossy(rest);
            let mut parts_iter = rest_str.trim_start().chars().peekable();
            let mut name = String::new();
            while let Some(&ch) = parts_iter.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    name.push(ch);
                    parts_iter.next();
                } else { break; }
            }
            if name.is_empty() { continue; }
            let mut remainder: String = parts_iter.collect();
            let mut ty: Option<String> = None;
            if let Some(colon_idx) = remainder.find(':') {
                let after_colon = remainder[colon_idx+1..].trim_start();
                let mut ty_end = 0usize;
                for (idx, ch) in after_colon.char_indices() {
                    if ch == '=' || ch == ';' { break; }
                    ty_end = idx + ch.len_utf8();
                }
                if ty_end > 0 {
                    let ty_str = &after_colon[..ty_end];
                    let trimmed = ty_str.trim();
                    if !trimmed.is_empty() { ty = Some(trimmed.to_string()); }
                }
                remainder = String::from(after_colon);
            }
            let mut init: Option<String> = None;
            if let Some(eq_pos) = remainder.find('=') {
                let val = &remainder[eq_pos+1..];
                let v = trim_semicolon(val);
                if !v.is_empty() { init = Some(v); }
            }
            out.push((name, ty, init));
            continue;
        }
        // Case 2: "name = expr" (implicit any)
        let line_str = String::from_utf8_lossy(slice);
        let mut chars = line_str.chars().peekable();
        let mut name = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                chars.next();
            } else { break; }
        }
        if name.is_empty() { continue; }
        let remainder: String = chars.collect();
        let mut init: Option<String> = None;
        if let Some(eq_pos) = remainder.find('=') {
            let val = &remainder[eq_pos+1..];
            let v = trim_semicolon(val);
            if !v.is_empty() { init = Some(v); }
        }
        if init.is_some() {
            out.push((name, None, init));
        }
    }
    out
}
