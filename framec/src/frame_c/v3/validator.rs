use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::RegionV3;
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::outline_scanner::OutlineItemV3;
use std::collections::HashSet;
use super::arcanum::Arcanum;
use super::ast::{ModuleAst, SystemSectionKind};
use super::system_param_semantics::{collect_domain_vars_per_system, first_state_for_system};

#[derive(Debug, Clone)]
pub struct ValidationIssueV3 { pub message: String }

#[derive(Debug, Clone)]
pub struct ValidationResultV3 { pub ok: bool, pub issues: Vec<ValidationIssueV3> }

pub struct ValidatorV3;

/// Aggregated module-level context for semantic validation.
/// Built once from the outline + bytes, then reused for per-body checks.
#[derive(Debug, Clone)]
pub struct ModuleSemanticContextV3 {
    /// Byte offset where the outline (sections/system body) begins.
    pub outline_start: usize,
    /// Outline items discovered by OutlineScannerV3 (handlers/actions/ops/functions).
    pub outline_items: Vec<OutlineItemV3>,
    /// Coarse set of known state names discovered from machine sections.
    pub known_states: HashSet<String>,
    /// Arcanum symbol table (systems, machines, states, params).
    pub arcanum: Arcanum,
}

impl ValidatorV3 {
    // Structural validation placeholder; block-scoped terminal rule enforced in validate_terminal_last_native.
    pub fn validate_regions_mir(&self, _regions: &[RegionV3], _mir: &[MirItemV3]) -> ValidationResultV3 {
        ValidationResultV3 { ok: true, issues: Vec::new() }
    }

    /// Build a semantic context for a module after syntactic/outline checks have succeeded.
    ///
    /// This folds together:
    /// - outline scan + section placement checks,
    /// - machine state header checks,
    /// - handler-in-state checks,
    /// - state name collection, and
    /// - Arcanum construction.
    ///
    /// It returns both the context and any validation issues discovered at the module level.
    pub fn build_module_semantic_context(
        &self,
        bytes: &[u8],
        outline_start: usize,
        lang: TargetLanguage,
    ) -> (ModuleSemanticContextV3, Vec<ValidationIssueV3>) {
        let mut issues = Vec::new();
        // Tolerant outline scan (collects E111 and similar diagnostics).
        let (items, outline_issues) =
            crate::frame_c::v3::outline_scanner::OutlineScannerV3.scan_collect(bytes, outline_start, lang);
        issues.extend(outline_issues);
        // Section placement: actions/operations/handlers must live in correct sections.
        let outer_issues = self.validate_outer_grammar(bytes, outline_start, lang, &items);
        issues.extend(outer_issues);
        // System block ordering and per-system machine state headers are driven from ModuleAst.
        let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, lang);
        let block_order_issues = self.validate_system_block_order_ast(&module_ast);
        issues.extend(block_order_issues);
        let state_issues = self.validate_machine_state_headers_ast(bytes, &module_ast);
        issues.extend(state_issues);
        // Collect coarse state names and build Arcanum symbol table.
        let known_states = self.collect_machine_state_names(bytes, outline_start);
        let arcanum = crate::frame_c::v3::arcanum::build_arcanum_from_module_ast(bytes, &module_ast);
        // Handlers must be nested inside a state block in machine:, now validated against Arcanum/AST.
        let handler_scope_issues =
            self.validate_handlers_in_state_ast(bytes, &items, &module_ast, &arcanum);
        issues.extend(handler_scope_issues);

        let ctx = ModuleSemanticContextV3 {
            outline_start,
            outline_items: items,
            known_states,
            arcanum,
        };
        (ctx, issues)
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

    /// Validate system parameter semantics against start state, entry handler, and domain block.
    /// - Start params `$(...)` must match the start state's parameter list.
    /// - Enter params `$>(...)` must match the start state's `$>()` handler parameter list (when present).
    /// - Domain params must correspond to variables declared in the system's `domain:` block.
    pub fn validate_system_param_semantics(
        &self,
        bytes: &[u8],
        start: usize,
        lang: TargetLanguage,
        arc: &Arcanum,
        _outline: &[OutlineItemV3],
    ) -> Vec<ValidationIssueV3> {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        // Parse all systems once to obtain system parameters per name.
        let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, lang);
        let mut sys_params_by_name: std::collections::HashMap<String, super::ast::SystemParamsAst> =
            std::collections::HashMap::new();
        for sys in module_ast.systems {
            sys_params_by_name.insert(sys.name.clone(), sys.params);
        }
        let domain_vars_by_system = collect_domain_vars_per_system(bytes, start, lang);

        for (sys_name, _sys_entry) in &arc.systems {
            let sys_params = match sys_params_by_name.get(sys_name) {
                Some(p) => p,
                None => continue,
            };
            let start_params = &sys_params.start_params;
            let enter_params = &sys_params.enter_params;
            let domain_params = &sys_params.domain_params;

            if start_params.is_empty() && enter_params.is_empty() && domain_params.is_empty() {
                continue;
            }

            // Resolve start state via Arcanum.
            let start_state = match first_state_for_system(arc, sys_name) {
                Some(s) => s,
                None => continue,
            };

            if std::env::var("FRAME_DEBUG_SYSPARAMS").ok().as_deref() == Some("1") {
                eprintln!(
                    "[sysparams] system={} start_state={} span=({},{}) start={:?} enter={:?} domain={:?}",
                    sys_name,
                    start_state.name,
                    start_state.span.start,
                    start_state.span.end,
                    start_params,
                    enter_params,
                    domain_params
                );
            }

            // E416: start params must match start state params.
            if !start_params.is_empty() || !start_state.params.is_empty() {
                if start_params.len() != start_state.params.len()
                    || *start_params != start_state.params
                {
                    issues.push(ValidationIssueV3 {
                        message: format!(
                            "E416: system '{}' start parameters ({:?}) must match start state '{}' parameters ({:?})",
                            sys_name, start_params, start_state.name, start_state.params
                        ),
                    });
                }
            }

            // E417: enter params must match start state's $>() handler params.
            if !enter_params.is_empty() {
                // Use the machine-section parser to locate the state's $>() handler.
                let module_ast = crate::frame_c::v3::system_parser::SystemParserV3::parse_module(bytes, lang);
                let mut machine_span: Option<crate::frame_c::v3::ast::Span> = None;
                for sys in &module_ast.systems {
                    if sys.name == *sys_name {
                        machine_span = sys.sections.machine.clone();
                        break;
                    }
                }
                let entry_params = match machine_span {
                    Some(span) => crate::frame_c::v3::machine_parser::MachineParserV3
                        .find_entry_params_in_machine(bytes, &span, &start_state.name, lang),
                    None => None,
                };
                if std::env::var("FRAME_DEBUG_SYSPARAMS")
                    .ok()
                    .as_deref()
                    == Some("1")
                {
                    eprintln!(
                        "[sysparams] system={} start_state={} entry_params={:?}",
                        sys_name, start_state.name, entry_params
                    );
                }
                match entry_params {
                    None => {
                        issues.push(ValidationIssueV3 {
                            message: format!(
                                "E417: system '{}' declares $>(...) enter parameters but start state '{}' has no $>() handler",
                                sys_name, start_state.name
                            ),
                        });
                    }
                    Some(hdr_params) => {
                        if hdr_params.len() != enter_params.len() || &hdr_params != enter_params {
                            issues.push(ValidationIssueV3 {
                                message: format!(
                                    "E417: system '{}' enter parameters ({:?}) must match start state '{}' $>() parameters ({:?})",
                                    sys_name, enter_params, start_state.name, hdr_params
                                ),
                            });
                        }
                    }
                }
            }

            // E418: domain params must map to variables in the domain: block.
            if !domain_params.is_empty() {
                let dom_vars = domain_vars_by_system.get(sys_name);
                for dp in domain_params {
                    let ok = dom_vars.map_or(false, |vars| vars.iter().any(|v| v == dp));
                    if !ok {
                        issues.push(ValidationIssueV3 {
                            message: format!(
                                "E418: system '{}' domain parameter '{}' has no matching variable in domain: block",
                                sys_name, dp
                            ),
                        });
                    }
                }
            }
        }

        issues
    }

    // `domain:` is now treated as native code for the target language; no
    // additional structural validation is applied beyond general parsing and
    // host-language rules.

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
                BodyKindV3::Handler | BodyKindV3::Function | BodyKindV3::Unknown => {}
            }
        }
        res.ok = res.issues.is_empty();
        res
    }

    /// Validate that `system.method(...)` calls inside native regions target
    /// interface methods. This runs over native text regions only.
    pub fn validate_system_calls_interface(
        &self,
        body_bytes: &[u8],
        regions: &[RegionV3],
        interface_methods: &HashSet<String>,
    ) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        if interface_methods.is_empty() {
            return issues;
        }
        for r in regions {
            if let RegionV3::NativeText { span } = r {
                if span.end <= span.start || span.end > body_bytes.len() { continue; }
                let seg = &body_bytes[span.start..span.end];
                let mut i = 0usize;
                while i + 6 < seg.len() {
                    // Look for "system" at an identifier boundary.
                    if &seg[i..i + 6] == b"system" {
                        let prev_is_ident = if i == 0 {
                            false
                        } else {
                            let b = seg[i - 1];
                            (b as char).is_ascii_alphanumeric() || b == b'_'
                        };
                        let mut j = i + 6;
                        if !prev_is_ident && j < seg.len() && seg[j] == b'.' {
                            j += 1;
                            if j >= seg.len() {
                                break;
                            }
                            // Parse method identifier after the dot.
                            let name_start = j;
                            if !((seg[j] as char).is_ascii_alphabetic() || seg[j] == b'_') {
                                i += 1;
                                continue;
                            }
                            j += 1;
                            while j < seg.len() {
                                let c = seg[j] as char;
                                if c.is_ascii_alphanumeric() || seg[j] == b'_' {
                                    j += 1;
                                } else {
                                    break;
                                }
                            }
                            let name = String::from_utf8_lossy(&seg[name_start..j]).to_string();
                            // Ignore the special `system.return` variable.
                            if name != "return" && !name.is_empty() && !interface_methods.contains(&name) {
                                issues.push(ValidationIssueV3 {
                                    message: format!("E406: system.{} call must target an interface method", name),
                                });
                            }
                            i = j;
                            continue;
                        }
                    }
                    i += 1;
                }
            }
        }
        issues
    }

    /// Validate `system.return` usage according to V3 policy.
    ///
    /// V3 semantics now treat `system.return` as a per-call slot that can be
    /// read or written from handlers, actions, and non-static operations.
    /// Sugar such as `return expr` is handled in the code generators for
    /// handlers only; the validator now only reserves a hook for future
    /// placement checks and does not emit E407 by default.
    pub fn validate_system_return_usage(
        &self,
        body_bytes: &[u8],
        regions: &[RegionV3],
        kind: super::validator::BodyKindV3,
    ) -> Vec<ValidationIssueV3> {
        let _ = (body_bytes, regions, kind);
        Vec::new()
    }

    // Outer grammar structural checks (headers inside sections)
    pub fn validate_outer_grammar(&self, bytes: &[u8], start: usize, _lang: TargetLanguage, outline: &[OutlineItemV3]) -> Vec<ValidationIssueV3> {
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
                    // Machine handlers must live under `machine:`, but interface
                    // handlers are also valid and should not trigger this check.
                    if sec_kind != Some(Sec::Machine) && sec_kind != Some(Sec::Interface) {
                        issues.push(ValidationIssueV3{
                            message: "handler body outside machine: section".into()
                        });
                    }
                }
                super::validator::BodyKindV3::Function => {
                    // Functions live at top level; they are not tied to a section.
                }
                super::validator::BodyKindV3::Unknown => {}
            }
        }
        issues
    }

    /// Ensure that there is at most one `fn main` per module.
    /// This is a module-level semantic constraint used to decide runnable modules.
    /// We detect `fn main` and `async fn main` from header text rather than relying
    /// on outline owner_id, to keep tolerant scan behavior simple.
    pub fn validate_main_functions(&self, bytes: &[u8], outline: &[OutlineItemV3]) -> Vec<ValidationIssueV3> {
        let mut main_count = 0usize;
        for it in outline {
            if let BodyKindV3::Function = it.kind {
                let span = it.header_span;
                if span.end <= span.start || span.end > bytes.len() { continue; }
                if let Ok(hdr) = std::str::from_utf8(&bytes[span.start..span.end]) {
                    let s = hdr.trim_start();
                    if s.starts_with("fn main(") || s.starts_with("async fn main(") {
                        main_count += 1;
                    }
                }
            }
        }
        if main_count > 1 {
            vec![ValidationIssueV3 {
                message: "E115: multiple 'main' functions in module".into(),
            }]
        } else {
            Vec::new()
        }
    }

    /// Enforce per-system block ordering and uniqueness using the outer ModuleAst:
    /// operations:, interface:, machine:, actions:, domain: (when present).
    /// Blocks are optional but, when present, must appear in this canonical order,
    /// and at most one of each block is allowed per system.
    pub fn validate_system_block_order_ast(&self, module: &ModuleAst) -> Vec<ValidationIssueV3> {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        for sys in &module.systems {
            if sys.section_order.is_empty() {
                continue;
            }
            let mut seen_ops = 0usize;
            let mut seen_iface = 0usize;
            let mut seen_machine = 0usize;
            let mut seen_actions = 0usize;
            let mut seen_domain = 0usize;
            let mut last_idx: i32 = -1;
            for kind in &sys.section_order {
                let (idx, counter, label) = match kind {
                    SystemSectionKind::Operations => {
                        seen_ops += 1;
                        (0, seen_ops, "operations:")
                    }
                    SystemSectionKind::Interface => {
                        seen_iface += 1;
                        (1, seen_iface, "interface:")
                    }
                    SystemSectionKind::Machine => {
                        seen_machine += 1;
                        (2, seen_machine, "machine:")
                    }
                    SystemSectionKind::Actions => {
                        seen_actions += 1;
                        (3, seen_actions, "actions:")
                    }
                    SystemSectionKind::Domain => {
                        seen_domain += 1;
                        (4, seen_domain, "domain:")
                    }
                };
                if counter > 1 {
                    issues.push(ValidationIssueV3 {
                        message: format!("E114: duplicate '{}' block in system", label),
                    });
                }
                if (idx as i32) < last_idx {
                    issues.push(ValidationIssueV3 {
                        message: "E113: system blocks out of order: expected operations:, interface:, machine:, actions:, domain:".into(),
                    });
                    // Once mis-ordered, further ordering diagnostics for this system add little value.
                    break;
                }
                last_idx = idx as i32;
            }
        }
        issues
    }

    /// Ensure state headers inside `machine:` sections have a `{` on the same line
    /// using the outer ModuleAst to locate machine spans.
    pub fn validate_machine_state_headers_ast(
        &self,
        bytes: &[u8],
        module: &ModuleAst,
    ) -> Vec<ValidationIssueV3> {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        let n = bytes.len();
        for sys in &module.systems {
            if let Some(span) = sys.sections.machine {
                let mut p = span.start.min(n);
                let end = span.end.min(n);
                while p < end {
                    // Skip leading whitespace and blank lines.
                    while p < end
                        && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n')
                    {
                        p += 1;
                    }
                    if p >= end {
                        break;
                    }
                    // Skip comment-only lines quickly.
                    if bytes[p] == b'#' {
                        while p < end && bytes[p] != b'\n' {
                            p += 1;
                        }
                        continue;
                    }
                    if p + 1 < end && bytes[p] == b'/' {
                        let c2 = bytes[p + 1];
                        if c2 == b'/' {
                            while p < end && bytes[p] != b'\n' {
                                p += 1;
                            }
                            continue;
                        } else if c2 == b'*' {
                            p += 2;
                            while p + 1 < end {
                                if bytes[p] == b'*' && bytes[p + 1] == b'/' {
                                    p += 2;
                                    break;
                                }
                                p += 1;
                            }
                            continue;
                        }
                    }
                    // Check for a potential state header starting with '$'.
                    if bytes[p] == b'$' {
                        let next = if p + 1 < end { bytes[p + 1] } else { b'\n' };
                        let is_ident_start =
                            next.is_ascii_alphabetic() || next == b'_';
                        if is_ident_start {
                            // Scan to end of line or first '{'.
                            let mut q = p;
                            let mut seen_lbrace = false;
                            while q < end && bytes[q] != b'\n' {
                                if bytes[q] == b'{' {
                                    seen_lbrace = true;
                                    break;
                                }
                                q += 1;
                            }
                            if !seen_lbrace {
                                issues.push(ValidationIssueV3 {
                                    message: "E112: missing '{' after state header in machine: section"
                                        .into(),
                                });
                            }
                        }
                    }
                    // Advance to next line.
                    while p < end && bytes[p] != b'\n' {
                        p += 1;
                    }
                }
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

    // Arcanum-backed variant: resolve against symbol table instead of coarse HashSet.
    // A transition is considered unknown only if it cannot be resolved via
    // Arcanum *and* the coarse known-state set does not contain the target.
    pub fn validate_transition_targets_arcanum(
        &self,
        mir: &[MirItemV3],
        arcanum: &Arcanum,
        known_states: &HashSet<String>,
        system_name: Option<&str>,
    ) -> Vec<ValidationIssueV3> {
        let mut issues = Vec::new();
        let sys = system_name.unwrap_or("_");
        for m in mir {
            if let MirItemV3::Transition{ target, .. } = m {
                let coarse_known = known_states.contains(target);
                let arc_known = arcanum
                    .resolve_state(sys, target)
                    .or_else(|| arcanum.resolve_state("_", target))
                    .is_some();
                if !coarse_known && !arc_known {
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

    /// Ensure handlers in machine: are nested within a state block, using the Arcanum/AST
    /// state spans rather than relying solely on OutlineScanner state_id. Interface handlers
    /// are explicitly excluded from this check.
    pub fn validate_handlers_in_state_ast(
        &self,
        bytes: &[u8],
        outline: &[OutlineItemV3],
        module: &ModuleAst,
        arc: &Arcanum,
    ) -> Vec<ValidationIssueV3> {
        // Collect machine section spans from the ModuleAst.
        let n = bytes.len();
        let mut machine_spans: Vec<(usize, usize)> = Vec::new();
        for sys in &module.systems {
            if let Some(machine_span) = sys.sections.machine {
                let start = machine_span.start.min(n);
                let end = machine_span.end.min(n);
                machine_spans.push((start, end));
            }
        }
        // If there is no machine: section at all, there is nothing to validate for E404.
        if machine_spans.is_empty() {
            return Vec::new();
        }

        // Collect all state spans from Arcanum.
        let mut state_spans: Vec<(usize, usize)> = Vec::new();
        for sys in arc.systems.values() {
            for mach in sys.machines.values() {
                for st in mach.states.values() {
                    state_spans.push((st.span.start, st.span.end));
                }
            }
        }

        let mut issues = Vec::new();
        for it in outline {
            if let BodyKindV3::Handler = it.kind {
                let header_pos = it.header_span.start;
                // Only enforce E404 for handlers whose headers lie within a machine: section.
                let in_machine = machine_spans
                    .iter()
                    .any(|(s, e)| header_pos >= *s && header_pos < *e);
                if !in_machine {
                    continue;
                }
                let mut in_state = false;
                for (s, e) in &state_spans {
                    if header_pos >= *s && header_pos < *e {
                        in_state = true;
                        break;
                    }
                }
                if !in_state {
                    issues.push(ValidationIssueV3 {
                        message: "E404: handler body must be inside a state block".into(),
                    });
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
pub enum BodyKindV3 { Handler, Action, Operation, Function, Unknown }

#[derive(Debug, Clone, Default)]
pub struct ValidatorPolicyV3 { pub body_kind: Option<BodyKindV3> }
