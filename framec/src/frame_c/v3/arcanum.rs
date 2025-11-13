use std::collections::HashMap;

use super::ast::{SystemDecl, MachineDecl, StateDecl};

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

pub fn build_arcanum_from_outline_bytes(bytes: &[u8], start: usize) -> Arcanum {
    // Try to detect system name: scan for "system <Ident> {"
    fn scan_system_name(bytes: &[u8], start: usize) -> Option<String> {
        let n = bytes.len();
        let mut i = start;
        while i < n {
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') { i += 1; }
            if i >= n { break; }
            // skip line comments
            if bytes[i] == b'/' && i+1 < n && bytes[i+1] == b'/' { while i < n && bytes[i] != b'\n' { i += 1; } continue; }
            if bytes[i] == b'#' { while i < n && bytes[i] != b'\n' { i += 1; } continue; }
            // skip block comments
            if bytes[i] == b'/' && i+1 < n && bytes[i+1] == b'*' { i += 2; while i+1 < n && !(bytes[i] == b'*' && bytes[i+1] == b'/') { i += 1; } if i+1 < n { i += 2; } continue; }
            // read ident
            let mut j = i; while j < n && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
            if j > i {
                let kw = String::from_utf8_lossy(&bytes[i..j]).to_ascii_lowercase();
                if kw == "system" {
                    while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') { j += 1; }
                    let name_start = j; while j < n && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') { j += 1; }
                    if j > name_start { return Some(String::from_utf8_lossy(&bytes[name_start..j]).to_string()); }
                }
            }
            while i < n && bytes[i] != b'\n' { i += 1; }
        }
        None
    }
    // Minimal scanner for machine: sections to populate state names and optional parents
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
            match kw.as_str() {
                "machine" => marks.push((line_start, Sec::Machine)),
                "actions" | "operations" | "interface" => marks.push((line_start, Sec::Other)),
                _ => {}
            }
        }
        while i < n && bytes[i] != b'\n' { i += 1; }
    }
    let mut arc = Arcanum::new();
    let sys_name = scan_system_name(bytes, 0).unwrap_or_else(|| "_".to_string());
    let mut sys = SystemEntry::default();
    // Build section ranges
    for idx in 0..marks.len() {
        let (spos, sec) = marks[idx];
        let epos = if idx + 1 < marks.len() { marks[idx+1].0 } else { n };
        if sec != Sec::Machine { continue; }
        // We don't track machine names here; create a single machine entry per file
        let mut machine = MachineEntry::default();
        let mut p = spos;
        while p < epos {
            while p < epos && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') { p += 1; }
            if p >= epos { break; }
            if bytes[p] == b'$' {
                let mut k = p + 1;
                if k < epos && (bytes[k].is_ascii_alphabetic() || bytes[k] == b'_') {
                    k += 1; while k < epos && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') { k += 1; }
                    let name = String::from_utf8_lossy(&bytes[p+1..k]).to_string();
                    let mut q = k; while q < epos && (bytes[q] == b' ' || bytes[q] == b'\t') { q += 1; }
                    let mut parent: Option<String> = None;
                    if q + 2 < epos && bytes[q] == b'=' && bytes[q+1] == b'>' {
                        let mut r = q + 2; while r < epos && (bytes[r] == b' ' || bytes[r] == b'\t') { r += 1; }
                        if r < epos && bytes[r] == b'$' {
                            let mut s = r + 1; if s < epos && (bytes[s].is_ascii_alphabetic() || bytes[s] == b'_') {
                                s += 1; while s < epos && (bytes[s].is_ascii_alphanumeric() || bytes[s] == b'_') { s += 1; }
                                parent = Some(String::from_utf8_lossy(&bytes[r+1..s]).to_string());
                                q = s;
                            }
                        }
                    }
                    // header must contain '{' on the same line
                    let mut has_lbrace = false; let mut t = q; while t < epos && bytes[t] != b'\n' { if bytes[t] == b'{' { has_lbrace = true; break; } t += 1; }
                    if has_lbrace {
                        machine.states.insert(name.clone(), StateDecl{ name, parent, span: super::ast::Span{ start: p, end: t } });
                    }
                }
            }
            while p < epos && bytes[p] != b'\n' { p += 1; }
        }
        // Use an auto name for machine entry
        let machine_name = format!("machine_{}", idx);
        sys.machines.insert(machine_name, machine);
    }
    arc.systems.insert(sys_name, sys);
    arc
}
