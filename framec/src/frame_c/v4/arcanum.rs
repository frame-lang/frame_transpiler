use std::collections::HashMap;

use super::ast::{SystemDecl, MachineDecl, StateDecl, ModuleAst, Span};

#[derive(Debug, Default, Clone)]
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

#[derive(Debug, Default, Clone)]
pub struct SystemEntry {
    pub decl: Option<SystemDecl>,
    pub machines: HashMap<String, MachineEntry>,
}

#[derive(Debug, Default, Clone)]
pub struct MachineEntry {
    pub decl: Option<MachineDecl>,
    pub states: HashMap<String, StateDecl>,
}

impl Arcanum {
    pub fn new() -> Self { Self { systems: HashMap::new() } }

    pub fn resolve_state<'a>(&'a self, system: &str, state: &str) -> Option<&'a StateDecl> {
        self.systems.get(system)
            .and_then(|s| s.machines.values().find_map(|m| m.states.get(state)))
    }

    pub fn has_parent(&self, system: &str, state: &str) -> bool {
        self.resolve_state(system, state).and_then(|d| d.parent.as_ref()).is_some()
    }

    pub fn any_parent_relation(&self) -> bool {
        self.systems.values().any(|sys| sys.machines.values().any(|m| m.states.values().any(|s| s.parent.is_some())))
    }

    pub fn all_state_names(&self) -> std::collections::HashSet<String> {
        let mut set = std::collections::HashSet::new();
        for sys in self.systems.values() {
            for m in sys.machines.values() {
                for s in m.states.values() { set.insert(s.name.clone()); }
            }
        }
        set
    }
}

pub fn build_arcanum_from_outline_bytes(bytes: &[u8], _start: usize) -> Arcanum {
    // DEPRECATED: kept temporarily for compatibility; prefer build_arcanum_from_module_ast.
    // For now, delegate to a ModuleAst-based builder using the default language
    // assumptions of the outer pipeline (Python3 is sufficient for brace/indent).
    let module = crate::frame_c::v4::system_parser::SystemParserV3::parse_module(bytes, crate::frame_c::visitors::TargetLanguage::Python3);
    build_arcanum_from_module_ast(bytes, &module)
}

fn collect_states_in_machine(bytes: &[u8], span: &Span) -> Vec<StateDecl> {
    let n = bytes.len();
    let start = span.start.min(n);
    let end = span.end.min(n);
    let mut temp_states: Vec<(String, Option<String>, Vec<String>, usize)> = Vec::new();
    let mut p = start;
    fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }
    while p < end {
        while p < end && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') { p += 1; }
        if p >= end { break; }
        if bytes[p] == b'$' {
            let mut k = p + 1;
            if k < end && (bytes[k].is_ascii_alphabetic() || bytes[k] == b'_') {
                k += 1; while k < end && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') { k += 1; }
                let name = String::from_utf8_lossy(&bytes[p+1..k]).to_string();
                let mut q = k; while q < end && (bytes[q] == b' ' || bytes[q] == b'\t') { q += 1; }
                let mut parent: Option<String> = None;
                if q + 2 < end && bytes[q] == b'=' && bytes[q+1] == b'>' {
                    let mut r = q + 2; while r < end && (bytes[r] == b' ' || bytes[r] == b'\t') { r += 1; }
                    if r < end && bytes[r] == b'$' {
                        let mut s = r + 1; if s < end && (bytes[s].is_ascii_alphabetic() || bytes[s] == b'_') {
                            s += 1; while s < end && (bytes[s].is_ascii_alphanumeric() || bytes[s] == b'_') { s += 1; }
                            parent = Some(String::from_utf8_lossy(&bytes[r+1..s]).to_string());
                            q = s;
                        }
                    }
                }
                // header must contain '{' on the same line
                let mut has_lbrace = false; let mut t = q; while t < end && bytes[t] != b'\n' { if bytes[t] == b'{' { has_lbrace = true; break; } t += 1; }
                if has_lbrace {
                    // Parse optional state parameter list: $State(param1, param2, ...) where
                    // each parameter is a bare identifier before any ':' or '='.
                    let mut params: Vec<String> = Vec::new();
                    let mut x = k;
                    while x < end && is_space(bytes[x]) { x += 1; }
                    if x < end && bytes[x] == b'(' {
                        x += 1; // after '('
                        loop {
                            while x < end && (is_space(bytes[x]) || bytes[x] == b',') { x += 1; }
                            if x >= end || bytes[x] == b')' { break; }
                            let ident_start = x;
                            while x < end {
                                let c = bytes[x];
                                if c == b':' || c == b'=' || c == b',' || c == b')' || is_space(c) {
                                    break;
                                }
                                x += 1;
                            }
                            if ident_start < x {
                                let token = String::from_utf8_lossy(&bytes[ident_start..x]).trim().to_string();
                                if !token.is_empty() {
                                    params.push(token);
                                }
                            }
                            while x < end && bytes[x] != b',' && bytes[x] != b')' { x += 1; }
                            if x < end && bytes[x] == b',' { x += 1; continue; }
                            if x < end && bytes[x] == b')' { break; }
                        }
                    }
                    temp_states.push((name, parent, params, p));
                }
            }
        }
        while p < end && bytes[p] != b'\n' { p += 1; }
    }
    let mut result = Vec::new();
    for idx in 0..temp_states.len() {
        let (ref name, ref parent, ref params, start_pos) = temp_states[idx];
        let span_end = if idx + 1 < temp_states.len() {
            temp_states[idx + 1].3
        } else {
            end
        };
        result.push(StateDecl {
            name: name.clone(),
            parent: parent.clone(),
            params: params.clone(),
            span: Span { start: start_pos, end: span_end },
        });
    }
    result
}

pub fn build_arcanum_from_module_ast(bytes: &[u8], module: &ModuleAst) -> Arcanum {
    let mut arc = Arcanum::new();
    for sys_ast in &module.systems {
        let mut sys_entry = SystemEntry::default();
        if let Some(machine_span) = sys_ast.sections.machine {
            let mut machine_entry = MachineEntry::default();
            let states = collect_states_in_machine(bytes, &machine_span);
            for s in states {
                machine_entry.states.insert(s.name.clone(), s);
            }
            // Single machine entry per system for now; name can be refined later.
            sys_entry.machines.insert("machine".to_string(), machine_entry);
        }
        arc.systems.insert(sys_ast.name.clone(), sys_entry);
    }
    arc
}
