use crate::frame_c::visitors::TargetLanguage;

#[derive(Debug, Clone)]
pub struct NativeDiagnostic {
    pub start: usize,
    pub end: usize,
    pub message: String,
}

pub trait NativeParseFacade: Send + Sync {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String>;
}

pub struct NativeFacadeRegistry;

impl NativeFacadeRegistry {
    pub fn get(lang: TargetLanguage) -> Option<&'static dyn NativeParseFacade> {
        match lang {
            TargetLanguage::Python3 => Some(&PY_FACADE),
            TargetLanguage::TypeScript => Some(&TS_FACADE),
            TargetLanguage::Rust => Some(&RUST_FACADE),
            TargetLanguage::C => Some(&C_FACADE),
            TargetLanguage::Cpp => Some(&CPP_FACADE),
            TargetLanguage::Java => Some(&JAVA_FACADE),
            TargetLanguage::CSharp => Some(&CS_FACADE),
            _ => None,
        }
    }
}

// Simple, hermetic facades that validate our wrapper-call syntax only.

struct PyWrapperFacade;
struct TsWrapperFacade;
struct CWrapperFacade;
struct CppWrapperFacade;
struct JavaWrapperFacade;
struct CsWrapperFacade;
struct RustWrapperFacade;

static PY_FACADE: PyWrapperFacade = PyWrapperFacade;
static TS_FACADE: TsWrapperFacade = TsWrapperFacade;
static C_FACADE: CWrapperFacade = CWrapperFacade;
static CPP_FACADE: CppWrapperFacade = CppWrapperFacade;
static JAVA_FACADE: JavaWrapperFacade = JavaWrapperFacade;
static CS_FACADE: CsWrapperFacade = CsWrapperFacade;
static RUST_FACADE: RustWrapperFacade = RustWrapperFacade;

impl NativeParseFacade for PyWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        let mut diags = Vec::new();
        let bytes = spliced_text.as_bytes();
        let mut i = 0usize; let n = bytes.len();
        while i < n {
            let line_start = i; while i < n && bytes[i] != b'\n' { i += 1; }
            let line_end = i; if i < n { i += 1; } // skip newline
            // trim leading spaces
            let mut s = line_start; while s < line_end && (bytes[s] == b' ' || bytes[s] == b'\t') { s += 1; }
            if s >= line_end { continue; }
            // Check wrapper calls
            if starts_with(bytes, s, b"__frame_transition") {
                // Must contain balanced parens and NOT end with ';'
                if !has_balanced_parens(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "unbalanced parentheses in wrapper".into() });
                } else {
                    if let Some((arg_start, arg_end)) = paren_payload(bytes, s, line_end) {
                        if let Some((state_ok, msg)) = check_transition_first_arg(bytes, arg_start, arg_end, /*require_semicolon*/ false, /*allow_semicolon*/ false) {
                            if !state_ok { diags.push(NativeDiagnostic{ start: s, end: line_end, message: msg }); }
                        }
                    }
                }
                if ends_with_semicolon(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "semicolon not allowed in Python wrapper".into() });
                }
            } else if starts_with(bytes, s, b"__frame_forward") || starts_with(bytes, s, b"__frame_stack_") {
                // forward/stack wrappers: no semicolon; check balanced parens and zero-arg
                if !has_balanced_parens(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "unbalanced parentheses in wrapper".into() });
                } else if let Some((arg_start, arg_end)) = paren_payload(bytes, s, line_end) {
                    if has_non_ws(bytes, arg_start, arg_end) {
                        diags.push(NativeDiagnostic{ start: s, end: line_end, message: "wrapper takes no arguments".into() });
                    }
                }
                if ends_with_semicolon(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "semicolon not allowed in Python wrapper".into() });
                }
            }
        }
        // Optional native parser diagnostics (Python) — prefer RustPython parser when enabled,
        // otherwise fall back to tree-sitter adapter. Both are feature-gated.
        if let Some(mut extra) = run_python_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for TsWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        let mut diags = Vec::new();
        let bytes = spliced_text.as_bytes();
        let mut i = 0usize; let n = bytes.len();
        while i < n {
            let line_start = i; while i < n && bytes[i] != b'\n' { i += 1; }
            let line_end = i; if i < n { i += 1; }
            let mut s = line_start; while s < line_end && (bytes[s] == b' ' || bytes[s] == b'\t') { s += 1; }
            if s >= line_end { continue; }
            if starts_with(bytes, s, b"__frame_transition") || starts_with(bytes, s, b"__frame_forward") || starts_with(bytes, s, b"__frame_stack_") {
                if !has_balanced_parens(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "unbalanced parentheses in wrapper".into() });
                } else {
                    if starts_with(bytes, s, b"__frame_transition") {
                        if let Some((arg_start, arg_end)) = paren_payload(bytes, s, line_end) {
                            if let Some((state_ok, msg)) = check_transition_first_arg(bytes, arg_start, arg_end, /*require_semicolon*/ true, /*allow_semicolon*/ true) {
                                if !state_ok { diags.push(NativeDiagnostic{ start: s, end: line_end, message: msg }); }
                            }
                        }
                    } else if let Some((arg_start, arg_end)) = paren_payload(bytes, s, line_end) {
                        if has_non_ws(bytes, arg_start, arg_end) {
                            diags.push(NativeDiagnostic{ start: s, end: line_end, message: "wrapper takes no arguments".into() });
                        }
                    }
                }
                if !ends_with_semicolon(bytes, s, line_end) {
                    diags.push(NativeDiagnostic{ start: s, end: line_end, message: "missing semicolon terminator".into() });
                }
            }
        }
        // Optional native parser diagnostics (SWC) — feature-gated
        if let Some(mut extra) = run_ts_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for CWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        // Wrapper-only checks, then optional structural C parsing
        let mut diags = TsWrapperFacade.parse(spliced_text)?;
        if let Some(mut extra) = run_c_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for CppWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        // Wrapper-only checks, then optional structural C++ parsing
        let mut diags = TsWrapperFacade.parse(spliced_text)?;
        if let Some(mut extra) = run_cpp_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for JavaWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        let mut diags = TsWrapperFacade.parse(spliced_text)?;
        if let Some(mut extra) = run_java_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for CsWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        let mut diags = TsWrapperFacade.parse(spliced_text)?;
        if let Some(mut extra) = run_csharp_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

impl NativeParseFacade for RustWrapperFacade {
    fn parse(&self, spliced_text: &str) -> Result<Vec<NativeDiagnostic>, String> {
        // Reuse C-like wrapper checks, then append Rust parser diagnostics when enabled
        let mut diags = C_FACADE.parse(spliced_text)?;
        if let Some(mut extra) = run_rust_adapter(spliced_text) { diags.append(&mut extra); }
        Ok(diags)
    }
}

fn starts_with(hay: &[u8], start: usize, needle: &[u8]) -> bool {
    let m = start + needle.len();
    m <= hay.len() && &hay[start..m] == needle
}

fn has_balanced_parens(hay: &[u8], start: usize, end: usize) -> bool {
    let mut seen_open = false; let mut depth = 0i32; let mut i = start;
    while i < end {
        let b = hay[i];
        if b == b'(' { seen_open = true; depth += 1; }
        if b == b')' { depth -= 1; }
        i += 1;
    }
    (!seen_open) || depth == 0
}

fn ends_with_semicolon(hay: &[u8], start: usize, end: usize) -> bool {
    let mut i = end; while i > start { i -= 1; let b = hay[i]; if b == b'\n' || b == b'\r' { continue; } if b == b' ' || b == b'\t' { continue; } return b == b';'; }
    false
}

fn has_non_ws(hay: &[u8], mut start: usize, mut end: usize) -> bool {
    while start < end && (hay[start] == b' ' || hay[start] == b'\t') { start += 1; }
    while end > start && (hay[end-1] == b' ' || hay[end-1] == b'\t') { end -= 1; }
    start < end
}

fn paren_payload(hay: &[u8], start: usize, end: usize) -> Option<(usize, usize)> {
    // find first '(' after start, match to corresponding ')'
    let mut i = start; while i < end && hay[i] != b'(' { i += 1; }
    if i >= end { return None; }
    let mut depth = 0i32; let open = i; i += 1; depth += 1;
    while i < end { let b = hay[i]; if b == b'(' { depth += 1; } else if b == b')' { depth -= 1; if depth == 0 { return Some((open+1, i)); } } i += 1; }
    None
}

fn check_transition_first_arg(hay: &[u8], arg_start: usize, arg_end: usize, _require_semicolon: bool, _allow_semicolon: bool) -> Option<(bool, String)> {
    // First non-ws must be quote (' or "), then state ident, then matching closing quote
    let mut i = arg_start; while i < arg_end && (hay[i] == b' ' || hay[i] == b'\t') { i += 1; }
    if i >= arg_end || (hay[i] != b'\'' && hay[i] != b'\"') { return Some((false, "transition wrapper: first argument must be quoted state".into())); }
    let q = hay[i];
    i += 1; let name_start = i; while i < arg_end && (hay[i].is_ascii_alphanumeric() || hay[i] == b'_') { i += 1; }
    if i == name_start { return Some((false, "transition wrapper: empty state name".into())); }
    let first = hay[name_start]; if !(first.is_ascii_alphabetic() || first == b'_') { return Some((false, "transition wrapper: invalid state identifier".into())); }
    if i >= arg_end || hay[i] != q { return Some((false, "transition wrapper: first argument must be quoted state".into())); }
    // ok; remaining can be optional comma and other args
    Some((true, String::new()))
}

// --- Optional native parser adapters ---

#[cfg(feature = "native-ts")]
fn run_ts_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    use swc_common::{sync::Lrc, FileName, SourceMap, Span, Spanned};
    use swc_ecma_ast::EsVersion;
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Custom("spliced.ts".into()).into(), text.to_string());
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    for e in parser.take_errors() {
        let span: Span = e.span();
        let start = span.lo.0 as usize; let end = span.hi.0 as usize;
        out.push(NativeDiagnostic { start, end, message: format!("native facade (TypeScript): {:?}", e) });
    }
    match parser.parse_script() {
        Ok(_) => {}
        Err(e) => {
            let span: Span = e.span();
            let start = span.lo.0 as usize; let end = span.hi.0 as usize;
            out.push(NativeDiagnostic { start, end, message: format!("native facade (TypeScript): {:?}", e) });
        }
    }
    // Collect any additional errors produced during/after parse
    for e in parser.take_errors() {
        let span: Span = e.span();
        let start = span.lo.0 as usize; let end = span.hi.0 as usize;
        out.push(NativeDiagnostic { start, end, message: format!("native facade (TypeScript): {:?}", e) });
    }
    Some(out)
}

#[cfg(not(feature = "native-ts"))]
fn run_ts_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }

#[cfg(feature = "native-rs")]
fn run_rust_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    let wrapped = format!("{{\n{}\n}}", text);
    if let Err(e) = syn::parse_str::<syn::Block>(&wrapped) {
        out.push(NativeDiagnostic { start: 0, end: 0, message: format!("native facade (Rust): {}", e) });
    }
    Some(out)
}

#[cfg(not(feature = "native-rs"))]
fn run_rust_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }

#[cfg(feature = "native-c")]
fn run_c_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    use tree_sitter::{Parser, Node};
    fn collect_errors(node: Node, out: &mut Vec<(usize, usize)>) {
        if node.is_error() || node.is_missing() { out.push((node.start_byte(), node.end_byte())); }
        for i in 0..node.child_count() {
            if let Some(ch) = node.child(i) { collect_errors(ch, out); }
        }
    }
    let prefix = "void __framec_facade(void){\n";
    let suffix = "\n}";
    let wrapped = format!("{}{}{}", prefix, text, suffix);
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c::language()).ok()?;
    let tree = parser.parse(&wrapped, None)?;
    let root = tree.root_node();
    let mut errs = Vec::new();
    collect_errors(root, &mut errs);
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    for (mut s, mut e) in errs {
        let pre = prefix.len();
        if s < pre { s = pre; }
        if e < pre { e = pre; }
        s -= pre; e -= pre;
        if s > text.len() { s = text.len(); }
        if e > text.len() { e = text.len(); }
        out.push(NativeDiagnostic { start: s, end: e, message: "native facade (C): parse error".into() });
    }
    Some(out)
}

#[cfg(not(feature = "native-c"))]
fn run_c_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }

#[cfg(feature = "native-cpp")]
fn run_cpp_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    use tree_sitter::{Parser, Node};
    fn collect_errors(node: Node, out: &mut Vec<(usize, usize)>) {
        if node.is_error() || node.is_missing() { out.push((node.start_byte(), node.end_byte())); }
        for i in 0..node.child_count() {
            if let Some(ch) = node.child(i) { collect_errors(ch, out); }
        }
    }
    let prefix = "void __framec_facade(){\n";
    let suffix = "\n}";
    let wrapped = format!("{}{}{}", prefix, text, suffix);
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_cpp::language()).ok()?;
    let tree = parser.parse(&wrapped, None)?;
    let root = tree.root_node();
    let mut errs = Vec::new();
    collect_errors(root, &mut errs);
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    for (mut s, mut e) in errs {
        let pre = prefix.len();
        if s < pre { s = pre; }
        if e < pre { e = pre; }
        s -= pre; e -= pre;
        if s > text.len() { s = text.len(); }
        if e > text.len() { e = text.len(); }
        out.push(NativeDiagnostic { start: s, end: e, message: "native facade (C++): parse error".into() });
    }
    Some(out)
}

#[cfg(not(feature = "native-cpp"))]
fn run_cpp_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }

// Python native parsing adapter selection:
// - If feature "native-py-rp" is enabled, use RustPython parser (pure Rust, hermetic)
// - Else if feature "native-py" is enabled, use tree-sitter-python
// - Else, return None
fn run_python_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> {
    // Prefer RustPython-based parser when available
    #[cfg(feature = "native-py-rp")]
    {
        // rustpython-parser reports errors with line/column; convert to byte offsets best-effort
        // to integrate with splice_map remapping. We keep messages intact.
        use rustpython_parser::parser;
        use rustpython_parser::mode::Mode;
        let parse_result = parser::parse(text, Mode::Module);
        match parse_result {
            Ok(_ast) => return Some(Vec::new()),
            Err(err) => {
                // Convert Location (1-based lines/columns) to byte index
                fn offset_from_line_col(src: &str, line: usize, col: usize) -> usize {
                    if line == 0 { return 0; }
                    let mut cur_line = 1usize; let mut idx = 0usize; let bytes = src.as_bytes();
                    while idx < bytes.len() && cur_line < line {
                        if bytes[idx] == b'\n' { cur_line += 1; }
                        idx += 1;
                    }
                    idx + col.saturating_sub(1)
                }
                let msg = format!("native facade (Python/RustPython): {}", err);
                // Best-effort: rustpython-parser::error::ParseError exposes location()
                let (s, e) = match err.location() {
                    Some(loc) => {
                        let start = offset_from_line_col(text, loc.row().get(), loc.column().get());
                        (start, start)
                    }
                    None => (0usize, 0usize)
                };
                return Some(vec![NativeDiagnostic { start: s, end: e, message: msg }]);
            }
        }
    }
    // Tree-sitter fallback when RustPython feature is not enabled
    #[cfg(all(not(feature = "native-py-rp"), feature = "native-py"))]
    {
        use tree_sitter::{Parser, Node};
        fn collect_errors(node: Node, out: &mut Vec<(usize, usize)>) {
            if node.is_error() || node.is_missing() { out.push((node.start_byte(), node.end_byte())); }
            for i in 0..node.child_count() { if let Some(ch) = node.child(i) { collect_errors(ch, out); } }
        }
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_python::language()).ok()?;
        let tree = parser.parse(text, None)?;
        let root = tree.root_node();
        let mut errs = Vec::new();
        collect_errors(root, &mut errs);
        let mut out: Vec<NativeDiagnostic> = Vec::new();
        for (s, e) in errs { out.push(NativeDiagnostic { start: s, end: e, message: "native facade (Python): parse error".into() }); }
        return Some(out);
    }
    // No adapter available
    #[cfg(all(not(feature = "native-py-rp"), not(feature = "native-py")))]
    {
        return None;
    }
}

#[cfg(feature = "native-java")]
fn run_java_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    use tree_sitter::{Parser, Node};
    fn collect_errors(node: Node, out: &mut Vec<(usize, usize)>) {
        if node.is_error() || node.is_missing() { out.push((node.start_byte(), node.end_byte())); }
        for i in 0..node.child_count() {
            if let Some(ch) = node.child(i) { collect_errors(ch, out); }
        }
    }
    let prefix = "class __framec_facade{ void __m(){\n";
    let suffix = "\n}}";
    let wrapped = format!("{}{}{}", prefix, text, suffix);
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_java::language()).ok()?;
    let tree = parser.parse(&wrapped, None)?;
    let root = tree.root_node();
    let mut errs = Vec::new();
    collect_errors(root, &mut errs);
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    for (mut s, mut e) in errs {
        let pre = prefix.len();
        if s < pre { s = pre; }
        if e < pre { e = pre; }
        s -= pre; e -= pre;
        if s > text.len() { s = text.len(); }
        if e > text.len() { e = text.len(); }
        out.push(NativeDiagnostic { start: s, end: e, message: "native facade (Java): parse error".into() });
    }
    Some(out)
}

#[cfg(not(feature = "native-java"))]
fn run_java_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }

#[cfg(feature = "native-csharp")]
fn run_csharp_adapter(text: &str) -> Option<Vec<NativeDiagnostic>> {
    use tree_sitter::{Parser, Node};
    fn collect_errors(node: Node, out: &mut Vec<(usize, usize)>) {
        if node.is_error() || node.is_missing() { out.push((node.start_byte(), node.end_byte())); }
        for i in 0..node.child_count() {
            if let Some(ch) = node.child(i) { collect_errors(ch, out); }
        }
    }
    let prefix = "class __FramecFacade{ void M(){\n";
    let suffix = "\n}}";
    let wrapped = format!("{}{}{}", prefix, text, suffix);
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c_sharp::language()).ok()?;
    let tree = parser.parse(&wrapped, None)?;
    let root = tree.root_node();
    let mut errs = Vec::new();
    collect_errors(root, &mut errs);
    let mut out: Vec<NativeDiagnostic> = Vec::new();
    for (mut s, mut e) in errs {
        let pre = prefix.len();
        if s < pre { s = pre; }
        if e < pre { e = pre; }
        s -= pre; e -= pre;
        if s > text.len() { s = text.len(); }
        if e > text.len() { e = text.len(); }
        out.push(NativeDiagnostic { start: s, end: e, message: "native facade (C#): parse error".into() });
    }
    Some(out)
}

#[cfg(not(feature = "native-csharp"))]
fn run_csharp_adapter(_text: &str) -> Option<Vec<NativeDiagnostic>> { None }
