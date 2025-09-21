// Semantic Analysis for Frame v0.62
// This module performs semantic analysis on the AST during the second parser pass
// Primary responsibility: Resolve call types to their actual semantic meaning

use crate::frame_c::ast::*;
use crate::frame_c::symbol_table::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SemanticAnalyzer<'a> {
    arcanum: &'a Arcanum,
    current_system: Option<&'a str>,
    current_class: Option<&'a str>,
    in_standalone_function: bool,
    is_static_context: bool,
    // v0.63: Cache for system symbol lookup
    current_system_symbol: Option<Rc<RefCell<SystemSymbol>>>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(arcanum: &'a Arcanum) -> Self {
        SemanticAnalyzer {
            arcanum,
            current_system: None,
            current_class: None,
            in_standalone_function: false,
            is_static_context: false,
            current_system_symbol: None,
        }
    }
    
    /// Resolve a call expression to its semantic type
    pub fn resolve_call(&self, call_expr: &CallExprNode) -> ResolvedCallType {
        let identifier = &call_expr.identifier.name.lexeme;
        
        // Handle self calls first
        if identifier.starts_with("self.") {
            let method_name = &identifier[5..]; // Strip "self."
            return self.resolve_self_call(method_name);
        }
        
        // Check if it's a qualified call (e.g., System.operation, Class.method)
        // These come as regular identifiers with dots in them during simple parsing
        if identifier.contains('.') {
            return self.resolve_qualified_call(identifier);
        }
        
        // Check if it's a call chain (parsed as actual chain)
        if let Some(chain) = &call_expr.call_chain {
            if !chain.is_empty() {
                return self.resolve_chained_call(identifier, chain);
            }
        }
        
        // Check if it's an action (with or without underscore)
        if identifier.starts_with('_') {
            let action_name = &identifier[1..];
            if let Some(_system) = self.current_system {
                // Actions are only valid in system context
                return ResolvedCallType::Action(action_name.to_string());
            }
        }
        
        // Check in current system context
        if let Some(_system) = self.current_system {
            // Check if it's an operation in the current system
            if self.is_operation_in_current_system(identifier) {
                return ResolvedCallType::Operation(identifier.to_string());
            }
            
            // Check if it's an action (without underscore)
            if self.is_action_in_current_system(identifier) {
                return ResolvedCallType::Action(identifier.to_string());
            }
        }
        
        // Check in current class context
        if let Some(_class) = self.current_class {
            // In a class, methods are resolved differently
            // For now, treat as external until we have better class method tracking
        }
        
        // Check if it's a known function in the symbol table
        if self.arcanum.lookup_function(identifier).is_some() {
            return ResolvedCallType::External(identifier.to_string());
        }
        
        // Default to external call (built-in functions, imports, etc.)
        ResolvedCallType::External(identifier.to_string())
    }
    
    fn resolve_self_call(&self, method_name: &str) -> ResolvedCallType {
        // Strip underscore if present
        let clean_name = if method_name.starts_with('_') {
            &method_name[1..]
        } else {
            method_name
        };
        
        if let Some(system) = self.current_system {
            if self.is_action_in_system(system, clean_name) {
                return ResolvedCallType::Action(clean_name.to_string());
            }
            
            if self.is_operation_in_system(system, clean_name) {
                return ResolvedCallType::Operation(clean_name.to_string());
            }
        }
        
        // Default to external (shouldn't happen for valid self calls)
        ResolvedCallType::External(method_name.to_string())
    }
    
    fn resolve_chained_call(&self, first: &str, chain: &[Box<dyn CallableExpr>]) -> ResolvedCallType {
        // v0.63: Improved chain resolution
        // Check if this is a System.operation() or Class.method() pattern
        // The parser creates this as a chain where 'first' is the system/class name
        // and chain[0] would be the operation/method call
        
        // Extract the method name from the chain if possible
        let method_name = if !chain.is_empty() {
            // Try to get the identifier from the first chain element
            // This is a simplified extraction - we need to handle CallableExpr properly
            "method" // Placeholder - need to extract actual name
        } else {
            return ResolvedCallType::External(first.to_string());
        };
        
        // Check if first is a system name
        if let Some(system_symbol) = self.arcanum.get_system_by_name(first) {
            // It's a system - check if the method is an operation in that system
            let sys = system_symbol.borrow();
            if let Some(ref operations) = sys.operations_block_symbol_opt {
                let ops_symtab = &operations.borrow().symtab_rcref;
                for (_name, symbol_rcref) in &ops_symtab.borrow().symbols {
                    if let SymbolType::OperationScope { operation_scope_symbol_rcref } = &*symbol_rcref.borrow() {
                        // For now, assume static if called with System.operation syntax
                        // TODO: Check actual @staticmethod attribute
                        return ResolvedCallType::SystemOperation {
                            system: first.to_string(),
                            operation: method_name.to_string(),
                            is_static: true,
                        };
                    }
                }
            }
        }
        
        // Check if this is a class (Frame v0.45+ supports classes)
        // For now, use uppercase heuristic
        if first.chars().next().map_or(false, |c| c.is_uppercase()) {
            return ResolvedCallType::ClassMethod {
                class: first.to_string(),
                method: method_name.to_string(),
                is_static: true, // Assume static for Class.method pattern
            };
        }
        
        ResolvedCallType::External(first.to_string())
    }
    
    fn resolve_qualified_call(&self, identifier: &str) -> ResolvedCallType {
        // Handle qualified calls like System.operation or Class.method
        let parts: Vec<&str> = identifier.split('.').collect();
        if parts.len() != 2 {
            return ResolvedCallType::External(identifier.to_string());
        }
        
        let qualifier = parts[0];
        let method = parts[1];
        
        // Check if it's a system operation call
        if self.arcanum.get_system_symbol(qualifier).is_ok() {
            // Check if the operation exists and is static
            // For now, assume it's static if called with qualified name
            return ResolvedCallType::SystemOperation {
                system: qualifier.to_string(),
                operation: method.to_string(),
                is_static: true, // TODO: Check actual @staticmethod attribute
            };
        }
        
        // Check if it's a class method call
        // TODO: Need class symbol lookup in arcanum
        // For now, check common patterns
        if qualifier.chars().next().map_or(false, |c| c.is_uppercase()) {
            // Likely a class name (starts with uppercase)
            return ResolvedCallType::ClassMethod {
                class: qualifier.to_string(),
                method: method.to_string(),
                is_static: true, // TODO: Check actual @staticmethod attribute
            };
        }
        
        // Check if it's a module function
        // For now, check if it's a known identifier that's not a system
        // Modules would start with lowercase typically
        if qualifier.chars().next().map_or(false, |c| c.is_lowercase()) {
            // Could be a module
            return ResolvedCallType::ModuleFunction {
                module: qualifier.to_string(),
                function: method.to_string(),
            };
        }
        
        // Default to external
        ResolvedCallType::External(identifier.to_string())
    }
    
    fn is_operation_in_current_system(&self, operation_name: &str) -> bool {
        // v0.63: Use cached system symbol for accurate lookup
        if let Some(ref system_symbol) = self.current_system_symbol {
            // Check if this specific system has the operation
            let sys = system_symbol.borrow();
            if let Some(ref operations) = sys.operations_block_symbol_opt {
                // Operations are in the symbol table of the operations block
                let ops_symtab = &operations.borrow().symtab_rcref;
                for (_name, symbol_rcref) in &ops_symtab.borrow().symbols {
                    if let SymbolType::OperationScope { operation_scope_symbol_rcref } = &*symbol_rcref.borrow() {
                        if operation_scope_symbol_rcref.borrow().name == operation_name {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    fn is_action_in_current_system(&self, action_name: &str) -> bool {
        // v0.63: Use cached system symbol for accurate lookup
        if let Some(ref system_symbol) = self.current_system_symbol {
            // Check if this specific system has the action
            let sys = system_symbol.borrow();
            if let Some(ref actions) = sys.actions_block_symbol_opt {
                // Actions are in the symbol table of the actions block
                let actions_symtab = &actions.borrow().symtab_rcref;
                for (_name, symbol_rcref) in &actions_symtab.borrow().symbols {
                    if let SymbolType::ActionScope { action_scope_symbol_rcref } = &*symbol_rcref.borrow() {
                        if action_scope_symbol_rcref.borrow().name == action_name {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    fn resolve_module_call(&self, module_name: &str) -> ResolvedCallType {
        // This is a simplified version - need to look at the actual chain
        ResolvedCallType::ModuleFunction {
            module: module_name.to_string(),
            function: "unknown".to_string(),
        }
    }
    
    fn is_action_in_system(&self, system_name: &str, action_name: &str) -> bool {
        // v0.63: Direct system lookup for accuracy
        if let Some(system_symbol) = self.arcanum.get_system_by_name(system_name) {
            let sys = system_symbol.borrow();
            if let Some(ref actions) = sys.actions_block_symbol_opt {
                // Actions are in the symbol table of the actions block
                let actions_symtab = &actions.borrow().symtab_rcref;
                for (_name, symbol_rcref) in &actions_symtab.borrow().symbols {
                    if let SymbolType::ActionScope { action_scope_symbol_rcref } = &*symbol_rcref.borrow() {
                        if action_scope_symbol_rcref.borrow().name == action_name {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    fn is_operation_in_system(&self, system_name: &str, operation_name: &str) -> bool {
        // v0.63: Direct system lookup for accuracy
        if let Some(system_symbol) = self.arcanum.get_system_by_name(system_name) {
            let sys = system_symbol.borrow();
            if let Some(ref operations) = sys.operations_block_symbol_opt {
                // Operations are in the symbol table of the operations block
                let ops_symtab = &operations.borrow().symtab_rcref;
                for (_name, symbol_rcref) in &ops_symtab.borrow().symbols {
                    if let SymbolType::OperationScope { operation_scope_symbol_rcref } = &*symbol_rcref.borrow() {
                        if operation_scope_symbol_rcref.borrow().name == operation_name {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    fn is_static_operation(&self, _system_name: &str, operation_name: &str) -> bool {
        // Check if an operation is marked with @staticmethod
        // This requires looking at the operation's attributes
        if let Some(op_symbol) = self.arcanum.lookup_operation(operation_name) {
            // Check the operation's AST node for staticmethod attribute
            // TODO: Implement attribute checking
            return false; // Placeholder
        }
        false
    }
    
    pub fn enter_system(&mut self, system_name: &'a str) {
        self.current_system = Some(system_name);
        self.current_class = None;
        self.in_standalone_function = false;
        
        // v0.63: Cache the system symbol for efficient lookups
        if let Some(sys_symbol) = self.arcanum.get_system_by_name(system_name) {
            self.current_system_symbol = Some(sys_symbol);
        }
    }
    
    pub fn enter_class(&mut self, class_name: &'a str) {
        self.current_class = Some(class_name);
        self.current_system = None;
        self.in_standalone_function = false;
    }
    
    pub fn enter_function(&mut self) {
        self.current_system = None;
        self.current_class = None;
        self.in_standalone_function = true;
    }
    
    pub fn set_static_context(&mut self, is_static: bool) {
        self.is_static_context = is_static;
    }
    
    pub fn exit_scope(&mut self) {
        self.current_system = None;
        self.current_class = None;
        self.in_standalone_function = false;
        self.is_static_context = false;
    }
}