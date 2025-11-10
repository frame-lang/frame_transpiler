use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::RegionV3;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::outline_scanner::{OutlineScannerV3, OutlineItemV3};

#[derive(Debug, Clone)]
pub struct ValidationIssueV3 { pub message: String }

#[derive(Debug, Clone)]
pub struct ValidationResultV3 { pub ok: bool, pub issues: Vec<ValidationIssueV3> }

pub struct ValidatorV3;

impl ValidatorV3 {
    // Minimal structural rule: once a terminal MIR is seen, no further MIR items may follow.
    pub fn validate_regions_mir(&self, regions: &[RegionV3], mir: &[MirItemV3]) -> ValidationResultV3 {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        // terminal kinds: Transition, Forward, StackPush, StackPop
        let is_terminal = |m: &MirItemV3| match m { MirItemV3::Transition{..} | MirItemV3::Forward{..} | MirItemV3::StackPush{..} | MirItemV3::StackPop{..} => true };
        // ensure no MIR after terminal
        if let Some((idx, _)) = mir.iter().enumerate().find(|(_, m)| is_terminal(m)) {
            if idx + 1 < mir.len() {
                issues.push(ValidationIssueV3 { message: "Terminal Frame statement must be last MIR item".to_string() });
            }
        }
        ValidationResultV3 { ok: issues.is_empty(), issues }
    }

    // Expanded API with body-kind policy (not yet wired with full module context).
    pub fn validate_regions_mir_with_policy(&self, regions: &[RegionV3], mir: &[MirItemV3], policy: ValidatorPolicyV3) -> ValidationResultV3 {
        let mut res = self.validate_regions_mir(regions, mir);
        if let Some(kind) = policy.body_kind {
            match kind {
                BodyKindV3::Action | BodyKindV3::Operation => {
                    if !mir.is_empty() {
                        // Frame statements are disallowed in actions/ops. Only advisory for now.
                        res.issues.push(ValidationIssueV3 { message: "Frame statements are not allowed in actions/operations".to_string() });
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
                    // scan to end of physical line or first '{'
                    let mut q = p;
                    let mut seen_lbrace = false;
                    while q < e && bytes[q] != b'\n' {
                        if bytes[q] == b'{' { seen_lbrace = true; break; }
                        q += 1;
                    }
                    if !seen_lbrace { issues.push(ValidationIssueV3{ message: "missing '{' after state header in machine: section".into() }); }
                }
                while p < e && bytes[p] != b'\n' { p += 1; }
            }
        }
        issues
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKindV3 { Handler, Action, Operation, Unknown }

#[derive(Debug, Clone, Default)]
pub struct ValidatorPolicyV3 { pub body_kind: Option<BodyKindV3> }
