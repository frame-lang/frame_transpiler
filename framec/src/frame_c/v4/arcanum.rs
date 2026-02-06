use std::collections::{HashMap, HashSet};

use super::ast::{SystemDecl, MachineDecl, StateDecl, ModuleAst, Span};
use super::frame_ast::{
    FrameAst, SystemAst as FrameSystemAst, StateAst as FrameStateAst,
    ModuleAst as FrameModuleAst, MachineAst as FrameMachineAst,
    HandlerAst as FrameHandlerAst, Span as FrameSpan,
};

/// Variable type information for Frame validation
#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Frame,      // Frame-specific types
    Native,     // Native language types
    Unknown,    // Type not yet determined
}

// ============================================================================
// Enhanced Arcanum Types (Phase 7)
// ============================================================================

/// Kind of Frame-declared symbol
#[derive(Debug, Clone, PartialEq)]
pub enum FrameSymbolKind {
    StateParam,
    HandlerParam,
    DomainVar,
    InterfaceMethod,
    Action,
    Operation,
}

/// A Frame-declared symbol with full metadata
#[derive(Debug, Clone)]
pub struct FrameSymbol {
    pub name: String,
    pub kind: FrameSymbolKind,
    pub declared_at: Span,
    pub symbol_type: Option<String>,  // Type annotation if present
}

/// Handler entry with parameters and body span
#[derive(Debug, Clone)]
pub struct HandlerEntry {
    pub event: String,
    pub params: Vec<FrameSymbol>,
    pub body_span: Span,              // For splicer to extract body content
    pub is_enter: bool,               // $>
    pub is_exit: bool,                // $<
}

/// Enhanced state entry with handlers
#[derive(Debug, Clone)]
pub struct EnhancedStateEntry {
    pub name: String,
    pub params: Vec<FrameSymbol>,
    pub parent: Option<String>,
    pub handlers: HashMap<String, HandlerEntry>,
    pub span: Span,
}

/// Enhanced machine entry with enhanced states
#[derive(Debug, Clone, Default)]
pub struct EnhancedMachineEntry {
    pub states: HashMap<String, EnhancedStateEntry>,
}

// ============================================================================
// Original Arcanum Types (kept for backwards compatibility)
// ============================================================================

/// Enhanced Arcanum - Frame's authoritative symbol table for validation
#[derive(Debug, Default, Clone)]
pub struct Arcanum {
    pub systems: HashMap<String, SystemEntry>,
}

/// Complete system metadata for validation
#[derive(Debug, Default, Clone)]
pub struct SystemEntry {
    pub decl: Option<SystemDecl>,
    pub machines: HashMap<String, MachineEntry>,

    // Enhanced for validation (Stage 1)
    pub interface_methods: HashSet<String>,
    pub actions: HashSet<String>,
    pub operations: HashSet<String>,
    pub domain_vars: HashMap<String, VarType>,

    // Phase 7: Enhanced with full symbol info
    pub domain_symbols: HashMap<String, FrameSymbol>,
    pub enhanced_machines: HashMap<String, EnhancedMachineEntry>,
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
    
    // Enhanced validation methods (Stage 1)
    
    /// Check if a method is in the system's interface (E406)
    pub fn is_interface_method(&self, system: &str, method: &str) -> bool {
        self.systems.get(system)
            .map(|s| s.interface_methods.contains(method))
            .unwrap_or(false)
    }
    
    /// Check if an action exists in the system
    pub fn has_action(&self, system: &str, action: &str) -> bool {
        self.systems.get(system)
            .map(|s| s.actions.contains(action))
            .unwrap_or(false)
    }
    
    /// Check if an operation exists in the system
    pub fn has_operation(&self, system: &str, operation: &str) -> bool {
        self.systems.get(system)
            .map(|s| s.operations.contains(operation))
            .unwrap_or(false)
    }
    
    /// Get state parameter count for arity validation (E405)
    pub fn get_state_param_count(&self, system: &str, state: &str) -> Option<usize> {
        self.resolve_state(system, state)
            .map(|s| s.params.len())
    }
    
    /// Validate state transition exists (E402)
    pub fn validate_transition(&self, system: &str, target_state: &str) -> Result<(), String> {
        if self.resolve_state(system, target_state).is_some() {
            Ok(())
        } else {
            let available = self.get_system_states(system);
            if available.is_empty() {
                Err(format!("Unknown state '{}' (system has no states)", target_state))
            } else {
                Err(format!("Unknown state '{}'. Available states: {}", 
                    target_state, 
                    available.join(", ")))
            }
        }
    }
    
    /// Get all states in a system (for error messages)
    pub fn get_system_states(&self, system: &str) -> Vec<String> {
        self.systems.get(system)
            .map(|s| {
                let mut states = Vec::new();
                for m in s.machines.values() {
                    for state in m.states.keys() {
                        states.push(format!("${}", state));
                    }
                }
                states.sort();
                states
            })
            .unwrap_or_default()
    }

    // ========================================================================
    // Phase 7: Enhanced Scope Resolution API
    // ========================================================================

    /// Resolve a Frame symbol in the given scope context.
    /// Searches handler scope → state scope → system scope.
    /// Returns None if not a Frame symbol (might be native variable).
    pub fn resolve_frame_symbol(
        &self,
        system: &str,
        state: Option<&str>,
        handler: Option<&str>,
        name: &str,
    ) -> Option<&FrameSymbol> {
        let sys = self.systems.get(system)?;

        // 1. Check handler scope (if in a handler)
        if let (Some(state_name), Some(handler_name)) = (state, handler) {
            if let Some(machine) = sys.enhanced_machines.values().next() {
                if let Some(state_entry) = machine.states.get(state_name) {
                    if let Some(handler_entry) = state_entry.handlers.get(handler_name) {
                        if let Some(sym) = handler_entry.params.iter().find(|p| p.name == name) {
                            return Some(sym);
                        }
                    }
                }
            }
        }

        // 2. Check state scope (if in a state)
        if let Some(state_name) = state {
            if let Some(machine) = sys.enhanced_machines.values().next() {
                if let Some(state_entry) = machine.states.get(state_name) {
                    if let Some(sym) = state_entry.params.iter().find(|p| p.name == name) {
                        return Some(sym);
                    }
                }
            }
        }

        // 3. Check system scope (domain vars)
        if let Some(sym) = sys.domain_symbols.get(name) {
            return Some(sym);
        }

        // Not a Frame symbol
        None
    }

    /// Check if a name is a Frame-declared symbol in the given context
    pub fn is_frame_symbol(
        &self,
        system: &str,
        state: Option<&str>,
        handler: Option<&str>,
        name: &str,
    ) -> bool {
        self.resolve_frame_symbol(system, state, handler, name).is_some()
    }

    /// Get all handlers for a state (for codegen iteration)
    pub fn get_state_handlers(&self, system: &str, state: &str) -> Vec<&HandlerEntry> {
        self.systems.get(system)
            .and_then(|s| s.enhanced_machines.values().next())
            .and_then(|m| m.states.get(state))
            .map(|s| s.handlers.values().collect())
            .unwrap_or_default()
    }

    /// Get enhanced state entry (for codegen)
    pub fn get_enhanced_state(&self, system: &str, state: &str) -> Option<&EnhancedStateEntry> {
        self.systems.get(system)
            .and_then(|s| s.enhanced_machines.values().next())
            .and_then(|m| m.states.get(state))
    }

    /// Get all enhanced states for a system (for codegen iteration)
    pub fn get_enhanced_states(&self, system: &str) -> Vec<&EnhancedStateEntry> {
        self.systems.get(system)
            .and_then(|s| s.enhanced_machines.values().next())
            .map(|m| m.states.values().collect())
            .unwrap_or_default()
    }
}

pub fn build_arcanum_from_outline_bytes(bytes: &[u8], _start: usize) -> Arcanum {
    // DEPRECATED: kept temporarily for compatibility; prefer build_arcanum_from_module_ast.
    // For now, delegate to a ModuleAst-based builder using the default language
    // assumptions of the outer pipeline (Python3 is sufficient for brace/indent).
    let module = crate::frame_c::v4::system_parser::SystemParserV3::parse_module(bytes, crate::frame_c::visitors::TargetLanguage::Python3);
    build_arcanum_from_module_ast(bytes, &module)
}

/// Extract method names from interface, actions, or operations sections
pub(crate) fn collect_methods_in_section(bytes: &[u8], span: &Span) -> HashSet<String> {
    let mut methods = HashSet::new();
    let n = bytes.len();
    let start = span.start.min(n);
    let end = span.end.min(n);
    let mut p = start;
    
    while p < end {
        // Skip whitespace
        while p < end && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') {
            p += 1;
        }
        if p >= end { break; }
        
        // Look for method names - they start with a letter or underscore
        if bytes[p].is_ascii_alphabetic() || bytes[p] == b'_' {
            let name_start = p;
            // Collect identifier
            while p < end && (bytes[p].is_ascii_alphanumeric() || bytes[p] == b'_') {
                p += 1;
            }
            let name = String::from_utf8_lossy(&bytes[name_start..p]).to_string();
            
            // Skip whitespace after name
            while p < end && (bytes[p] == b' ' || bytes[p] == b'\t') {
                p += 1;
            }
            
            // Check if followed by '(' indicating a method
            if p < end && bytes[p] == b'(' {
                methods.insert(name);
            }
        }
        
        // Move to next line or character
        while p < end && bytes[p] != b'\n' {
            p += 1;
        }
    }
    
    methods
}

/// Extract domain variable names from domain section
pub(crate) fn collect_domain_vars(bytes: &[u8], span: &Span) -> HashMap<String, VarType> {
    let mut vars = HashMap::new();
    let n = bytes.len();
    let start = span.start.min(n);
    let end = span.end.min(n);
    let mut p = start;
    
    while p < end {
        // Skip whitespace
        while p < end && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r' || bytes[p] == b'\n') {
            p += 1;
        }
        if p >= end { break; }
        
        // Look for variable declarations (simple heuristic)
        // Could be: var name, name:, or just name
        let mut var_name = None;
        
        // Check for 'var' keyword
        if p + 3 < end && &bytes[p..p+3] == b"var" && 
           (p + 3 >= end || bytes[p+3] == b' ' || bytes[p+3] == b'\t') {
            p += 3;
            while p < end && (bytes[p] == b' ' || bytes[p] == b'\t') {
                p += 1;
            }
        }
        
        // Get identifier
        if p < end && (bytes[p].is_ascii_alphabetic() || bytes[p] == b'_') {
            let name_start = p;
            while p < end && (bytes[p].is_ascii_alphanumeric() || bytes[p] == b'_') {
                p += 1;
            }
            var_name = Some(String::from_utf8_lossy(&bytes[name_start..p]).to_string());
            
            // Skip whitespace
            while p < end && (bytes[p] == b' ' || bytes[p] == b'\t') {
                p += 1;
            }
            
            // Check if it's a variable declaration (has : or =)
            if p < end && (bytes[p] == b':' || bytes[p] == b'=') {
                if let Some(name) = var_name {
                    // For now, mark all as Unknown type
                    // Later we can parse type annotations
                    vars.insert(name, VarType::Unknown);
                }
            }
        }
        
        // Move to next line
        while p < end && bytes[p] != b'\n' {
            p += 1;
        }
    }
    
    vars
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
    if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[build_arcanum] Module has {} systems", module.systems.len());
        for sys_ast in &module.systems {
            eprintln!("[build_arcanum] System: {}", sys_ast.name);
        }
    }
    for sys_ast in &module.systems {
        let mut sys_entry = SystemEntry::default();
        
        // Collect interface methods
        if let Some(iface_span) = sys_ast.sections.interface {
            let methods = collect_methods_in_section(bytes, &iface_span);
            sys_entry.interface_methods = methods;
            if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[build_arcanum] System {} interface methods: {:?}", sys_ast.name, sys_entry.interface_methods);
            }
        }
        
        // Collect actions
        if let Some(actions_span) = sys_ast.sections.actions {
            let actions = collect_methods_in_section(bytes, &actions_span);
            sys_entry.actions = actions;
            if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[build_arcanum] System {} actions: {:?}", sys_ast.name, sys_entry.actions);
            }
        }
        
        // Collect operations
        if let Some(ops_span) = sys_ast.sections.operations {
            let operations = collect_methods_in_section(bytes, &ops_span);
            sys_entry.operations = operations;
            if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[build_arcanum] System {} operations: {:?}", sys_ast.name, sys_entry.operations);
            }
        }
        
        // Collect domain variables
        if let Some(domain_span) = sys_ast.sections.domain {
            let vars = collect_domain_vars(bytes, &domain_span);
            sys_entry.domain_vars = vars;
            if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[build_arcanum] System {} domain vars: {} vars", sys_ast.name, sys_entry.domain_vars.len());
            }
        }
        
        // Collect machine states
        if let Some(machine_span) = sys_ast.sections.machine {
            let mut machine_entry = MachineEntry::default();
            let states = collect_states_in_machine(bytes, &machine_span);
            if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[build_arcanum] System {} has {} states", sys_ast.name, states.len());
                for s in &states {
                    eprintln!("[build_arcanum]   State: {}", s.name);
                }
            }
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

// ============================================================================
// New AST-based Arcanum builder (Phase 1.3)
// ============================================================================

/// Build Arcanum from the proper Frame AST (replaces byte-scanning approach)
///
/// This is the preferred way to build the symbol table. It extracts all
/// information directly from the parsed Frame AST, avoiding the error-prone
/// byte-level scanning.
pub fn build_arcanum_from_frame_ast(ast: &FrameAst) -> Arcanum {
    let mut arc = Arcanum::new();

    match ast {
        FrameAst::System(system) => {
            let entry = build_system_entry_from_frame_ast(system);
            arc.systems.insert(system.name.clone(), entry);
        }
        FrameAst::Module(module) => {
            for system in &module.systems {
                let entry = build_system_entry_from_frame_ast(system);
                arc.systems.insert(system.name.clone(), entry);
            }
        }
    }

    if std::env::var("FRAME_TRANSPILER_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[build_arcanum_from_frame_ast] Built arcanum with {} systems", arc.systems.len());
        for (name, entry) in &arc.systems {
            eprintln!("[build_arcanum_from_frame_ast]   System '{}': {} states, {} interface, {} actions, {} operations",
                name,
                entry.machines.values().map(|m| m.states.len()).sum::<usize>(),
                entry.interface_methods.len(),
                entry.actions.len(),
                entry.operations.len()
            );
        }
    }

    arc
}

/// Build a SystemEntry from a Frame AST SystemAst
fn build_system_entry_from_frame_ast(system: &FrameSystemAst) -> SystemEntry {
    let mut entry = SystemEntry::default();

    // Extract interface methods
    for method in &system.interface {
        entry.interface_methods.insert(method.name.clone());
    }

    // Extract actions
    for action in &system.actions {
        entry.actions.insert(action.name.clone());
    }

    // Extract operations
    for operation in &system.operations {
        entry.operations.insert(operation.name.clone());
    }

    // Extract domain variables (old style)
    for var in &system.domain {
        let var_type = if var.is_frame {
            VarType::Frame
        } else {
            VarType::Native
        };
        entry.domain_vars.insert(var.name.clone(), var_type);
    }

    // Phase 7: Extract domain variables with full symbol info
    for var in &system.domain {
        let symbol = FrameSymbol {
            name: var.name.clone(),
            kind: FrameSymbolKind::DomainVar,
            declared_at: Span { start: var.span.start, end: var.span.end },
            symbol_type: Some(format!("{:?}", var.var_type)),
        };
        entry.domain_symbols.insert(var.name.clone(), symbol);
    }

    // Extract machine states (old style for backwards compat)
    if let Some(ref machine) = system.machine {
        let mut machine_entry = MachineEntry::default();

        for state in &machine.states {
            let state_decl = build_state_decl_from_frame_ast(state);
            machine_entry.states.insert(state.name.clone(), state_decl);
        }

        entry.machines.insert("machine".to_string(), machine_entry);
    }

    // Phase 7: Build enhanced machine with handlers
    if let Some(ref machine) = system.machine {
        let enhanced_machine = build_enhanced_machine_from_frame_ast(machine);
        entry.enhanced_machines.insert("machine".to_string(), enhanced_machine);
    }

    entry
}

/// Build an EnhancedMachineEntry from Frame AST MachineAst
fn build_enhanced_machine_from_frame_ast(machine: &FrameMachineAst) -> EnhancedMachineEntry {
    let mut entry = EnhancedMachineEntry::default();

    for state in &machine.states {
        let enhanced_state = build_enhanced_state_from_frame_ast(state);
        entry.states.insert(state.name.clone(), enhanced_state);
    }

    entry
}

/// Build an EnhancedStateEntry from Frame AST StateAst
fn build_enhanced_state_from_frame_ast(state: &FrameStateAst) -> EnhancedStateEntry {
    let mut handlers = HashMap::new();

    // Helper to convert frame_ast::Span to ast::Span
    fn convert_span(s: &FrameSpan) -> Span {
        Span { start: s.start, end: s.end }
    }

    // Convert state parameters to FrameSymbols
    let params: Vec<FrameSymbol> = state.params.iter().map(|p| {
        FrameSymbol {
            name: p.name.clone(),
            kind: FrameSymbolKind::StateParam,
            declared_at: convert_span(&p.span),
            symbol_type: Some(format!("{:?}", p.param_type)),
        }
    }).collect();

    // Convert regular handlers
    for handler in &state.handlers {
        let handler_entry = build_handler_entry_from_ast(handler);
        handlers.insert(handler.event.clone(), handler_entry);
    }

    // Convert enter handler ($>)
    if let Some(ref enter) = state.enter {
        let handler_entry = HandlerEntry {
            event: "$>".to_string(),
            params: enter.params.iter().map(|p| {
                FrameSymbol {
                    name: p.name.clone(),
                    kind: FrameSymbolKind::HandlerParam,
                    declared_at: convert_span(&p.span),
                    symbol_type: Some(format!("{:?}", p.param_type)),
                }
            }).collect(),
            body_span: convert_span(&enter.body.span),
            is_enter: true,
            is_exit: false,
        };
        handlers.insert("$>".to_string(), handler_entry);
    }

    // Convert exit handler ($<)
    if let Some(ref exit) = state.exit {
        let handler_entry = HandlerEntry {
            event: "$<".to_string(),
            params: Vec::new(),  // Exit handlers don't have params
            body_span: convert_span(&exit.body.span),
            is_enter: false,
            is_exit: true,
        };
        handlers.insert("$<".to_string(), handler_entry);
    }

    EnhancedStateEntry {
        name: state.name.clone(),
        params,
        parent: state.parent.clone(),
        handlers,
        span: convert_span(&state.span),
    }
}

/// Build a HandlerEntry from Frame AST HandlerAst
fn build_handler_entry_from_ast(handler: &FrameHandlerAst) -> HandlerEntry {
    // Helper to convert frame_ast::Span to ast::Span
    fn convert_span(s: &FrameSpan) -> Span {
        Span { start: s.start, end: s.end }
    }

    let params: Vec<FrameSymbol> = handler.params.iter().map(|p| {
        FrameSymbol {
            name: p.name.clone(),
            kind: FrameSymbolKind::HandlerParam,
            declared_at: convert_span(&p.span),
            symbol_type: Some(format!("{:?}", p.param_type)),
        }
    }).collect();

    HandlerEntry {
        event: handler.event.clone(),
        params,
        body_span: convert_span(&handler.body.span),
        is_enter: false,
        is_exit: false,
    }
}

/// Build a StateDecl from a Frame AST StateAst
fn build_state_decl_from_frame_ast(state: &FrameStateAst) -> StateDecl {
    StateDecl {
        name: state.name.clone(),
        parent: state.parent.clone(),
        params: state.params.iter().map(|p| p.name.clone()).collect(),
        span: Span {
            start: state.span.start,
            end: state.span.end,
        },
    }
}
