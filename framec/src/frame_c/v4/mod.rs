use crate::frame_c::utils::{frame_exitcode, RunError};
pub use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::native_region_scanner as nscan;
use crate::frame_c::v4::native_region_scanner::NativeRegionScanner;
use crate::frame_c::v4::mir_assembler::MirAssembler;
use crate::frame_c::v4::splice::Splicer;
use crate::frame_c::v4::validator::{Validator, ValidationResult, ValidatorPolicy, BodyKind};
use crate::frame_c::v4::system_parser::SystemParser;
use crate::frame_c::v4::interface_parser::InterfaceParser;
use crate::frame_c::v4::body_closer::BodyCloser;

pub mod body_closer;
pub mod native_region_scanner;
pub mod frame_statement_parser;
pub mod mir;
pub mod mir_assembler;
pub mod expander;
pub mod splice;
pub mod validator;
// multifile_demo module removed - demo mode no longer supported
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
pub mod python_transpiler;
pub mod system_param_semantics;
pub mod rust_domain_scanner;
pub mod machines;
pub mod ts_harness_machine;
pub mod system_transformer;
// Phase 1: Frame AST modules
pub mod frame_ast;
pub mod frame_parser;
pub mod frame_validator;
pub mod parser_debug;
// Phase 3: Pipeline infrastructure
pub mod pipeline;
// Phase 4: Code generation infrastructure
pub mod codegen;
// Phase 5: Modular validation infrastructure
pub mod validation;

// Re-export new architecture types for easier access
pub use pipeline::{
    PipelineConfig, CompileMode, compile_ast_based, CompileResult, CompileError,
};
pub use codegen::{CodegenNode, LanguageBackend, get_backend, generate_system};
// Test infrastructure moved to shared environment - using stub for backward compatibility
pub mod test_harness_rs {
    pub use super::test_harness_stub::*;
}
mod test_harness_stub;

// Unit tests for v4 components
#[cfg(test)]
mod arcanum_tests;
#[cfg(test)]
mod compile_tests;
#[cfg(test)]
mod frame_parser_tests;
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

/// Convert Python-style import statements to TypeScript equivalents
fn convert_python_import_to_typescript(import_text: &str) -> String {
    let trimmed = import_text.trim();
    
    // Handle Python import patterns and convert to TypeScript
    if trimmed.starts_with("import ") {
        // Simple imports like "import math" -> "// import math (Python - needs TS equivalent)"
        // For now, comment them out as they need proper TS module equivalents
        if trimmed.contains(" as ") {
            // "import os.path as osp" -> comment out
            return format!("// {} (Python import - needs TypeScript equivalent)", trimmed);
        } else if trimmed.contains(".") && !trimmed.contains(" from ") {
            // "import os.path" -> comment out
            return format!("// {} (Python import - needs TypeScript equivalent)", trimmed);
        } else {
            // Simple module imports
            let module = trimmed.trim_start_matches("import ").trim();
            // Common Python to TypeScript module mappings
            match module {
                "math" => return "// import math (use Math built-in in TypeScript)".to_string(),
                "json" => return "// import json (use JSON built-in in TypeScript)".to_string(),
                _ => return format!("// {} (Python import - needs TypeScript equivalent)", trimmed),
            }
        }
    } else if trimmed.starts_with("from ") {
        // "from collections import defaultdict" -> needs TS equivalent
        // "from typing import List, Dict" -> TypeScript has built-in types
        if trimmed.contains("from typing import") {
            return "// TypeScript has built-in types (List -> Array, Dict -> Record/Map)".to_string();
        }
        return format!("// {} (Python import - needs TypeScript equivalent)", trimmed);
    }
    
    // Return empty string for non-import lines
    String::new()
}

/// Best-effort scanner for TypeScript class instance fields used in native bodies.
/// Looks for `this.<ident>` patterns inside native regions of handlers/actions/operations
/// and returns the set of candidate field names. This is used to synthesize class
/// field declarations so `tsc` does not report TS2339 for state stored on `this`.
fn collect_ts_field_candidates(
    content: &str,
    parts: &crate::frame_c::v4::module_partitioner::ModulePartitions,
) -> std::collections::HashSet<String> {
    use crate::frame_c::v4::native_region_scanner::Region;
    let mut fields: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in &parts.bodies {
        // Only scan handler/action/operation bodies; ignore functions/unknown for now.
        if !matches!(
            b.kind,
            BodyKind::Handler | BodyKind::Action | BodyKind::Operation
        ) {
            continue;
        }
        if b.close_byte <= b.open_byte || b.close_byte >= content.len() {
            continue;
        }
        let body_src = &content[b.open_byte..=b.close_byte];
        let scan = match nscan::typescript::NativeRegionScannerTs.scan(body_src.as_bytes(), 0)
        {
            Ok(s) => s,
            Err(_) => crate::frame_c::v4::native_region_scanner::ScanResult {
                close_byte: body_src.len().saturating_sub(1),
                regions: Vec::new(),
            },
        };
        for r in &scan.regions {
            if let Region::NativeText { span } = r {
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

/// Compiler entrypoint (MVP scaffold).
///
/// This will replace the legacy pipeline incrementally. For now it returns a
/// deterministic error so the CLI remains usable while we bring up stages.
pub struct Compiler;

impl Compiler {
// Demo mode functions removed - all fixtures converted to proper Frame modules

    pub fn compile_multifile_unsupported() -> Result<String, RunError> {
        Err(RunError::new(
            frame_exitcode::PARSE_ERR,
            "Multi-file build is temporarily unavailable during rebuild",
        ))
    }
}

fn build_errors_json(issues: &[crate::frame_c::v4::validator::ValidationIssue]) -> String {
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

// validate_single_body removed - demo mode no longer supported

#[derive(Debug, Clone)]
pub(crate) struct SystemParamGroups {
    pub declared: Vec<String>,
    pub start: Vec<String>,
    pub enter: Vec<String>,
    pub domain: Vec<String>,
}

pub(crate) fn parse_system_params(bytes: &[u8], sys_name: &str) -> SystemParamGroups {
    // Returns name-based groups for system parameters:
    // - declared: ordered list of all params as written in the header (names only)
    // - start: names inside any $(...) group (union)
    // - enter: names inside any $>(...) group (union)
    // - domain: declared names not in start or enter
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
            // Keep scanning the same line so we can catch `system` after
            // annotations like `@@persist @@system Foo`.
            i = j;
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
            return SystemParamGroups{ declared: Vec::new(), start: Vec::new(), enter: Vec::new(), domain: Vec::new() };
        }
        i += 1; // after '('
        let mut declared: Vec<String> = Vec::new();
        let mut start_params: Vec<String> = Vec::new();
        let mut enter_params: Vec<String> = Vec::new();
        let mut seen_declared: std::collections::HashSet<String> = std::collections::HashSet::new();
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
                        declared.push(ident.clone());
                        if !start_params.contains(&ident) {
                            start_params.push(ident);
                        }
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
                        declared.push(ident.clone());
                        if !enter_params.contains(&ident) {
                            enter_params.push(ident);
                        }
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
                declared.push(ident.clone());
                seen_declared.insert(ident);
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
        // Domain params are any declared names not already assigned to start/enter.
        let mut domain_params: Vec<String> = Vec::new();
        for name in declared.iter() {
            if start_params.contains(name) || enter_params.contains(name) {
                continue;
            }
            domain_params.push(name.clone());
        }
        return SystemParamGroups{
            declared,
            start: start_params,
            enter: enter_params,
            domain: domain_params,
        };
    }
    SystemParamGroups{ declared: Vec::new(), start: Vec::new(), enter: Vec::new(), domain: Vec::new() }
}

/// Best-effort selection of the textual first state in the given system.
/// Uses Arcanum state spans to choose the earliest state header for a stable
/// start state name instead of hard-coding "A".
fn find_start_state_name(
    arc: &crate::frame_c::v4::arcanum::Arcanum,
    sys_name: &str,
) -> Option<String> {
    if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[find_start_state_name] Looking for system: {}", sys_name);
        eprintln!("[find_start_state_name] Available systems: {:?}", arc.systems.keys().collect::<Vec<_>>());
    }
    let sys = arc.systems.get(sys_name)?;
    let mut best_name: Option<String> = None;
    let mut best_start: Option<usize> = None;
    for mach in sys.machines.values() {
        if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
            eprintln!("[find_start_state_name] Machine states: {:?}", mach.states.keys().collect::<Vec<_>>());
        }
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
    if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[find_start_state_name] Found start state: {:?}", best_name);
    }
    best_name
}

/// Main module compiler for `@@target` files.
///
/// This is the V4 AST-based compilation entry point. All Frame code is
/// now processed through the V4 pipeline: parse -> validate -> codegen.
pub fn compile_module(content_str: &str, lang: TargetLanguage) -> Result<String, RunError> {
    use crate::frame_c::v4::pipeline::config::PipelineConfig;
    use crate::frame_c::v4::pipeline::compiler;

    // Create config from environment, falling back to production defaults
    let config = PipelineConfig::from_env(lang);

    // Use V4 AST-based compilation
    match compiler::compile_ast_based(content_str.as_bytes(), &config) {
        Ok(result) if result.errors.is_empty() => Ok(result.code),
        Ok(result) => {
            // Return validation/compilation errors
            let error_msgs: Vec<String> = result.errors
                .iter()
                .map(|e| format!("{}: {}", e.code, e.message))
                .collect();
            Err(RunError::new(
                frame_exitcode::CONFIG_ERR,
                &format!("Compilation failed:\n{}", error_msgs.join("\n"))
            ))
        }
        Err(e) => Err(e)
    }
}

pub fn validate_module_demo(content_str: &str, lang: TargetLanguage) -> Result<ValidationResult, RunError> {
    validate_module_demo_with_mode(content_str, lang, false)
}

pub fn validate_module_demo_with_mode(content_str: &str, lang: TargetLanguage, strict_native: bool) -> Result<ValidationResult, RunError> {
    let bytes = content_str.as_bytes();
    // Partition the module. If partitioning fails due to outline issues (e.g., missing '{' after a header),
    // fall back to a tolerant outline scan to surface structured diagnostics (E-codes) instead of a hard error.
    let parts = match module_partitioner::ModulePartitioner::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => {
            // Map body close and prolog errors from the partitioner into structured E-codes where possible.
            let emsg = e.0;
            if emsg.starts_with("prolog error:") {
                // E105: Missing/invalid prolog; ensure a proper validator failure rather than a hard error
                let mut issues = Vec::new();
                let msg = if emsg.contains("NotFirstNonWhitespace") {
                    "E105: expected @@target prolog as first non-whitespace token"
                } else if emsg.contains("Missing") {
                    "E105: expected @@target <lang> at start of file"
                } else {
                    "E105: invalid @@target prolog"
                };
                issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: msg.into() });
                return Ok(ValidationResult { ok: false, issues });
            }
            if emsg.starts_with("body close error:") {
                let mapped = if emsg.contains("UnterminatedComment") || emsg.to_lowercase().contains("unterminated comment") {
                    vec![crate::frame_c::v4::validator::ValidationIssue{ message: "E106: unterminated comment".into() }]
                } else if emsg.contains("UnterminatedString") || emsg.to_lowercase().contains("unterminated string") {
                    vec![crate::frame_c::v4::validator::ValidationIssue{ message: "E100: unterminated string".into() }]
                } else if emsg.contains("UnmatchedBraces") || emsg.to_lowercase().contains("body not closed") {
                    vec![crate::frame_c::v4::validator::ValidationIssue{ message: "E103: unterminated body".into() }]
                } else {
                    Vec::new()
                };
                if !mapped.is_empty() {
                    return Ok(ValidationResult { ok: false, issues: mapped });
                }
            }
            // Tolerant outline scan will collect E111 and similar diagnostics.
            let outline_start = 0usize; // tolerant scan will walk whole file
            let (_items, outline_issues) = crate::frame_c::v4::outline_scanner::OutlineScanner.scan_collect(bytes, outline_start, lang);
            if !outline_issues.is_empty() {
                return Ok(ValidationResult { ok: false, issues: outline_issues });
            } else {
                // If we couldn't recover any diagnostics, return the original partition error
                return Err(RunError::new(frame_exitcode::PARSE_ERR, "module partition error"));
            }
        }
    };
    let validator = Validator;
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
        let (items, outline_issues) = crate::frame_c::v4::outline_scanner::OutlineScanner.scan_collect(bytes, outline_start, lang);
        all_issues.extend(outline_issues);
        let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
        all_issues.extend(outer_issues);
        // Enforce per-system block ordering and uniqueness using ModuleAst (operations:, interface:, machine:, actions:, domain:)
        // and validate machine state headers from the same AST.
        let module_ast = SystemParser::parse_module(bytes, lang);
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
            crate::frame_c::v4::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
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
            InterfaceParser.collect_all_interface_method_names(bytes, &module_ast, lang);
        // Best-effort scan for system name
        let system_name = find_system_name(bytes, 0);
        // Debug hook removed: known_states reporting was temporary for triage
        (known_states, system_name, interface_methods, arcanum_symtab)
    };
    for b in parts.bodies {
        let body_bytes = &bytes[b.open_byte..=b.close_byte];
        // scan and assemble
        let scan_res = match lang {
            TargetLanguage::Python3 => nscan::python::NativeRegionScannerPy.scan(body_bytes, 0),
            TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTs.scan(body_bytes, 0),
            TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCs.scan(body_bytes, 0),
            TargetLanguage::C => nscan::c::NativeRegionScannerC.scan(body_bytes, 0),
            TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCpp.scan(body_bytes, 0),
            TargetLanguage::Java => nscan::java::NativeRegionScannerJava.scan(body_bytes, 0),
            TargetLanguage::Rust => nscan::rust::NativeRegionScannerRust.scan(body_bytes, 0),
            _ => return Err(RunError::new(frame_exitcode::PARSE_ERR, "target not supported")),
        };
        let scan = match scan_res { Ok(s) => s, Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e))) };
        let (mir, parse_issues) = MirAssembler.assemble_collect(body_bytes, &scan.regions);
        if !parse_issues.is_empty() { all_issues.extend(parse_issues); }
        let policy = ValidatorPolicy { body_kind: Some(b.kind) };
        let mut res = validator.validate_regions_mir_with_policy(&scan.regions, &mir, policy);
        // Validate that transition targets refer to known states.
        match lang {
            TargetLanguage::Python3 | TargetLanguage::TypeScript | TargetLanguage::Rust => {
                if let Some(ref arc) = arcanum_symtab {
                    let sys = system_name.as_deref();
                    if !known_states.is_empty() {
                        res.issues.extend(
                            validator.validate_transition_targets_arcanum(&mir, arc, &known_states, sys)
                        );
                    }
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
            if matches!(b.kind, BodyKind::Handler | BodyKind::Unknown) {
                if mir.iter().any(|m| matches!(m, crate::frame_c::v4::mir::MirItem::Forward { .. })) {
                    let enclosing_state = b.state_id.as_deref();
                    let mut ok_parent = false;
                    if let Some(state_name) = enclosing_state {
                        if let Some(ref arc) = arcanum_symtab {
                            let sys = system_name.as_deref().unwrap_or("_");
                            ok_parent = arc.has_parent(sys, state_name) || arc.has_parent("_", state_name);
                        }
                    }
                    if !ok_parent {
                        all_issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: "E403: Cannot forward to parent: no parent available".into() });
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
        if matches!(b.kind, BodyKind::Handler | BodyKind::Action | BodyKind::Operation) {
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
                use crate::frame_c::v4::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v4::native_region_scanner::Region::FrameSegment{ indent, .. } = r {
                        if mi >= mir.len() { break; }
                        let m = &mir[mi];
                        mi += 1;
                        let s = match lang {
                            TargetLanguage::Python3 => PyFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::TypeScript => TsFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::CSharp => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::C => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Cpp => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Java => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Rust => RustFacadeExpander.expand(m, *indent, None),
                            _ => String::new(),
                        };
                        v.push(s);
                    }
                }
                v
            };
            let spliced = Splicer.splice(body_bytes, &scan.regions, &exps);
            // Optional native parsing via facades (adapter may no-op if feature not enabled)
            if let Some(facade) = crate::frame_c::v4::facade::NativeFacadeRegistry::get(lang) {
                if let Ok(diags) = facade.parse(&spliced.text) {
                    for d in diags {
                        if let Some((origin, src)) = spliced.map_spliced_range_to_origin(d.start, d.end) {
                            let origin_str = match origin { crate::frame_c::v4::splice::Origin::Frame{..} => "frame", crate::frame_c::v4::splice::Origin::Native{..} => "native" };
                            all_issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: format!("native syntax ({}:{}-{}): {}", origin_str, src.start, src.end, d.message) });
                        } else {
                            all_issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: format!("native syntax: {}", d.message) });
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
    Ok(ValidationResult { ok, issues: all_issues })
}

/// Validate a module file using a pre-built project Arcanum (cross-file symbol table).
/// Mirrors validate_module_demo_with_mode but uses the provided Arcanum for
/// transition target and parent-forward checks.
pub fn validate_module_with_arcanum(
    content_str: &str,
    lang: TargetLanguage,
    arc: &crate::frame_c::v4::arcanum::Arcanum,
    strict_native: bool,
) -> Result<ValidationResult, RunError> {
    let bytes = content_str.as_bytes();
    let parts = match module_partitioner::ModulePartitioner::partition(bytes, lang) {
        Ok(p) => p,
        Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &e.0)),
    };
    let validator = Validator;
    let mut all_issues = Vec::new();
    // include import scanning issues
    all_issues.extend(parts.import_issues.into_iter());
    // Outline grammar and section checks
    let outline_start = parts.imports.last().map(|s| s.end).or(parts.prolog.as_ref().map(|p| p.end)).unwrap_or(0);
    let (items, outline_issues) = crate::frame_c::v4::outline_scanner::OutlineScanner.scan_collect(bytes, outline_start, lang);
    all_issues.extend(outline_issues);
    let outer_issues = validator.validate_outer_grammar(bytes, outline_start, lang, &items);
    all_issues.extend(outer_issues);
    // Per-system block ordering and uniqueness (operations:, interface:, machine:, actions:, domain:) using ModuleAst.
    let module_ast = SystemParser::parse_module(bytes, lang);
    let block_order_issues = validator.validate_system_block_order_ast(&module_ast);
    all_issues.extend(block_order_issues);
    // Enforce single `fn main` per module.
    let main_issues = validator.validate_main_functions(bytes, &items);
    all_issues.extend(main_issues);
    // Machine state headers and handler placement must be validated via ModuleAst/Arcanum.
    let state_issues = validator.validate_machine_state_headers_ast(bytes, &module_ast);
    all_issues.extend(state_issues);
    let arc_for_ctx = crate::frame_c::v4::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
    let handler_scope_issues =
        validator.validate_handlers_in_state_ast(bytes, &items, &module_ast, &arc_for_ctx);
    all_issues.extend(handler_scope_issues);

    let system_name = find_system_name(bytes, 0);
    let known_states = validator.collect_machine_state_names(bytes, outline_start);

    for b in parts.bodies {
        let body_bytes = &bytes[b.open_byte..=b.close_byte];
        let scan_res = match lang {
            TargetLanguage::Python3 => nscan::python::NativeRegionScannerPy.scan(body_bytes, 0),
            TargetLanguage::TypeScript => nscan::typescript::NativeRegionScannerTs.scan(body_bytes, 0),
            TargetLanguage::CSharp => nscan::csharp::NativeRegionScannerCs.scan(body_bytes, 0),
            TargetLanguage::C => nscan::c::NativeRegionScannerC.scan(body_bytes, 0),
            TargetLanguage::Cpp => nscan::cpp::NativeRegionScannerCpp.scan(body_bytes, 0),
            TargetLanguage::Java => nscan::java::NativeRegionScannerJava.scan(body_bytes, 0),
            TargetLanguage::Rust => nscan::rust::NativeRegionScannerRust.scan(body_bytes, 0),
            _ => return Err(RunError::new(frame_exitcode::PARSE_ERR, "target not supported")),
        };
        let scan = match scan_res { Ok(s) => s, Err(e) => return Err(RunError::new(frame_exitcode::PARSE_ERR, &format!("Scan error: {:?}", e))) };
        let (mir, parse_issues) = MirAssembler.assemble_collect(body_bytes, &scan.regions);
        if !parse_issues.is_empty() { all_issues.extend(parse_issues); }
        let policy = ValidatorPolicy { body_kind: Some(b.kind) };
        let mut res = validator.validate_regions_mir_with_policy(&scan.regions, &mir, policy);
        // Cross-file transition targets
        let sys = system_name.as_deref();
        if !known_states.is_empty() {
            res.issues.extend(
                validator.validate_transition_targets_arcanum(&mir, arc, &known_states, sys)
            );
        }
        // Optional advisory policy: state parameter arity (Stage 10B).
        if std::env::var("FRAME_VALIDATE_NATIVE_POLICY").ok().as_deref() == Some("1") {
            res.issues.extend(validator.validate_transition_state_arity_arcanum(&mir, arc, sys));
        }
        // Parent-forward availability via Arcanum
        if validator.has_machine_section(bytes, outline_start) {
            if matches!(b.kind, BodyKind::Handler | BodyKind::Unknown) {
                if mir.iter().any(|m| matches!(m, crate::frame_c::v4::mir::MirItem::Forward { .. })) {
                    let enclosing_state = b.state_id.as_deref();
                    let mut ok_parent = false;
                    if let Some(state_name) = enclosing_state {
                        let sys_name = system_name.as_deref().unwrap_or("_");
                        ok_parent = arc.has_parent(sys_name, state_name) || arc.has_parent("_", state_name);
                    }
                    if !ok_parent {
                        res.issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: "E403: Cannot forward to parent: no parent available".into() });
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
                use crate::frame_c::v4::expander::*;
                let mut v = Vec::new();
                let mut mi = 0usize;
                for r in &scan.regions {
                    if let crate::frame_c::v4::native_region_scanner::Region::FrameSegment{ indent, .. } = r {
                        if mi >= mir.len() { break; }
                        let m = &mir[mi]; mi += 1;
                        let s = match lang {
                            TargetLanguage::Python3 => PyFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::TypeScript => TsFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::CSharp => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::C => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Cpp => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Java => CFacadeExpander.expand(m, *indent, None),
                            TargetLanguage::Rust => RustFacadeExpander.expand(m, *indent, None),
                            _ => String::new(),
                        };
                        v.push(s);
                    }
                }
                v
            };
            let spliced = Splicer.splice(body_bytes, &scan.regions, &exps);
            if let Some(facade) = crate::frame_c::v4::facade::NativeFacadeRegistry::get(lang) {
                if let Ok(diags) = facade.parse(&spliced.text) {
                    for d in diags {
                        if let Some((_origin, src)) = spliced.map_spliced_range_to_origin(d.start, d.end) {
                            all_issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: format!("native syntax (frame:{}-{}): {}", src.start, src.end, d.message) });
                        } else {
                            all_issues.push(crate::frame_c::v4::validator::ValidationIssue{ message: format!("native syntax: {}", d.message) });
                        }
                    }
                }
            }
        }
    }
    let ok = all_issues.is_empty();
    Ok(ValidationResult { ok, issues: all_issues })
}

// SOL-anchored scan for `system <Ident> {` ignoring common comments
pub fn find_system_name(bytes: &[u8], start: usize) -> Option<String> {
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
        
        // V4: Check for @@system first
        if i + 8 < n && &bytes[i..i+8] == b"@@system" {
            i += 8;
            // skip whitespace
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
            // read system name
            let name_start = i;
            while i < n && ((bytes[i] as char).is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            if i > name_start {
                return Some(String::from_utf8_lossy(&bytes[name_start..i]).to_string());
            }
        }
        
        // read ident (for compatibility)
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
            // Continue scanning the rest of the line so we can catch `system`
            // after leading annotations (e.g., `@@persist @@system Foo {`).
            i = j;
            continue;
        }
        // Non-identifier character: advance one byte and keep scanning
        i += 1;
    }
    None
}

// SOL-anchored scan for `module <Ident> {` ignoring common comments
pub fn find_module_name(bytes: &[u8], start: usize) -> Option<String> {
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
            if kw == "module" {
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                let name_start = j;
                while j < n && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
                if j > name_start {
                    return Some(String::from_utf8_lossy(&bytes[name_start..j]).to_string());
                }
            }
            // Continue scanning the rest of the line so we can catch `module`
            // after leading annotations.
            i = j;
            continue;
        }
        // Non-identifier character: advance one byte and keep scanning
        i += 1;
    }
    None
}

// Minimal domain: scanner for Python runnable modules.
// Scans for a top-level `domain:` block and extracts Frame-style domain
// variables into (name, type, initializer) triples.
fn scan_py_domain_fields(bytes: &[u8], target_system: &str) -> Vec<(String, Option<String>, Option<String>)> {
    // Reuse the TypeScript scanner since the Frame syntax is the same
    scan_ts_domain_fields(bytes, target_system)
}

// Minimal domain: scanner for TypeScript runnable modules.
// Scans for a top-level `domain:` block and extracts Frame-style domain
// variables into (name, type, initializer) triples.
fn scan_ts_domain_fields(bytes: &[u8], target_system: &str) -> Vec<(String, Option<String>, Option<String>)> {
    // Find the target system block and its domain: span, mirroring the
    // structure used by DomainBlockScanner.
    fn is_space(b: u8) -> bool {
        b == b' ' || b == b'\t'
    }

    // Locate the start/end byte offsets of the domain block for the given system.
    fn find_domain_span(
        bytes: &[u8],
        target_system: &str,
    ) -> Option<(usize, usize)> {
        use crate::frame_c::v4::body_closer;
        let n = bytes.len();
        let mut i = 0usize;
        while i < n {
            // Skip whitespace and blank lines.
            while i < n
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n')
            {
                i += 1;
            }
            if i >= n {
                break;
            }
            // Skip comment-only lines.
            if bytes[i] == b'#' {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            if i + 1 < n && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }

            // Look for `system` keyword at SOL.
            let mut j = i;
            while j < n && is_space(bytes[j]) {
                j += 1;
            }
            let kw_start = j;
            while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if kw_start == j {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
            if kw.as_str() != "system" {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }

            // Read system name.
            let mut k = j;
            while k < n && is_space(bytes[k]) {
                k += 1;
            }
            let name_start = k;
            while k < n && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') {
                k += 1;
            }
            if name_start == k {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let sys_name_cur = String::from_utf8_lossy(&bytes[name_start..k]).to_string();

            // Only consider the target system.
            if sys_name_cur != target_system {
                // Skip this system's block entirely.
                while i < n && bytes[i] != b'{' && bytes[i] != b'\n' {
                    i += 1;
                }
                if i >= n || bytes[i] != b'{' {
                    continue;
                }
                let open = i;
                let close = body_closer::typescript::BodyCloserTs
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c)
                    .unwrap_or(open);
                i = close + 1;
                continue;
            }

            // Find opening '{' for this system.
            while k < n && bytes[k] != b'{' && bytes[k] != b'\n' {
                k += 1;
            }
            if k >= n || bytes[k] != b'{' {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let open = k;
            let close = match body_closer::typescript::BodyCloserTs.close_byte(&bytes[open..], 0)
            {
                Ok(c) => open + c,
                Err(_) => {
                    while i < n && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
            };

            // Collect section markers inside this system.
            let mut marks: Vec<(usize, String)> = Vec::new();
            let mut q = open + 1;
            while q < close {
                while q < close
                    && (bytes[q] == b' ' || bytes[q] == b'\t' || bytes[q] == b'\r' || bytes[q] == b'\n')
                {
                    q += 1;
                }
                if q >= close {
                    break;
                }
                let line = q;
                // Skip comments.
                if bytes[q] == b'#' {
                    while q < close && bytes[q] != b'\n' {
                        q += 1;
                    }
                    continue;
                }
                if q + 1 < close && bytes[q] == b'/' {
                    let c2 = bytes[q + 1];
                    if c2 == b'/' {
                        while q < close && bytes[q] != b'\n' {
                            q += 1;
                        }
                        continue;
                    } else if c2 == b'*' {
                        q += 2;
                        while q + 1 < close {
                            if bytes[q] == b'*' && bytes[q + 1] == b'/' {
                                q += 2;
                                break;
                            }
                            q += 1;
                        }
                        continue;
                    }
                }
                let mut s = q;
                while s < close && (bytes[s] == b' ' || bytes[s] == b'\t') {
                    s += 1;
                }
                let sec_start = s;
                while s < close && (bytes[s].is_ascii_alphanumeric() || bytes[s] == b'_') {
                    s += 1;
                }
                if sec_start < s && s < close && bytes[s] == b':' {
                    let kw_sec =
                        String::from_utf8_lossy(&bytes[sec_start..s]).to_ascii_lowercase();
                    if kw_sec.as_str() == "domain"
                        || kw_sec.as_str() == "operations"
                        || kw_sec.as_str() == "interface"
                        || kw_sec.as_str() == "machine"
                        || kw_sec.as_str() == "actions"
                    {
                        marks.push((line, kw_sec));
                    }
                }
                while q < close && bytes[q] != b'\n' {
                    q += 1;
                }
            }

            // Locate the domain block span within this system, if present.
            if let Some((idx, _)) = marks
                .iter()
                .enumerate()
                .find(|(_, (_, kw_sec))| kw_sec.as_str() == "domain")
            {
                let dom_start_line = marks[idx].0;
                let dom_end = if idx + 1 < marks.len() {
                    marks[idx + 1].0
                } else {
                    close
                };
                // Skip the `domain:` line itself.
                let mut p = dom_start_line;
                while p < dom_end && bytes[p] != b'\n' {
                    p += 1;
                }
                if p < dom_end {
                    p += 1;
                }
                return Some((p, dom_end));
            }

            // No domain block for this system.
            return None;
        }
        None
    }

    let (dom_start, dom_end) = match find_domain_span(bytes, target_system) {
        Some(span) => span,
        None => return Vec::new(),
    };

    let mut p = dom_start;
    let mut out: Vec<(String, Option<String>, Option<String>)> = Vec::new();
    while p < dom_end {
        let line_start = p;
        while p < dom_end && bytes[p] != b'\n' { p += 1; }
        let line_end = p;
        if p < dom_end { p += 1; }
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

// V4 Compiler interface for CLI compatibility
pub struct FrameV4Compiler {
    target: TargetLanguage,
}

impl FrameV4Compiler {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
    
    pub fn compile(&self, source: &str, _file_path: &str) -> FrameV4Result {
        match compile_module(source, self.target) {
            Ok(code) => FrameV4Result::Ok(FrameV4Output {
                code,
                warnings: Vec::new(),
                source_map: None,
            }),
            Err(e) => FrameV4Result::Err(ErrorsAcc {
                errors: vec![e.error],
            }),
        }
    }
}

// V4 Result types for CLI compatibility
pub enum FrameV4Result {
    Ok(FrameV4Output),
    Err(ErrorsAcc),
}

pub struct FrameV4Output {
    pub code: String,
    pub warnings: Vec<String>,
    pub source_map: Option<String>,
}

pub struct ErrorsAcc {
    pub errors: Vec<String>,
}

impl ErrorsAcc {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn push_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}
