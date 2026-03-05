use super::native_region_scanner::RegionSpan;
use super::validator::BodyKind;
use super::body_closer as closer;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::validator::ValidationIssue;

#[derive(Debug, Clone)]
pub struct OutlineItem {
    pub header_span: RegionSpan,
    pub owner_id: Option<String>,
    pub state_id: Option<String>,
    pub kind: BodyKind,
    pub open_byte: usize,
    pub close_byte: usize,
}

#[derive(Debug)]
pub struct OutlineError { pub message: String }

pub struct OutlineScanner;

impl OutlineScanner {
    /// Strict scan: returns items on success, or the first error encountered.
    pub fn scan(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> Result<Vec<OutlineItem>, OutlineError> {
        let (items, issues) = self.scan_internal(bytes, start, lang, true);
        if let Some(issue) = issues.first() {
            Err(OutlineError { message: issue.message.clone() })
        } else {
            Ok(items)
        }
    }

    /// Tolerant scan: collect items and outline issues without aborting on first error.
    pub fn scan_collect(&self, bytes: &[u8], start: usize, lang: TargetLanguage) -> (Vec<OutlineItem>, Vec<ValidationIssue>) {
        self.scan_internal(bytes, start, lang, false)
    }

    /// Unified internal scanner.
    /// When `strict` is true, scanning stops at the first E111 error (for `scan()` compatibility).
    fn scan_internal(&self, bytes: &[u8], start: usize, lang: TargetLanguage, strict: bool) -> (Vec<OutlineItem>, Vec<ValidationIssue>) {
        let mut items: Vec<OutlineItem> = Vec::new();
        let mut issues: Vec<ValidationIssue> = Vec::new();
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
            while let Some((_, close)) = state_scopes.last() {
                if *close <= line_start { state_scopes.pop(); } else { break; }
            }

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
            let kw_start = j;
            while j < n && is_ident(bytes[j]) { j += 1; }
            if kw_start == j { while i < n && bytes[i] != b'\n' { i += 1; } continue; }
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
                while i < n && bytes[i] != b'\n' { i += 1; }
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
                if is_state_header {
                    // find '{' on this line
                    let mut p = s;
                    while p < n && bytes[p] != b'\n' && bytes[p] != b'{' { p += 1; }
                    if p < n && bytes[p] == b'{' {
                        let open = p;
                        // compute close; ignore errors in this fast path
                        if let Ok(close) = closer::close_body(bytes, open, lang) {
                            let name = String::from_utf8_lossy(&bytes[ident_start..s]).to_string();
                            state_scopes.push((name, close));
                        }
                    }
                    while i < n && bytes[i] != b'\n' { i += 1; }
                    continue;
                }
                // If this is not a valid state header (e.g., '$>()'), fall through
                // and let the section-member detection handle it as a handler.
            }

            // Recognize headers:
            // - Global functions: 'fn name(...) { ... }' or 'async fn name(...) { ... }'.
            // - Section members (machine/actions/operations/interface): IDENT '(' ... ')'
            //   with optional leading 'async' before the name (e.g., 'async run() { ... }').
            //   After the ')', we allow an optional header type/default segment
            //   (': Type = default') before the opening '{'.
            //   (Interface headers without a '{' are treated as prototypes and ignored.)
            let mut name_start = kw_start;
            let mut name_end = j;
            let first_tok = to_lower_ascii(&bytes[name_start..name_end]);
            let mut k = j;
            while k < n && is_space(bytes[k]) { k += 1; }
            let mut is_func_header = false;
            let mut is_global_fn = false;

            // Global functions: fn name(...) or async fn name(...)
            let mut is_global_candidate = false;
            if matches!(section, Section::None) {
                if first_tok == "fn" {
                    is_global_candidate = true;
                } else if first_tok == "async" {
                    // Look ahead for 'fn' after async
                    let mut next = j;
                    while next < n && is_space(bytes[next]) { next += 1; }
                    let mut w = next;
                    while w < n && is_ident(bytes[w]) { w += 1; }
                    let maybe_fn = to_lower_ascii(&bytes[next..w]);
                    if maybe_fn == "fn" {
                        is_global_candidate = true;
                        // Position k at start of name after 'fn'
                        k = w;
                        while k < n && is_space(bytes[k]) { k += 1; }
                    }
                }
            }

            if is_global_candidate {
                // Parse function name
                let mut p = k;
                while p < n && is_ident(bytes[p]) { p += 1; }
                if p > k {
                    name_start = k;
                    name_end = p;
                    k = p;
                    while k < n && is_space(bytes[k]) { k += 1; }
                    is_func_header = true;
                    is_global_fn = true;
                }
            } else if matches!(section, Section::Machine) || matches!(section, Section::Actions)
                || matches!(section, Section::Operations) || matches!(section, Section::Interface)
            {
                // Section members: bare names or 'async name(...) { ... }'
                // Special-case entry handlers `$>() { ... }` in machine:.
                if matches!(section, Section::Machine) && bytes[kw_start] == b'$'
                    && kw_start + 1 < n && bytes[kw_start + 1] == b'>'
                {
                    // Find the '(' after `$>`.
                    let mut p = k;
                    while p < n && bytes[p] != b'(' && bytes[p] != b'\n' { p += 1; }
                    if p < n && bytes[p] == b'(' {
                        k = p;
                        is_func_header = true;
                    }
                } else {
                    if first_tok == "async" {
                        // Advance to the actual function name after 'async' (and an optional 'fn')
                        let mut p = k;
                        let mut ident_start = p;
                        while p < n && is_ident(bytes[p]) { p += 1; }
                        if to_lower_ascii(&bytes[ident_start..p]) == "fn" {
                            while p < n && is_space(bytes[p]) { p += 1; }
                            ident_start = p;
                            while p < n && is_ident(bytes[p]) { p += 1; }
                        }
                        if p > ident_start {
                            name_start = ident_start;
                            name_end = p;
                            k = p;
                            while k < n && is_space(bytes[k]) { k += 1; }
                        }
                    }
                    if k < n && bytes[k] == b'(' && !is_control_flow_keyword(&first_tok) {
                        is_func_header = true;
                    }
                }
            }

            if is_func_header && k < n && bytes[k] == b'(' {
                // balance parens
                let mut depth: i32 = 0;
                let mut p = k;
                while p < n {
                    let c = bytes[p];
                    match c {
                        b'(' => { depth += 1; p += 1; },
                        b')' => { depth -= 1; p += 1; if depth == 0 { break; } },
                        _ => { p += 1; }
                    }
                }
                while p < n && is_space(bytes[p]) { p += 1; }

                // For non-global section members, allow optional header type/default
                // segments like ': Type = default' between ')' and '{'.
                if !is_global_fn && (matches!(section, Section::Machine)
                    || matches!(section, Section::Actions)
                    || matches!(section, Section::Operations)
                    || matches!(section, Section::Interface))
                {
                    // Optional ': Type ...'
                    if p < n && bytes[p] == b':' {
                        p += 1;
                        // Skip type expression up to '{', '=', or end-of-line.
                        while p < n && bytes[p] != b'{' && bytes[p] != b'=' && bytes[p] != b'\n' {
                            p += 1;
                        }
                        // Optional '= default' after the type.
                        if p < n && bytes[p] == b'=' {
                            p += 1;
                            while p < n && bytes[p] != b'{' && bytes[p] != b'\n' {
                                p += 1;
                            }
                        }
                        while p < n && is_space(bytes[p]) { p += 1; }
                    } else if p < n && bytes[p] == b'=' {
                        // Header default without an explicit type: name(...) = expr { ... }
                        p += 1;
                        while p < n && bytes[p] != b'{' && bytes[p] != b'\n' {
                            p += 1;
                        }
                        while p < n && is_space(bytes[p]) { p += 1; }
                    } else if p + 1 < n && bytes[p] == b'-' && bytes[p + 1] == b'>' {
                        // Rust-style return type `-> Type { ... }`
                        p += 2;
                        while p < n && bytes[p] != b'{' && bytes[p] != b'\n' {
                            p += 1;
                        }
                        while p < n && is_space(bytes[p]) { p += 1; }
                    }
                } else if is_global_fn && p < n && bytes[p] == b':' {
                    // Global Frame functions (fn name(...) : Type { ... }) may also
                    // carry a return type between ')' and '{'. Skip the type
                    // expression up to the opening brace or end-of-line.
                    p += 1;
                    while p < n && bytes[p] != b'{' && bytes[p] != b'\n' {
                        p += 1;
                    }
                    while p < n && is_space(bytes[p]) { p += 1; }
                }

                if p < n && bytes[p] != b'{' {
                    // Last chance: scan forward on the same line for an opening brace,
                    // but ignore braces that appear inside line comments.
                    let mut q = p;
                    let mut found = None;
                    let mut saw_comment = false;
                    while q < n && bytes[q] != b'\n' {
                        if bytes[q] == b'/' && q + 1 < n && bytes[q + 1] == b'/' {
                            saw_comment = true;
                            break;
                        }
                        if bytes[q] == b'{' {
                            found = Some(q);
                            break;
                        }
                        q += 1;
                    }
                    if let Some(pos) = found {
                        p = pos;
                    } else if saw_comment {
                        // No brace before comment; leave p unchanged so we surface E111.
                    }
                }

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
                    match closer::close_body(bytes, open, lang) {
                        Ok(close) => {
                            let owner_id = Some(String::from_utf8_lossy(&bytes[name_start..name_end]).to_string());
                            let state_id = state_scopes.last().map(|(n, _)| n.clone());
                            let kind = if is_global_fn {
                                BodyKind::Function
                            } else {
                                match section {
                                    Section::Actions => BodyKind::Action,
                                    Section::Operations => BodyKind::Operation,
                                    _ => BodyKind::Handler
                                }
                            };
                            if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
                                eprintln!("[outline] push kind={:?} section={:?} owner={:?} state={:?}", kind, section, owner_id, state_id);
                            }
                            // Record this body scope so subsequent lines inside it are not treated as headers.
                            body_scopes.push((open, close));
                            items.push(OutlineItem {
                                header_span: RegionSpan { start: line_start, end: p },
                                owner_id,
                                state_id,
                                kind,
                                open_byte: open,
                                close_byte: close,
                            });
                            i = close + 1;
                            continue;
                        }
                        Err(e) => {
                            issues.push(ValidationIssue { message: format!("body close error: {:?}", e) });
                            // recovery: skip to next line after '{'
                            while i < n && bytes[i] != b'\n' { i += 1; }
                            continue;
                        }
                    }
                }

                // malformed header: header '(' ... ')' but no '{'
                // In interface: treat as prototype (no body) and ignore signature-only declarations.
                // Headers starting with `fn` remain invalid here and are reported as E111.
                if matches!(section, Section::Interface) && first_tok != "fn" {
                    while i < n && bytes[i] != b'\n' { i += 1; }
                    continue;
                }

                // For other sections, only treat true function artifacts (`fn` / `async fn`)
                // as E111; regular statements like `print(...)` should not trigger.
                if first_tok == "fn" || first_tok == "async" {
                    if std::env::var("FRAME_DEBUG_OUTLINE").ok().as_deref() == Some("1") {
                        let line_end = {
                            let mut q = line_start;
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
                    issues.push(ValidationIssue { message: "E111: missing '{' after module artifact header".into() });
                    if strict {
                        return (items, issues);
                    }
                }

                // Otherwise treat this as a regular statement, not a header.
                while i < n && bytes[i] != b'\n' { i += 1; }
                continue;
            }

            // Otherwise skip to next line
            while i < n && bytes[i] != b'\n' { i += 1; }
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

        (items, issues)
    }
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b == b'_' || b == b'$' || b == b'^' }
fn to_lower_ascii(s: &[u8]) -> String { s.iter().map(|b| (*b as char).to_ascii_lowercase()).collect() }

// Control-flow keywords that should never be interpreted as section-member names
// even when followed by '(...) { ... }' in actions/operations/machine/interface sections.
fn is_control_flow_keyword(tok: &str) -> bool {
    matches!(tok, "if" | "for" | "while" | "switch")
}
