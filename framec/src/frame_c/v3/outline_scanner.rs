use super::native_region_scanner::RegionSpan;
use super::validator::BodyKindV3;
use super::body_closer as closer;
use super::body_closer::BodyCloserV3;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::validator::ValidationIssueV3;

#[derive(Debug, Clone)]
pub struct OutlineItemV3 {
    pub header_span: RegionSpan,
    pub owner_id: Option<String>,
    pub state_id: Option<String>,
    pub kind: BodyKindV3,
    pub open_byte: usize,
    pub close_byte: usize,
}

#[derive(Debug)]
pub struct OutlineErrorV3 { pub message: String }

pub struct OutlineScannerV3;

impl OutlineScannerV3 {
    // SOL-anchored scan for module artifacts per V3 grammar: sections + IDENT '('...')' '{'
    pub fn scan(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> Result<Vec<OutlineItemV3>, OutlineErrorV3> {
        let mut items: Vec<OutlineItemV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Section { None, Actions, Operations, Interface, Machine }
        let mut section = Section::None;
        // Track active state scopes (name, close_index) inside machine:
        let mut state_scopes: Vec<(String, usize)> = Vec::new();
        // Track body scopes for handlers/actions/operations/functions so we don't
        // misinterpret inner statements (e.g., `print(...)`) as headers.
        let mut body_scopes: Vec<(usize, usize)> = Vec::new();
        while i < n {
            // skip to SOL non-space
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // Drop any state scopes that ended before this line
            while let Some((_, close)) = state_scopes.last() { if *close <= line_start { state_scopes.pop(); } else { break; } }
            // Skip lines that are inside an already-recorded body scope
            let mut inside_body = false;
            for (open, close) in &body_scopes {
                if line_start > *open && line_start < *close {
                    inside_body = true;
                    break;
                }
            }
            if inside_body {
                while i < n && bytes[i] != b'\n' { i += 1; }
                continue;
            }
            // first token
            let mut j = i;
            while j < n && is_space(bytes[j]) { j += 1; }
            let kw_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            let kw = to_lower_ascii(&bytes[kw_start..j]);
            // Section markers
            if j < n && bytes[j] == b':' {
                match kw.as_str() {
                    "actions" => section = Section::Actions,
                    "operations" => section = Section::Operations,
                    "interface" => section = Section::Interface,
                    "machine" => section = Section::Machine,
                    _ => {}
                }
                if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
                    eprintln!("[outline] section={:?} at byte {}", section, line_start);
                }
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            // Track state scopes: in machine:, a line starting with '$Name {' begins a state block.
            // Require an identifier after '$' to avoid treating entry handlers like '$>()' as states.
            if matches!(section, Section::Machine) && bytes[kw_start] == b'$' {
                let ident_start = kw_start + 1;
                let mut s = ident_start;
                while s < n && is_ident(bytes[s]) { s += 1; }
                // Must have at least one ident char and a valid ident start.
                let is_state_header = s > ident_start
                    && ((bytes[ident_start] as char).is_ascii_alphabetic() || bytes[ident_start] == b'_');
                if !is_state_header {
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                    continue;
                }
                // find '{' on this line
                let mut p = s; while p < n && bytes[p] != b'\n' && bytes[p] != b'{' { p += 1; }
                if p < n && bytes[p] == b'{' {
                    let open = p;
                    // compute close; ignore errors in this fast path
                    let close_opt: Option<usize> = match lang {
                        TargetLanguage::Python3 => closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::C => closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Java => closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Rust => closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        _ => None,
                    };
                    if let Some(close) = close_opt {
                        let name = String::from_utf8_lossy(&bytes[ident_start..s]).to_string();
                        state_scopes.push((name, close));
                    }
                }
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            // Recognize headers:
            // - Global functions: 'fn name(...) { ... }' or 'async fn name(...) { ... }'.
            // - Section members (machine/actions/operations/interface): IDENT '(' ... ')' '{'
            //   with optional leading 'async' before the name (e.g., 'async run() { ... }').
            //   (Interface headers without a '{' are treated as prototypes and ignored.)
            let mut name_start = kw_start;
            let mut name_end = j;
            let first_tok = to_lower_ascii(&bytes[name_start..name_end]);
            let mut k = j; while k < n && is_space(bytes[k]) { k += 1; }
            let mut is_func_header = false;
            let mut is_global_fn = false;
            // Global functions: fn name(...) or async fn name(...)
            let mut is_global_candidate = false;
            if first_tok == "fn" {
                is_global_candidate = true;
            } else if first_tok == "async" {
                // Look ahead for 'fn' after async
                let mut next = j; while next < n && is_space(bytes[next]) { next += 1; }
                let mut w = next; while w < n && is_ident(bytes[w]) { w += 1; }
                let maybe_fn = to_lower_ascii(&bytes[next..w]);
                if maybe_fn == "fn" {
                    is_global_candidate = true;
                    // Position k at start of name after 'fn'
                    k = w; while k < n && is_space(bytes[k]) { k += 1; }
                }
            }
            if is_global_candidate {
                // Parse function name
                let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                if p > k {
                    name_start = k;
                    name_end = p;
                    j = p;
                    k = p;
                    while k < n && is_space(bytes[k]) { k += 1; }
                    is_func_header = true;
                    is_global_fn = true;
                }
            } else if matches!(section, Section::Machine) || matches!(section, Section::Actions) || matches!(section, Section::Operations) || matches!(section, Section::Interface) {
                // Section members: bare names or 'async name(...) { ... }'
                if first_tok == "async" {
                    // Advance to the actual function name after 'async'
                    let mut p = k;
                    while p < n && is_ident(bytes[p]) { p += 1; }
                    if p > k {
                        name_start = k;
                        name_end = p;
                        k = p;
                        while k < n && is_space(bytes[k]) { k += 1; }
                    }
                }
                if k < n && bytes[k] == b'(' { is_func_header = true; }
            }
            if is_func_header && k < n && bytes[k] == b'(' {
                // balance parens
                let mut depth: i32 = 0; let mut p = k;
                while p < n {
                    let c = bytes[p];
                    match c { b'(' => { depth+=1; p+=1; }, b')' => { depth-=1; p+=1; if depth==0 { break; } }, _ => { p+=1; } }
                }
                while p < n && is_space(bytes[p]) { p += 1; }
                if p < n && bytes[p] == b'{' {
                    if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
                        let hdr = String::from_utf8_lossy(&bytes[line_start..p]).to_string();
                        eprintln!(
                            "[outline] header_detected section={:?} first_tok={} name={} line_start={} header_end={}",
                            section,
                            first_tok,
                            String::from_utf8_lossy(&bytes[name_start..name_end]).to_string(),
                            line_start,
                            p
                        );
                        eprintln!("[outline] header_text={}", hdr);
                    }
                    let open = p;
                    let close = match lang {
                        TargetLanguage::Python3 => closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::C => closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Java => closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        TargetLanguage::Rust => closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0).map(|c| open + c),
                        _ => Err(closer::CloseErrorV3{ kind: closer::CloseErrorV3Kind::Unimplemented, message: "unsupported language".into() }),
                    }.map_err(|e| OutlineErrorV3{ message: format!("body close error: {:?}", e) })?;
                    let owner_id = Some(String::from_utf8_lossy(&bytes[name_start..name_end]).to_string());
                    let state_id = state_scopes.last().map(|(n, _)| n.clone());
                    let kind = if is_global_fn {
                        BodyKindV3::Function
                    } else {
                        match section {
                            Section::Actions => BodyKindV3::Action,
                            Section::Operations => BodyKindV3::Operation,
                            _ => BodyKindV3::Handler
                        }
                    };
                    // Record this body scope so subsequent lines inside it are not treated as headers.
                    body_scopes.push((open, close));
                    items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close });
                    i = close + 1; continue;
                }
                // malformed header: header '(' ... ')' but no '{'
                // In interface: treat as prototype (no body) and ignore signature‑only declarations.
                if matches!(section, Section::Interface) {
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                    continue;
                }
                // For other sections, only treat true function artifacts (`fn` / `async fn`)
                // as E111; regular statements like `print(...)` should not trigger.
                if first_tok == "fn" || first_tok == "async" {
                    if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
                        let line_end = {
                            let mut q = p;
                            while q < n && bytes[q] != b'\n' { q += 1; }
                            q
                        };
                        let hdr = String::from_utf8_lossy(&bytes[line_start..line_end]).to_string();
                        eprintln!(
                            "[outline] E111 at section={:?} line_start={} header_text={}",
                            section,
                            line_start,
                            hdr
                        );
                    }
                    return Err(OutlineErrorV3{ message: "E111: missing '{' after module artifact header".into() });
                }
                // Otherwise treat this as a regular statement, not a header.
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            // Otherwise skip to next line
            while i<n && bytes[i]!=b'\n' { i+=1; }
        }
        if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
            eprintln!("[outline] items={} (lang={:?})", items.len(), lang);
            for it in &items {
                eprintln!(
                    "[outline] kind={:?} owner={:?} state={:?} header_span=({},{}) body_span=({},{})",
                    it.kind,
                    it.owner_id,
                    it.state_id,
                    it.header_span.start,
                    it.header_span.end,
                    it.open_byte,
                    it.close_byte
                );
            }
        }
        Ok(items)
    }

    // Tolerant scan: collect items and outline issues without aborting on first error.
    pub fn scan_collect(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> (Vec<OutlineItemV3>, Vec<ValidationIssueV3>) {
        let mut items: Vec<OutlineItemV3> = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        let mut i = start;
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Section { None, Actions, Operations, Interface, Machine }
        let mut section = Section::None;
        let mut state_scopes: Vec<(String, usize)> = Vec::new();
        while i < n {
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            while let Some((_, close)) = state_scopes.last() { if *close <= line_start { state_scopes.pop(); } else { break; } }
            let mut j = i; while j < n && is_space(bytes[j]) { j += 1; }
            let kw_start = j; while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { while i<n && bytes[i]!=b'\n' { i+=1; } continue; }
            let kw = to_lower_ascii(&bytes[kw_start..j]);
            if j < n && bytes[j] == b':' {
                section = match kw.as_str() { "actions" => Section::Actions, "operations" => Section::Operations, "interface" => Section::Interface, "machine" => Section::Machine, _ => Section::None };
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            if matches!(section, Section::Machine) && bytes[kw_start] == b'$' {
                let mut s = kw_start + 1; while s < n && is_ident(bytes[s]) { s += 1; }
                let mut p = s; while p < n && bytes[p] != b'\n' && bytes[p] != b'{' { p += 1; }
                if p < n && bytes[p] == b'{' {
                    let open = p;
                    let close_opt: Option<usize> = match lang {
                        TargetLanguage::Python3 => closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::C => closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Java => closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        TargetLanguage::Rust => closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0).ok().map(|c| open + c),
                        _ => None,
                    };
                    if let Some(close) = close_opt {
                        let name = String::from_utf8_lossy(&bytes[kw_start+1..s]).to_string();
                        state_scopes.push((name, close));
                    }
                }
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            let mut k = j; while k < n && is_space(bytes[k]) { k += 1; }
            // Recognize headers:
            // - Global functions: 'fn name(...) { ... }' or 'async fn name(...) { ... }' (regardless of section).
            // - Section members: IDENT '(' … ')' '{', with optional leading 'async'.
            let first_tok = to_lower_ascii(&bytes[kw_start..j]);
            let mut is_func_header = false;
            let mut _name_start = kw_start;
            let mut _name_end = j;
            if first_tok == "fn" || first_tok == "async" {
                // Global function or 'async fn'
                let mut next = k; while next < n && is_space(bytes[next]) { next += 1; }
                if first_tok == "async" {
                    let mut w = next; while w < n && is_ident(bytes[w]) { w += 1; }
                    let maybe_fn = to_lower_ascii(&bytes[next..w]);
                    if maybe_fn == "fn" {
                        k = w; while k < n && is_space(bytes[k]) { k += 1; }
                        let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                        if p > k { _name_start = k; _name_end = p; is_func_header = true; k = p; while k < n && is_space(bytes[k]) { k += 1; } }
                    }
                } else {
                    let mut p = k; while p < n && is_ident(bytes[p]) { p += 1; }
                    if p > k { _name_start = k; _name_end = p; is_func_header = true; k = p; while k < n && is_space(bytes[k]) { k += 1; } }
                }
            } else if matches!(section, Section::Machine) || matches!(section, Section::Actions) || matches!(section, Section::Operations) || matches!(section, Section::Interface) {
                // Section members: bare names or 'async name(...) { ... }'
                if first_tok == "async" {
                    // Support 'async name(...) { ... }' inside these sections by advancing to the actual name.
                    let mut p = k;
                    while p < n && is_ident(bytes[p]) { p += 1; }
                    if p > k {
                        _name_start = k;
                        _name_end = p;
                        k = p;
                        while k < n && is_space(bytes[k]) { k += 1; }
                    }
                }
                if k < n && bytes[k] == b'(' { is_func_header = true; }
            }
            if is_func_header && k < n && bytes[k] == b'(' {
                let mut depth: i32 = 0; let mut p = k;
                while p < n { let c = bytes[p]; match c { b'(' => { depth+=1; p+=1; }, b')' => { depth-=1; p+=1; if depth==0 { break; } }, _ => { p+=1; } } }
                while p < n && is_space(bytes[p]) { p += 1; }
                if p < n && bytes[p] == b'{' {
                    let open = p;
                    match lang {
                        TargetLanguage::Python3 => match closer::python::BodyCloserPyV3.close_byte(&bytes[open..], 0) { Ok(c) => {
                            let close = open + c;
                            let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string());
                            let state_id = state_scopes.last().map(|(n, _)| n.clone());
                            let kind = match section {
                                Section::Actions => BodyKindV3::Action,
                                Section::Operations => BodyKindV3::Operation,
                                Section::None => BodyKindV3::Function,
                                _ => BodyKindV3::Handler
                            };
                            items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close });
                            i = close + 1; continue;
                        }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::TypeScript => match closer::typescript::BodyCloserTsV3.close_byte(&bytes[open..], 0) { Ok(c) => {
                            let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue;
                        }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::CSharp => match closer::csharp::BodyCloserCsV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::C => match closer::c::BodyCloserCV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Cpp => match closer::cpp::BodyCloserCppV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Java => match closer::java::BodyCloserJavaV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        TargetLanguage::Rust => match closer::rust::BodyCloserRustV3.close_byte(&bytes[open..], 0) { Ok(c) => { let close = open + c; let owner_id = Some(String::from_utf8_lossy(&bytes[kw_start..j]).to_string()); let state_id = state_scopes.last().map(|(n, _)| n.clone()); let kind = match section { Section::Actions => BodyKindV3::Action, Section::Operations => BodyKindV3::Operation, Section::None => BodyKindV3::Function, _ => BodyKindV3::Handler }; items.push(OutlineItemV3{ header_span: RegionSpan{ start: line_start, end: p }, owner_id, state_id, kind, open_byte: open, close_byte: close }); i = close + 1; continue; }, Err(e) => { issues.push(ValidationIssueV3{ message: format!("body close error: {:?}", e) }); } },
                        _ => {}
                    }
                    // recovery: skip to next line after '{'
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                    continue;
                }
                // malformed header: header '(' ... ')' but no '{'
                // In interface, treat as prototype (no body) and ignore for
                // signature-style declarations like `e()` or `async e()`.
                // Headers starting with `fn` remain invalid here and are
                // reported as E111.
                if matches!(section, Section::Interface) && first_tok != "fn" {
                    while i<n && bytes[i]!=b'\n' { i+=1; }
                    continue;
                }
                // Only treat function-like constructs as E111; regular statements
                // like `print(...)` inside bodies should not surface this error.
                if first_tok == "fn" || first_tok == "async" {
                    issues.push(ValidationIssueV3{ message: "E111: missing '{' after module artifact header".into() });
                }
                while i<n && bytes[i]!=b'\n' { i+=1; }
                continue;
            }
            while i<n && bytes[i]!=b'\n' { i+=1; }
        }
        (items, issues)
    }
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'^' }
fn to_lower_ascii(s: &[u8]) -> String { s.iter().map(|b| (*b as char).to_ascii_lowercase()).collect() }
