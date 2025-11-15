use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::RegionV3;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::outline_scanner::OutlineItemV3;
use std::collections::HashSet;
use super::arcanum::Arcanum;

#[derive(Debug, Clone)]
pub struct ValidationIssueV3 { pub message: String }

#[derive(Debug, Clone)]
pub struct ValidationResultV3 { pub ok: bool, pub issues: Vec<ValidationIssueV3> }

pub struct ValidatorV3;

impl ValidatorV3 {
    // Structural validation placeholder; block-scoped terminal rule enforced in validate_terminal_last_native.
    pub fn validate_regions_mir(&self, _regions: &[RegionV3], _mir: &[MirItemV3]) -> ValidationResultV3 {
        ValidationResultV3 { ok: true, issues: Vec::new() }
    }

    // Optional: check that transition state_args arity matches state parameter arity from Arcanum
    pub fn validate_transition_state_arity_arcanum(&self, mir: &[MirItemV3], arc: &Arcanum, system: Option<&str>) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        let sys = system.unwrap_or("_");
        for m in mir {
            if let MirItemV3::Transition{ target, state_args, .. } = m {
                if let Some(state) = arc.resolve_state(sys, target) {
                    let expected = state.params.len();
                    let got = state_args.len();
                    if expected != got {
                        issues.push(ValidationIssueV3 { message: format!("E405: State '{}' expects {} param(s) but transition supplies {}", target, expected, got) });
                    }
                }
            }
        }
        issues
    }

    // Strict terminal check: Transition must be last statement in its containing block
    pub fn validate_terminal_last_native(&self, bytes: &[u8], regions: &[RegionV3], mir: &[MirItemV3], lang: crate::frame_c::visitors::TargetLanguage) -> Vec<ValidationIssueV3> {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let mut mi = 0usize;
        let mut idx = 0usize;
        while idx < regions.len() {
            match &regions[idx] {
                RegionV3::FrameSegment{ span, .. } => {
                    if mi >= mir.len() {
                        // Parse failed earlier; avoid indexing beyond MIR items.
                        break;
                    }
                    let m = &mir[mi];
                    mi += 1;
                    if let MirItemV3::Transition{..} = m {
                        // Enforce block-scope terminal: from end of this segment forward, allow only comment/whitespace
                        // until the containing block closes; first non-comment token before that is a violation.
                        let start = span.end;
                        if let Some(_) = find_violation_before_block_close(bytes, regions, idx+1, start, lang) {
                            issues.push(ValidationIssueV3 { message: "E400: Transition must be last statement in its containing block".to_string() });
                            // continue scanning to report multiple violations if present
                        }
                    }
                    idx += 1;
                }
                RegionV3::NativeText{ .. } => { idx += 1; }
            }
        }
        issues
    }

    // Expanded API with body-kind policy (not yet wired with full module context).
    pub fn validate_regions_mir_with_policy(&self, regions: &[RegionV3], mir: &[MirItemV3], policy: ValidatorPolicyV3) -> ValidationResultV3 {
        let mut res = self.validate_regions_mir(regions, mir);
        if let Some(kind) = policy.body_kind {
            match kind {
                BodyKindV3::Action | BodyKindV3::Operation => {
                    if !mir.is_empty() {
                        // Frame statements are disallowed in actions/ops. Only advisory for now.
                        res.issues.push(ValidationIssueV3 { message: "E401: Frame statements are not allowed in actions/operations".to_string() });
                    }
                }
                BodyKindV3::Handler | BodyKindV3::Unknown => {}
            }
        }
        res.ok = res.issues.is_empty();
        res
    }

    // Outer grammar structural checks (headers inside sections)
    pub fn validate_outer_grammar(&self, bytes: &[u8], start: usize, lang: TargetLanguage, outline: &[OutlineItemV3]) -> Vec<ValidationIssueV3> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Sec { Actions, Operations, Interface, Machine }
        // Collect section spans: [start,end)
        let mut secs: Vec<(usize, usize, Sec)> = Vec::new();
        // Single pass to record section line starts
        let n = bytes.len();
        let mut i = start;
        let mut marks: Vec<(usize, Sec)> = Vec::new();
        while i < n {
            // skip to SOL non-space
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // read ident
            let mut j = i; while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
            let kw_start = j; while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if kw_start < j && j < n && bytes[j] == b':' {
                let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                let sec = match kw.as_str() {
                    "actions" => Some(Sec::Actions),
                    "operations" => Some(Sec::Operations),
                    "interface" => Some(Sec::Interface),
                    "machine" => Some(Sec::Machine),
                    _ => None,
                };
                if let Some(s) = sec { marks.push((line_start, s)); }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        // Build ranges
        for idx in 0..marks.len() {
            let (spos, sec) = marks[idx];
            let epos = if idx + 1 < marks.len() { marks[idx+1].0 } else { n };
            secs.push((spos, epos, sec));
        }
        // Validate each outline item header lies within an appropriate section
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        for it in outline {
            let hs = it.header_span.start;
            // find containing section
            let mut sec_kind: Option<Sec> = None;
            for (s,e,sec) in &secs { if hs >= *s && hs < *e { sec_kind = Some(*sec); break; } }
            match it.kind {
                super::validator::BodyKindV3::Action => {
                    if sec_kind != Some(Sec::Actions) { issues.push(ValidationIssueV3{ message: "action body outside actions: section".into() }); }
                }
                super::validator::BodyKindV3::Operation => {
                    if sec_kind != Some(Sec::Operations) { issues.push(ValidationIssueV3{ message: "operation body outside operations: section".into() }); }
                }
                super::validator::BodyKindV3::Handler => {
                    if sec_kind != Some(Sec::Machine) { issues.push(ValidationIssueV3{ message: "handler body outside machine: section".into() }); }
                }
                super::validator::BodyKindV3::Unknown => {}
            }
        }
        issues
    }

    pub fn validate_machine_state_headers(&self, bytes: &[u8], start: usize) -> Vec<ValidationIssueV3> {
        // Find machine: sections and ensure any '$State' header has a following '{'
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Sec { Machine, Other }
        let n = bytes.len();
        let mut i = start;
        let mut marks: Vec<(usize, Sec)> = Vec::new();
        while i < n {
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            let mut j = i; while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
            let kw_start = j; while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if kw_start < j && j < n && bytes[j] == b':' {
                let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                if kw.as_str() == "machine" { marks.push((line_start, Sec::Machine)); }
                else { marks.push((line_start, Sec::Other)); }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        // Build section ranges
        let mut secs: Vec<(usize, usize, Sec)> = Vec::new();
        for idx in 0..marks.len() {
            let (spos, sec) = marks[idx];
            let epos = if idx + 1 < marks.len() { marks[idx+1].0 } else { n };
            secs.push((spos, epos, sec));
        }
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        for (s, e, sec) in secs {
            if sec != Sec::Machine { continue; }
            let mut p = s;
            while p < e {
                // next SOL
                while p < e && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') { p += 1; }
                if p >= e { break; }
                // check for state header starting with '$'
                if bytes[p] == b'$' {
                    // Distinguish state headers ("$Name {") from Frame statements ("$$[+]", "=> $^", etc.).
                    // State header requires an identifier start after '$'.
                    let next = if p + 1 < e { bytes[p+1] } else { b'\n' };
                    let is_ident_start = next.is_ascii_alphabetic() || next == b'_';
                    if is_ident_start {
                        // scan to end of physical line or first '{'
                        let mut q = p;
                        let mut seen_lbrace = false;
                        while q < e && bytes[q] != b'\n' {
                            if bytes[q] == b'{' { seen_lbrace = true; break; }
                            q += 1;
                        }
                        if !seen_lbrace { issues.push(ValidationIssueV3{ message: "E112: missing '{' after state header in machine: section".into() }); }
                    } else {
                        // It's a Frame statement at SOL inside machine; skip for state-header validation.
                    }
                }
                while p < e && bytes[p] != b'\n' { p += 1; }
            }
        }
        issues
    }

    // Collect all state names declared inside machine: sections as "$Name {".
    pub fn collect_machine_state_names(&self, bytes: &[u8], start: usize) -> HashSet<String> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Sec { Machine, Other }
        let n = bytes.len();
        let mut i = start;
        let mut marks: Vec<(usize, Sec)> = Vec::new();
        while i < n {
            // SOL skip
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // read ident and ':'
            let mut j = i; while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
            let kw_start = j; while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if kw_start < j && j < n && bytes[j] == b':' {
                let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                match kw.as_str() {
                    "machine" | "actions" | "operations" | "interface" => {
                        let sec = if kw.as_str() == "machine" { Sec::Machine } else { Sec::Other };
                        marks.push((line_start, sec));
                    }
                    _ => {}
                }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        // Build section ranges
        let mut secs: Vec<(usize, usize, Sec)> = Vec::new();
        for idx in 0..marks.len() {
            let (spos, sec) = marks[idx];
            let epos = if idx + 1 < marks.len() { marks[idx+1].0 } else { n };
            secs.push((spos, epos, sec));
        }
        let mut names = HashSet::new();
        for (s, e, sec) in secs {
            if sec != Sec::Machine { continue; }
            let mut p = s;
            while p < e {
                // skip ws
                while p < e && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') { p += 1; }
                if p >= e { break; }
                if bytes[p] == b'$' {
                    let mut k = p + 1;
                    if k < e && (bytes[k].is_ascii_alphabetic() || bytes[k] == b'_') {
                        k += 1;
                        while k < e && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') { k += 1; }
                        let name = String::from_utf8_lossy(&bytes[p+1..k]).to_string();
                        // ensure this looks like a state header line (has '{' before EOL)
                        let mut q = k; let mut has_lbrace = false; while q < e && bytes[q] != b'\n' { if bytes[q] == b'{' { has_lbrace = true; break; } q += 1; }
                        if has_lbrace { names.insert(name); }
                    }
                }
                while p < e && bytes[p] != b'\n' { p += 1; }
            }
        }
        names
    }

    pub fn validate_transition_targets(&self, mir: &[MirItemV3], known_states: &HashSet<String>) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        for m in mir {
            if let MirItemV3::Transition{ target, .. } = m {
                if !known_states.contains(target) {
                    issues.push(ValidationIssueV3{ message: format!("E402: unknown state '{}'", target) });
                }
            }
        }
        issues
    }

    // Arcanum-backed variant: resolve against symbol table instead of coarse HashSet
    pub fn validate_transition_targets_arcanum(&self, mir: &[MirItemV3], arcanum: &Arcanum, system_name: Option<&str>) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        let sys = system_name.unwrap_or("_");
        for m in mir {
            if let MirItemV3::Transition{ target, .. } = m {
                let found = arcanum.resolve_state(sys, target).or_else(|| arcanum.resolve_state("_", target)).is_some();
                if !found {
                    issues.push(ValidationIssueV3{ message: format!("E402: unknown state '{}'", target) });
                }
            }
        }
        issues
    }

    pub fn has_machine_section(&self, bytes: &[u8], start: usize) -> bool {
        let n = bytes.len();
        let mut i = start;
        while i < n {
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let mut j = i; while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
            let kw_start = j; while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if kw_start < j && j < n && bytes[j] == b':' {
                let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                if kw.as_str() == "machine" { return true; }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        false
    }

    // Detect whether any state in any machine section declares a parent ("$Child => $Parent").
    pub fn has_any_parent_relationship(&self, bytes: &[u8], start: usize) -> bool {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Sec { Machine, Other }
        let n = bytes.len();
        let mut i = start;
        // Mark section starts
        let mut marks: Vec<(usize, Sec)> = Vec::new();
        while i < n {
            // SOL skip
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            let line_start = i;
            // read ident and ':'
            let mut j = i; while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
            let kw_start = j; while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if kw_start < j && j < n && bytes[j] == b':' {
                let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                match kw.as_str() {
                    "machine" | "actions" | "operations" | "interface" => {
                        let sec = if kw.as_str() == "machine" { Sec::Machine } else { Sec::Other };
                        marks.push((line_start, sec));
                    }
                    _ => {}
                }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        // Build section ranges and scan only machine sections for "$... => $..." on header lines
        for idx in 0..marks.len() {
            let (spos, sec) = marks[idx];
            let epos = if idx + 1 < marks.len() { marks[idx+1].0 } else { n };
            if sec != Sec::Machine { continue; }
            let mut p = spos;
            while p < epos {
                // skip ws
                while p < epos && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') { p += 1; }
                if p >= epos { break; }
                if bytes[p] == b'$' {
                    // scan "$Child"
                    let mut k = p + 1;
                    if k < epos && (bytes[k].is_ascii_alphabetic() || bytes[k] == b'_') {
                        k += 1; while k < epos && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') { k += 1; }
                        // skip whitespace
                        while k < epos && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                        // check for "=> $"
                        if k + 3 < epos && bytes[k] == b'=' && bytes[k+1] == b'>' {
                            let mut q = k + 2; while q < epos && (bytes[q] == b' ' || bytes[q] == b'\t') { q += 1; }
                            if q < epos && bytes[q] == b'$' { return true; }
                        }
                    }
                }
                while p < epos && bytes[p] != b'\n' { p += 1; }
            }
        }
        false
    }

    // Arcanum-backed variant: check if any state declares a parent in the module
    pub fn any_parent_relation_arcanum(&self, arcanum: &Arcanum) -> bool {
        arcanum.any_parent_relation()
    }

    // Ensure handlers in machine: are nested within a state block
    pub fn validate_handlers_in_state(&self, outline: &[OutlineItemV3]) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        for it in outline {
            if let BodyKindV3::Handler = it.kind {
                if it.state_id.is_none() {
                    issues.push(ValidationIssueV3 { message: "E404: handler body must be inside a state block".into() });
                }
            }
        }
        issues
    }
}

fn trailing_is_effectively_comment_only(slice: &[u8], lang: crate::frame_c::visitors::TargetLanguage) -> bool {
    // Allow whitespace only
    let mut i = 0usize; let n = slice.len();
    while i<n && (slice[i].is_ascii_whitespace()) { i+=1; }
    // Optional semicolon before comment for C-like and Python
    if i<n && slice[i]==b';' {
        i+=1; while i<n && (slice[i].is_ascii_whitespace()) { i+=1; }
    }
    if i>=n { return true; }
    match lang {
        crate::frame_c::visitors::TargetLanguage::Python3 => {
            // For Python targets, allow comment-only tails or closing braces from the module DSL on the same line.
            // Skip an optional ';' and whitespace
            while i<n && (slice[i].is_ascii_whitespace() || slice[i]==b';') { i+=1; }
            if i>=n { return true; }
            if slice[i]==b'#' { return true; }
            // Accept one or more closing braces '}' (module DSL) followed by only whitespace/semicolons
            let mut j = i;
            let mut saw_brace = false;
            while j<n && slice[j]==b'}' { saw_brace = true; j+=1; }
            if saw_brace {
                while j<n && (slice[j].is_ascii_whitespace() || slice[j]==b';') { j+=1; }
                return j>=n;
            }
            false
        }
        _ => {
            if i+1<n && slice[i]==b'/' && slice[i+1]==b'/' { return true; }
            if i+1<n && slice[i]==b'/' && slice[i+1]==b'*' {
                // ensure that after closing */ there is nothing non-whitespace on this line
                let mut j = i + 2;
                while j + 1 < n {
                    if slice[j]==b'*' && slice[j+1]==b'/' { j += 2; break; }
                    j += 1;
                }
                // if never closed, treat as comment-only for this slice
                while j < n && (slice[j].is_ascii_whitespace()) { j += 1; }
                return j >= n;
            }
            false
        }
    }
}

// Scan forward from the end of a Transition up to the close of its containing block.
// Returns Some(()) if a non-comment/non-whitespace token is found before the block closes.
fn find_violation_before_block_close(bytes: &[u8], regions: &[RegionV3], mut ridx: usize, mut pos: usize, lang: crate::frame_c::visitors::TargetLanguage) -> Option<()> {
    match lang {
        crate::frame_c::visitors::TargetLanguage::Python3 => scan_python_block(bytes, regions, &mut ridx, &mut pos, |violate| if violate { Some(()) } else { None }),
        _ => scan_c_like_block(bytes, regions, &mut ridx, &mut pos, lang, |violate| if violate { Some(()) } else { None }),
    }
}

fn scan_c_like_block<F, T>(bytes: &[u8], regions: &[RegionV3], ridx: &mut usize, pos: &mut usize, lang: crate::frame_c::visitors::TargetLanguage, mut out: F) -> Option<T>
where F: FnMut(bool) -> Option<T> {
    // Simple DPDA: skip whitespace/comments/strings; stop OK at first top-level '}' encountered; any other token => violation.
    let mut i = *pos;
    // scanning spans across regions; treat region boundaries as contiguous
    let mut in_line = false; let mut in_block = false; let mut in_str: Option<u8> = None; let mut in_tpl = false; let mut tmpl_brace: i32 = 0;
    let mut end = current_region_end(regions, *ridx).unwrap_or(bytes.len());
    // First, allow a comment-only tail on the same physical line (optional leading ';').
    let mut j = i; while j < end && bytes[j] != b'\n' { j += 1; }
    if trailing_is_effectively_comment_only(&bytes[i..j], lang) { i = j; }
    loop {
        if i >= end {
            // advance to next region's NativeText; if FrameSegment appears before block close, that is a violation inside same block
            *ridx += 1;
            if *ridx >= regions.len() { return None; }
            match &regions[*ridx] {
                RegionV3::NativeText{ span } => { i = span.start; end = span.end; }
                RegionV3::FrameSegment{ .. } => { return out(true); }
            }
        }
        let b = bytes[i];
        if in_line {
            if b == b'\n' { in_line = false; }
            i += 1; continue;
        }
        if in_block {
            if i+1 < end && b == b'*' && bytes[i+1] == b'/' { in_block = false; i += 2; continue; }
            i += 1; continue;
        }
        if let Some(q) = in_str { // string literal
            if b == b'\\' { i += 2; continue; }
            if b == q { in_str = None; i += 1; continue; }
            i += 1; continue;
        }
        if in_tpl {
            if b == b'`' { in_tpl = false; i += 1; continue; }
            if b == b'\\' { i += 2; continue; }
            if b == b'$' && i+1<end && bytes[i+1]==b'{' { tmpl_brace += 1; i += 2; continue; }
            if b == b'}' && tmpl_brace > 0 { tmpl_brace -= 1; i += 1; continue; }
            i += 1; continue;
        }
        // not in protected region
        if b.is_ascii_whitespace() { i += 1; continue; }
        if b == b'/' && i+1<end && bytes[i+1]==b'/' { in_line = true; i += 2; continue; }
        if b == b'/' && i+1<end && bytes[i+1]==b'*' { in_block = true; i += 2; continue; }
        if b == b'\'' || b == b'"' { in_str = Some(b); i += 1; continue; }
        if let crate::frame_c::visitors::TargetLanguage::TypeScript = lang { if b == b'`' { in_tpl = true; i += 1; continue; } }
        if b == b'}' { return out(false); }
        // any other token before close brace => violation
        return out(true);
    }
}

fn scan_python_block<F, T>(bytes: &[u8], regions: &[RegionV3], ridx: &mut usize, pos: &mut usize, mut out: F) -> Option<T>
where F: FnMut(bool) -> Option<T> {
    // Determine indent of containing block from preceding FrameSegment
    let mut t_indent: Option<usize> = None;
    if *ridx > 0 { for back in (0..*ridx).rev() { if let RegionV3::FrameSegment{ indent, .. } = regions[back] { t_indent = Some(indent); break; } } }
    let base = t_indent.unwrap_or(0);
    let mut i = *pos;
    let mut end = current_region_end(regions, *ridx).unwrap_or(bytes.len());
    // Same line tail: allow optional ';' then comment-only, else violation
    let mut line_end = i; while line_end < end && bytes[line_end] != b'\n' { line_end += 1; }
    if !trailing_is_effectively_comment_only(&bytes[i..line_end], crate::frame_c::visitors::TargetLanguage::Python3) {
        // Any non-comment tail on same line is a violation
        // Detect non-whitespace non-';' content
        let tail = &bytes[i..line_end];
        let mut k = 0; while k < tail.len() && (tail[k].is_ascii_whitespace() || tail[k]==b';') { k+=1; }
        if k < tail.len() && tail[k] != b'#' { return out(true); }
    }
    // After the newline: any content within the same block is a violation
    i = line_end;
    loop {
        if i >= end {
            *ridx += 1; if *ridx >= regions.len() { return None; }
            match &regions[*ridx] { RegionV3::NativeText{ span } => { i = span.start; end = span.end; }, RegionV3::FrameSegment{ .. } => { return out(true); } }
        }
        // consume newline
        if i < end && bytes[i] == b'\n' { i += 1; }
        // compute indent
        let mut col = 0usize; while i < end && (bytes[i]==b' ' || bytes[i]==b'\t') { col += 1; i += 1; }
        // blank or comment-only line
        if i >= end || bytes[i] == b'\n' { continue; }
        if bytes[i] == b'#' { while i < end && bytes[i] != b'\n' { i += 1; } continue; }
        // if dedented, block ended OK
        if col < base { return out(false); }
        // still inside block and found content => violation
        return out(true);
    }
}

fn current_region_end(regions: &[RegionV3], ridx: usize) -> Option<usize> {
    regions.get(ridx).map(|r| match r { RegionV3::NativeText{ span } => span.end, RegionV3::FrameSegment{ span, .. } => span.end })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKindV3 { Handler, Action, Operation, Unknown }

#[derive(Debug, Clone, Default)]
pub struct ValidatorPolicyV3 { pub body_kind: Option<BodyKindV3> }
