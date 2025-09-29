#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]  // Many parser methods are part of the complete grammar even if not currently used

use super::ast::AssignmentExprNode;
use super::ast::AssignmentOperator;
use super::ast::CallChainNodeType;
use super::ast::CallChainLiteralExprNode;
use super::ast::CallExprListNode;
use super::ast::DeclOrStmtType;
use super::ast::ExprStmtType::*;
use super::ast::ExprType;
use super::ast::ExprType::*;
use super::ast::TerminatorType::Return;
use super::ast::*;
use super::ast::TargetStateContextType;
use std::collections::VecDeque;
use super::scanner::*;
use super::semantic_analyzer::SemanticCallAnalyzer;
use super::symbol_table::*;
use crate::frame_c::ast::ModuleElement;
use crate::frame_c::ast::CallContextType;
use crate::frame_c::utils::SystemHierarchy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;


pub struct ParseError {
    // TODO:
    pub error: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            error: String::from(msg),
        }
    }
}

struct StateEventHandlers {
    pub enter_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
    pub exit_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
    pub event_handlers: Vec<Rc<RefCell<EventHandlerNode>>>
}

impl StateEventHandlers {
    pub fn new(    enter_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
               exit_event_handler_opt: Option<Rc<RefCell<EventHandlerNode>>>,
               event_handlers: Vec<Rc<RefCell<EventHandlerNode>>>) -> StateEventHandlers {
        StateEventHandlers {
            enter_event_handler_opt,
            exit_event_handler_opt,
            event_handlers,
        }
    }
}

// TODO
impl fmt::Display for ParseError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ParseError")
    }
}

// // @todo
// struct StateSemanticValidator {
//
//     // exitEventHandlerOpt:Option<Rc<RefCell<EventHandlerNode>>>,
//     // transtitions:Vec<Rc<RefCell<TransitionStatementNode>>>,
// }
//
// impl StateSemanticValidator {
//
//     pub fn new() -> StateSemanticValidator {
//         StateSemanticValidator {}
//     }
//
//     pub fn has_valid_exit_semantics(&self, _:&StateNode) -> bool {
//         // if any transition has exit args then
//         //  - there must be an exit handler for the state
//         //  - all exit args must be of same number and type for all transitions
//         //  - all transition exit args must match the exit handler parameter list
//
//         // for evt_handler in state_node.evt_handlers {
//         //
//         //     for statement in evt_handler.statements {
//         //         match statement {
//         //             DeclOrStmtType::StmtT {stmt_t} => {
//         //                 match stmt_t {
//         //                     StatementType::TransitionStmt{transition_statement}
//         //                         => {
//         //                         transition_statement.
//         //                     }
//         //                 }
//         //             },
//         //             _ => {},
//         //         }
//         //     }
//         // }
//
//         true
//     }
// }

pub struct Parser<'a> {
    tokens: &'a [Token],
    comments: &'a mut Vec<Token>,
    current: usize,
    current_token: String,
    current_tok_ref: &'a Token,
    current_event_symbol_opt: Option<Rc<RefCell<EventSymbol>>>,
    processed_tokens: String,
    //    reset_pos:usize,
    is_building_symbol_table: bool,
    arcanum: Arcanum,
    state_name_opt: Option<String>,
    state_parent_opt: Option<String>,
    had_error: bool,
    panic_mode: bool,
    errors: String,
    last_sync_token_idx: usize,
    system_hierarchy_opt: Option<SystemHierarchy>,
    is_parsing_rhs: bool,
    is_parsing_collection: bool,  // v0.53: Track when inside collection literals
    event_handler_has_transition: bool,
    is_action_scope: bool,
    operation_scope_depth: i32,
    is_static_operation: bool,
    is_class_method: bool,  // v0.45: Track if we're in a class method
    is_function_scope: bool,
    is_system_scope: bool,
    is_loop_scope: bool,
    // v0.63: Context tracking for accurate semantic resolution
    current_system_name: Option<String>,
    current_class_name: Option<String>,
    current_function_name: Option<String>,
    stmt_idx: i32,
    interface_method_called: bool,
    pub generate_enter_args: bool,
    pub generate_exit_args: bool,
    pub generate_state_context: bool,
    pub generate_state_stack: bool,
    pub generate_change_state: bool,
    pub generate_transition_state: bool,
    pub sync_tokens_from_error_context: Vec<TokenType>,
    debug_mode: bool,
}

// Helper enum for call type classification
enum CallType {
    Interface,
    Action,
    Operation,
    Function,
    Unknown,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        tokens: &'a [Token],
        comments: &'a mut Vec<Token>,
        is_building_symbol_table: bool,
        mut arcanum: Arcanum,
    ) -> Parser<'a> {
        let debug_mode = std::env::var("FRAME_TRANSPILER_DEBUG").is_ok();
        // Initialize foundational scopes ONLY for first pass (symbol table building)
        // Second pass reuses the populated symbol tables from first pass
        if is_building_symbol_table {
            arcanum.initialize_scope_stack();
        } else {
            // Second pass: Set current scope to module scope to start semantic analysis
            arcanum.current_symtab = Rc::clone(&arcanum.module_symtab);
        }
        
        Parser {
            tokens,
            comments,
            current: 0,
            last_sync_token_idx: 0,
            current_token: String::from(""),
            current_event_symbol_opt: None,
            processed_tokens: String::from(""),
            is_building_symbol_table,
            arcanum,
            state_name_opt: None,
            state_parent_opt: None,
            had_error: false,
            panic_mode: false,
            errors: String::new(),
            current_tok_ref: &tokens[0],
            system_hierarchy_opt: None,
            is_parsing_rhs: false,
            is_parsing_collection: false,  // v0.53: Initialize
            event_handler_has_transition: false,
            generate_enter_args: false,
            generate_exit_args: false,
            generate_state_context: false,
            generate_state_stack: false,
            generate_change_state: false,
            generate_transition_state: false,
            is_action_scope: false,
            operation_scope_depth: 0,
            is_static_operation: false,
            is_class_method: false,
            is_function_scope: false,
            is_system_scope: false,
            is_loop_scope: false,
            current_system_name: None,  // v0.63
            current_class_name: None,   // v0.63
            current_function_name: None, // v0.63
            stmt_idx: 0,
            interface_method_called: false,
            sync_tokens_from_error_context: Vec::new(),
            debug_mode,
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn parse(&mut self) -> Result<FrameModule, ParseError> {
        self.module()
    }

    /* --------------------------------------------------------------------- */

    fn module(&mut self) -> Result<FrameModule, ParseError> {
        // Properly manage module scope for both passes
        self.module_scope()
    }
    
    fn module_scope(&mut self) -> Result<FrameModule, ParseError> {
        // Module scope management following the function_scope() pattern
        
        if self.is_building_symbol_table {
            // First pass: lexical analysis
            // Module scope is already created by initialize_scope_stack()
            // but we need to set the context to Global for module-level parsing
            self.arcanum.set_scope_context(ScopeContext::Global);
        } else {
            // Second pass: semantic analysis
            // The module scope should already be set from the first pass
            // Just ensure we're in the right context
            self.arcanum.set_scope_context(ScopeContext::Global);
        }
        
        // Parse the module content
        let module = self.parse_module_content()?;
        
        // Note: We don't exit scope here because the module scope
        // should remain active for the entire file parsing
        
        Ok(module)
    }
    
    
    fn parse_module_content(&mut self) -> Result<FrameModule, ParseError> {
        if self.match_token(&[TokenType::Eof]) {
            return Err(ParseError::new("Empty module - Frame files must contain at least one function or system."));
        }

        let mut module_elements_opt = self.header()?;

        // v0.57: Collect imports from module elements for multi-file support
        let mut imports = Vec::new();
        if let Some(ref elements) = module_elements_opt {
            for element in elements {
                if let crate::frame_c::ast::ModuleElement::Import { import_node } = element {
                    imports.push(import_node.clone());
                }
            }
        }

        // v0.31: Parse module-level variables, functions, systems, and statements
        let mut functions = Vec::new();
        let mut systems = Vec::new();
        let mut classes = Vec::new();   // v0.45: Classes
        let mut variables = Vec::new();
        let mut enums = Vec::new();
        let mut modules = Vec::new();  // v0.34: Nested modules
        // Note: statements are not allowed at module level (like C)
        
        // Parse all entities in sequence
        loop {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Top of parsing loop, current token: {:?}", self.peek());
            }
            // Check for module-level enum declarations
            if self.match_token(&[TokenType::Enum]) {
                match self.enum_decl() {
                    Ok(enum_decl) => {
                        enums.push(enum_decl.clone());
                        // Also add to module elements for backward compatibility
                        if let Some(ref mut elements) = module_elements_opt {
                            elements.push(crate::frame_c::ast::ModuleElement::Enum { enum_decl_node: enum_decl });
                        }
                    }
                    Err(e) => return Err(e),
                }
                continue;
            }
            
            // Check for class decorators (only if @ is followed eventually by 'class')
            // We need to look ahead to see if this is a decorator for a class
            if self.check(TokenType::At) {
                // Save position in case we need to backtrack
                let saved_pos = self.current;
                let mut temp_decorators = Vec::new();
                
                // Try to parse decorators
                while self.check(TokenType::At) {
                    match self.parse_class_decorator() {
                        Ok(decorator_str) => temp_decorators.push(decorator_str),
                        Err(_) => {
                            // Not a valid decorator, backtrack
                            self.current = saved_pos;
                            break;
                        }
                    }
                }
                
                // Check if followed by 'class'
                if self.check(TokenType::Class) {
                    // This is a decorated class
                    self.advance();  // consume 'class'
                    match self.class_decl_with_decorators(temp_decorators) {
                        Ok(class_decl) => {
                            classes.push(class_decl);
                        }
                        Err(e) => return Err(e),
                    }
                    continue;
                } else if !temp_decorators.is_empty() {
                    // We parsed decorators but no class follows
                    return Err(ParseError::new("Expected 'class' after decorators"));
                } else {
                    // No decorators parsed, backtrack completely
                    self.current = saved_pos;
                }
            }
            
            // Check for undecorated class declarations
            if self.match_token(&[TokenType::Class]) {
                match self.class_decl() {
                    Ok(class_decl) => {
                        classes.push(class_decl);
                    }
                    Err(e) => return Err(e),
                }
                continue;
            }
            
            // Check for module-level variable declarations
            if self.match_token(&[TokenType::Var, TokenType::Const]) {
                match self.var_declaration(IdentifierDeclScope::ModuleScope) {
                    Ok(var_decl) => {
                        variables.push(var_decl.clone());
                        // Also add to module elements for backward compatibility
                        if let Some(ref mut elements) = module_elements_opt {
                            elements.push(crate::frame_c::ast::ModuleElement::Variable { var_decl_node: var_decl });
                        }
                    }
                    Err(e) => return Err(e),
                }
                continue;
            }
            
            // Check for attributes that might precede a system
            let entity_attributes_opt = if self.check(TokenType::OuterAttributeOrDomainParams) {
                self.entity_attributes()?
            } else {
                None
            };
            
            // Check what entity type we have
            if self.match_token(&[TokenType::System]) {
                // v0.30: All individual systems get empty modules - module elements belong to FrameModule
                let module_for_system = crate::frame_c::ast::Module::new(vec![]);
                
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Parsing system at token index {}", self.current);
                }
                let system = self.system_scope(Some(module_for_system), entity_attributes_opt, None)?;
                systems.push(system);
            } else if self.match_token(&[TokenType::Async]) {
                // Check if next token is 'fn' for async function
                if self.match_token(&[TokenType::Function]) {
                    if entity_attributes_opt.is_some() {
                        return Err(ParseError::new("Functions do not support attributes. Remove attributes or change to system."));
                    }
                    let function = self.function_scope_async(true)?;
                    functions.push(function);
                } else {
                    return Err(ParseError::new("Expected 'fn' after 'async' keyword"));
                }
            } else if self.match_token(&[TokenType::Function]) {
                // Functions shouldn't have system attributes, but warn if present
                if entity_attributes_opt.is_some() {
                    return Err(ParseError::new("Functions do not support attributes. Remove attributes or change to system."));
                }
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Parsing function at token index {}", self.current);
                }
                let function = self.function_scope_async(false)?;
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Successfully parsed function");
                }
                functions.push(function);
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: After parsing function, next token: {:?}", self.peek());
                }
            } else if self.match_token(&[TokenType::Module]) {
                // v0.34: Parse nested module declaration
                if entity_attributes_opt.is_some() {
                    return Err(ParseError::new("Modules do not support attributes."));
                }
                let module = self.module_declaration()?;
                modules.push(module);
            } else if self.match_token(&[TokenType::Class]) {
                // v0.45: Parse class declaration
                if entity_attributes_opt.is_some() {
                    return Err(ParseError::new("Classes do not support attributes in the current implementation."));
                }
                let class = self.class_decl()?;
                classes.push(class);
            } else if entity_attributes_opt.is_some() {
                // We parsed attributes but found no system or function - this is an error
                return Err(ParseError::new("Expected 'system' after attributes. Functions do not support attributes."));
            } else if self.check(TokenType::Identifier) {
                // Frame does not allow module-level statements (like C)
                // Check if this looks like a function call or system instantiation
                let saved_pos = self.current;
                let identifier = self.advance();
                let identifier_name = identifier.lexeme.clone();
                if self.check(TokenType::LParen) {
                    // Check if this is a system instantiation or function call at module scope
                    // Both are not allowed
                    self.current = saved_pos;  // Reset for better error reporting
                    
                    // Check if identifier matches any system name or class name
                    let is_system = systems.iter().any(|s| s.name == identifier_name);
                    let is_class = classes.iter().any(|c| c.borrow().name == identifier_name);
                    
                    // Also check if it starts with uppercase (likely a class/system)
                    let starts_with_uppercase = identifier_name.chars().next()
                        .map_or(false, |c| c.is_uppercase());
                    
                    if is_system || is_class || starts_with_uppercase {
                        return Err(ParseError::new(&format!(
                            "Module-level instantiation is not allowed. '{}' cannot be instantiated at module scope. \
                            Classes and systems must be instantiated inside functions.",
                            identifier_name
                        )));
                    } else {
                        return Err(ParseError::new(&format!(
                            "Module-level function calls are not allowed. Function '{}' cannot be called at module scope. \
                            Frame automatically calls main() if it exists.",
                            identifier_name
                        )));
                    }
                } else {
                    // Reset and try to parse as something else
                    self.current = saved_pos;
                    // If we can't parse anything, break out of the loop
                    break;
                }
            } else {
                // Skip any token that's not a valid top-level entity
                // This allows us to continue parsing after close braces from systems/functions
                if !self.is_at_end() {
                    // If we see something we don't recognize at the top level,
                    // it might be the start of the next entity or an error
                    // For now, just advance past it if it's a close brace or similar
                    let current_token = self.peek();
                    if current_token.token_type == TokenType::CloseBrace {
                        // Skip close braces from previous entities
                        self.advance();
                        continue;
                    }
                }
                // No more entities found - exit loop
                break;
            }
        }
        
        // Check for any remaining module-level statements (which are not allowed)
        // This catches cases where someone tries to call functions or instantiate systems at module level
        while !self.is_at_end() {
            // Skip any remaining comments
            if self.check(TokenType::PythonComment) || self.check(TokenType::MultiLineComment) {
                self.advance();
                continue;
            }
            
            // Debug: print current token
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Checking for module-level statements, current token: {:?}", self.peek());
            }
            
            // Check for identifier followed by parenthesis (function call or instantiation)
            if self.check(TokenType::Identifier) {
                let saved_pos = self.current;
                let identifier = self.advance();
                let identifier_name = identifier.lexeme.clone();
                
                if self.check(TokenType::LParen) {
                    // This is a module-level call/instantiation - not allowed!
                    self.current = saved_pos;  // Reset for better error reporting
                    
                    // Check if identifier matches any system name or class name
                    let is_system = systems.iter().any(|s| s.name == identifier_name);
                    let is_class = classes.iter().any(|c| c.borrow().name == identifier_name);
                    
                    // Also check if it starts with uppercase (likely a class/system)
                    let starts_with_uppercase = identifier_name.chars().next()
                        .map_or(false, |c| c.is_uppercase());
                    
                    if is_system || is_class || starts_with_uppercase {
                        return Err(ParseError::new(&format!(
                            "Module-level instantiation is not allowed. '{}' cannot be instantiated at module scope. \
                            Classes and systems must be instantiated inside functions.",
                            identifier_name
                        )));
                    } else {
                        return Err(ParseError::new(&format!(
                            "Module-level function calls are not allowed. Function '{}' cannot be called at module scope. \
                            Frame automatically calls main() if it exists.",
                            identifier_name
                        )));
                    }
                }
                
                // If it's not a call, it might be an invalid statement
                // For now, skip it and let the next iteration handle it
                self.current = saved_pos;
                break;  // Exit to avoid infinite loop
            }
            
            // If we encounter any other token, break to avoid infinite loop
            break;
        }
        
        // v0.37: Analyze runtime async requirements for all systems
        if !self.is_building_symbol_table {
            // Only do runtime analysis in the semantic pass
            for system in &mut systems {
                self.analyze_system_runtime_info(system)?;
            }
        }
        
        // v0.31: Create FrameModule with all parsed elements
        let final_module = match module_elements_opt {
            Some(module_elements) => crate::frame_c::ast::Module::new(module_elements),
            None => crate::frame_c::ast::Module::new(vec![]),
        };
        
        // Module-level statements not allowed - pass empty vector
        Ok(FrameModule::new(final_module, imports, functions, systems, classes, variables, enums, modules, Vec::new()))
    }

    /* --------------------------------------------------------------------- */

    fn header(&mut self) -> Result<Option<Vec<ModuleElement>>, ParseError> {
        let mut module_elements = Vec::new();

        loop {
            // #![module_attribute]
            if self.match_token(&[TokenType::InnerAttribute]) {
                match self.attribute(AttributeAffinity::Inner) {
                    Ok(attribute_node) => {
                        if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.")
                        {
                            return Err(parse_error);
                        }
                        module_elements.push(crate::frame_c::ast::ModuleElement::ModuleAttribute { attribute_node });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else if false {
                // SuperString/backtick support removed
                let mut code_block = String::new();
                let tok = self.previous();
                code_block.push_str(&tok.lexeme.clone());
                module_elements.push(crate::frame_c::ast::ModuleElement::CodeBlock { code_block });
            } else if self.match_token(&[TokenType::Import]) {
                // Parse import statement: import module [as alias]
                match self.parse_import_statement() {
                    Ok(import_node) => {
                        module_elements.push(crate::frame_c::ast::ModuleElement::Import { import_node });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else if self.match_token(&[TokenType::From]) {
                // Parse from import: from module import ...
                match self.parse_from_import_statement() {
                    Ok(import_node) => {
                        module_elements.push(crate::frame_c::ast::ModuleElement::Import { import_node });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else if self.check(TokenType::Identifier) && self.peek().lexeme == "type" {
                // Parse type alias: type Name = type_expression
                self.advance(); // consume 'type'
                match self.parse_type_alias() {
                    Ok(type_alias_node) => {
                        module_elements.push(crate::frame_c::ast::ModuleElement::TypeAlias { type_alias_node });
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            // else if self.match_token(&[TokenType::ThreeTicks]) {
            //     // Parse code_block ```whatever```
            //     let mut code_block = String::new();
            //     while self.match_token(&[TokenType::SuperString]) {
            //         let tok = self.previous();
            //         code_block.push_str(&tok.lexeme.clone());
            //     }
            //     if self
            //         .consume(TokenType::ThreeTicks, "Expected '```'.")
            //         .is_err()
            //     {
            //         // TODO
            //         self.error_at_current("Expected closing ```.");
            //         let sync_tokens = vec![TokenType::System];
            //         self.synchronize(&sync_tokens);
            //     }
            //     module_elements.push(CodeBlock {code_block});
            // }
            else {
                break;
            }
        }

        Ok(Some(module_elements))
    }

    /* --------------------------------------------------------------------- */

    fn system(
        &mut self,
        system_name: String,
        module_opt: Option<Module>,
        system_attributes_opt: Option<HashMap<String, AttributeNode>>,
        _functions_opt: Option<Vec<Rc<RefCell<FunctionNode>>>>,
    ) -> Result<SystemNode, ParseError> {
        let interface_block_node_opt;
        let machine_block_node_opt;
        let actions_block_node_opt;
        let operations_block_node_opt;
        let domain_block_node_opt;

        // SystemHierarchy is used in the GraphViz visitor rather than the AST
        self.system_hierarchy_opt = Some(SystemHierarchy::new(system_name.clone()));
        
        // v0.63: Set current system context for semantic resolution
        self.current_system_name = Some(system_name.clone());
        self.is_system_scope = true;

        // Parse system parameters and set up scope
        let (system_start_state_state_params_opt, system_enter_params_opt, domain_params_opt) = 
            self.parse_system_params_and_setup_scope(&system_name)?;

        if self.consume(TokenType::OpenBrace, "Expected '{'").is_err() {
            self.error_at_current("Expected '{'.");
        }

        // Parse system blocks in the correct order
        operations_block_node_opt = self.parse_operations_block()?;
        interface_block_node_opt = self.parse_interface_block()?;
        machine_block_node_opt = self.parse_machine_block()?;

        // Validate start state parameters
        if !self.is_building_symbol_table {
            self.validate_start_state_params(
                &machine_block_node_opt,
                &system_start_state_state_params_opt,
                &system_enter_params_opt,
            );
        }

        // Parse actions and domain blocks
        actions_block_node_opt = self.parse_actions_block()?;
        domain_block_node_opt = self.parse_domain_block()?;

        if !self.match_token(&[TokenType::CloseBrace]) {
            if self.peek().lexeme == "$" {
                let err_msg = &format!("Found {} token. Possible missing machine block.", self.peek().lexeme);
                self.error_at_current(err_msg);
            } else {
                let err_msg = &format!("Expected '}}' - found '{}'.", self.peek().lexeme);
                self.error_at_current(err_msg);
            }

        }

        let line = self.previous().line;

        self.arcanum.exit_scope();
        
        // v0.63: Clear system context after parsing
        self.current_system_name = None;
        self.is_system_scope = false;

        let module = match module_opt {
            Some(m) => m,
            None => crate::frame_c::ast::Module {
                module_elements: vec![],
            },
        };
        let system_node = SystemNode::new(
            system_name,
            module,
            system_attributes_opt,
            system_start_state_state_params_opt,
            system_enter_params_opt,
            domain_params_opt,
            interface_block_node_opt,
            machine_block_node_opt,
            actions_block_node_opt,
            operations_block_node_opt,
            domain_block_node_opt,
            line,
        );

        // TODO - change reference to SystemNode to use an rc refcell data structure.
        // if self.is_building_symbol_table {
        //     if let Some(system_symbol_rcref) = self.arcanum.system_symbol_opt {
        //         let system_node_rcref = Rc::new(RefCell::new(system_node));
        //         system_symbol_rcref.borrow_mut().set_ast_node(system_node_rcref);
        //     }
        // }

        Ok(system_node)
    }

    /* --------------------------------------------------------------------- */
    // Helper function to parse system parameters and set up scope
    
    fn parse_system_params_and_setup_scope(
        &mut self,
        system_name: &str,
    ) -> Result<(Option<Vec<ParameterNode>>, Option<Vec<ParameterNode>>, Option<Vec<ParameterNode>>), ParseError> {
        if self.is_building_symbol_table {
            let mut system_symbol = SystemSymbol::new(system_name.to_string());
            
            let (system_start_state_state_params_opt, system_enter_params_opt, domain_params_opt) = 
                self.system_params();
            
            // Cache off param count for instance arg verification
            if let Some(system_start_state_state_params) = &system_start_state_state_params_opt {
                system_symbol.start_state_params_cnt = system_start_state_state_params.len();
            }
            if let Some(system_enter_params) = &system_enter_params_opt {
                system_symbol.state_enter_params_cnt = system_enter_params.len();
            }
            if let Some(domain_params) = &domain_params_opt {
                system_symbol.domain_params_cnt = domain_params.len();
            }
            
            let system_symbol_rcref = Rc::new(RefCell::new(system_symbol));
            self.arcanum.enter_scope(ParseScopeType::System {
                system_symbol: system_symbol_rcref,
            });
            
            Ok((system_start_state_state_params_opt, system_enter_params_opt, domain_params_opt))
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(system_name) {
                return Err(ParseError::new(&format!("Failed to set system scope '{}': {}", system_name, err)));
            }
            Ok(self.system_params())
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse operations block with ordering check
    
    fn parse_operations_block(&mut self) -> Result<Option<OperationsBlockNode>, ParseError> {
        if self.match_token(&[TokenType::OperationsBlock]) {
            Ok(Some(self.operations_block()))
        } else {
            Ok(None)
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse interface block with ordering check
    
    fn parse_interface_block(&mut self) -> Result<Option<InterfaceBlockNode>, ParseError> {
        // Check for operations block appearing after interface (wrong order)
        if self.peek().token_type == TokenType::OperationsBlock {
            let err_msg = "Block ordering error: 'operations:' block must come before 'interface:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if self.match_token(&[TokenType::InterfaceBlock]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let interface_block = self.interface_block();
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            Ok(Some(interface_block))
        } else {
            Ok(None)
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse machine block with ordering check
    
    fn parse_machine_block(&mut self) -> Result<Option<MachineBlockNode>, ParseError> {
        // Check for blocks appearing after machine in wrong order
        if self.peek().token_type == TokenType::InterfaceBlock {
            let err_msg = "Block ordering error: 'interface:' block must come before 'machine:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::OperationsBlock {
            let err_msg = "Block ordering error: 'operations:' block must come before 'machine:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if self.match_token(&[TokenType::MachineBlock]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let machine_block = self.machine_block();
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            Ok(Some(machine_block))
        } else {
            Ok(None)
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse actions block with ordering check
    
    fn parse_actions_block(&mut self) -> Result<Option<ActionsBlockNode>, ParseError> {
        // Check for blocks appearing after actions (wrong order)
        if self.peek().token_type == TokenType::MachineBlock {
            let err_msg = "Block ordering error: 'machine:' block must come before 'actions:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::InterfaceBlock {
            let err_msg = "Block ordering error: 'interface:' block must come before 'actions:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::OperationsBlock {
            let err_msg = "Block ordering error: 'operations:' block must come before 'actions:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if self.match_token(&[TokenType::ActionsBlock]) {
            Ok(Some(self.actions_block()))
        } else {
            Ok(None)
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse domain block with ordering check
    
    fn parse_domain_block(&mut self) -> Result<Option<DomainBlockNode>, ParseError> {
        // Check for blocks appearing after domain (wrong order - domain must be last)
        if self.peek().token_type == TokenType::MachineBlock {
            let err_msg = "Block ordering error: 'machine:' block must come before 'domain:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::InterfaceBlock {
            let err_msg = "Block ordering error: 'interface:' block must come before 'domain:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::OperationsBlock {
            let err_msg = "Block ordering error: 'operations:' block must come before 'domain:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        if self.peek().token_type == TokenType::ActionsBlock {
            let err_msg = "Block ordering error: 'actions:' block must come before 'domain:' block";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if self.match_token(&[TokenType::DomainBlock]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let domain_block = self.domain_block();
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            Ok(Some(domain_block))
        } else {
            Ok(None)
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to validate start state parameters
    
    fn validate_start_state_params(
        &mut self,
        machine_block_node_opt: &Option<MachineBlockNode>,
        system_start_state_state_params_opt: &Option<Vec<ParameterNode>>,
        system_enter_params_opt: &Option<Vec<ParameterNode>>,
    ) {
        if let Some(machine_block_node) = machine_block_node_opt.as_ref() {
            if machine_block_node.states.is_empty() {
                self.validate_empty_states(system_start_state_state_params_opt, system_enter_params_opt);
            } else {
                self.validate_start_state_with_states(
                    &machine_block_node.states[0],
                    system_start_state_state_params_opt,
                    system_enter_params_opt,
                );
            }
        } else if system_start_state_state_params_opt.is_some() {
            self.error_at_current("System start state parameters declared but no start state exists.");
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper to validate when there are no states
    
    fn validate_empty_states(
        &mut self,
        system_start_state_state_params_opt: &Option<Vec<ParameterNode>>,
        system_enter_params_opt: &Option<Vec<ParameterNode>>,
    ) {
        if system_start_state_state_params_opt.is_some() {
            self.error_at_current("System start state parameters declared but no start state exists.");
        }
        if system_enter_params_opt.is_some() {
            self.error_at_current("System start state enter parameters declared but no start state exists.");
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper to validate start state when states exist
    
    fn validate_start_state_with_states(
        &mut self,
        start_state_rcref: &Rc<RefCell<StateNode>>,
        system_start_state_state_params_opt: &Option<Vec<ParameterNode>>,
        system_enter_params_opt: &Option<Vec<ParameterNode>>,
    ) {
        let start_state = start_state_rcref.borrow();
        
        // Validate state parameters
        self.validate_state_parameters(&start_state, system_start_state_state_params_opt);
        
        // Validate enter event parameters
        self.validate_enter_event_parameters(&start_state, system_enter_params_opt);
    }
    
    /* --------------------------------------------------------------------- */
    // Helper to validate state parameters match system parameters
    
    fn validate_state_parameters(
        &mut self,
        start_state: &StateNode,
        system_start_state_state_params_opt: &Option<Vec<ParameterNode>>,
    ) {
        match (&start_state.params_opt, system_start_state_state_params_opt) {
            (None, None) => {}, // Both None - ok
            (Some(_), None) => {
                self.error_at_current("Start state parameters declared but no system start state parameters are declared.");
            }
            (None, Some(_)) => {
                self.error_at_current("System start state parameters declared but no start state exists.");
            }
            (Some(start_state_params), Some(system_params)) => {
                if start_state_params.len() != system_params.len() {
                    self.error_at_current("System start state parameters do not match actual start state parameters.");
                } else {
                    for (i, state_param) in start_state_params.iter().enumerate() {
                        let system_param = &system_params[i];
                        if system_param != state_param {
                            self.error_at_current("System start state parameters do not match actual start state parameters.");
                        }
                    }
                }
            }
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper to validate enter event parameters
    
    fn validate_enter_event_parameters(
        &mut self,
        start_state: &StateNode,
        system_enter_params_opt: &Option<Vec<ParameterNode>>,
    ) {
        if let Some(enter_event_handler) = &start_state.enter_event_handler_opt {
            let event_handler = enter_event_handler.borrow();
            let event_symbol = event_handler.event_symbol_rcref.borrow();
            let enter_event_handler_params_opt = &event_symbol.event_symbol_params_opt;
            
            match (enter_event_handler_params_opt, system_enter_params_opt) {
                (None, None) => {}, // Both None - ok
                (None, Some(_)) => {
                    self.error_at_current("System has enter parameters but start state enter handler does not.");
                }
                (Some(_), None) => {
                    self.error_at_current("Start state has enter parameters but system does not define any.");
                }
                (Some(enter_params), Some(system_params)) => {
                    self.validate_matching_enter_params(enter_params, system_params);
                }
            }
        } else if system_enter_params_opt.is_some() {
            self.error_at_current("System has enter parameters but the start state does not have an enter event handler.");
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper to validate that enter parameters match
    
    fn validate_matching_enter_params(
        &mut self,
        enter_event_handler_params: &Vec<ParameterSymbol>,
        system_enter_params: &Vec<ParameterNode>,
    ) {
        if system_enter_params.len() != enter_event_handler_params.len() {
            self.error_at_current("Start state and system enter parameters are different.");
            return;
        }
        
        for (i, param) in system_enter_params.iter().enumerate() {
            let parameter_symbol = &enter_event_handler_params[i];
            
            if parameter_symbol.name != param.param_name {
                self.error_at_current("Start state and system enter parameters are different.");
            } else if let (Some(param_symbol_type), Some(param_type)) = 
                (&parameter_symbol.param_type_opt, &param.param_type_opt) {
                if param_symbol_type != param_type {
                    self.error_at_current("System enter params do not match start state enter params.");
                }
            } else if parameter_symbol.param_type_opt.is_some() != param.param_type_opt.is_some() {
                self.error_at_current("Start state and system enter parameters are different.");
            }
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Parse optional system args.
    // [ $(start_state_param), >(start_state_enter_param), #(domain_param) ]

    fn system_arguments(
        &mut self,
    ) -> Result<
        (
            Option<ExprListNode>,
            Option<ExprListNode>,
            Option<ExprListNode>,
        ),
        ParseError,
    > {
        let mut start_state_args_opt = Option::None;
        let mut start_enter_args_opt = Option::None;
        let mut domain_args_opt = Option::None;

        match self.system_start_state_args() {
            Ok(Some(expr_list_node)) => {
                start_state_args_opt = Some(expr_list_node);
                if self.match_token(&[TokenType::Comma]) {
                    match self.system_enter_or_domain_args() {
                        Ok((start_enter_args, domain_args)) => {
                            start_enter_args_opt = start_enter_args;
                            domain_args_opt = domain_args;
                            // as there was a comma, one of these must exist
                            if start_enter_args_opt.is_none() && domain_args_opt.is_none() {
                                let err_msg = "Expected start state enter or domain args.";
                                self.error_at_current(err_msg);
                                return Err(ParseError::new(err_msg));
                            }
                        }
                        Err(parse_err) => {
                            return Err(parse_err);
                        }
                    }
                }
            }
            Ok(None) => match self.system_enter_or_domain_args() {
                Ok((start_enter_args, domain_args)) => {
                    start_enter_args_opt = start_enter_args;
                    domain_args_opt = domain_args;
                }
                Err(parse_err) => {
                    return Err(parse_err);
                }
            },
            Err(parse_err) => {
                return Err(parse_err);
            }
        }
        // if !start_enter_args.is_empty() {
        //     if self.match_token(&[TokenType::Comma]) {
        //         (start_enter_args,domain_args) = self.system_enter_or_domain_args();
        //         if start_enter_args.is_empty() && domain_args.is_empty() {
        //             self.error_at_current("Expected ], found ','")
        //         }
        //     }
        //
        // } else {
        //     (start_enter_args,domain_args) = self.system_enter_or_domain_args();
        // }

        if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
            return Err(parse_error);
        }

        let mut start_state_args_cnt = 0;
        let mut start_state_params_cnt = 0;
        let mut start_enter_args_cnt = 0;
        let mut start_enter_params_cnt = 0;
        let mut domain_args_cnt = 0;
        let mut domain_params_cnt = 0;

        if let Some(ref start_state_args) = start_state_args_opt {
            start_state_args_cnt = start_state_args.exprs_t.len();
        }
        if let Some(ref start_enter_args) = start_enter_args_opt {
            start_enter_args_cnt = start_enter_args.exprs_t.len();
        }
        if let Some(ref domain_args) = domain_args_opt {
            domain_args_cnt = domain_args.exprs_t.len();
        }
        if !self.is_building_symbol_table {
            // This section is organized slightly strangely because we have to release
            // the mutable reference to "self" before calling self.error_at_current().
            // So we get it in a scope to get the system_symbol and release it first.
            if let Some(ref _system_symbol_rcref) = &self.arcanum.system_symbol_opt {
                let system_symbol = &self.arcanum.system_symbol_opt.as_ref().unwrap().borrow();
                start_state_params_cnt = system_symbol.start_state_params_cnt;
                start_enter_params_cnt = system_symbol.state_enter_params_cnt;
                domain_params_cnt = system_symbol.domain_params_cnt;
            }
            if start_state_args_cnt != start_state_params_cnt {
                let err_msg =
                    "System start state arg count not equal to system start state param count.";
                self.error_at_previous(err_msg);
            }
            if start_enter_args_cnt != start_enter_params_cnt {
                let err_msg =
                    "System start enter arg count not equal to system start enter param count.";
                self.error_at_previous(err_msg);
            }
            if domain_args_cnt != domain_params_cnt {
                let err_msg = "System initialize domain arg count not equal to system intialize domain param count.";
                self.error_at_previous(err_msg);
            }
        }

        Ok((start_state_args_opt, start_enter_args_opt, domain_args_opt))
    }

    /* --------------------------------------------------------------------- */
    // Parse optional system start state args.
    // >(start_state_enter_args_list)

    fn system_start_state_args(&mut self) -> Result<Option<ExprListNode>, ParseError> {
        if self.match_token(&[TokenType::State]) {
            if let Err(parse_err) = self.consume(TokenType::LParen, "Expected '('") {
                return Err(parse_err);
            }

            match self.expr_list_node() {
                Ok(Some(expr_list_node)) => {
                    return Ok(Some(expr_list_node));
                }
                Ok(None) => {
                    let err_msg = "If present, start state arguments cannot have an empty list.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg));
                }
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    fn system_enter_or_domain_args(
        &mut self,
    ) -> Result<(Option<ExprListNode>, Option<ExprListNode>), ParseError> {
        let mut system_enter_args_opt = Option::None;
        let mut domain_args_opt = Option::None;

        match self.system_enter_args() {
            Ok(Some(expr_list_node_opt)) => {
                system_enter_args_opt = Some(expr_list_node_opt);
                if self.match_token(&[TokenType::Comma]) {
                    match self.system_domain_args() {
                        Ok(expr_list_node_opt) => {
                            domain_args_opt = expr_list_node_opt;
                        }
                        Err(parse_err) => {
                            return Err(parse_err);
                        }
                    }
                }
            }
            Ok(None) => match self.system_domain_args() {
                Ok(expr_list_node_opt) => {
                    domain_args_opt = expr_list_node_opt;
                }
                Err(parse_err) => {
                    return Err(parse_err);
                }
            },
            Err(parse_err) => {
                return Err(parse_err);
            }
        }

        Ok((system_enter_args_opt, domain_args_opt))
    }

    /* --------------------------------------------------------------------- */

    fn system_enter_args(&mut self) -> Result<Option<ExprListNode>, ParseError> {
        // v0.20 syntax: $>("args") instead of >("args")
        if self.match_token(&[TokenType::EnterStateMsg]) {
            if self.match_token(&[TokenType::LParen]) {
                match self.expr_list_node() {
                    Ok(Some(expr_list_node)) => {
                        return Ok(Some(expr_list_node));
                    }
                    Ok(None) => {
                        return Ok(None);
                    }
                    Err(parse_err) => {
                        return Err(parse_err);
                    }
                }
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    fn system_domain_args(&mut self) -> Result<Option<ExprListNode>, ParseError> {
        // v0.20 syntax: domain args are just plain arguments (no # prefix)
        // They're parsed as part of a flat argument list: Foo($(a), $>(b), c, d)
        // But at instantiation: Foo($("val1"), $>("val2"), "val3", "val4")
        
        // Check if we have plain arguments (not $ or $> prefixed)
        if self.peek().token_type != TokenType::RParen 
            && self.peek().token_type != TokenType::State
            && self.peek().token_type != TokenType::EnterStateMsg {
            match self.expr_list_node() {
                Ok(Some(expr_list_node)) => {
                    return Ok(Some(expr_list_node));
                }
                Ok(None) => {
                    return Ok(None);
                }
                Err(parse_err) => {
                    return Err(parse_err);
                }
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */
    // Parse optional system params.
    // [ $[start_state_param:T], >[start_state_enter_param:U], #[domain_params:V] ]

    fn system_params(
        &mut self,
    ) -> (
        Option<Vec<ParameterNode>>,
        Option<Vec<ParameterNode>>,
        Option<Vec<ParameterNode>>,
    ) {
        let mut system_start_state_state_params_opt: Option<Vec<ParameterNode>> = Option::None;
        let mut system_enter_params_opt: Option<Vec<ParameterNode>> = Option::None;
        let mut domain_params_opt: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::LParen]) {
            system_start_state_state_params_opt = self.system_start_state_params();
            if system_start_state_state_params_opt.is_some() {
                if self.match_token(&[TokenType::Comma]) {
                    (system_enter_params_opt, domain_params_opt) =
                        self.system_enter_or_domain_params();
                    if system_enter_params_opt.is_none() && domain_params_opt.is_none() {
                        self.error_at_current("Expected ), found ','")
                    }
                }
            } else {
                (system_enter_params_opt, domain_params_opt) = self.system_enter_or_domain_params();
            }

            if let Err(_parse_error) = self.consume(TokenType::RParen, "Expected ']'.") {
                let sync_tokens = vec![
                    TokenType::Identifier,
                    TokenType::MachineBlock,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
            }
            // else {
            //     if system_start_state_state_params_opt.is_none()
            //         && system_enter_params_opt.is_none()
            //         && domain_params_opt.is_none()
            //     {
            //        // self.error_at_current("Empty system parameter list.")
            //     }
            // }
        }

        (
            system_start_state_state_params_opt,
            system_enter_params_opt,
            domain_params_opt,
        )
    }

    // /* --------------------------------------------------------------------- */
    //
    // fn system_enter_or_domain_params(
    //     &mut self,
    // ) -> (Option<Vec<ParameterNode>>, Option<Vec<ParameterNode>>) {
    //     let mut system_enter_params_opt: Option<Vec<ParameterNode>> = Option::None;
    //     let mut domain_params_opt: Option<Vec<ParameterNode>> = Option::None;
    //
    //     system_enter_params_opt = self.system_enter_params();
    //     if system_enter_params_opt.is_some() {
    //         if self.match_token(&[TokenType::Comma]) {
    //             domain_params_opt = self.system_domain_params();
    //             if domain_params_opt.is_none() {
    //                 self.error_at_current("Expected ], found ','")
    //             }
    //         }
    //     } else {
    //         domain_params_opt = self.system_domain_params();
    //     }
    //
    //     (system_enter_params_opt, domain_params_opt)
    // }

    /* --------------------------------------------------------------------- */

    fn system_start_state_params(&mut self) -> Option<Vec<ParameterNode>> {
        let mut system_start_state_state_params_opt: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::State]) {
            if self.consume(TokenType::LParen, "Expected '('").is_err() {
                let sync_tokens = vec![
                    TokenType::GT,
                    TokenType::InterfaceBlock,
                    TokenType::ActionsBlock,
                    TokenType::MachineBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
            }
            match self.parameters() {
                Ok(Some(parameters)) => system_start_state_state_params_opt = Some(parameters),
                Ok(None) => {}
                Err(_) => {}
            }
            if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                let sync_tokens = vec![
                    TokenType::OpenBrace,
                ];
                self.synchronize(&sync_tokens);
            }
        }

        system_start_state_state_params_opt
    }

    /* --------------------------------------------------------------------- */

    fn system_enter_or_domain_params(
        &mut self,
    ) -> (Option<Vec<ParameterNode>>, Option<Vec<ParameterNode>>) {
        let system_enter_params_opt; //: Option<Vec<ParameterNode>> = Option::None;
        let mut domain_params_opt: Option<Vec<ParameterNode>> = Option::None;

        system_enter_params_opt = self.system_enter_params();
        if system_enter_params_opt.is_some() {
            if self.match_token(&[TokenType::Comma]) {
                domain_params_opt = self.system_domain_params();
                if domain_params_opt.is_none() {
                    self.error_at_current("Expected ), found ','")
                }
            }
        } else {
            domain_params_opt = self.system_domain_params();
        }

        (system_enter_params_opt, domain_params_opt)
    }

    /* --------------------------------------------------------------------- */

    fn system_enter_params(&mut self) -> Option<Vec<ParameterNode>> {
        let mut system_enter_params_opt: Option<Vec<ParameterNode>> = Option::None;

        // v0.20 syntax: $>(params) instead of >[params]
        if self.match_token(&[TokenType::EnterStateMsg]) {
            if self.consume(TokenType::LParen, "Expected '('").is_err() {
                let sync_tokens = vec![
                    TokenType::InterfaceBlock,
                    TokenType::ActionsBlock,
                    TokenType::MachineBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
            }
            match self.parameters() {
                Ok(Some(parameters)) => system_enter_params_opt = Some(parameters),
                Ok(None) => {}
                Err(_) => {}
            }
            if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                let sync_tokens = vec![
                    TokenType::Comma,
                    TokenType::RParen,
                ];
                self.synchronize(&sync_tokens);
            }
        }

        system_enter_params_opt
    }

    /* --------------------------------------------------------------------- */

    fn system_domain_params(&mut self) -> Option<Vec<ParameterNode>> {
        let mut domain_params_opt: Option<Vec<ParameterNode>> = Option::None;

        // v0.20: Domain params are now just plain parameters without #[ prefix
        // Check if we have parameters that aren't start state or enter params
        if self.peek().token_type == TokenType::Identifier {
            match self.parameters() {
                Ok(Some(parameters)) => {
                    if !self.is_building_symbol_table {
                        // check system domain params override a domain variable and match type
                        for param in &parameters {
                            let name = &param.param_name;
                            let domain_symbol_rcref_opt = self
                                .arcanum
                                .lookup(name, &IdentifierDeclScope::DomainBlockScope);
                            if domain_symbol_rcref_opt.is_none() {
                                self.error_at_current(&format!(
                                    "System domain parameter '{}' does not exist in the domain.",
                                    name
                                ));
                                let sync_tokens = vec![
                                    TokenType::InterfaceBlock,
                                    TokenType::MachineBlock,
                                    TokenType::ActionsBlock,
                                    TokenType::DomainBlock,
                                    TokenType::CloseBrace,
                                ];
                                self.synchronize(&sync_tokens);
                            } else {
                                // domain var exists, check type matches
                                let symbol_type_rcref = domain_symbol_rcref_opt.unwrap();
                                let symbol_type = symbol_type_rcref.borrow();
                                match &*symbol_type {
                                    SymbolType::DomainVariable {
                                        domain_variable_symbol_rcref,
                                    } => {
                                        let domain_variable_symbol =
                                            domain_variable_symbol_rcref.borrow();
                                        let domain_variable_symbol_type_node_opt =
                                            &domain_variable_symbol.var_type;
                                        let param_type_node_opt = &param.param_type_opt;
                                        if domain_variable_symbol_type_node_opt.is_none()
                                            && param_type_node_opt.is_none()
                                        {
                                            // ok
                                        } else if domain_variable_symbol_type_node_opt.is_some()
                                            && param_type_node_opt.is_some()
                                        {
                                            // maybe ok, check types match
                                            let domain_variable_type_node =
                                                domain_variable_symbol_type_node_opt
                                                    .as_ref()
                                                    .unwrap();
                                            let param_type_node =
                                                param_type_node_opt.as_ref().unwrap();
                                            if domain_variable_type_node
                                                .get_type_str()
                                                .ne(&param_type_node.get_type_str())
                                            {
                                                // error - one has a type and the other does not.
                                                self.error_at_current(&format!("System domain parameter '{}' type does not match domain variable type.", name));
                                                let sync_tokens = vec![
                                                    TokenType::InterfaceBlock,
                                                    TokenType::MachineBlock,
                                                    TokenType::ActionsBlock,
                                                    TokenType::DomainBlock,
                                                    TokenType::CloseBrace,
                                                ];
                                                self.synchronize(&sync_tokens);
                                            }
                                        } else {
                                            // error - one has a type and the other does not.
                                            self.error_at_current(&format!("System domain parameter '{}' type does not match domain variable type.", name));
                                            let sync_tokens = vec![
                                                TokenType::InterfaceBlock,
                                                TokenType::MachineBlock,
                                                TokenType::ActionsBlock,
                                                TokenType::DomainBlock,
                                                TokenType::CloseBrace,
                                            ];
                                            self.synchronize(&sync_tokens);
                                        }
                                    }
                                    _ => {
                                        self.error_at_current(&format!(
                                            "Compiler error - wrong type found for '{}'.",
                                            name
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    domain_params_opt = Some(parameters)
                }
                Ok(None) => {}
                Err(_) => {}
            }
        }

        domain_params_opt
    }

    /* --------------------------------------------------------------------- */

    fn functions(&mut self) -> Result<Option<Vec<Rc<RefCell<FunctionNode>>>>, ParseError> {
        let mut functions = Vec::new();

        while self.match_token(&[TokenType::Function]) {
            if let Ok(function) = self.function_scope_async(false) {
                functions.push(function);
            }
        }

        if functions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(functions))
        }
    }

    /* --------------------------------------------------------------------- */

    // This method wraps the call to the function_context() call which does
    // the parsing. Here the scope stack is managed including
    // the scope symbol creation and association with the AST node.

    // ===================== Scope Management Functions =====================
    
    // ===================== Entity Scope Functions =====================
    
    fn function_scope_async(&mut self, is_async: bool) -> Result<Rc<RefCell<FunctionNode>>, ParseError> {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Entered function_scope_async, next token: {:?}", self.peek());
        }
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected function name.";
            self.error_at_current(&err_msg);
            return Err(ParseError::new(err_msg));
        }

        let line = self.previous().line;
        let function_name = self.previous().lexeme.clone();
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Parsing function named '{}'", function_name);
        }

        // The 'is_function_context' flag is used to determine which statements are valid
        // to be called in the context of an function. Transitions, for example, are not
        // allowed.
        self.is_function_scope = true;
        
        // v0.63: Set current function context for semantic resolution
        self.current_function_name = Some(function_name.clone());

        if self.is_building_symbol_table {
            // lexical pass
            if self.debug_mode {
                // eprintln!("DEBUG: Building symbol table for function: {}", function_name);
            }
            let function_symbol = FunctionScopeSymbol::new(function_name.clone());
            //            function_symbol_opt = Some(function_symbol);

            let function_scope_symbol_rcref = Rc::new(RefCell::new(function_symbol));
            let function_symbol_parse_scope_t = ParseScopeType::Function {
                function_scope_symbol_rcref,
            };
            // eprintln!("DEBUG: Entering function scope for: {}", function_name);
            self.arcanum.enter_scope(function_symbol_parse_scope_t);
            
            // Set scope context to Function
            self.arcanum.set_scope_context(ScopeContext::Function(function_name.clone()));
            // eprintln!("DEBUG: Function symbol table building complete for: {}", function_name);
        } else {
            // semantic pass
            // link function symbol to function declaration node

            // TODO - remove?
            // let a = self
            //     .arcanum
            //     .current_symtab
            //     .borrow()
            //     .lookup(&*function_name, &IdentifierDeclScope::None);

            // see if we can get the function symbol set in the lexical pass. if so, then move
            // all this to the calling function and pass inthe symbol
            if let Err(err) = self.arcanum.set_parse_scope(&function_name) {
                return Err(ParseError::new(&format!("Failed to set function scope '{}': {}", function_name, err)));
            }
            
            // Set scope context to Function for semantic pass too
            self.arcanum.set_scope_context(ScopeContext::Function(function_name.clone()));
        }

        let ret = self.function(function_name.clone(), is_async, line);

        if self.is_building_symbol_table {
            match &ret {
                Ok(function_node_rcref) => {
                    // associate AST node with symbol

                    let function_scope_symbol_rcref_opt =
                        self.arcanum.lookup_function(&function_name.clone());
                    if let Some(function_scope_symbol_rcref) = function_scope_symbol_rcref_opt {
                        let mut function_scope_symbol = function_scope_symbol_rcref.borrow_mut();
                        function_scope_symbol.ast_node_opt = Some(function_node_rcref.clone());
                    } else {
                        // Function symbol not found - this shouldn't happen in normal cases
                        // Continue processing but log for debugging
                        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                            eprintln!("Warning: Function symbol '{}' not found during AST association", function_name);
                        }
                    }
                }
                Err(_err) => {
                    // just return the error upon exiting the function
                }
            }
        }

        self.arcanum.exit_scope();
        // Reset scope context back to Global (module level)
        self.arcanum.set_scope_context(ScopeContext::Global);
        self.is_function_scope = false;
        
        // v0.63: Clear function context after parsing
        self.current_function_name = None;
        ret
    }

    /* --------------------------------------------------------------------- */

    fn system_scope(
        &mut self,
        module_opt: Option<crate::frame_c::ast::Module>,
        system_attributes_opt: Option<HashMap<String, AttributeNode>>,
        functions_opt: Option<Vec<Rc<RefCell<FunctionNode>>>>,
    ) -> Result<SystemNode, ParseError> {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Entered system_scope, current token: {:?}", self.peek());
        }
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected system identifier.";
            self.error_at_current(&err_msg);
            return Err(ParseError::new(err_msg));
        }

        let system_name = self.previous().lexeme.clone();

        // The 'is_system_scope' flag is used to determine parsing context
        let prev_is_system_scope = self.is_system_scope;
        self.is_system_scope = true;

        if self.is_building_symbol_table {
            // lexical pass - create system symbol and enter scope
            let system_symbol = SystemSymbol::new(system_name.clone());
            let system_symbol_rcref = Rc::new(RefCell::new(system_symbol));
            let system_symbol_parse_scope_t = ParseScopeType::System {
                system_symbol: system_symbol_rcref.clone(),
            };
            self.arcanum.enter_scope(system_symbol_parse_scope_t);
            
            // Set scope context to System
            self.arcanum.set_scope_context(ScopeContext::System(system_name.clone()));
        } else {
            // semantic pass
            if let Err(err) = self.arcanum.set_parse_scope(&system_name) {
                return Err(ParseError::new(&format!("Failed to set system scope '{}': {}", system_name, err)));
            }
            
            // Set scope context to System for semantic pass too
            self.arcanum.set_scope_context(ScopeContext::System(system_name.clone()));
            
            // CRITICAL: Also set the system symbol for the semantic pass
            // Look up the system symbol from the module scope
            if let Some(system_symbol) = self.arcanum.get_system_by_name(&system_name) {
                // Debug: Check if the system symbol has blocks
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    let sys = system_symbol.borrow();
                    // eprintln!("DEBUG: Retrieved system '{}' in semantic pass", sys.name);
                    eprintln!("  Has actions block: {}", sys.actions_block_symbol_opt.is_some());
                    eprintln!("  Has machine block: {}", sys.machine_block_symbol_opt.is_some());
                    eprintln!("  Has interface block: {}", sys.interface_block_symbol_opt.is_some());
                }
                self.arcanum.set_current_system_symbol(Some(system_symbol));
            } else {
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("WARNING: Could not find system symbol '{}' in semantic pass", system_name);
                }
            }
        }

        // Call the actual system parsing method
        let ret = self.system(system_name.clone(), module_opt, system_attributes_opt, functions_opt);

        if self.is_building_symbol_table {
            // Associate AST node with symbol if successful  
            // (System symbols are managed in the symbol table enter/exit scope logic)
        }

        // Exit scope and restore state
        self.arcanum.exit_scope();
        // Reset scope context back to Global (module level)
        self.arcanum.set_scope_context(ScopeContext::Global);
        // Clear the system symbol when exiting system scope
        self.arcanum.set_current_system_symbol(None);
        self.is_system_scope = prev_is_system_scope;
        
        ret
    }

    /* --------------------------------------------------------------------- */

    fn function(
        &mut self,
        function_name: String,
        is_async: bool,
        line: usize,
    ) -> Result<Rc<RefCell<FunctionNode>>, ParseError> {
        // foo(
        if let Err(parse_error) = self.consume(TokenType::LParen, &format!("Expected '(' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        // foo(...)
        let params = match self.parameters_scope() {
            Ok(Some(parameters)) => Some(parameters),
            Ok(None) => None,
            Err(parse_error) => return Err(parse_error),
        };

        let mut type_opt: Option<TypeNode> = None;

        // foo(...) : type
        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // foo(...) : type {
        if let Err(parse_error) = self.consume(TokenType::OpenBrace, &format!("Expected '{{' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        let mut terminator_expr = TerminatorExpr::new(
            Return,
            None,
            self.previous().line,
        );
        let is_implemented = true;
        // TODO - figure out how this needs to be added to statements
        // if self.match_token(&[TokenType::SuperString]) {
        //     let token = self.previous();
        //     code_opt = Some(token.lexeme.clone());
        // }

        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: About to parse function body statements, current token: {:?}", self.peek());
        }
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Finished parsing function body, current token: {:?}", self.peek());
        }

        // foo(...) : type { ... return
        if self.match_token(&[TokenType::Return_]) {

            let expr_t = match self.expression() {
                Ok(Some(expr_t)) => expr_t,
                _ => {
                    let err_msg = "Expected expression as return value.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg));
                }
            };

            terminator_expr = TerminatorExpr::new(
                Return,
                Some(expr_t),
                self.previous().line,
            );
        }

        // foo(...) : type { ... return True }
        if let Err(parse_error) = self.consume(TokenType::CloseBrace, &format!("Expected '}}' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        let function_node = FunctionNode::new(
            function_name.clone(),
            params,
            is_implemented,
            statements,
            terminator_expr,
            type_opt,
            is_async,
            line,
        );

        let x = RefCell::new(function_node);
        let y = Rc::new(x);
        Ok(y)
    }

    /* --------------------------------------------------------------------- */

    // These are attributes that relate to a system, not a module.

    fn entity_attributes(&mut self) -> Result<Option<HashMap<String, AttributeNode>>, ParseError> {
        let mut attributes: HashMap<String, AttributeNode> = HashMap::new();

        loop {
            if self.match_token(&[TokenType::InnerAttribute]) {
                // not supported yet
                let parse_error = ParseError::new(
                    "Found '#![' token - inner attribute syntax not currently supported.",
                );
                return Err(parse_error);
            } else if self.match_token(&[TokenType::OuterAttributeOrDomainParams]) {
                let attribute_node = match self.attribute(AttributeAffinity::Outer) {
                    Ok(attribute_node) => attribute_node,
                    Err(err) => {
                        return Err(err);
                    }
                };
                attributes.insert(attribute_node.get_name(), attribute_node);
                if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                    return Err(parse_error);
                }
            } else if self.match_token(&[TokenType::At]) {
                // Handle @static, @async, etc. decorator syntax
                if !self.match_token(&[TokenType::Identifier]) {
                    let err_msg = "Expected attribute name after '@'.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg));
                }
                let attribute_name = self.previous().lexeme.clone();
                let attr = AttributeMetaWord::new(attribute_name.clone(), AttributeAffinity::Outer);
                let attribute_node = AttributeNode::MetaWord { attr };
                attributes.insert(attribute_name, attribute_node);
            } else {
                break;
            }
        }

        if attributes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(attributes))
        }
    }

    /* --------------------------------------------------------------------- */

    fn attribute(&mut self, affinity: AttributeAffinity) -> Result<AttributeNode, ParseError> {
        // attribute name: identifier (identifier | : | .)*
        let mut name;
        if self.match_token(&[TokenType::Identifier]) {
            name = self.previous().lexeme.clone();
        } else {
            let err_msg = "Expected attribute name.";
            self.error_at_current(err_msg);
            let parse_error = ParseError::new(err_msg);
            return Err(parse_error);
        }
        while self.match_token(&[TokenType::Identifier, TokenType::Colon, TokenType::Dot]) {
            name.push_str(&self.previous().lexeme.clone());
        }

        if self.match_token(&[TokenType::LParen]) {
            // MetaListIdents
            match self.meta_list_idents() {
                Ok(idents) => {
                    let attrib_idents = AttributeMetaListIdents::new(name, idents, affinity);
                    return Ok(AttributeNode::MetaListIdents {
                        attr: attrib_idents,
                    });
                }
                Err(err) => return Err(err),
            }
        } else if self.match_token(&[TokenType::Equals]) {
            // attribute value: string
            let value;
            if self.match_token(&[TokenType::String]) {
                value = self.previous().lexeme.clone();
            } else {
                let err_msg = "Expected attribute value.";
                self.error_at_current(err_msg);
                let parse_error = ParseError::new(err_msg);
                return Err(parse_error);
            }
            let attr_namevalue = AttributeMetaNameValueStr::new(name, value, affinity);
            Ok(AttributeNode::MetaNameValueStr {
                attr: attr_namevalue,
            })
        } else {
            let attr_word = AttributeMetaWord::new(name, affinity);
            Ok(AttributeNode::MetaWord { attr: attr_word })
        }
    }

    /* --------------------------------------------------------------------- */

    //  ( ',' Name )* ')'

    fn meta_list_idents(&mut self) -> Result<Vec<String>, ParseError> {
        let mut idents: Vec<String> = Vec::new();
        loop {
            // TODO: need to identify all possible valid tokens
            if !(self.match_token(&[TokenType::Identifier])
                || self.match_token(&[TokenType::True])
                || self.match_token(&[TokenType::False])
                || self.match_token(&[TokenType::Number]))
            {
                break;
            }
            let ident = self.previous().lexeme.clone();
            idents.push(ident);
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }

        if let Err(err) = self.consume(TokenType::RParen, "Expected ')'") {
            // equals
            return Err(err);
        }

        Ok(idents)
    }

    /* --------------------------------------------------------------------- */

    fn interface_block(&mut self) -> InterfaceBlockNode {
        if self.is_building_symbol_table {
            let interface_symbol = Rc::new(RefCell::new(InterfaceBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::InterfaceBlock {
                interface_block_scope_symbol_rcref: interface_symbol,
            });
        } else {
            // Semantic pass: Get the interface block from the current system
            if let Some(system_symbol) = self.arcanum.get_current_system_symbol() {
                let interface_symtab = {
                    let system = system_symbol.borrow();
                    if let Some(interface_block_symbol) = &system.interface_block_symbol_opt {
                        Some(interface_block_symbol.borrow().symtab_rcref.clone())
                    } else {
                        None
                    }
                };
                
                if let Some(symtab) = interface_symtab {
                    self.arcanum.set_current_symtab(symtab);
                } else {
                    eprintln!("ERROR: interface_block() called but system has no interface block symbol");
                }
            } else {
                eprintln!("ERROR: interface_block() called but no current system symbol");
            }
        }

        let x = &self.arcanum.current_symtab;
        self.arcanum.debug_print_current_symbols(x.clone());

        let mut interface_methods = Vec::new();

        // NOTE: this loop peeks() ahead and then interface_method() consumes
        // the identifier. Not sure if this is the best way.

        loop {
            // v0.35: Check for async interface methods
            let is_async = self.match_token(&[TokenType::Async]);
            
            if self.match_token(&[TokenType::Identifier]) {
                match self.interface_method(is_async) {
                    Ok(interface_method_node) => {
                        interface_methods.push(interface_method_node);
                    }
                        Err(_parse_error) => {
                        let sync_tokens = vec![
                            TokenType::Identifier,
                            TokenType::MachineBlock,
                            TokenType::ActionsBlock,
                            TokenType::DomainBlock,
                            TokenType::CloseBrace,
                        ];
                        self.synchronize(&sync_tokens);
                    }
                }
            } else {
                break;  // Exit loop when no more identifiers
            }
        }

        let y = &self.arcanum.current_symtab;
        self.arcanum.debug_print_current_symbols(y.clone());

        self.arcanum.exit_scope();

        InterfaceBlockNode::new(interface_methods)
    }

    /* --------------------------------------------------------------------- */

    // interface_method -> identifier ('[' parameters ']')? (':' return_type)?

    fn interface_method(&mut self, is_async: bool) -> Result<Rc<RefCell<InterfaceMethodNode>>, ParseError> {
        let method_token = self.previous();
        let name = method_token.lexeme.clone();
        let line = method_token.line;

        let mut params_opt: Option<Vec<ParameterNode>> = Option::None;
        let mut return_type_opt: Option<TypeNode> = Option::None;
        let mut alias_opt: Option<MessageNode> = Option::None;

        if self.consume(TokenType::LParen, &format!("Expected '('.")).is_err() {
            let sync_tokens = vec![
                TokenType::MachineBlock,
            ];
            self.synchronize(&sync_tokens);
            // if !self.follows(
            //     self.peek(),
            //     &[
            //         TokenType::Colon,
            //         TokenType::OpenBrace,],
            // ) {
            //     return Err(ParseError::new(&format!("Unparseable state {}",state_name)));
            // }
        }

        match self.parameters() {
            Ok(Some(parameters)) => params_opt = Some(parameters),
            Ok(None) => {},
            Err(parse_error) => return Err(parse_error),
        }

        if self.consume(TokenType::RParen, "Expected ')'").is_err() {
            let sync_tokens = vec![
                TokenType::Colon,
                // TokenType::Caret, // Removed - old return syntax
                TokenType::At,
            ];
            self.synchronize(&sync_tokens);
            // if !self.follows(
            //     self.peek(),
            //     &[
            //         TokenType::Colon,
            //         TokenType::OpenBrace,],
            // ) {
            //     return Err(ParseError::new(&format!("Unparseable state {}",state_name)));
            // }
        }

        let mut return_init_expr_opt = Option::None;

        // Parse return type
        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => return_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
            
            // Parse default return value: = value (with type)
            if self.match_token(&[TokenType::Equals]) {
                let return_expr_result = self.expression();
                match return_expr_result {
                    Ok(Some(expr_type)) => {
                        return_init_expr_opt = Some(expr_type);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        } else if self.match_token(&[TokenType::Equals]) {
            // Parse default return value: = value (without type, type inferred)
            let return_expr_result = self.expression();
            match return_expr_result {
                Ok(Some(expr_type)) => {
                    return_init_expr_opt = Some(expr_type);
                }
                Ok(None) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }

        // REMOVED: Old return syntax ^("foo") has been completely removed
        // Use 'return value' instead

        // Parse alias
        if self.match_token(&[TokenType::At]) {
            if self.consume(TokenType::LParen, "Expected '('").is_err() {
                self.error_at_current("Expected '('.");
                let sync_tokens = vec![TokenType::Pipe];
                self.synchronize(&sync_tokens);
            }

            match self.message_alias() {
                Ok(MessageType::CustomMessage { message_node }) => alias_opt = Some(message_node),
                Ok(MessageType::None) => {
                    let err_msg = "Unknown message type.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg));
                }
                Err(err) => return Err(err),
            }

            if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                let sync_tokens = vec![
                    TokenType::Identifier,
                    TokenType::MachineBlock,
                    TokenType::ActionsBlock,
                    TokenType::OperationsBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
            }
        }

        let mut param_symbols_opt = None;
        match &params_opt {
            Some(param_nodes) => {
                let mut vec = Vec::new();
                for param_node in param_nodes {
                    let param_symbol = ParameterSymbol::new(
                        param_node.param_name.clone(),
                        param_node.param_type_opt.clone(),
                        IdentifierDeclScope::UnknownScope,
                    );
                    vec.push(param_symbol);
                }
                param_symbols_opt = Some(vec);
            }
            None => {}
        }

        // if the alias exists, that is the name of the event message.
        // if not, the interface method name becomes the event message name.

        let msg = match &alias_opt {
            Some(alias) => alias.name.clone(),
            None => name.clone(),
        };

        // get or create the event symbol for the message we found
        let event_symbol_rcref;
        match self.arcanum.get_event(&*msg, &self.state_name_opt) {
            Some(_existing_event_symbol_rc_ref) => {
                // found message
                // event_symbol_rcref = existing_event_symbol_rc_ref.clone();
            }
            None => {
                let event_symbol = EventSymbol::new(
                    &self.arcanum.symbol_config,
                    &msg,
                    Some(name.clone()),
                    param_symbols_opt,
                    return_type_opt.clone(),
                    self.state_name_opt.clone(),
                );
                event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                // This is the first time we are seeing this event.
                // Set flag so parameters and return type are added to event symbol
                // during this parse.
                //               is_declaring_event = true;
            }
        }

        let interface_method_node = InterfaceMethodNode::new(
            name.clone(),
            params_opt,
            return_type_opt,
            return_init_expr_opt,
            alias_opt,
            is_async,  // v0.35: async interface methods support
            line,      // v0.77: source map support for interface definitions
        );
        let interface_method_rcref = Rc::new(RefCell::new(interface_method_node));

        if self.is_building_symbol_table {
            let mut interface_method_symbol = InterfaceMethodSymbol::new(name);
            // TODO: note what is being done. We are linking to the AST node generated in the lexical pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            interface_method_symbol.set_ast_node(Rc::clone(&interface_method_rcref));
            let interface_method_symbol_rcref = Rc::new(RefCell::new(interface_method_symbol));
            let interface_method_symbol_t = SymbolType::InterfaceMethod {
                interface_method_symbol_rcref,
            };
            // TODO: just insert into arcanum directly
            let ret = self
                .arcanum
                .current_symtab
                .borrow_mut()
                .define(&interface_method_symbol_t);
            match ret {
                Ok(()) => {}
                Err(err_msg) => {
                    self.error_at_previous(err_msg.as_str());
                    return Err(ParseError::new(err_msg.as_str()));
                }
            }
        } else {
            // TODO? - link action symbol to action declaration node
        }

        Ok(interface_method_rcref)
    }

    /* --------------------------------------------------------------------- */

    // TODO: Type resolution for Frame native types is very ad hoc.

    fn type_decl(&mut self) -> Result<TypeNode, ParseError> {
        let mut is_reference = false;
        let is_system = false;

        let mut type_str = String::new();

        if false {
            // SuperString/backtick support removed
            let id = self.previous();
            let type_str = id.lexeme.clone();
            Ok(TypeNode::new(true, false, false, false, None, type_str))
        } else {
            if self.match_token(&[TokenType::Ampersand]) {
                is_reference = true
            }
            let mut frame_event_part_opt = None;
            if self.match_token(&[TokenType::DollarAt]) {
                // TODO - review this
                frame_event_part_opt = Some(FrameEventPart::Event { is_reference })
            } else if self.match_token(&[TokenType::Identifier]) {
                type_str = self.previous().lexeme.clone();
            } else {
                let err_msg = &format!("Expected variable type name.");
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }

            // let id = self.previous();
            // let type_str = id.lexeme.clone();
            let mut is_enum = false;

            if !self.is_building_symbol_table {
                // See if this is an enumerated type.
                let symbol_t_refcell_opt = self
                    .arcanum
                    .lookup(&type_str, &IdentifierDeclScope::DomainBlockScope);
                if let Some(symbol_t_rcref) = symbol_t_refcell_opt {
                    let symbol_t = symbol_t_rcref.borrow();
                    if let SymbolType::EnumDeclSymbolT { .. } = *symbol_t {
                        is_enum = true;
                    }
                }
            }

            Ok(TypeNode::new(
                false,
                is_system,
                is_reference,
                is_enum,
                frame_event_part_opt,
                type_str,
            ))
        }
    }

    /* --------------------------------------------------------------------- */

    // message => '|' ( identifier | string | '>' | '<' ) '|'

    fn message_alias(&mut self) -> Result<MessageType, ParseError> {
        let message_node;

        if self.peek().token_type == TokenType::At {
            if let Err(parse_error) = self.consume(TokenType::At, "Expected '@'.") {
                return Err(parse_error);
            }
        }

        let tt = self.peek().token_type;
        match tt {
            TokenType::Identifier
            | TokenType::String
            | TokenType::GT
            | TokenType::LT
            // SuperString removed
            // | TokenType::GTx2
            // | TokenType::GTx3
            // | TokenType::LTx2
            // | TokenType::LTx3

            => {
                message_node = self.create_message_node(tt)
            },
            _ => {
                let token_str = self.peek().lexeme.clone();
                let err_msg = &format!("Expected closing '|' in message selector. Found {}. ", token_str);
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        }

        // let token_str = self.peek().lexeme.clone();
        // let err_msg = &format!(
        //     "Expected closing '|' in message selector. Found {}. ",
        //     token_str
        // );
        // if let Err(parse_error) = self.consume(TokenType::Pipe, err_msg) {
        //     return Err(parse_error);
        // }

        Ok(MessageType::CustomMessage { message_node })
    }

    /* --------------------------------------------------------------------- */

    // message => '|' ( identifier | string | '>' | '<' ) '|'

    fn message_selector(&mut self) -> Result<MessageType, ParseError> {
        let message_node;

        // if self.peek().token_type == TokenType::At {
        //     if let Err(parse_error) = self.consume(TokenType::At, "Expected '@'.") {
        //         return Err(parse_error);
        //     }
        // }
        // if !self.match_token(&[TokenType::Pipe]) {
        //     let token_str = self.peek().lexeme.clone();
        //     let err_msg = &format!(
        //         "Expected closing '|' in message selector. Found {}. ",
        //         token_str
        //     );
        //     self.error_at_previous(err_msg);
        //     return Err(ParseError::new(err_msg));
        // }

        let tt = self.peek().token_type;
        match tt {
            TokenType::Identifier
            | TokenType::String
            | TokenType::GT
            | TokenType::LT
            // SuperString removed
            // | TokenType::GTx2
            // | TokenType::GTx3
            // | TokenType::LTx2
            // | TokenType::LTx3

            => {
                message_node = self.create_message_node(tt)
            },
            _ => {
                let token_str = self.peek().lexeme.clone();
                let err_msg = &format!("Expected closing '|' in message selector. Found {}. ", token_str);
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        }

        let token_str = self.peek().lexeme.clone();
        let err_msg = &format!(
            "Expected closing '|' in message selector. Found {}. ",
            token_str
        );

        if let Err(parse_error) = self.consume(TokenType::Pipe, err_msg) {
            return Err(parse_error);
        }

        Ok(MessageType::CustomMessage { message_node })
    }

    /* --------------------------------------------------------------------- */

    fn create_message_node(&mut self, token_type: TokenType) -> MessageNode {
        self.match_token(&[token_type]);
        let id = self.previous();
        let name = id.lexeme.clone();

        MessageNode::new(name, id.line)
    }

    /* --------------------------------------------------------------------- */

    // TODO- see if all parameter lists can use a common parsing function and AST/symbol data and logic.
    // This is currently to implement scope for parameters for actions but should
    // be expanded to other parameter types if possible.

    fn parameters_scope(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        self.is_loop_scope = true;
        if self.is_building_symbol_table {
            let params_scope_symbol_rcref = Rc::new(RefCell::new(ParamsScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::Params {
                params_scope_symbol_rcref,
            });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(ParamsScopeSymbol::scope_name()) {
                return Err(ParseError::new(&format!("Failed to set parameters scope: {}", err)));
            }
        }

        let ret = self.parameters2();

        self.arcanum.exit_scope();

        ret
    }

    /* --------------------------------------------------------------------- */

    // TODO - unify parameters() and parameters2(). parameters2() is called by
    // methods that manage the scope and expect parameters2() to insert parameter symbols.

    // TODO: consider removing ParseError as it is currently not returned.
    fn parameters2(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        let mut parameters: Vec<ParameterNode> = Vec::new();

        loop {
            match self.parameter() {
                Ok(parameter_opt) => match parameter_opt {
                    Some(parameter_node) => {
                        let param_name = &parameter_node.param_name.clone();
                        let mut param_type_opt: Option<TypeNode> = None;
                        if parameter_node.param_type_opt.is_some() {
                            let pt = &parameter_node.param_type_opt.as_ref().unwrap().clone();
                            param_type_opt = Some(pt.clone());
                        }
                        let scope = self.arcanum.get_current_identifier_scope();
                        let param_symbol =
                            ParameterSymbol::new(param_name.clone(), param_type_opt.clone(), scope);
                        let param_symbol_rcref = Rc::new(RefCell::new(param_symbol));
                        let param_symbol_enum = SymbolType::ParamSymbol { param_symbol_rcref };
                        //  let params_scope = ParseScopeType::Params {params_scope_symbol_rcref};

                        if self.is_building_symbol_table {
                            let ret = self.arcanum.insert_symbol(param_symbol_enum);
                            match ret {
                                Ok(()) => {}
                                Err(err_msg) => {
                                    self.error_at_previous(err_msg.as_str());
                                    return Err(ParseError::new(err_msg.as_str()));
                                }
                            }
                        } else {
                            // TODO?
                        }

                        parameters.push(parameter_node);
                    }
                    None => {}
                },
                Err(parse_error) => {
                    // // TODO: just return error and don't sync. Not enough context
                    // // to know what follows.
                    // let sync_tokens = vec![
                    //     TokenType::Identifier,
                    //     TokenType::Colon,
                    //     TokenType::RBracket,
                    //     TokenType::MachineBlock,
                    //     TokenType::ActionsBlock,
                    //     TokenType::DomainBlock,
                    //     TokenType::CloseBrace,
                    // ];
                    // self.synchronize(&sync_tokens);
                    // if !self.follows(
                    //     self.peek(),
                    //     &[TokenType::Identifier, TokenType::Colon, TokenType::RBracket],
                    // ) {
                    //     break;
                    // }
                    return Err(parse_error);
                }
            }
            if self.match_token(&[TokenType::RParen]) {
                break;
            } else if let Err(parse_error) = self.consume(TokenType::Comma, &format!("Expected comma - found '{}'", self.peek().lexeme)) {
                return Err(parse_error);
            }
        }

        if parameters.is_empty() {
            Ok(None)
        } else {
            Ok(Some(parameters))
        }
    }

    /* --------------------------------------------------------------------- */

    // Just get the parameters here. The calling routine will either build or
    // validate with the EventSymbol.

    // TODO: consider removing ParseError as it is currently not returned.
    fn parameters(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        let mut parameters: Vec<ParameterNode> = Vec::new();

        loop {
            match self.parameter() {
                Ok(parameter_opt) => match parameter_opt {
                    Some(parameter_node) => {
                        parameters.push(parameter_node);
                    }
                    None => {
                        break;
                    }
                },
                Err(_parse_error) => {
                    let sync_tokens = vec![
                        TokenType::RParen,
                        TokenType::Colon,
                        TokenType::RBracket,
                    ];
                    self.synchronize(&sync_tokens);
                    if !self.follows(
                        self.peek(),
                        &[TokenType::Identifier, TokenType::Colon, TokenType::RBracket],
                    ) {
                        break;
                    }
                }
            }
            // if self.match_token(&[TokenType::RParen]) {
            //     break;
            // } else if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
            //     return Err(parse_error);
            // }

            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }

        if !parameters.is_empty() {
            return Ok(Some(parameters))
        }

        Ok(None)

        // else {
        //     self.error_at_current("Error - empty list declaration.");
        //     Err(ParseError::new("Error - empty list declaration."))
        // }
    }

    /* --------------------------------------------------------------------- */

    // parameter -> param_name ( ':' param_type )?

    fn parameter(&mut self) -> Result<Option<ParameterNode>, ParseError> {
        // v0.46: Allow 'cls' keyword as parameter name for class methods
        if !self.match_token(&[TokenType::Identifier, TokenType::Cls]) {
            // self.error_at_current("Expected parameter name.");
            // return Err(ParseError::new("TODO"));
            return Ok(None);
        }

        let id = self.previous();
        let param_name = id.lexeme.clone();

        let mut param_type_opt: Option<TypeNode> = None;

        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => param_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }

            // let id = self.previous();
            // let param_type = id.lexeme.clone();

            //param_type_opt = Some(param_type);
        }

        let scope = self.arcanum.get_current_identifier_scope();
        let param_node = ParameterNode::new(param_name, param_type_opt, scope);

        if self.is_building_symbol_table {}
        Ok(Some(param_node))
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn machine_block(&mut self) -> MachineBlockNode {
        if self.is_building_symbol_table {
            let machine_symbol = Rc::new(RefCell::new(MachineBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::MachineBlock {
                machine_scope_symbol_rcref: machine_symbol,
            });
        } else {
            // Semantic pass: Get the machine block from the current system
            if let Some(system_symbol) = self.arcanum.get_current_system_symbol() {
                let machine_symtab = {
                    let system = system_symbol.borrow();
                    if let Some(machine_block_symbol) = &system.machine_block_symbol_opt {
                        Some(machine_block_symbol.borrow().symtab_rcref.clone())
                    } else {
                        None
                    }
                };
                
                if let Some(symtab) = machine_symtab {
                    self.arcanum.set_current_symtab(symtab);
                } else {
                    eprintln!("ERROR: machine_block() called but system has no machine block symbol");
                }
            } else {
                eprintln!("ERROR: machine_block() called but no current system symbol");
            }
        }

        let mut states = Vec::new();

        while self.match_token(&[TokenType::State]) {
            match self.state() {
                Ok(state_rcref) => {
                    states.push(state_rcref);
                }
                Err(_) => {
                    self.error_at_current("Error parsing Machine Block.");
                    let sync_tokens = vec![TokenType::State];
                    if self.synchronize(&sync_tokens) {
                        continue;
                    } else {
                        let sync_tokens = vec![
                            TokenType::ActionsBlock,
                            TokenType::DomainBlock,
                            TokenType::CloseBrace,
                        ];
                        self.synchronize(&sync_tokens);
                        break;
                    }
                }
            }
        }

        self.arcanum.exit_scope();

        MachineBlockNode::new(states)
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn actions_block(&mut self) -> ActionsBlockNode {
        if self.is_building_symbol_table {
            let actions_block_scope_symbol = Rc::new(RefCell::new(ActionsBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::ActionsBlock {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            });
        } else {
            // Semantic pass: Get the actions block from the current system
            if let Some(system_symbol) = self.arcanum.get_current_system_symbol() {
                let actions_symtab = {
                    let system = system_symbol.borrow();
                    if let Some(actions_block_symbol) = &system.actions_block_symbol_opt {
                        Some(actions_block_symbol.borrow().symtab_rcref.clone())
                    } else {
                        None
                    }
                };
                
                if let Some(symtab) = actions_symtab {
                    self.arcanum.set_current_symtab(symtab);
                } else {
                    eprintln!("ERROR: actions_block() called but system has no actions block symbol");
                }
            } else {
                eprintln!("ERROR: actions_block() called but no current system symbol");
            }
        }

        let mut actions = Vec::new();

        loop {
            // v0.37: Check for async keyword before action identifier
            let is_async = self.match_token(&[TokenType::Async]);
            
            if self.match_token(&[TokenType::Identifier]) {
                if let Ok(action_decl_node) = self.action_scope(is_async) {
                    actions.push(action_decl_node);
                } else {
                    // TODO - see operations block for approach
                }
            } else {
                // If we consumed 'async' but didn't find an identifier, error
                if is_async {
                    self.error_at_current("Expected action name after 'async' keyword");
                }
                break;
            }
        }

        self.arcanum.exit_scope();

        ActionsBlockNode::new(actions)
    }

    /* --------------------------------------------------------------------- */
    //
    // fn action_decl(&mut self) -> Result<Rc<RefCell<ActionNode>>, ParseError> {
    //     let action_name = self.previous().lexeme.clone();
    //
    //     let mut params: Option<Vec<ParameterNode>> = Option::None;
    //
    //     if self.match_token(&[TokenType::LBracket]) {
    //         params = match self.parameters() {
    //             Ok(Some(parameters)) => Some(parameters),
    //             Ok(None) => None,
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //     }
    //
    //     let mut type_opt: Option<TypeNode> = None;
    //
    //     if self.match_token(&[TokenType::Colon]) {
    //         match self.type_decl() {
    //             Ok(type_node) => type_opt = Some(type_node),
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //     }
    //
    //     let mut code_opt: Option<String> = None;
    //
    //     if self.match_token(&[TokenType::OpenBrace]) {
    //         // TODO - figure out how this needes to be added to statements
    //         if self.match_token(&[TokenType::SuperString]) {
    //             let token = self.previous();
    //             code_opt = Some(token.lexeme.clone());
    //         }
    //         if self.is_building_symbol_table {
    //             let event_handler_symbol =
    //                 EventHandlerScopeSymbol::new(&msg, Rc::clone(&event_symbol_rcref));
    //             let event_handler_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_symbol));
    //
    //             self.arcanum.enter_scope(ParseScopeType::Action {
    //                 event_handler_scope_symbol_rcref,
    //             });
    //         } else {
    //             self.arcanum.set_parse_scope(&msg);
    //         }
    //
    //         if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
    //             return Err(parse_error);
    //         }
    //     }
    //
    //     let action_decl_node = ActionNode::new(action_name.clone(), params, type_opt, code_opt);
    //
    //     if self.is_building_symbol_table {
    //         let s = action_name;
    //         let mut action_decl_symbol = ActionDeclSymbol::new(s);
    //         // TODO: note what is being done. We are linking to the AST node generated in the lexical pass.
    //         // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
    //         // we could copy this information out of the node and into the symbol.
    //         // TODO: just insert into arcanum directly
    //         self.arcanum
    //             .current_symtab
    //             .borrow_mut()
    //             .insert_symbol(&action_decl_symbol_t);
    //     } else {
    //         let action_decl_rcref = Rc::new(RefCell::new(action_decl_node));
    //
    //         action_decl_symbol.set_ast_node(Rc::clone(&action_decl_rcref));
    //         let action_decl_symbol_rcref = Rc::new(RefCell::new(action_decl_symbol));
    //         let action_decl_symbol_t = SymbolType::ActionScope {
    //             action_decl_symbol_rcref,
    //         };
    //     }
    //
    //     Ok(action_decl_rcref)
    // }

    /* --------------------------------------------------------------------- */

    // This method wraps the call to the action() call which does
    // the parsing. Here the scope stack is managed including
    // the scope symbol creation and association with the AST node.

    fn action_scope(&mut self, is_async: bool) -> Result<Rc<RefCell<ActionNode>>, ParseError> {
        let action_token = self.previous();
        let action_name = action_token.lexeme.clone();
        let action_line = action_token.line;

        // The 'is_action_context' flag is used to determine which statements are valid
        // to be called in the context of an action. Transitions, for example, are not
        // allowed.
        self.is_action_scope = true;

        if self.is_building_symbol_table {
            // lexical pass
            let action_symbol = ActionScopeSymbol::new(action_name.clone());
            //            action_symbol_opt = Some(action_symbol);

            let action_scope_symbol_rcref = Rc::new(RefCell::new(action_symbol));
            let action_symbol_parse_scope_t = ParseScopeType::Action {
                action_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(action_symbol_parse_scope_t);
        } else {
            // semantic pass

            // see if we can get the action symbol set in the lexical pass. if so, then move
            // all this to the calling function and pass inthe symbol
            if let Err(err) = self.arcanum.set_parse_scope(&action_name) {
                return Err(ParseError::new(&format!("Failed to set action scope '{}': {}", action_name, err)));
            }
        }

        let ret = self.action(action_name.clone(), action_line, is_async);

        if self.is_building_symbol_table {
            match &ret {
                Ok(action_node_rcref) => {
                    // associate AST node with symbol
                    // let a = action_node_rcref.borrow();
                    let b = self.arcanum.lookup_action(&action_name.clone());
                    let c = b.unwrap();
                    let mut d = c.borrow_mut();
                    d.ast_node_opt = Some(action_node_rcref.clone());
                }
                Err(_err) => {
                    // just return the error upon exiting the function
                }
            }
        }

        self.arcanum.exit_scope();

        self.is_action_scope = false;

        ret
    }

    /* --------------------------------------------------------------------- */

    fn action(&mut self, action_name: String, action_line: usize, is_async: bool) -> Result<Rc<RefCell<ActionNode>>, ParseError> {
        // foo(
        if let Err(parse_error) = self.consume(TokenType::LParen, &format!("Expected '(' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        // foo(...
        let params = match self.parameters_scope() {
            Ok(Some(parameters)) => Some(parameters),
            Ok(None) => None,
            Err(parse_error) => return Err(parse_error),
        };

        let mut type_opt: Option<TypeNode> = None;
        let mut _default_return_expr_opt: Option<ExprType> = None;

        // foo(...) : type
        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
            
            // Parse default return value: = value
            if self.match_token(&[TokenType::Equals]) {
                let return_expr_result = self.expression();
                match return_expr_result {
                    Ok(Some(expr_type)) => {
                        _default_return_expr_opt = Some(expr_type);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }

        let code_opt: Option<String> = None;
        let mut terminator_node  = TerminatorExpr::new(
            Return,
            None,
            self.previous().line,
        );

        // foo(...) : type {
        if let Err(parse_error) = self.consume(TokenType::OpenBrace, &format!("Expected '{{' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        let is_implemented = true;
        // TODO - figure out how this needs to be added to statements
        // if self.match_token(&[TokenType::SuperString]) {
        //     let token = self.previous();
        //     code_opt = Some(token.lexeme.clone());
        // }

        let statements = self.statements(IdentifierDeclScope::BlockVarScope);

        // foo(...) : type { ... return
        if self.match_token(&[TokenType::Return_]) {

            let expr_t = match self.expression() {
                Ok(Some(expr_t)) => expr_t,
                _ => {
                    let err_msg = "Expected expression as return value.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg));
                }
            };

            terminator_node = TerminatorExpr::new(
                Return,
                Some(expr_t),
                self.previous().line,
            );

        }

        // foo(...) : type { ... return True }
        if let Err(parse_error) = self.consume(TokenType::CloseBrace, &format!("Expected '}}' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        //
        // if self.match_token(&[TokenType::RParen]) {
        //     params = match self.parameters_scope() {
        //         Ok(Some(parameters)) => Some(parameters),
        //         Ok(None) => None,
        //         Err(parse_error) => return Err(parse_error),
        //     }
        // }
        //
        // let mut type_opt: Option<TypeNode> = None;
        //
        // if self.match_token(&[TokenType::Colon]) {
        //     match self.type_decl() {
        //         Ok(type_node) => type_opt = Some(type_node),
        //         Err(parse_error) => return Err(parse_error),
        //     }
        // }
        //
        // let code_opt: Option<String> = None;
        // let mut statements = Vec::new();
        // let mut terminator_node_opt = None;
        // let mut is_implemented = false;
        //
        // if self.match_token(&[TokenType::OpenBrace]) {
        //     is_implemented = true;
        //     // TODO - figure out how this needes to be added to statements
        //     // if self.match_token(&[TokenType::SuperString]) {
        //     //     let token = self.previous();
        //     //     code_opt = Some(token.lexeme.clone());
        //     // }
        //
        //     statements = self.statements(IdentifierDeclScope::BlockVarScope);
        //
        //     if self.match_token(&[TokenType::Caret]) {
        //         if self.match_token(&[TokenType::LParen]) {
        //             // let expr_t = match self.decorated_unary_expression() {
        //             let expr_t = match self.equality() {
        //                 Ok(Some(expr_t)) => expr_t,
        //                 _ => {
        //                     self.error_at_current("Expected expression as return value.");
        //                     //  self.arcanum.exit_parse_scope();
        //                     return Err(ParseError::new("TODO"));
        //                 }
        //             };
        //
        //             if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
        //                 // self.arcanum.exit_parse_scope();
        //                 return Err(parse_error);
        //             }
        //
        //             terminator_node_opt = Some(TerminatorExpr::new(
        //                 Return,
        //                 Some(expr_t),
        //                 self.previous().line,
        //             ));
        //         } else {
        //             terminator_node_opt =
        //                 Some(TerminatorExpr::new(Return, None, self.previous().line));
        //         }
        //     }
        //
        //     if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
        //         //   self.arcanum.exit_parse_scope();
        //         return Err(parse_error);
        //     } else {
        //     }
        // }

        let action_node = ActionNode::new(
            action_line,  // v0.78.7: source map support
            action_name.clone(),
            params,
            is_implemented,
            statements,
            terminator_node,
            type_opt,
            is_async,
            code_opt,
        );

        let action_node_ref = RefCell::new(action_node);
        let action_node_rcref = Rc::new(action_node_ref);
        Ok(action_node_rcref)
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn operations_block(&mut self) -> OperationsBlockNode {
        if self.is_building_symbol_table {
            let operations_block_scope_symbol =
                Rc::new(RefCell::new(OperationsBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::OperationsBlock {
                operations_block_scope_symbol_rcref: operations_block_scope_symbol,
            });
        } else {
            // Semantic pass: Get the operations block from the current system
            // Don't use set_parse_scope with a generic name, instead get it from the system
            if let Some(system_symbol) = self.arcanum.get_current_system_symbol() {
                let operations_symtab = {
                    let system = system_symbol.borrow();
                    if let Some(operations_block_symbol) = &system.operations_block_symbol_opt {
                        Some(operations_block_symbol.borrow().symtab_rcref.clone())
                    } else {
                        None
                    }
                };
                
                if let Some(symtab) = operations_symtab {
                    self.arcanum.set_current_symtab(symtab);
                } else {
                    // No operations block in this system (which is fine, it's optional)
                    // But we're here, so there must be one - this is an error
                    eprintln!("ERROR: operations_block() called but system has no operations block symbol");
                }
            } else {
                eprintln!("ERROR: operations_block() called but no current system symbol");
            }
        }

        let mut operations = Vec::new();

        loop {
            // Comments are dealt with in match_token().
            // As we do peek() checks next we need to consume any
            // comments that preceed them.
            self.match_token(&[TokenType::PythonComment, TokenType::MultiLineComment]);

            if matches!(
                self.peek().token_type,
                TokenType::OuterAttributeOrDomainParams
            ) || matches!(self.peek().token_type, TokenType::At)
                || matches!(self.peek().token_type, TokenType::Identifier)
                || matches!(self.peek().token_type, TokenType::Async)  // v0.35: async operations
            {
                if let Ok(operation_node) = self.operation_scope() {
                    operations.push(operation_node);
                } else {
                    // TODO: resync on next operation
                    let sync_tokens = vec![TokenType::DomainBlock, TokenType::CloseBrace];
                    self.synchronize(&sync_tokens);
                    break;
                }
            } else {
                break;
            }
        }

        self.arcanum.exit_scope();

        OperationsBlockNode::new(operations)
    }

    /* --------------------------------------------------------------------- */

    // This method wraps the call to the action() call which does
    // the parsing. Here the scope stack is managed including
    // the scope symbol creation and association with the AST node.

    fn operation_scope(&mut self) -> Result<Rc<RefCell<OperationNode>>, ParseError> {
        let attributes_opt;

        // parse any outer attributes for operation
        match self.entity_attributes() {
            Ok(attributes_opt_tmp) => {
                attributes_opt = attributes_opt_tmp;
            }
            Err(parse_err) => {
                return Err(parse_err);
            }
        }
        
        // v0.35: Check for async operations
        let is_async = self.match_token(&[TokenType::Async]);

        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected Identifier.";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }

        let operation_token = self.previous();
        let operation_name = operation_token.lexeme.clone();
        let operation_line = operation_token.line;

        // The 'is_operation_scope' flag is used to determine which statements are valid
        // to be called in the context of an operation. Transitions, for example, are not
        // allowed.
        self.operation_scope_depth += 1;
        
        // Check if this is a static operation
        if let Some(ref attrs) = attributes_opt {
            if attrs.contains_key("staticmethod") {
                self.is_static_operation = true;
            }
        }

        if self.is_building_symbol_table {
            // lexical pass
            let operation_symbol = OperationScopeSymbol::new(operation_name.clone());
            //            operation_symbol_opt = Some(operation_symbol);

            let operation_scope_symbol_rcref = Rc::new(RefCell::new(operation_symbol));
            let operation_symbol_parse_scope_t = ParseScopeType::Operation {
                operation_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(operation_symbol_parse_scope_t);
        } else {
            // semantic pass

            // see if we can get the operation symbol set in the lexical pass. if so, then move
            // all this to the calling function and pass inthe symbol
            if let Err(err) = self.arcanum.set_parse_scope(&operation_name) {
                return Err(ParseError::new(&format!("Failed to set operation scope '{}': {}", operation_name, err)));
            }
        }

        let ret = self.operation(operation_name.clone(), attributes_opt, is_async, operation_line);

        if self.is_building_symbol_table {
            match &ret {
                Ok(operation_node_rcref) => {
                    // associate AST node with symbol
                    // let a = operation_node_rcref.borrow();
                    let b = self.arcanum.lookup_operation(&operation_name.clone());
                    let c = b.unwrap();
                    let mut d = c.borrow_mut();
                    d.ast_node_opt = Some(operation_node_rcref.clone());
                }
                Err(_err) => {
                    // just return the error upon exiting the function
                }
            }
        }

        self.arcanum.exit_scope();

        self.operation_scope_depth -= 1;
        self.is_static_operation = false;

        ret
    }

    /* --------------------------------------------------------------------- */

    fn operation(
        &mut self,
        operation_name: String,
        attributes_opt: Option<HashMap<String, AttributeNode>>,
        is_async: bool,  // v0.35: async operations support
        operation_line: usize,  // v0.78.2: source map support
    ) -> Result<Rc<RefCell<OperationNode>>, ParseError> {
        // foo(
        if let Err(parse_error) = self.consume(TokenType::LParen, &format!("Expected '(' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        // foo(...
        let params = match self.parameters_scope() {
            Ok(Some(parameters)) => Some(parameters),
            Ok(None) => None,
            Err(parse_error) => return Err(parse_error),
        };

        let mut type_opt: Option<TypeNode> = None;
        let mut _default_return_expr_opt: Option<ExprType> = None;

        // foo(...) : type
        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
            
            // Parse default return value: = value
            if self.match_token(&[TokenType::Equals]) {
                let return_expr_result = self.expression();
                match return_expr_result {
                    Ok(Some(expr_type)) => {
                        _default_return_expr_opt = Some(expr_type);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }

        let code_opt: Option<String> = None;
        let mut terminator_node =  TerminatorExpr::new(
            Return,
            None,
            self.previous().line,
        );

        // foo(...) : type {
        if let Err(parse_error) = self.consume(TokenType::OpenBrace, &format!("Expected '{{' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        let is_implemented = true;
        // TODO - figure out how this needs to be added to statements
        // if self.match_token(&[TokenType::SuperString]) {
        //     let token = self.previous();
        //     code_opt = Some(token.lexeme.clone());
        // }

        let statements = self.statements(IdentifierDeclScope::BlockVarScope);

        // TODO P0: align/factor return statements into a function
        // foo(...) : type { ... return
        if self.match_token(&[TokenType::Return_]) {

            let mut expr_t_opt: Option<ExprType> = None;
            let return_expr_result =  self.expression();
            match return_expr_result {
                Ok(Some(expr_t)) => {
                    expr_t_opt = Some(expr_t)
                },
                Ok(None) => {},
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }

            terminator_node = TerminatorExpr::new(
                Return,
                expr_t_opt,
                self.previous().line,
            );

        }

        // foo(...) : type { ... return True }
        if let Err(parse_error) = self.consume(TokenType::CloseBrace, &format!("Expected '}}' - found '{}'", self.current_token)) {
            return Err(parse_error);
        }

        let operation_node = OperationNode::new(
            operation_name.clone(),
            params,
            attributes_opt,
            is_implemented,
            statements,
            terminator_node,
            type_opt,
            is_async,  // v0.35: async operations support
            code_opt,
            operation_line,  // v0.78.2: source map support for operations
        );

        let operation_node_ref = RefCell::new(operation_node);
        let operation_node_rcref = Rc::new(operation_node_ref);
        Ok(operation_node_rcref)
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn domain_block(&mut self) -> DomainBlockNode {
        self.arcanum
            .debug_print_current_symbols(self.arcanum.get_current_symtab());
        if self.is_building_symbol_table {
            let domain_symbol = Rc::new(RefCell::new(DomainBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::DomainBlock {
                domain_block_scope_symbol_rcref: domain_symbol,
            });
        } else {
            // Semantic pass: Get the domain block from the current system
            if let Some(system_symbol) = self.arcanum.get_current_system_symbol() {
                let domain_symtab = {
                    let system = system_symbol.borrow();
                    if let Some(domain_block_symbol) = &system.domain_block_symbol_opt {
                        Some(domain_block_symbol.borrow().symtab_rcref.clone())
                    } else {
                        None
                    }
                };
                
                if let Some(symtab) = domain_symtab {
                    self.arcanum.set_current_symtab(symtab);
                } else {
                    eprintln!("ERROR: domain_block() called but system has no domain block symbol");
                }
            } else {
                eprintln!("ERROR: domain_block() called but no current system symbol");
            }
        }

        let mut domain_variables = Vec::new();
        let mut enums = Vec::new();

        while self.match_token(&[TokenType::Var, TokenType::Const, TokenType::Enum]) {
            if self.previous().token_type == TokenType::Enum {
                match self.enum_decl() {
                    Ok(enum_decl_node) => {
                        enums.push(enum_decl_node);
                    }
                    Err(_parse_err) => {
                        let sync_tokens =
                            vec![TokenType::Var, TokenType::Const, TokenType::CloseBrace];
                        self.synchronize(&sync_tokens);
                    }
                }
            } else {
                match self.var_declaration(IdentifierDeclScope::DomainBlockScope) {
                    Ok(domain_variable_node) => domain_variables.push(domain_variable_node),
                    Err(_parse_err) => {
                        // TODO: TokenType::Const isn't a real thing yet
                        let sync_tokens =
                            vec![TokenType::Var, TokenType::Const, TokenType::CloseBrace];
                        self.synchronize(&sync_tokens);
                    }
                }
            }
        }

        self.arcanum
            .debug_print_current_symbols(self.arcanum.get_current_symtab());
        self.arcanum.exit_scope();

        DomainBlockNode::new(domain_variables, enums)
    }

    //* --------------------------------------------------------------------- *//

    // enum Days {
    //     SUNDAY
    //     MONDAY = 2
    //     TUESDAY = 2
    // }

    fn enum_decl(&mut self) -> Result<Rc<RefCell<EnumDeclNode>>, ParseError> {
        let identifier = match self.match_token(&[TokenType::Identifier]) {
            false => {
                self.error_at_current("Expected enum identifier");
                return Err(ParseError::new("TODO"));
            }
            true => self.previous().lexeme.clone(),
        };
        
        // v0.78.8: Capture line for source mapping
        let enum_line = self.previous().line;

        // Check for type annotation (: string or : int)
        let enum_type = if self.match_token(&[TokenType::Colon]) {
            if self.match_token(&[TokenType::Identifier]) {
                match self.previous().lexeme.as_str() {
                    "string" => EnumType::String,
                    "int" => EnumType::Integer,
                    _ => {
                        self.error_at_current("Invalid enum type. Use 'string' or 'int'");
                        EnumType::Integer
                    }
                }
            } else {
                self.error_at_current("Expected type after ':'");
                EnumType::Integer
            }
        } else {
            EnumType::Integer  // Default to integer
        };

        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected enum {identifier} '{'.");
            return Err(ParseError::new("TODO"));
        }

        let mut enums = Vec::new();
        let mut enum_value = 0;  // For auto-incrementing integers
        
        while self.match_token(&[TokenType::Identifier]) {
            let enum_member_token = self.previous();
            let identifier = enum_member_token.lexeme.clone();
            let enum_member_line = enum_member_token.line;  // v0.78.9: capture line for source mapping
            let value = if self.match_token(&[TokenType::Equals]) {
                // Explicit value provided
                match enum_type {
                    EnumType::Integer => {
                        // Handle negative numbers
                        let is_negative = self.match_token(&[TokenType::Dash]);
                        
                        if self.match_token(&[TokenType::Number]) {
                            let tok = self.previous();
                            let tok_lit = &tok.literal;
                            if let TokenLiteral::Integer(val) = tok_lit {
                                let final_value = if is_negative { -val } else { *val };
                                enum_value = final_value;
                                EnumValue::Integer(final_value)
                            } else {
                                let err_msg = "Expected integer in enum assignment. Found float.";
                                self.error_at_current(&err_msg);
                                return Err(ParseError::new(err_msg));
                            }
                        } else {
                            let err_msg = "Expected number after '='.";
                            self.error_at_current(&err_msg);
                            return Err(ParseError::new(err_msg));
                        }
                    }
                    EnumType::String => {
                        if self.match_token(&[TokenType::String]) {
                            let string_value = self.previous().lexeme.clone();
                            EnumValue::String(string_value)
                        } else {
                            let err_msg = "Expected string value for string enum";
                            self.error_at_current(&err_msg);
                            return Err(ParseError::new(err_msg));
                        }
                    }
                }
            } else {
                // Auto value
                match enum_type {
                    EnumType::Integer => {
                        let val = EnumValue::Integer(enum_value);
                        enum_value += 1;
                        val
                    }
                    EnumType::String => {
                        // For string enums without explicit values, use the member name
                        EnumValue::String(identifier.clone())
                    }
                }
            };
            
            // For explicit integer values, update the auto-increment counter
            if let EnumValue::Integer(val) = &value {
                enum_value = val + 1;
            }
            
            let enumerator_node = Rc::new(EnumeratorDeclNode::new(enum_member_line, identifier, value));
            enums.push(enumerator_node);
        }

        if !self.match_token(&[TokenType::CloseBrace]) {
            self.error_at_current("Expected '}' for enum {identifier}.");
            return Err(ParseError::new("TODO"));
        }

        let enum_decl_node = EnumDeclNode::new(enum_line, identifier.clone(), enum_type, enums);
        let enum_decl_node_rcref = Rc::new(RefCell::new(enum_decl_node));

        if self.is_building_symbol_table {
            // lexical pass
            let scope = self.arcanum.get_current_identifier_scope();
            let mut enum_symbol = EnumSymbol::new(identifier.clone(), scope);

            // TODO: note what is being done. We are linking to the AST node generated in the lexical pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            enum_symbol.set_ast_node(Rc::clone(&enum_decl_node_rcref));

            let enum_symbol_rcref = Rc::new(RefCell::new(enum_symbol));
            let enum_symbol_t = SymbolType::EnumDeclSymbolT { enum_symbol_rcref };
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let ret = self
                .arcanum
                .current_symtab
                .borrow_mut()
                .define(&enum_symbol_t);
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            match ret {
                Ok(()) => {}
                Err(err_msg) => {
                    self.error_at_previous(err_msg.as_str());
                    return Err(ParseError::new(err_msg.as_str()));
                }
            }
        } else {
            // semantic pass

            // TODO
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self
                .arcanum
                .lookup(&identifier, &IdentifierDeclScope::UnknownScope);
            let y = x.unwrap();
            let z = y.borrow();
            match &*z {
                SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                    // assign enum decl node to symbol created in lexical pass
                    enum_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(enum_decl_node_rcref.clone());
                }
                _ => return Err(ParseError::new("Unrecognized enum scope.")),
            }
        }
        Ok(enum_decl_node_rcref)
    }

    //* --------------------------------------------------------------------- *//

    fn class_decl(&mut self) -> Result<Rc<RefCell<ClassNode>>, ParseError> {
        // Get class name
        let class_name = match self.match_token(&[TokenType::Identifier]) {
            false => {
                self.error_at_current("Expected class name");
                return Err(ParseError::new("Expected class name"));
            }
            true => self.previous().lexeme.clone(),
        };
        
        let line = self.previous().line;
        
        // v0.66: Set current class context for semantic resolution
        self.current_class_name = Some(class_name.clone());
        if self.debug_mode {
            eprintln!("DEBUG v0.66: Set current_class_name to {}", class_name);
        }
        
        // Check for inheritance with Python-style syntax: class Child(Parent)
        let parent = if self.match_token(&[TokenType::LParen]) {
            if !self.match_token(&[TokenType::Identifier]) {
                self.error_at_current("Expected parent class name after '('");
                return Err(ParseError::new("Expected parent class name after '('"));
            }
            let parent_name = self.previous().lexeme.clone();
            
            if !self.match_token(&[TokenType::RParen]) {
                self.error_at_current("Expected ')' after parent class name");
                return Err(ParseError::new("Expected ')' after parent class name"));
            }
            Some(parent_name)
        } else {
            None
        };
        
        // Expect opening brace
        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected '{' after class declaration");
            return Err(ParseError::new("Expected '{' after class declaration"));
        }
        
        let instance_vars = Vec::new();
        let mut static_vars = Vec::new();
        let mut methods = Vec::new();
        let mut static_methods = Vec::new();
        let mut class_methods = Vec::new();
        let mut properties: Vec<Rc<RefCell<PropertyNode>>> = Vec::new();
        let mut constructor = None;
        
        // Parse class body
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: In class body, current token: {:?}", self.peek());
            }
            
            // Skip comments
            if self.match_token(&[TokenType::PythonComment]) {
                continue;
            }
            
            // Check for decorators
            if self.check(TokenType::At) {
                eprintln!("DEBUG: Found @ token");
                let saved_pos = self.current;
                self.advance(); // consume @
                eprintln!("DEBUG: After @, next token: {:?}", self.peek());
                
                // Check decorator type - decorators come as identifiers or keywords
                if self.check(TokenType::Identifier) || self.check(TokenType::ClassMethod) || 
                   self.check(TokenType::Property) {
                    
                    let decorator_name = if self.check(TokenType::ClassMethod) {
                        "classmethod".to_string()
                    } else if self.check(TokenType::Property) {
                        "property".to_string()
                    } else {
                        self.peek().lexeme.clone()
                    };
                    
                    if decorator_name == "staticmethod" {
                        self.advance(); // consume "staticmethod"
                        if self.match_token(&[TokenType::Function]) {
                            let method = self.parse_class_method_v2(true, false)?;
                            static_methods.push(method);
                            continue;
                        } else {
                            self.error_at_current("Expected 'fn' after @staticmethod");
                            return Err(ParseError::new("Expected 'fn' after @staticmethod"));
                        }
                    } else if decorator_name == "classmethod" {
                        self.advance(); // consume "classmethod"
                        if self.match_token(&[TokenType::Function]) {
                            let method = self.parse_class_method_v2(false, true)?;
                            class_methods.push(method);
                            continue;
                        } else {
                            self.error_at_current("Expected 'fn' after @classmethod");
                            return Err(ParseError::new("Expected 'fn' after @classmethod"));
                        }
                    } else if decorator_name == "property" {
                        self.advance(); // consume "property"
                        if self.match_token(&[TokenType::Function]) {
                            let getter = self.parse_class_method_v2(false, false)?;
                            let prop_name = getter.borrow().name.clone();
                            let prop = Rc::new(RefCell::new(PropertyNode::new(prop_name)));
                            prop.borrow_mut().getter = Some(getter);
                            properties.push(prop);
                            continue;
                        } else {
                            self.error_at_current("Expected 'fn' after @property");
                            return Err(ParseError::new("Expected 'fn' after @property"));
                        }
                    } else {
                        // Check if it's a property setter or deleter
                        // Format: @property_name.setter or @property_name.deleter
                        let property_name = decorator_name.clone();
                        self.advance(); // consume property name
                        
                        if self.match_token(&[TokenType::Dot]) {
                            if self.check(TokenType::Identifier) || self.check(TokenType::Setter) || self.check(TokenType::Deleter) {
                                let decorator_type = self.peek().lexeme.clone();
                                if decorator_type == "setter" || decorator_type == "deleter" {
                                    self.advance(); // consume "setter" or "deleter"
                                    
                                    // Find the existing property
                                    let mut found_property = None;
                                    for prop in &properties {
                                        if prop.borrow().name == property_name {
                                            found_property = Some(Rc::clone(prop));
                                            break;
                                        }
                                    }
                                    
                                    if let Some(prop) = found_property {
                                        if self.match_token(&[TokenType::Function]) {
                                            let method = self.parse_class_method_v2(false, false)?;
                                            if decorator_type == "setter" {
                                                prop.borrow_mut().setter = Some(method);
                                            } else {
                                                prop.borrow_mut().deleter = Some(method);
                                            }
                                            continue;
                                        } else {
                                            self.error_at_current(&format!("Expected 'fn' after @{}.{}", property_name, decorator_type));
                                            return Err(ParseError::new(&format!("Expected 'fn' after @{}.{}", property_name, decorator_type)));
                                        }
                                    } else {
                                        self.error_at_current(&format!("Property '{}' not found. @property must be defined before @{}.setter or @{}.deleter", property_name, property_name, property_name));
                                        return Err(ParseError::new(&format!("Property '{}' not found", property_name)));
                                    }
                                } else {
                                    // Unknown decorator type after dot, this is an error
                                    self.error_at_current(&format!("Unknown property decorator @{}.{}", property_name, decorator_type));
                                    return Err(ParseError::new(&format!("Unknown property decorator @{}.{}", property_name, decorator_type)));
                                }
                            } else {
                                // No identifier after dot, this is an error
                                self.error_at_current(&format!("Expected 'setter' or 'deleter' after @{}.", property_name));
                                return Err(ParseError::new(&format!("Expected 'setter' or 'deleter' after @{}.", property_name)));
                            }
                        } else {
                            // No dot after property name, this must be an unknown decorator
                            // Don't rewind since we already consumed the name
                            self.error_at_current(&format!("Unknown decorator '@{}'", property_name));
                            return Err(ParseError::new(&format!("Unknown decorator '@{}'", property_name)));
                        }
                    }
                } else {
                    // Not an identifier after @, rewind
                    self.current = saved_pos;
                    eprintln!("DEBUG: @ not followed by identifier, rewinding");
                }
            }
            // Check for function/method
            else if self.match_token(&[TokenType::Function]) {
                let method = self.parse_class_method_v2(false, false)?;
                // Check if it's a constructor
                if method.borrow().name == "init" || method.borrow().name == "__init__" || method.borrow().is_constructor {
                    constructor = Some(method);
                } else {
                    methods.push(method);
                }
                continue;
            }
            // Check for variable declaration
            else if self.match_token(&[TokenType::Var]) {
                let var_decl = self.var_declaration(IdentifierDeclScope::DomainBlockScope)?;
                // For now, treat all class-level variables as static/class variables
                // In the future, we might want to use a decorator to distinguish
                static_vars.push(var_decl);
                continue;
            }
            else {
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Unexpected token in class body: {:?}", self.peek());
                    eprintln!("DEBUG: Current position: {}", self.current);
                    eprintln!("DEBUG: Previous token: {:?}", self.previous());
                }
                self.error_at_current("Expected method or variable declaration in class body");
                return Err(ParseError::new("Unexpected token in class body"));
            }
        }
        
        // Expect closing brace
        if !self.match_token(&[TokenType::CloseBrace]) {
            self.error_at_current("Expected '}' after class body");
            return Err(ParseError::new("Expected '}' after class body"));
        }
        
        let class_node = ClassNode::new(
            class_name,
            parent,
            Vec::new(),  // v0.58: No decorators in this path (comes from class_declaration)
            methods,
            static_methods,
            class_methods,
            properties,
            instance_vars,
            static_vars,
            constructor,
            line,
        );
        
        // v0.66: Clear class context after parsing
        if self.debug_mode {
            eprintln!("DEBUG v0.66: Clearing current_class_name (was {:?})", self.current_class_name);
        }
        self.current_class_name = None;
        
        Ok(Rc::new(RefCell::new(class_node)))
    }
    
    // v0.45: The active class method parser (supports @classmethod)
    // TODO: Merge with parse_class_method and remove duplication
    fn parse_class_method_v2(&mut self, is_static: bool, is_class: bool) -> Result<Rc<RefCell<MethodNode>>, ParseError> {
        // Get method name
        let method_name = match self.match_token(&[TokenType::Identifier]) {
            false => {
                self.error_at_current("Expected method name");
                return Err(ParseError::new("Expected method name"));
            }
            true => self.previous().lexeme.clone(),
        };
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Parsing method '{}'", method_name);
        }
        
        let line = self.previous().line;
        let is_constructor = method_name == "init" || method_name == "__init__";
        
        // Set is_class_method flag for non-static methods
        let saved_is_class_method = self.is_class_method;
        let saved_is_static = self.is_static_operation;
        self.is_class_method = !is_static && !is_class;
        self.is_static_operation = is_static || is_class;
        
        // Parse parameters - parentheses are required for methods
        if !self.match_token(&[TokenType::LParen]) {
            self.error_at_current("Expected '(' after method name");
            return Err(ParseError::new("Expected '(' after method name"));
        }
        let params_opt = self.parameters()?;
        
        // Consume closing parenthesis
        if !self.match_token(&[TokenType::RParen]) {
            self.error_at_current("Expected ')' after parameters");
            return Err(ParseError::new("Expected ')' after parameters"));
        }
        
        // Parse return type if specified
        let type_opt = if self.match_token(&[TokenType::Colon]) {
            Some(self.type_decl()?)
        } else {
            None
        };
        
        // Expect opening brace for method body
        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected '{' after method declaration");
            return Err(ParseError::new("Expected '{' after method declaration"));
        }
        
        // Parse method body
        let mut statements = Vec::new();
        let mut explicit_return_expr_opt: Option<ExprType> = None;
        
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            // Check for explicit return statement as terminator
            if self.check(TokenType::Return_) {
                self.advance(); // consume 'return'
                
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Found return in method_decl, is_class_method={}", self.is_class_method);
                }
                
                // Parse the return expression
                explicit_return_expr_opt = self.expression()?;
                
                // A return statement should be the last thing in the method
                break;
            }
            // Use EventHandlerVarScope for local variables in methods
            // This prevents them from getting the self. prefix
            else if let Some(stmt) = self.decl_or_stmt(IdentifierDeclScope::EventHandlerVarScope)? {
                statements.push(stmt);
            }
        }
        
        // Expect closing brace
        if !self.match_token(&[TokenType::CloseBrace]) {
            self.error_at_current("Expected '}' after method body");
            return Err(ParseError::new("Expected '}' after method body"));
        }
        
        // Use explicit return expression if found, otherwise empty terminator
        let terminator_expr = TerminatorExpr::new(
            TerminatorType::Return,
            explicit_return_expr_opt,
            line,
        );
        
        // Restore flags
        self.is_class_method = saved_is_class_method;
        self.is_static_operation = saved_is_static;
        
        let method_node = MethodNode::new(
            method_name,
            params_opt,
            statements,
            terminator_expr,
            type_opt,
            is_constructor,
            is_static,
            is_class,
            line,
        );
        
        Ok(Rc::new(RefCell::new(method_node)))
    }

    //* --------------------------------------------------------------------- *//

    fn var_declaration(
        &mut self,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> Result<Rc<RefCell<VariableDeclNode>>, ParseError> {
        let var_token = self.previous().clone();
        let var_line = var_token.line;
        let is_constant = match var_token.token_type {
            TokenType::Var => false,
            TokenType::Const => true,
            _ => return Err(ParseError::new("TODO")),
        };

        // v0.54: Check for star expression at the beginning
        let first_name = if self.match_token(&[TokenType::Star]) {
            if !self.match_token(&[TokenType::Identifier]) {
                self.error_at_current("Expected identifier after '*' in unpacking");
                return Err(ParseError::new("Expected identifier after '*'"));
            }
            format!("*{}", self.previous().lexeme)
        } else if self.match_token(&[TokenType::Identifier]) {
            self.previous().lexeme.clone()
        } else {
            self.error_at_current("Expected declaration identifier or star expression");
            return Err(ParseError::new("Expected identifier or *identifier"));
        };
        
        // v0.52: Check for multiple variable declarations (var x, y = ...)
        // v0.54: Support star expressions (var x, *rest, y = ...)
        let mut names = vec![first_name.clone()];
        let mut star_index = if first_name.starts_with("*") { Some(0) } else { None };
        
        while self.match_token(&[TokenType::Comma]) {
            // Check for star expression
            if self.match_token(&[TokenType::Star]) {
                if star_index.is_some() {
                    self.error_at_current("Only one star expression allowed in unpacking");
                    return Err(ParseError::new("Only one star expression allowed"));
                }
                if !self.match_token(&[TokenType::Identifier]) {
                    self.error_at_current("Expected identifier after '*' in unpacking");
                    return Err(ParseError::new("Expected identifier after '*'"));
                }
                star_index = Some(names.len());
                names.push(format!("*{}", self.previous().lexeme));
            } else if self.match_token(&[TokenType::Identifier]) {
                names.push(self.previous().lexeme.clone());
            } else {
                self.error_at_current("Expected identifier or star expression after ',' in variable declaration");
                return Err(ParseError::new("Expected identifier or star expression after ','"));
            }
        }
        
        // If we have multiple names or a star expression, handle it specially
        // Note: Single star expression (*rest = ...) is also handled as multiple var declaration
        if names.len() > 1 || first_name.starts_with("*") {
            return self.handle_multiple_var_declaration(var_line, names, is_constant, identifier_decl_scope);
        }
        
        // For single regular variable, use the name directly
        let name = first_name.clone();

        let mut type_node_opt: Option<TypeNode> = None;

        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_node_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut value = Rc::new(ExprType::DefaultLiteralValueForTypeExprT);

        if self.match_token(&[TokenType::Equals]) {
            // eprintln!("DEBUG: Parsing initializer for variable '{}'", name);
            value = self.parse_variable_initializer(&name)?;
        } else if matches!(self.peek().token_type, TokenType::In) {
            // TODO!! - develop for-in statement
            // pass
        } else {
            // All variables should be initialized to something.
            let err_msg = "Expected '='. All variables must be initialized.";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }

        // if !self.is_building_symbol_table {
        //     let _debug = 0;
        //     value.debug_print();
        // }
        let variable_decl_node = VariableDeclNode::new(
            var_line,
            name.clone(),
            type_node_opt.clone(),
            is_constant,
            value.clone(),
            value.clone(),
            identifier_decl_scope.clone(),
        );

        let variable_decl_node_rcref = Rc::new(RefCell::new(variable_decl_node));

        if self.is_building_symbol_table {
            // lexical pass - add variable to current symbol table
            self.create_variable_symbol(name, type_node_opt, &identifier_decl_scope, variable_decl_node_rcref.clone())?;
        } else {
            // semantic pass
            
            // Check for variable shadowing in functions/event handlers
            self.check_variable_shadowing(&name, &identifier_decl_scope)?;

            // TODO
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let symbol_t_opt = self
                .arcanum
                .lookup(&name, &IdentifierDeclScope::UnknownScope);
            let symbol_t_rcref = symbol_t_opt.unwrap();
            let mut symbol_t = symbol_t_rcref.borrow_mut();
            // TODO - NOTE! setting the ast node
            match symbol_t.set_ast_node(variable_decl_node_rcref.clone()) {
                Ok(()) => {}
                Err(err_str) => {
                    self.error_at_current(&format!("Symbol table error: {}", err_str));
                    return Err(ParseError::new("Symbol table error"));
                }
            }
            // match &*z {
            //     SymbolType::DomainVariable {
            //         domain_variable_symbol_rcref,
            //     } => {
            //         domain_variable_symbol_rcref.borrow_mut().set_ast_node(
            //             Some(variable_decl_node_rcref.clone()));
            //     }
            //     SymbolType::StateVariable {
            //         state_variable_symbol_rcref,
            //     } => {
            //         //                    let a = state_variable_symbol_rcref.borrow();
            //         state_variable_symbol_rcref.borrow_mut().ast_node_opt =
            //             Some(variable_decl_node_rcref.clone());
            //     }
            //     SymbolType::EventHandlerVariable {
            //         event_handler_variable_symbol_rcref,
            //     } => {
            //         event_handler_variable_symbol_rcref
            //             .borrow_mut()
            //             .ast_node_opt = Some(variable_decl_node_rcref.clone());
            //     }
            //     SymbolType::LoopVar {
            //         loop_variable_symbol_rcref,
            //     } => {
            //         loop_variable_symbol_rcref.borrow_mut().ast_node_opt =
            //             Some(variable_decl_node_rcref.clone());
            //     }
            //     SymbolType::BlockVar {
            //         block_variable_symbol_rcref,
            //     } => {
            //         block_variable_symbol_rcref.borrow_mut().ast_node_opt =
            //             Some(variable_decl_node_rcref.clone());
            //     }
            //     _ => {
            //         let err_msg = "Unrecognized variable scope.";
            //         self.error_at_current(err_msg);
            //         return Err(ParseError::new(err_msg));
            //     }
            // }
            // now need to keep current_symtab when in semantic parse phase and link to
            // ast nodes as necessary.
        }

        Ok(variable_decl_node_rcref)
    }

    /* --------------------------------------------------------------------- */
    
    // v0.52: Handle multiple variable declaration (var x, y = ...)
    fn handle_multiple_var_declaration(
        &mut self,
        var_line: usize,
        names: Vec<String>,
        is_constant: bool,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> Result<Rc<RefCell<VariableDeclNode>>, ParseError> {
        // Multiple variable declaration doesn't support type annotations
        if self.match_token(&[TokenType::Colon]) {
            self.error_at_current("Type annotations not supported in multiple variable declarations");
            return Err(ParseError::new("Type annotations not supported in multiple variable declarations"));
        }
        
        // Require equals for initialization
        if !self.match_token(&[TokenType::Equals]) {
            self.error_at_current("Multiple variable declaration requires initialization");
            return Err(ParseError::new("Multiple variable declaration requires initialization"));
        }
        
        // Parse the right-hand side
        // For multiple variable declarations, if the RHS is not parenthesized,
        // we need to explicitly parse it as a tuple
        let value = if self.check(TokenType::LParen) {
            // If it starts with '(', parse normally (will be a tuple)
            self.parse_variable_initializer(&names.join(", "))?
        } else {
            // Otherwise, we need to collect comma-separated values into a tuple
            // Parse the first value
            let mut values = Vec::new();
            match self.logical_or() {
                Ok(Some(expr)) => values.push(expr),
                Ok(None) => {
                    return Err(ParseError::new("Expected value in multiple variable initialization"));
                }
                Err(e) => return Err(e),
            }
            
            // Parse remaining comma-separated values
            while self.match_token(&[TokenType::Comma]) {
                match self.logical_or() {
                    Ok(Some(expr)) => values.push(expr),
                    Ok(None) => {
                        return Err(ParseError::new("Expected value after comma in multiple variable initialization"));
                    }
                    Err(e) => return Err(e),
                }
            }
            
            // If we have multiple values, create a tuple. If single value, use it directly
            if values.len() > 1 {
                Rc::new(ExprType::TupleLiteralT {
                    tuple_literal_node: TupleLiteralNode::new(self.previous().line, values),
                })
            } else {
                // Single value - use it directly (for unpacking like a, b, c = lst)
                Rc::new(values.into_iter().next().unwrap())
            }
        };
        
        // v0.53: Properly register ALL variables in the symbol table during first pass
        if self.is_building_symbol_table {
            // During first pass, create placeholder symbols for all variables
            for name in &names {
                // v0.54: Handle star expressions - strip the * prefix for the variable name
                let actual_name = if name.starts_with("*") {
                    name[1..].to_string()
                } else {
                    name.clone()
                };
                
                // Create a simple placeholder node for each variable
                let var_node = VariableDeclNode::new(
                    var_line,
                    actual_name.clone(),
                    None,
                    is_constant,
                    value.clone(),
                    value.clone(),
                    identifier_decl_scope.clone(),
                );
                let var_node_rcref = Rc::new(RefCell::new(var_node));
                
                // Register each variable in the symbol table
                self.create_variable_symbol(
                    actual_name, 
                    None, 
                    &identifier_decl_scope, 
                    var_node_rcref
                )?;
            }
        }
        
        // Create a special node that encodes all the variable names for the visitor
        // We'll use a comma-separated string in the name field as a signal to the visitor
        let multi_var_marker = format!("__multi_var__:{}", names.join(","));
        let variable_decl_node = VariableDeclNode::new(
            var_line,
            multi_var_marker,
            None,
            is_constant,
            value.clone(),
            value.clone(),
            identifier_decl_scope.clone(),
        );
        
        let variable_decl_node_rcref = Rc::new(RefCell::new(variable_decl_node));
        
        Ok(variable_decl_node_rcref)
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse variable initializer value
    
    fn parse_variable_initializer(&mut self, _name: &str) -> Result<Rc<ExprType>, ParseError> {
        match self.assignment() {
            Ok(Some(expr)) => {
                let value = match expr {
                    LiteralExprT { literal_expr_node } => Rc::new(LiteralExprT { literal_expr_node }),
                    VariableExprT { var_node } => Rc::new(VariableExprT { var_node }),
                    ActionCallExprT { action_call_expr_node } => Rc::new(ActionCallExprT { action_call_expr_node }),
                    CallChainExprT { call_chain_expr_node } => Rc::new(CallChainExprT { call_chain_expr_node }),
                    UnaryExprT { unary_expr_node } => Rc::new(UnaryExprT { unary_expr_node }),
                    BinaryExprT { binary_expr_node } => Rc::new(BinaryExprT { binary_expr_node }),
                    FrameEventExprT { frame_event_part } => Rc::new(FrameEventExprT { frame_event_part }),
                    EnumeratorExprT { enum_expr_node } => Rc::new(EnumeratorExprT { enum_expr_node }),
                    SystemInstanceExprT { system_instance_expr_node } => Rc::new(SystemInstanceExprT { system_instance_expr_node }),
                    SystemTypeExprT { system_type_expr_node } => Rc::new(SystemTypeExprT { system_type_expr_node }),
                    CallExprT { call_expr_node } => Rc::new(CallExprT { call_expr_node }),
                    DefaultLiteralValueForTypeExprT => Rc::new(DefaultLiteralValueForTypeExprT),
                    NilExprT => Rc::new(NilExprT),
                    SelfExprT { self_expr_node } => Rc::new(SelfExprT { self_expr_node }),
                    ListT { list_node } => Rc::new(ListT { list_node }),
                    DictLiteralT { dict_literal_node } => Rc::new(DictLiteralT { dict_literal_node }),
                    SetLiteralT { set_literal_node } => Rc::new(SetLiteralT { set_literal_node }),
                    TupleLiteralT { tuple_literal_node } => Rc::new(TupleLiteralT { tuple_literal_node }),
                    ListComprehensionExprT { list_comprehension_node } => Rc::new(ListComprehensionExprT { list_comprehension_node }),
                    UnpackExprT { unpack_expr_node } => Rc::new(UnpackExprT { unpack_expr_node }),
                    DictUnpackExprT { dict_unpack_expr_node } => Rc::new(DictUnpackExprT { dict_unpack_expr_node }),
                    AwaitExprT { await_expr_node } => Rc::new(AwaitExprT { await_expr_node }),
                    LambdaExprT { lambda_expr_node } => Rc::new(LambdaExprT { lambda_expr_node }),
                    DictComprehensionExprT { dict_comprehension_node } => Rc::new(DictComprehensionExprT { dict_comprehension_node }),
                    SetComprehensionExprT { set_comprehension_node } => Rc::new(SetComprehensionExprT { set_comprehension_node }),
                    FunctionRefT { name } => Rc::new(FunctionRefT { name }),
                    YieldExprT { yield_expr_node } => Rc::new(YieldExprT { yield_expr_node }),
                    YieldFromExprT { yield_from_expr_node } => Rc::new(YieldFromExprT { yield_from_expr_node }),
                    GeneratorExprT { generator_expr_node } => Rc::new(GeneratorExprT { generator_expr_node }),
                    StarExprT { star_expr_node } => Rc::new(StarExprT { star_expr_node }),
                    WalrusExprT { assignment_expr_node } => Rc::new(WalrusExprT { assignment_expr_node }),
                    
                    // Invalid assignment types
                    ExprListT { expr_list_node } => {
                        self.error_at_current("Expr type 'ExprList' is not a valid rvalue assignment type.");
                        Rc::new(ExprListT { expr_list_node })
                    }
                    TransitionExprT { transition_expr_node } => {
                        self.error_at_current("Expr type 'TransitionExpr' is not a valid rvalue assignment type.");
                        Rc::new(TransitionExprT { transition_expr_node })
                    }
                    AssignmentExprT { assignment_expr_node } => {
                        self.error_at_current("Expr type 'AssignmentExpr' is not a valid rvalue assignment type.");
                        Rc::new(AssignmentExprT { assignment_expr_node })
                    }
                    StateStackOperationExprT { state_stack_op_node } => {
                        self.error_at_current("Expr type 'StateStackOperationExpr' is not a valid rvalue assignment type.");
                        Rc::new(StateStackOperationExprT { state_stack_op_node })
                    }
                    CallExprListT { call_expr_list_node } => {
                        self.error_at_current("Expr type 'CallExprList' is not a valid rvalue assignment type.");
                        Rc::new(CallExprListT { call_expr_list_node })
                    }
                };
                Ok(value)
            }
            Ok(None) => {
                let err_msg = "Unexpected assignment expression value.";
                self.error_at_current(err_msg);
                Err(ParseError::new(err_msg))
            }
            Err(parse_err) => Err(parse_err),
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to check for variable shadowing
    
    fn check_variable_shadowing(&mut self, name: &str, identifier_decl_scope: &IdentifierDeclScope) -> Result<(), ParseError> {
        match identifier_decl_scope {
            IdentifierDeclScope::EventHandlerVarScope | 
            IdentifierDeclScope::BlockVarScope | 
            IdentifierDeclScope::LoopVarScope => {
                // Check if this shadows a module-level variable
                let lookup_result = self.arcanum.lookup(&name, &IdentifierDeclScope::UnknownScope);
                
                // If found, check if it's a module variable
                let shadows_module_var = if let Some(symbol_rcref) = lookup_result {
                    let symbol = symbol_rcref.borrow();
                    matches!(&*symbol, SymbolType::ModuleVariable { .. })
                } else {
                    false
                };
                
                if shadows_module_var {
                    let err_msg = format!(
                        "Cannot shadow module-level variable '{}' with local variable. \
                        Module variables cannot be shadowed in functions or event handlers. \
                        Please use a different variable name.", 
                        name
                    );
                    self.error_at_previous(&err_msg);
                    return Err(ParseError::new(&err_msg));
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to create and register variable symbol
    
    fn create_variable_symbol(
        &mut self,
        name: String,
        type_node_opt: Option<TypeNode>,
        identifier_decl_scope: &IdentifierDeclScope,
        variable_decl_node_rcref: Rc<RefCell<VariableDeclNode>>,
    ) -> Result<(), ParseError> {
        let scope = self.arcanum.get_current_identifier_scope();
        let variable_symbol = VariableSymbol::new(name, type_node_opt, scope, variable_decl_node_rcref.clone());
        let variable_symbol_rcref = Rc::new(RefCell::new(variable_symbol));
        
        let variable_symbol_t = match identifier_decl_scope {
            IdentifierDeclScope::DomainBlockScope => SymbolType::DomainVariable {
                domain_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::StateVarScope => SymbolType::StateVariable {
                state_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::EventHandlerVarScope => SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::LoopVarScope => SymbolType::LoopVar {
                loop_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::BlockVarScope => SymbolType::BlockVar {
                block_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::ModuleScope => SymbolType::ModuleVariable {
                module_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::ClassStaticScope => SymbolType::BlockVar {
                // Use BlockVar for class static variables for now
                block_variable_symbol_rcref: variable_symbol_rcref,
            },
            IdentifierDeclScope::ClassInstanceScope => SymbolType::BlockVar {
                // Use BlockVar for class instance variables for now
                block_variable_symbol_rcref: variable_symbol_rcref,
            },
            _ => {
                let err_msg = "Unrecognized variable scope.";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        };
        
        self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        let ret = self.arcanum.current_symtab.borrow_mut().define(&variable_symbol_t);
        match ret {
            Ok(()) => {}
            Err(err_msg) => {
                self.error_at_previous(err_msg.as_str());
                return Err(ParseError::new(err_msg.as_str()));
            }
        }
        self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        Ok(())
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse state name and setup scope
    
    fn parse_state_name_and_scope(&mut self, line: usize) -> Result<(String, Rc<RefCell<StateSymbol>>), ParseError> {
        if !self.match_token(&[TokenType::Identifier]) {
            // error message and synchronize
            self.error_at_current("Expected state name.");
            let sync_tokens = vec![
                TokenType::State,
                TokenType::ActionsBlock,
                TokenType::DomainBlock,
                TokenType::CloseBrace,
            ];
            self.synchronize(&sync_tokens);

            let state_node = StateNode::new(
                String::from("error"),
                None,
                None,
                Option::None,
                Vec::new(),
                Option::None,
                Option::None,
                None,
                line,
            );
            let _state_node_rcref = Rc::new(RefCell::new(state_node));
            return Err(ParseError::new("Expected state name"));
        }
        
        let id = self.previous();
        let state_name = id.lexeme.clone();
        self.state_name_opt = Some(state_name.clone());

        let state_symbol_rcref = if self.is_building_symbol_table {
            if self.arcanum.get_state(&state_name).is_some() {
                self.error_at_previous(&format!("Duplicate state name {}.", &state_name));
            }
            let state_symbol = StateSymbol::new(&state_name, self.arcanum.get_current_symtab());
            let state_symbol_rcref = Rc::new(RefCell::new(state_symbol));
            self.arcanum.enter_scope(ParseScopeType::State {
                state_symbol: state_symbol_rcref.clone(),
            });
            state_symbol_rcref
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(&state_name) {
                return Err(ParseError::new(&format!("Failed to set state scope '{}': {}", state_name, err)));
            }
            self.arcanum.get_state(&state_name).unwrap()
        };
        
        Ok((state_name, state_symbol_rcref))
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse state parameters
    
    fn parse_state_parameters(&mut self, state_name: &str) -> Result<(Option<Vec<ParameterNode>>, bool), ParseError> {
        if !self.match_token(&[TokenType::LParen]) {
            return Ok((None, false));
        }
        
        // generate StateContext mechanism for state parameter support
        self.generate_state_context = true;
        
        match self.parameters() {
            Ok(Some(parameters)) => {
                let pop_state_params_scope = true;
                
                if self.is_building_symbol_table {
                    match self.arcanum.get_state(&state_name) {
                        Some(state_symbol) => {
                            let state_params_scope_symbol = StateParamsScopeSymbol::new();
                            let state_params_scope_symbol_rcref = 
                                Rc::new(RefCell::new(state_params_scope_symbol));
                            self.arcanum.enter_scope(ParseScopeType::StateParams {
                                state_params_scope_symbol_rcref,
                            });
                            for param in &parameters {
                                let scope = self.arcanum.get_current_identifier_scope();
                                let symbol_t = state_symbol.borrow_mut().add_parameter(
                                    param.param_name.clone(),
                                    param.param_type_opt.clone(),
                                    scope,
                                );
                                let ret = self.arcanum.insert_symbol(symbol_t);
                                match ret {
                                    Ok(()) => {}
                                    Err(err_msg) => {
                                        self.error_at_previous(err_msg.as_str());
                                        return Err(ParseError::new(err_msg.as_str()));
                                    }
                                }
                            }
                        }
                        None => {
                            return Err(ParseError::new(&format!(
                                "Fatal error: unable to find state {}.",
                                state_name
                            )));
                        }
                    }
                } else {
                    if let Err(err) = self.arcanum.set_parse_scope(StateParamsScopeSymbol::scope_name()) {
                        return Err(ParseError::new(&format!("Failed to set state params scope: {}", err)));
                    }
                }
                
                // Consume closing paren
                if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                    let sync_tokens = vec![
                        TokenType::Colon,
                        TokenType::Dispatch,
                        TokenType::OpenBrace,
                    ];
                    self.synchronize(&sync_tokens);
                    if !self.follows(
                        self.peek(),
                        &[
                            TokenType::Colon,
                            TokenType::Dispatch,
                            TokenType::OpenBrace,
                        ],
                    ) {
                        return Err(ParseError::new(&format!("Unparseable state {}", state_name)));
                    }
                }
                
                Ok((Some(parameters), pop_state_params_scope))
            }
            Err(parse_error) => Err(parse_error),
            Ok(None) => {
                // Consume closing paren
                if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                    let sync_tokens = vec![
                        TokenType::Colon,
                        TokenType::Dispatch,
                        TokenType::OpenBrace,
                    ];
                    self.synchronize(&sync_tokens);
                }
                Ok((None, false))
            }
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to parse dispatch clause
    
    fn parse_state_dispatch(&mut self, state_name: &str) -> Result<Option<DispatchNode>, ParseError> {
        if !self.match_token(&[TokenType::Dispatch]) {
            // Update hierarchy without dispatch
            match &mut self.system_hierarchy_opt {
                Some(system_hierarchy) => {
                    system_hierarchy.add_node(state_name.to_string(), String::new());
                }
                None => {
                    return Err(ParseError::new("System Hierarchy should always be here."));
                }
            }
            return Ok(None);
        }
        
        match self.consume(TokenType::State, "Expected '$'") {
            Ok(_) => {
                if self.match_token(&[TokenType::Identifier]) {
                    let id = self.previous();
                    let target_state_name = id.lexeme.clone();
                    
                    let target_state_ref = StateRefNode::new(target_state_name.clone());
                    let dispatch_node = DispatchNode::new(target_state_ref, id.line);
                    
                    // Update hierarchy with dispatch
                    match &mut self.system_hierarchy_opt {
                        Some(system_hierarchy) => {
                            system_hierarchy.add_node(state_name.to_string(), target_state_name);
                        }
                        None => {
                            return Err(ParseError::new("System Hierarchy should always be here."));
                        }
                    }
                    
                    // Track parent state for => $^ validation
                    self.state_parent_opt = Some(dispatch_node.target_state_ref.name.clone());
                    
                    Ok(Some(dispatch_node))
                } else {
                    self.error_at_current("Expected dispatch target state identifier.");
                    let sync_tokens = vec![
                        TokenType::Pipe,
                        TokenType::State,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::CloseBrace,
                    ];
                    self.synchronize(&sync_tokens);
                    Ok(None)
                }
            }
            Err(_) => {
                // synchronize to next event handler, state, remaining blocks or the end token
                let sync_tokens = vec![
                    TokenType::Pipe,
                    TokenType::State,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
                Ok(None)
            }
        }
    }
    
    /* --------------------------------------------------------------------- */
    // Helper function to setup state local scope
    
    fn setup_state_local_scope(&mut self) -> Result<(), ParseError> {
        let _ = self.consume(TokenType::OpenBrace, "Expected '{'");
        
        if self.is_building_symbol_table {
            let state_local_scope_struct = StateLocalScopeSymbol::new();
            let state_local_scope_symbol_rcref = Rc::new(RefCell::new(state_local_scope_struct));
            let state_local_scope = ParseScopeType::StateLocal {
                state_local_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(state_local_scope);
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(StateLocalScopeSymbol::scope_name()) {
                return Err(ParseError::new(&format!("Failed to set state local scope: {}", err)));
            }
        }
        Ok(())
    }
    
    /* --------------------------------------------------------------------- */

    // TODO return result
    //    fn state(&mut self) -> Rc<RefCell<StateNode>> {
    fn state(&mut self) -> Result<Rc<RefCell<StateNode>>, ParseError> {
        let line = self.previous().line;
        
        // Parse state name and setup scope
        let (state_name, state_symbol_rcref) = self.parse_state_name_and_scope(line)?;

        // Parse state parameters
        let (params_opt, pop_state_params_scope) = self.parse_state_parameters(&state_name)?;


        // Parse dispatch clause
        let dispatch_opt = self.parse_state_dispatch(&state_name)?;
        
        // Setup state local scope
        self.setup_state_local_scope()?;

        // Parse state local variables
        let vars_opt = self.parse_state_variables()?;

        // Parse state calls (currently unused)
        let calls_opt = self.parse_state_calls()?;

        // Parse event handlers
        let state_event_handlers = self.parse_state_event_handlers(&state_name)?;


        let _ = self.consume(TokenType::CloseBrace, "Expected '}'")? ;

        // Create state node and update symbol table
        let state_node_rcref = self.create_and_register_state_node(
            state_name,
            params_opt,
            vars_opt,
            calls_opt,
            state_event_handlers,
            dispatch_opt,
            line,
            &state_symbol_rcref,
        )?;

        // Cleanup scopes and state
        self.cleanup_state_parsing(pop_state_params_scope)?;

        Ok(state_node_rcref)
    }

    fn parse_state_variables(&mut self) -> Result<Option<Vec<Rc<RefCell<VariableDeclNode>>>>, ParseError> {
        // state local variables
        // $S1 {
        //    var a = 1
        // }
        self.state_variables()
    }

    fn parse_state_calls(&mut self) -> Result<Option<Vec<CallChainExprNode>>, ParseError> {
        // TODO: I don't know what state calls are.
        // State Calls - currently unused
        Ok(None)
    }

    fn parse_state_event_handlers(&mut self, state_name: &str) -> Result<StateEventHandlers, ParseError> {
        let mut state_event_handlers = StateEventHandlers {
            enter_event_handler_opt: None,
            exit_event_handler_opt: None,
            event_handlers: vec![],
        };

        // Event handler syntax:
        // a(x:int,y:string) : bool(True) { }
        // v0.37: async $>() { ... }

        // Check for async keyword before event handler
        let is_async_handler = self.match_token(&[TokenType::Async]);
        
        if self.match_token(&[TokenType::Identifier,TokenType::EnterStateMsg,TokenType::ExitStateMsg]) {
            state_event_handlers = self.event_handlers(&state_name.to_string(), is_async_handler)?;
        }
        
        Ok(state_event_handlers)
    }

    fn create_and_register_state_node(
        &mut self,
        state_name: String,
        params_opt: Option<Vec<ParameterNode>>,
        vars_opt: Option<Vec<Rc<RefCell<VariableDeclNode>>>>,
        calls_opt: Option<Vec<CallChainExprNode>>,
        state_event_handlers: StateEventHandlers,
        dispatch_opt: Option<DispatchNode>,
        line: usize,
        state_symbol_rcref: &Rc<RefCell<StateSymbol>>,
    ) -> Result<Rc<RefCell<StateNode>>, ParseError> {
        // TODO: Moved this down here as I think is a bug to hve it above but not sure.
        self.arcanum.exit_scope(); // state block scope (StateBlockScopeSymbol)

        let state_node = StateNode::new(
            state_name,
            params_opt,
            vars_opt,
            calls_opt,
            state_event_handlers.event_handlers,
            state_event_handlers.enter_event_handler_opt,
            state_event_handlers.exit_event_handler_opt,
            dispatch_opt,
            line,
        );
        let state_node_rcref = Rc::new(RefCell::new(state_node));

        // NOTE!! There was a very challenging bug introduced when
        // the parser started supporting variables being assigned
        // values properly. Search python visitor for #STATE_NODE_UPDATE_BUG
        // to see where this happened.
        // I'm leaving in the redundant identical code here in both paths to highlight
        // setting the state IS ABSOLUTELY NECESSARY in both syntactic and semantic passes.
        // The first pass sets a state with variables that their
        // type identified as CallChainNodeType::UndeclaredIdentifierNodeT because
        // we don't know what they are. In the semantic pass we can look them up
        // the variable declarations are typed properly as CallChainNodeType::VariableNodeT.
        // So we MUST update the state (this is all very tricky) which contains the
        // variable declarations in both passes.
        if self.is_building_symbol_table {
            // Set state with dummy variable values (CallChainNodeType::UndeclaredIdentifierNodeT)
            // in lexical pass.
            state_symbol_rcref
                .borrow_mut()
                .set_state_node(Rc::clone(&state_node_rcref));
        } else {
            // Set state with variable decl values for the type (CallChainNodeType::VariableNodeT)
            // in lexical pass.
            state_symbol_rcref
                .borrow_mut()
                .set_state_node(Rc::clone(&state_node_rcref));
        }

        Ok(state_node_rcref)
    }

    fn cleanup_state_parsing(&mut self, pop_state_params_scope: bool) -> Result<(), ParseError> {
        self.state_name_opt = None;
        self.state_parent_opt = None;

        if pop_state_params_scope {
            self.arcanum.exit_scope(); // state params scope
        }
        self.arcanum.exit_scope(); // state scope
        
        Ok(())
    }

    /* --------------------------------------------------------------------- */

    fn state_variables(&mut self) -> Result<Option<Vec<Rc<RefCell<VariableDeclNode>>>>, ParseError> {
        // variable decl
        // let v     (mutable)
        // const c   (immutable)

        let mut vars = Vec::new();

        while self.match_token(&[TokenType::Var, TokenType::Const]) {
            self.generate_state_context = true;
            match self.var_declaration(IdentifierDeclScope::StateVarScope) {
                Ok(variable_node) => {
                    vars.push(variable_node);
                }
                Err(err) => {
                    // TODO - The main sync logic is in statements().
                    // TODO - Need to add here as well to continue parse.
                    return Err(err);
                }
            }
        }

        if !vars.is_empty() {
            Ok(Some(vars))
        } else {
            Ok(None)
        }
    }

    /* --------------------------------------------------------------------- */

    fn event_handlers(&mut self,state_name:&String, mut is_async: bool) -> Result<StateEventHandlers, ParseError> {

        let mut evt_handlers: Vec<Rc<RefCell<EventHandlerNode>>> = Vec::new();
        let mut enter_event_handler = Option::None;
        let mut exit_event_handler = Option::None;

        let mut event_names = HashMap::new();

        loop {
            match self.event_handler(is_async) {
                Ok(eh_opt) => {
                    if let Some(eh) = eh_opt {
                        let eh_rcref = Rc::new(RefCell::new(eh));

                        // find enter/exit event handlers
                        {
                            // new scope to make BC happy
                            let eh_ref = eh_rcref.as_ref().borrow();
                            let evt = eh_ref.event_symbol_rcref.as_ref().borrow();

                            if evt.is_enter_msg {
                                if enter_event_handler.is_some() {
                                    self.error_at_current(&format!(
                                        "State ${} has more than one enter event handler.",
                                        &state_name
                                    ));
                                } else {
                                    enter_event_handler = Some(eh_rcref.clone());
                                }
                            } else if evt.is_exit_msg {
                                if exit_event_handler.is_some() {
                                    self.error_at_current(&format!(
                                        "State ${} has more than one exit event handler.",
                                        &state_name
                                    ));
                                } else {
                                    exit_event_handler = Some(eh_rcref.clone());
                                }
                            } else {
                                if event_names.contains_key(&evt.msg) {
                                    let err_msg = &format!(
                                        "Event handler {} already exists.",
                                        evt.msg
                                    );
                                    self.error_at_previous(&err_msg);
                                    //                                            return Err(ParseError::new(err_msg));
                                } else {
                                    event_names.insert(evt.msg.clone(), evt.msg.clone());
                                }
                            }
                        }

                        self.current_event_symbol_opt = None;
                        evt_handlers.push(eh_rcref);
                    }
                }
                Err(_) => {
                    let sync_tokens = vec![
                        TokenType::Pipe,
                        TokenType::State,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::CloseBrace,
                    ];
                    self.synchronize(&sync_tokens);
                }
            }
            // Check for async keyword before next event handler
            let next_is_async = self.match_token(&[TokenType::Async]);
            
            if !self.match_token(&[TokenType::Identifier,TokenType::EnterStateMsg,TokenType::ExitStateMsg]) {
                // If we consumed async but no event handler follows, that's an error
                if next_is_async {
                    self.error_at_current("Expected event handler after 'async' keyword");
                    return Err(ParseError::new("Expected event handler after 'async' keyword"));
                }
                break;
            }
            
            // Update is_async for the next iteration
            is_async = next_is_async;
        }

        let state_event_handlers = StateEventHandlers::new(
            enter_event_handler,
            exit_event_handler,
            evt_handlers);
        Ok(state_event_handlers)
    }

    /* --------------------------------------------------------------------- */

    // event_handler -> '|' Identifier '|' event_handler_terminator

    // TODO: This is a mess and needs to be cleaned up.

    // Helper: Consume single line comments before event handler
    fn consume_event_handler_comments(&mut self) {
        while self.match_token(&[TokenType::PythonComment]) {
            // consume
            // @TODO: fix this. see https://app.asana.com/0/1199651557660024/1199953268166075/f
            // this is a hack because we don't use
            // match on the next tests but instead use peek().
            // this causes an error for this situation:
            /*
            $State
                |continueEvent|
                >       --- continue terminator
                |returnEvent|
                ^       --- return terminator

             */
        }
    }

    // Helper: Parse and validate event message
    fn parse_event_message(&mut self) -> Result<(MessageType, String, usize), ParseError> {
        let tt = self.previous().token_type;
        let message_node = match tt {
            TokenType::Identifier
            | TokenType::EnterStateMsg
            | TokenType::ExitStateMsg => {
                self.create_message_node(tt)
            },
            _ => {
                let token_str = self.previous().lexeme.clone();
                let err_msg = &format!("Invalid event type. Found {}. ", token_str);
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        };
        
        let message_type = MessageType::CustomMessage { message_node };
        let id = self.previous();
        let msg = id.lexeme.clone();
        let line_number = id.line;
        
        Ok((message_type, msg, line_number))
    }

    // Helper: Set up event handler scope and symbol
    fn setup_event_handler_scope(&mut self, msg: &str) -> Result<bool, ParseError> {
        let mut is_declaring_event = false;

        if self.is_building_symbol_table {
            let event_symbol_rcref;

            // get or create the event symbol for the message we found
            match self.arcanum.get_event(msg, &self.state_name_opt) {
                Some(x) => {
                    event_symbol_rcref = Rc::clone(&x);
                }
                None => {
                    let event_symbol = EventSymbol::new(
                        &self.arcanum.symbol_config,
                        msg,
                        None,
                        None,
                        None,
                        self.state_name_opt.clone(),
                    );
                    event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                    self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                    // This is the first time we are seeing this event.
                    // Set flag so parameters and return type are added to event symbol
                    // during this parse.
                    is_declaring_event = true;
                }
            }

            // create the event handler symbol and enter the event handler scope
            let event_handler_symbol =
                EventHandlerScopeSymbol::new(msg, Rc::clone(&event_symbol_rcref));
            let event_handler_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_symbol));

            self.arcanum.enter_scope(ParseScopeType::EventHandler {
                event_handler_scope_symbol_rcref,
            });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(msg) {
                return Err(ParseError::new(&format!("Failed to set event handler scope '{}': {}", msg, err)));
            }
        }

        Ok(is_declaring_event)
    }

    // Helper: Parse event handler parameters
    fn parse_event_handler_parameters(&mut self, msg: &str, is_declaring_event: bool) -> Result<bool, ParseError> {
        let mut pop_params_scope = false;

        if self.match_token(&[TokenType::LParen]) {
            if msg == self.arcanum.symbol_config.enter_msg_symbol {
                self.generate_state_context = true;
            }

            match self.parameters() {
                Ok(Some(parameters)) => {
                    pop_params_scope = true;
                    
                    if self.is_building_symbol_table {
                        self.process_event_handler_parameters_symbol_table(msg, &parameters, is_declaring_event)?;
                    } else {
                        if let Err(err) = self.arcanum.set_parse_scope(EventHandlerParamsScopeSymbol::scope_name()) {
                            return Err(ParseError::new(&format!("Failed to set event handler params scope: {}", err)));
                        }
                    }
                }
                Ok(None) => { },
                Err(parse_error) => return Err(parse_error),
            }
        } else {
            // no parameter list - validate no parameters expected
            let event_symbol_rcref = self.arcanum.get_event(msg, &self.state_name_opt).unwrap();
            if event_symbol_rcref.borrow().event_symbol_params_opt.is_some() {
                self.error_at_current(&format!(
                    "Event handler {} parameters do not match a previous declaration.",
                    msg
                ));
            }
        }

        Ok(pop_params_scope)
    }

    // Helper: Process event handler parameters for symbol table
    fn process_event_handler_parameters_symbol_table(
        &mut self,
        msg: &str,
        parameters: &Vec<ParameterNode>,
        is_declaring_event: bool
    ) -> Result<(), ParseError> {
        let event_symbol_rcref = self.arcanum.get_event(msg, &self.state_name_opt).unwrap();

        if is_declaring_event {
            // First time seeing this event - add parameters to symbol
            let mut vec = Vec::new();
            for param_node in parameters {
                let param_symbol = ParameterSymbol::new(
                    param_node.param_name.clone(),
                    param_node.param_type_opt.clone(),
                    IdentifierDeclScope::UnknownScope,
                );
                vec.push(param_symbol);
            }
            event_symbol_rcref.borrow_mut().event_symbol_params_opt = Some(vec);
        } else {
            // Validate parameters match previous declaration
            if event_symbol_rcref.borrow().event_symbol_params_opt.is_none() && !parameters.is_empty() {
                self.error_at_current(&format!(
                    "Event handler {} parameters do not match a previous declaration.",
                    msg
                ));
            }
        }

        // Set up parameter scope
        let event_handler_params_scope_struct = EventHandlerParamsScopeSymbol::new(event_symbol_rcref);
        let event_handler_params_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_params_scope_struct));
        let event_handler_params_scope = ParseScopeType::EventHandlerParams {
            event_handler_params_scope_symbol_rcref,
        };
        self.arcanum.enter_scope(event_handler_params_scope);

        // Process and validate individual parameters
        self.process_individual_parameters(msg, parameters)?;

        Ok(())
    }

    // Helper: Process individual parameters
    fn process_individual_parameters(
        &mut self,
        msg: &str,
        parameters: &Vec<ParameterNode>
    ) -> Result<(), ParseError> {
        let event_symbol_rcref = match self.arcanum.get_event(msg, &self.state_name_opt) {
            Some(x) => x,
            None => {
                return Err(ParseError::new(&format!(
                    "Fatal error - could not find event {}.",
                    msg
                )));
            }
        };

        let mut event_handler_params_scope_symbol = EventHandlerParamsScopeSymbol::new(Rc::clone(&event_symbol_rcref));
        let event_symbol_rcref = self.arcanum.get_event(msg, &self.state_name_opt).unwrap();
        
        let mut event_symbol_params_opt: Option<Vec<ParameterSymbol>> = None;

        match &event_symbol_rcref.borrow().event_symbol_params_opt {
            Some(symbol_params) => {
                // Compare existing event symbol params with parsed ones
                for (i, x) in symbol_params.iter().enumerate() {
                    match parameters.get(i) {
                        Some(parameter_node) => {
                            if x.is_eq(parameter_node) {
                                let scope = self.arcanum.get_current_identifier_scope();
                                let symbol_type = event_handler_params_scope_symbol.add_parameter(
                                    parameter_node.param_name.clone(),
                                    parameter_node.param_type_opt.clone(),
                                    scope,
                                );
                                self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                                let ret = self.arcanum.insert_symbol(symbol_type);
                                self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                                if let Err(err_msg) = ret {
                                    self.error_at_previous(err_msg.as_str());
                                    return Err(ParseError::new(err_msg.as_str()));
                                }
                            } else {
                                self.error_at_current(
                                    "Parameters for event handler do not match declaration in interface or a previous event handler for the message.",
                                );
                            }
                        }
                        None => {
                            self.error_at_current("Incorrect number of parameters");
                        }
                    }
                }
            }
            None => {
                // First time seeing parameters for this event
                let mut event_symbol_params = Vec::new();

                for param in parameters {
                    let param_name = &param.param_name.clone();
                    let mut param_type_opt: Option<TypeNode> = None;
                    if param.param_type_opt.is_some() {
                        let pt = &param.param_type_opt.as_ref().unwrap().clone();
                        param_type_opt = Some(pt.clone());
                    }
                    let scope = self.arcanum.get_current_identifier_scope();
                    let b = ParameterSymbol::new(
                        param_name.clone(),
                        param_type_opt.clone(),
                        scope,
                    );
                    event_symbol_params.push(b);

                    // Add to event handler scope symbol for scope chain lookups
                    let scope = self.arcanum.get_current_identifier_scope();
                    let x = event_handler_params_scope_symbol.add_parameter(
                        param_name.clone(),
                        param_type_opt.clone(),
                        scope,
                    );
                    if let Err(err_msg) = self.arcanum.insert_symbol(x) {
                        self.error_at_previous(err_msg.as_str());
                        return Err(ParseError::new(err_msg.as_str()));
                    }
                }
                event_symbol_params_opt = Some(event_symbol_params);
            }
        }

        if let Some(parameter_symbols) = event_symbol_params_opt {
            event_symbol_rcref.borrow_mut().event_symbol_params_opt = Some(parameter_symbols);
        }

        Ok(())
    }

    // Helper: Consume closing paren with error synchronization
    fn consume_rparen_with_sync(&mut self) -> Result<(), ParseError> {
        if self.consume(TokenType::RParen, "Expected ')'").is_err() {
            let sync_tokens = vec![
                TokenType::Colon,
                TokenType::OpenBrace,
            ];
            self.synchronize(&sync_tokens);
        }
        Ok(())
    }

    // Helper: Parse event handler return type
    fn parse_event_handler_return_type(&mut self, msg: &str, is_declaring_event: bool) -> Result<(), ParseError> {
        if self.match_token(&[TokenType::Colon]) {
            let return_type_opt = match self.type_decl() {
                Ok(type_node) => Some(type_node),
                Err(parse_error) => return Err(parse_error),
            };
            
            if is_declaring_event {
                // declaring event so add return type to event symbol
                let event_symbol_rcref = self.arcanum.get_event(msg, &self.state_name_opt).unwrap();
                event_symbol_rcref.borrow_mut().ret_type_opt = return_type_opt;
            }
            // Event handlers can have their own return type declarations
        }
        Ok(())
    }

    // Helper: Parse event handler return initialization
    fn parse_event_handler_return_init(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::Equals]) {
            match self.expression() {
                Ok(Some(expr_type)) => Ok(Some(expr_type)),
                Ok(None) => Ok(None),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }

    // Helper: Enter event handler local scope
    fn enter_event_handler_local_scope(&mut self) -> Result<(), ParseError> {
        if self.is_building_symbol_table {
            let event_handler_local_scope_struct = EventHandlerLocalScopeSymbol::new();
            let event_handler_local_scope_symbol_rcref =
                Rc::new(RefCell::new(event_handler_local_scope_struct));
            let event_handler_local_scope = ParseScopeType::EventHandlerLocal {
                event_handler_local_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(event_handler_local_scope);
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(EventHandlerLocalScopeSymbol::scope_name()) {
                return Err(ParseError::new(&format!("Failed to set event handler local scope: {}", err)));
            }
        }
        Ok(())
    }

    // Helper: Parse event handler terminator
    fn parse_event_handler_terminator(&mut self) -> Result<Option<TerminatorExpr>, ParseError> {
        if self.match_token(&[TokenType::Return_]) {
            let expr_t_opt = match self.expression() {
                Ok(Some(expr_t)) => Some(expr_t),
                Ok(None) => None,
                Err(parse_error) => return Err(parse_error),
            };

            Ok(Some(TerminatorExpr::new(
                Return,
                expr_t_opt,
                self.previous().line,
            )))
        } else {
            Ok(None)
        }
    }

    fn event_handler(&mut self, is_async: bool) -> Result<Option<EventHandlerNode>, ParseError> {
        // Variables initialized at point of use - see line 3632-3635
        self.interface_method_called = false;
        self.event_handler_has_transition = false;

        // consume single line comments
        self.consume_event_handler_comments();

        //    let a = self.message();


 //        let message_node;
 //        let tt = self.previous().token_type;
 //        match tt {
 //            TokenType::Identifier
 //  //          | TokenType::String
 //            | TokenType::GT
 //            | TokenType::LT
 //  //          | TokenType::SuperString
 //            // | TokenType::GTx2
 //            // | TokenType::GTx3
 //            // | TokenType::LTx2
 //            // | TokenType::LTx3
 //
 //            => {
 // //               message_node = self.create_message_node(tt)
 //                let id = self.previous();
 //                msg = id.lexeme.clone();
 //
 //            },
 //            _ => {
 //                let token_str = self.peek().lexeme.clone();
 //                let err_msg = &format!("Invalid event handler name {}. ", token_str);
 //                self.error_at_current(err_msg);
 //    //            return Err(ParseError::new(err_msg));
 //            }
 //        }

        // match self.message_selector() {
        //     Ok(MessageType::CustomMessage { message_node }) => {
        //         line_number = message_node.line;
        //         msg = message_node.name.clone();
        //
        //         message_type = CustomMessage { message_node };
        //     }
        //     Ok(MessageType::None) => {
        //         let err_msg = "Unknown message type.";
        //         self.error_at_current(err_msg);
        //         return Err(ParseError::new(err_msg));
        //     }
        //     Err(parse_error) => {
        //         // I don't think I need this:
        //         // self.error_at_current("Error parsing event handler message.");
        //         //return Err(parse_error);
        //         let sync_tokens = vec![TokenType::Caret];
        //         if !self.synchronize(&sync_tokens) {
        //             return Err(parse_error);
        //         }
        //     }
        // }
        // Parse and validate the event message
        let (message_type, msg, line_number) = self.parse_event_message()?;
        // Set up event symbol and scope
        let is_declaring_event = self.setup_event_handler_scope(&msg)?;

        // Parse event handler parameters and handle scoping
        let pop_params_scope = self.parse_event_handler_parameters(&msg, is_declaring_event)?;

        // Consume closing paren and handle return type
        self.consume_rparen_with_sync()?;
        self.parse_event_handler_return_type(&msg, is_declaring_event)?;
        
        // Parse default return value for event handler: = value
        let event_handler_return_init_expr_opt = self.parse_event_handler_return_init()?;

        let _ = self.consume(TokenType::OpenBrace, "Expected '{'");

        // Set up local scope and parse body
        self.enter_event_handler_local_scope()?;
        
        let event_symbol_rcref = self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();
        self.current_event_symbol_opt = Some(event_symbol_rcref);

        let statements = self.statements(IdentifierDeclScope::EventHandlerVarScope);
        let event_symbol_rcref = self.arcanum.get_event(&msg, &self.state_name_opt).unwrap();
        let ret_event_symbol_rcref = Rc::clone(&event_symbol_rcref);
        // TODO v.20: update sync for new syntax
        // let terminator_node = match self.event_handler_terminator(event_symbol_rcref) {
        //     Ok(terminator_node) => terminator_node,
        //     Err(_parse_error) => {
        //         // TODO: this vec keeps the parser from hanging. don't know why
        //         let sync_tokens = vec![
        //             TokenType::Pipe,
        //             TokenType::State,
        //             TokenType::ActionsBlock,
        //             TokenType::DomainBlock,
        //             TokenType::CloseBrace,
        //         ];
        //         self.synchronize(&sync_tokens);
        //         // create "dummy" node to keep processing
        //         // TODO: 1) make line # an int so as to set it to -1 when it is a dummy node and 2) confirm this is the best way
        //         // to keep going
        //         TerminatorExpr::new(TerminatorType::Return, None, 0)
        //     }
        // };

        // Parse optional terminator
        let terminator_node_opt = self.parse_event_handler_terminator()?;

        let _ = self.consume(TokenType::CloseBrace, "Expected '}'");

        // The state name must be set in an enclosing context. Otherwise fail
        // with extreme prejudice.

        let st_name = match &self.state_name_opt {
            Some(state_name) => state_name.clone(),
            None => {
                return Err(ParseError::new(&format!("[line {}] Fatal error - event handler {} missing enclosing state context. Please notify bugs@frame-lang.org.", line_number, msg)));
            }
        };

        // TODO: Moved this down here as I think is a bug to hve it above but not sure.
        self.arcanum.exit_scope(); // event handler local block scope (EventHandlerLocalScopeSymbol)
        if pop_params_scope {
            self.arcanum.exit_scope(); // event handler params scope (EventHandlerParamsScopeSymbol)
        }
        self.arcanum.exit_scope(); // event handler lscope (EventHandlerScopeSymbol)

        if self.panic_mode {
            return Err(ParseError::new("TODO"));
        }

        self.current_event_symbol_opt = None;

        // Auto-add return terminator if none provided and last statement isn't already a return
        let final_terminator_opt = match terminator_node_opt {
            Some(terminator) => Some(terminator),
            None => {
                // Check if last statement is already a return statement
                let needs_return = match statements.last() {
                    Some(DeclOrStmtType::StmtT { stmt_t }) => {
                        !matches!(stmt_t, StatementType::ReturnStmt { .. })
                    },
                    Some(DeclOrStmtType::VarDeclT { .. }) => true, // Variable declaration, need return
                    None => true, // Empty handler needs return
                };
                
                if needs_return {
                    Some(TerminatorExpr::new(TerminatorType::Return, None, line_number))
                } else {
                    None // Last statement is already a return, don't add another
                }
            }
        };

        Ok(Some(EventHandlerNode::new(
            st_name,
            message_type,
            statements,
            final_terminator_opt,
            ret_event_symbol_rcref,
            self.event_handler_has_transition,
            line_number,
            event_handler_return_init_expr_opt,
            is_async,
        )))
    }

    /* --------------------------------------------------------------------- */

    // event_handler_terminator -> '^' | '>'

    // TODO: - explore just returning the TerminatorType
    fn event_handler_terminator(
        &mut self,
        _: Rc<RefCell<EventSymbol>>,
    ) -> Result<TerminatorExpr, ParseError> {
        // let x = event_symbol_rcfef.borrow();
        // let ret_type = match &x.ret_type_opt {
        //     Some(ret_type) => ret_type.clone(),
        //     None => None,
        // };

        // REMOVED: Old caret return syntax (^ and ^(value))
        // Now just return None since Caret token is removed
        Ok(TerminatorExpr::new(Return, None, self.previous().line))
    }

    /* --------------------------------------------------------------------- */

    // statements ->

    // TODO: need result and optional
    #[allow(clippy::vec_init_then_push)] // false positive in 1.51, fixed by 1.55
    fn statements(&mut self, identifier_decl_scope: IdentifierDeclScope) -> Vec<DeclOrStmtType> {
        let mut statements = Vec::new();
        let mut is_err = false;

        // self.stmt_idx = 0;

        loop {
            self.stmt_idx = self.stmt_idx + 1;
            
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: statements() loop iteration {}, current token: {:?}", self.stmt_idx, self.peek());
            }

            match self.decl_or_stmt(identifier_decl_scope.clone()) {
                Ok(opt_smt) => match opt_smt {
                    Some(decl_or_statement) => {
                        match &decl_or_statement {
                            DeclOrStmtType::StmtT { stmt_t } => {
                                // Transitions or state changes must be the last statement in
                                // an event handler.
                                match stmt_t {
                                    StatementType::TransitionStmt { .. } => {
                                        statements.push(decl_or_statement);
                                        // Check for optional return statement after transition (v0.20)
                                        // Consume the return token if present but don't generate AST node
                                        // since transitions already terminate execution
                                        if self.check(TokenType::Return_) {
                                            self.advance(); // consume 'return' token
                                            // Don't add return statement to AST - transition already terminates
                                        }
                                        // Transition (with optional return) must be last in event handler
                                        return statements;
                                    }
                                    StatementType::ExpressionStmt { expr_stmt_t } => {
                                        if let ExprStmtType::CallChainStmtT {
                                            call_chain_literal_stmt_node,
                                        } = expr_stmt_t
                                        {
                                            match call_chain_literal_stmt_node
                                                .call_chain_literal_expr_node
                                                .call_chain
                                                .get(0)
                                            {
                                                Some(CallChainNodeType::InterfaceMethodCallT {
                                                    ..
                                                }) => {
                                                    // interface method call must be last statement.
                                                    // TODO!!! - add this back when scope issue is fixed with parse errors
                                                    // self.interface_method_called = true;
                                                    statements.push(decl_or_statement);

                                                    // TODO!!! - add this back when scope issue is fixed with parse errors
                                                    // return statements;
                                                }
                                                _ => {
                                                    statements.push(decl_or_statement);
                                                }
                                            }
                                        } else {
                                            statements.push(decl_or_statement);
                                        }
                                    }
                                    // StatementType::ChangeStateStmt { .. } => {
                                    //     statements.push(decl_or_statement);
                                    //     // state changes disallowed in actions
                                    //     if self.is_action_scope {
                                    //         self.error_at_current(
                                    //             "Transitions disallowed in actions.",
                                    //         );
                                    //         // is_err = true;
                                    //     }
                                    //     // must be last statement so return
                                    //     return statements;
                                    // }
                                    StatementType::LoopStmt { .. } => {
                                        statements.push(decl_or_statement);
                                    }
                                    StatementType::ContinueStmt { .. } => {
                                        if self.is_loop_scope {
                                            statements.push(decl_or_statement);
                                        } else {
                                            let err_msg =
                                                "'continue' statement is only allowed inside a loop.";
                                            self.error_at_current(err_msg);
                                            is_err = true;
                                        }
                                    }
                                    StatementType::BreakStmt { .. } => {
                                        if self.is_loop_scope {
                                            statements.push(decl_or_statement);
                                        } else {
                                            is_err = true;
                                        }
                                    }
                                    _ => {
                                        statements.push(decl_or_statement);
                                    }
                                }
                            }
                            _ => {
                                statements.push(decl_or_statement);
                            }
                        }
                    }
                    None => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                            eprintln!("DEBUG: statements() returning early - decl_or_stmt returned None");
                        }
                        return statements;
                    }
                },
                Err(_err) => {
                    is_err = true;
                }
            }

            if is_err {
                is_err = false;
                let sync_tokens = vec![
                    TokenType::Identifier,
                    TokenType::LParen,
                    // TokenType::Caret, // Removed - old return syntax
                    TokenType::GT,
                    TokenType::State,
                    TokenType::PipePipe,
                    TokenType::Dot,
                    TokenType::Colon,
                    TokenType::Pipe,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                // Concat contextual sync tokens.
                // TODO - removing this until I can confirm this isn't screwing up
                // scope management.
                //  sync_tokens.append(self.sync_tokens_from_error_context.as_mut());
                self.sync_tokens_from_error_context = Vec::new();
                self.synchronize(&sync_tokens);
            }
        }
    }

    /* --------------------------------------------------------------------- */

    fn decl_or_stmt(
        &mut self,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> Result<Option<DeclOrStmtType>, ParseError> {
        if self.match_token(&[TokenType::Var, TokenType::Const]) {
            // this is hardcoded and needs to be set based on context. specifically BlockVar
            match self.var_declaration(identifier_decl_scope) {
                Ok(var_decl_t_rc_ref) => {
                    return Ok(Some(DeclOrStmtType::VarDeclT {
                        var_decl_t_rcref: var_decl_t_rc_ref,
                    }));
                }
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }
        }

        match self.statement() {
            Ok(opt_smt) => match opt_smt {
                Some(stmt_t) => Ok(Some(DeclOrStmtType::StmtT { stmt_t })),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    /* --------------------------------------------------------------------- */

    // statement ->

    // Helper: Parse initial expression for statement processing
    fn parse_statement_expression(&mut self) -> Option<ExprType> {
        match self.expression() {
            Ok(Some(expr_t)) => Some(expr_t),
            Ok(None) => None,
            Err(_) => {
                let sync_tokens = vec![
                    TokenType::CloseBrace,
                    TokenType::Identifier,
                    TokenType::Pipe,
                    TokenType::State,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::CloseBrace,
                ];
                self.synchronize(&sync_tokens);
                None
            }
        }
    }

    // Helper: Parse control flow statements (if, for, while, etc.)
    fn parse_control_flow_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // v0.50: Add del statement support
        if self.match_token(&[TokenType::Del]) {
            return match self.del_statement() {
                Ok(Some(del_stmt_t)) => Ok(Some(del_stmt_t)),
                Ok(None) => Err(ParseError::new("Expected del statement")),
                Err(parse_error) => Err(parse_error),
            };
        }
        
        if self.match_token(&[TokenType::If]) {
            return match self.if_statement() {
                Ok(Some(if_stmt_t)) => Ok(Some(if_stmt_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Try]) {
            return match self.try_statement() {
                Ok(Some(try_stmt_t)) => Ok(Some(try_stmt_t)),
                Ok(None) => Err(ParseError::new("Expected try statement")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Raise]) {
            return match self.raise_statement() {
                Ok(Some(raise_stmt_t)) => Ok(Some(raise_stmt_t)),
                Ok(None) => Err(ParseError::new("Expected raise statement")),
                Err(parse_error) => Err(parse_error),
            };
        }

        // Check for 'async with' or 'with' statements
        if self.match_token(&[TokenType::Async]) {
            if self.match_token(&[TokenType::With]) {
                return match self.with_statement(true) {
                    Ok(Some(with_stmt_t)) => Ok(Some(with_stmt_t)),
                    Ok(None) => Err(ParseError::new("Expected async with statement")),
                    Err(parse_error) => Err(parse_error),
                };
            } else {
                // If we matched 'async' but not 'with', it's an error
                return Err(ParseError::new("Expected 'with' after 'async' keyword"));
            }
        }

        if self.match_token(&[TokenType::With]) {
            return match self.with_statement(false) {
                Ok(Some(with_stmt_t)) => Ok(Some(with_stmt_t)),
                Ok(None) => Err(ParseError::new("Expected with statement")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Match]) {
            return match self.match_statement() {
                Ok(Some(match_stmt_t)) => Ok(Some(match_stmt_t)),
                Ok(None) => Err(ParseError::new("Expected match statement")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::For]) {
            return match self.for_statement() {
                Ok(Some(for_stmt_t)) => Ok(Some(for_stmt_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::While]) {
            return match self.while_statement() {
                Ok(Some(while_stmt_t)) => Ok(Some(while_stmt_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Loop]) {
            // Emit deprecation warning
            eprintln!("Warning: 'loop' keyword is deprecated. Use 'while true' instead.");
            return match self.loop_statement_scope() {
                Ok(Some(loop_stmt_t)) => Ok(Some(loop_stmt_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::OpenBrace]) {
            let block_line = self.previous().line;  // v0.78.10: capture line for block mapping
            return match self.block_scope(block_line) {
                Ok(block_stmt_t) => Ok(Some(block_stmt_t)),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Continue]) {
            let line = self.previous().line;
            let continue_stmt_node = ContinueStmtNode::new(line);
            return Ok(Some(StatementType::ContinueStmt { continue_stmt_node }));
        }
        
        if self.match_token(&[TokenType::Break]) {
            let line = self.previous().line;
            let break_stmt_node = BreakStmtNode::new(line);
            return Ok(Some(StatementType::BreakStmt { break_stmt_node }));
        }
        
        // v0.46: Handle assert statements
        if self.match_token(&[TokenType::Assert]) {
            // Parse the condition expression
            let expr = match self.expression() {
                Ok(Some(expr)) => expr,
                Ok(None) => {
                    self.error_at_current("Expected expression after 'assert'");
                    return Err(ParseError::new("Expected expression after 'assert'"));
                }
                Err(e) => return Err(e),
            };
            
            // Create an assert statement node
            let assert_stmt_node = AssertStmtNode::new(self.previous().line, expr);
            let stmt_type = StatementType::AssertStmt { assert_stmt_node };
            return Ok(Some(stmt_type));
        }
        
        if self.match_token(&[TokenType::Return_]) {
            return self.parse_return_statement();
        }

        Ok(None)
    }

    // Helper: Parse return statement
    fn parse_return_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let line = self.previous().line;  // Get line from return token
        
        // Check if this is an incorrect return assignment: "return = expr"
        if self.check(TokenType::Equals) {
            // This is the deprecated "return = expr" syntax - provide helpful error
            let err_msg = "Syntax error: 'return = value' is not valid. Use 'system.return = value' to set the interface method return value, or use 'return value' for a regular return statement.";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Regular return statement: "return expr?"
        let expr_t_opt = match self.expression() {
            Ok(Some(expr_t)) => Some(expr_t),
            Ok(None) => None,
            Err(parse_error) => return Err(parse_error),
        };
        
        let return_stmt_node = ReturnStmtNode::new(line, expr_t_opt);
        Ok(Some(StatementType::ReturnStmt { return_stmt_node }))
    }

    // Helper: Convert expression to statement
    fn convert_expression_to_statement(&mut self, expr_t: ExprType) -> Result<Option<StatementType>, ParseError> {
        use ExprType::*;
        
        match expr_t {
            SystemInstanceExprT { system_instance_expr_node } => {
                let system_instance_stmt_node = SystemInstanceStmtNode::new(system_instance_expr_node.line, system_instance_expr_node);
                let expr_stmt_t = SystemInstanceStmtT { system_instance_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            SystemTypeExprT { system_type_expr_node } => {
                let system_type_stmt_node = SystemTypeStmtNode::new(system_type_expr_node.line, system_type_expr_node);
                let expr_stmt_t = SystemTypeStmtT { system_type_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            ListT { list_node } => {
                let list_stmt_node = ListStmtNode::new(self.previous().line, list_node);
                let expr_stmt_t = ListStmtT { list_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            DictLiteralT { .. } => {
                self.error_at_previous("Dictionary literal expressions not allowed as statements.");
                Err(ParseError::new("Dictionary literal must be part of an assignment or expression"))
            }
            SetLiteralT { .. } => {
                self.error_at_previous("Set literal expressions not allowed as statements.");
                Err(ParseError::new("Set literal must be part of an assignment or expression"))
            }
            TupleLiteralT { .. } => {
                self.error_at_previous("Tuple literal expressions not allowed as statements.");
                Err(ParseError::new("Tuple literal must be part of an assignment or expression"))
            }
            ExprListT { expr_list_node } => {
                // path for transitions **with** an exit params group
                if self.match_token(&[TokenType::Transition]) {
                    match self.transition(Some(expr_list_node)) {
                        Ok(transition_statement_node) => {
                            let statement_type = StatementType::TransitionStmt {
                                transition_statement_node,
                            };
                            Ok(Some(statement_type))
                        }
                        Err(parse_err) => Err(parse_err),
                    }
                } else {
                    // Just a group not associated with a transition.
                    let expr_list_stmt_node = ExprListStmtNode::new(self.previous().line, expr_list_node);
                    let expr_stmt_t = ExprListStmtT { expr_list_stmt_node };
                    Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
                }
            }
            CallExprT { call_expr_node } => {
                let line = call_expr_node.line;
                let call_stmt_node = CallStmtNode::new(line, call_expr_node);
                let expr_stmt_t = CallStmtT { call_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            CallExprListT { .. } => {
                // this should never happen as it is the () in a call like foo()
                Err(ParseError::new("TODO"))
            }
            VariableExprT { var_node } => {
                let line = var_node.line;
                let variable_stmt_node = VariableStmtNode::new(line, var_node);
                let expr_stmt_t = VariableStmtT { variable_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            ActionCallExprT { action_call_expr_node } => {
                let action_call_stmt_node = ActionCallStmtNode::new(self.previous().line, action_call_expr_node);
                let expr_stmt_t = ActionCallStmtT { action_call_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            CallChainExprT { call_chain_expr_node } => {
                let call_chain_literal_stmt_node = CallChainStmtNode::new(call_chain_expr_node.line, call_chain_expr_node);
                let expr_stmt_t = CallChainStmtT { call_chain_literal_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            StateStackOperationExprT { state_stack_op_node } => {
                let state_stack_operation_statement_node =
                    StateStackOperationStatementNode::new(state_stack_op_node);
                Ok(Some(StatementType::StateStackStmt {
                    state_stack_operation_statement_node,
                }))
            }
            AssignmentExprT { assignment_expr_node } => {
                let assignment_stmt_node = AssignmentStmtNode::new(assignment_expr_node.line, assignment_expr_node);
                let expr_stmt_t = AssignmentStmtT { assignment_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            EnumeratorExprT { enum_expr_node } => {
                let enumerator_stmt_node = EnumeratorStmtNode::new(enum_expr_node.line, enum_expr_node);
                let expr_stmt_t = EnumeratorStmtT { enumerator_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            BinaryExprT { binary_expr_node } => {
                let binary_stmt_node = BinaryStmtNode::new(binary_expr_node.line, binary_expr_node);
                let expr_stmt_t = BinaryStmtT { binary_stmt_node };
                Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }))
            }
            LiteralExprT { .. } => {
                self.error_at_previous("Literal statements not allowed.");
                Err(ParseError::new("TODO"))
            }
            TransitionExprT { transition_expr_node } => {
                let transition_statement_node =
                    TransitionStatementNode::new(transition_expr_node.line, transition_expr_node, None);
                Ok(Some(StatementType::TransitionStmt {
                    transition_statement_node,
                }))
            }
            FrameEventExprT { .. } => {
                self.error_at_previous("Frame Event statements not allowed.");
                Err(ParseError::new("TODO"))
            }
            UnaryExprT { .. } => {
                self.error_at_previous("Unary expression statements not allowed.");
                Err(ParseError::new("TODO"))
            }
            NilExprT => {
                self.error_at_current("Unexpected use of ExprType::NilExprT");
                return Err(ParseError::new("Unexpected ExprType::NilExprT"));
            }
            SelfExprT { .. } => {
                self.error_at_current("Unexpected use of ExprType::SelfExprT");
                return Err(ParseError::new("Unexpected ExprType::SelfExprT"));
            }
            DefaultLiteralValueForTypeExprT => {
                self.error_at_current("Unexpected use of ExprType::DefaultLiteralValueForTypeExprT");
                return Err(ParseError::new("Unexpected ExprType::DefaultLiteralValueForTypeExprT"));
            }
            UnpackExprT { .. } | DictUnpackExprT { .. } => {
                self.error_at_previous("Unpacking expressions not allowed as statements.");
                Err(ParseError::new("Unpacking must be part of an assignment or function call"))
            }
            ListComprehensionExprT { .. } => {
                self.error_at_previous("List comprehension expressions not allowed as statements.");
                Err(ParseError::new("List comprehension must be part of an assignment or expression"))
            }
            SetComprehensionExprT { .. } => {
                self.error_at_previous("Set comprehension expressions not allowed as statements.");
                Err(ParseError::new("Set comprehension must be part of an assignment or expression"))
            }
            AwaitExprT { await_expr_node } => {
                // Await expressions can be statements (like await some_async_call())
                let expr_list_node = ExprListNode::new(vec![AwaitExprT { await_expr_node }]);
                let stmt = StatementType::ExpressionStmt {
                    expr_stmt_t: ExprListStmtT {
                        expr_list_stmt_node: ExprListStmtNode::new(0, expr_list_node),
                    },
                };
                Ok(Some(stmt))
            }
            DictComprehensionExprT { .. } => {
                self.error_at_previous("Dictionary comprehension expressions not allowed as statements.");
                Err(ParseError::new("Dictionary comprehension must be part of an assignment or expression"))
            }
            LambdaExprT { .. } => {
                self.error_at_previous("Lambda expressions not allowed as statements.");
                Err(ParseError::new("Lambda expression must be part of an assignment or expression"))
            }
            FunctionRefT { .. } => {
                self.error_at_previous("Function reference expressions not allowed as statements.");
                Err(ParseError::new("Function reference must be part of an assignment or call"))
            }
            YieldExprT { yield_expr_node } => {
                // Yield expressions can be statements (like yield value)
                let expr_list_node = ExprListNode::new(vec![YieldExprT { yield_expr_node }]);
                let stmt = StatementType::ExpressionStmt {
                    expr_stmt_t: ExprListStmtT {
                        expr_list_stmt_node: ExprListStmtNode::new(0, expr_list_node),
                    },
                };
                Ok(Some(stmt))
            }
            YieldFromExprT { yield_from_expr_node } => {
                // Yield from expressions can be statements (like yield from iterator)
                let expr_list_node = ExprListNode::new(vec![YieldFromExprT { yield_from_expr_node }]);
                let stmt = StatementType::ExpressionStmt {
                    expr_stmt_t: ExprListStmtT {
                        expr_list_stmt_node: ExprListStmtNode::new(0, expr_list_node),
                    },
                };
                Ok(Some(stmt))
            }
            GeneratorExprT { .. } => {
                self.error_at_previous("Generator expressions not allowed as statements.");
                Err(ParseError::new("Generator expression must be part of an assignment or expression"))
            }
            StarExprT { .. } => {
                self.error_at_previous("Star expressions not allowed as statements.");
                Err(ParseError::new("Star expression must be part of an unpacking assignment"))
            }
            WalrusExprT { .. } => {
                self.error_at_previous("Walrus operator expressions not allowed as statements.");
                Err(ParseError::new("Walrus operator must be part of a larger expression"))
            }
        }
    }

    fn statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Due to Frame test and transition syntax, we need to get the first expression
        // and then see if it is an expression in the "first set" of expressions for tests
        // and transitions.
        let expr_t_opt = self.parse_statement_expression();

        // if there was an expression found, now see if it is valid to start a test.

        match expr_t_opt {
            Some(expr_t) => {
                // REMOVED: Ternary test syntax (?, ?!, ?~, ?#, ?:) has been deprecated
                // Use if/elif/else statements instead
                // The code below was removed as part of v0.31 cleanup
                /*
                if self.is_bool_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.bool_test(expr_t);
                    return match result {
                        Ok(bool_test_node) => {
                            let bool_test_t = TestType::BoolTest { bool_test_node };
                            let test_stmt_node = TestStatementNode::new(bool_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_string_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.string_match_test(expr_t);
                    return match result {
                        Ok(string_match_test_node) => {
                            let match_test_t = TestType::StringMatchTest {
                                string_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_number_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.number_match_test(expr_t);
                    return match result {
                        Ok(number_match_test_node) => {
                            let match_test_t = TestType::NumberMatchTest {
                                number_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_enum_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.enum_match_test(expr_t);
                    return match result {
                        Ok(enum_match_test_node) => {
                            let match_test_t = TestType::EnumMatchTest {
                                enum_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                }
                */

                // Not a test statement. Now see if we are at an expression statement.
                return self.convert_expression_to_statement(expr_t);
            }
            None => {
                // This path is for transitions w/o an exit params group
                if self.match_token(&[TokenType::Transition]) {
                    match self.transition(None) {
                        Ok(transition_statement_node) => {
                            let statement_type = StatementType::TransitionStmt {
                                transition_statement_node,
                            };
                            return Ok(Some(statement_type));
                        }
                        Err(parse_err) => return Err(parse_err),
                    }
                }
            }
        }
        
        // Check for => $^ (parent dispatch)
        if self.match_token(&[TokenType::Dispatch]) {
            if self.match_token(&[TokenType::ParentState]) {
                // Validate that we're in a hierarchical state
                if self.state_parent_opt.is_none() {
                    self.error_at_current("Cannot use '=> $^' parent dispatch in non-hierarchical state");
                    return Err(ParseError::new("Parent dispatch not allowed here"));
                }
                
                let target_state_ref_opt = self.state_parent_opt.as_ref().map(|parent_name| {
                    StateRefNode {
                        name: parent_name.clone()
                    }
                });
                let parent_dispatch_stmt_node = ParentDispatchStmtNode::new(target_state_ref_opt, self.previous().line);
                return Ok(Some(StatementType::ParentDispatchStmt {
                    parent_dispatch_stmt_node,
                }));
            } else {
                // Put the dispatch token back for error reporting
                self.current -= 1;
                let err_msg = "Expected '$^' after '=>' for parent dispatch";
                return Err(ParseError::new(err_msg));
            }
        }

        // if self.match_token(&[TokenType::ChangeState]) {
        //     return match self.change_state() {
        //         Ok(Some(state_context_t)) => Ok(Some(state_context_t)),
        //         Ok(None) => Err(ParseError::new("TODO")),
        //         Err(parse_error) => Err(parse_error),
        //     };
        // }


        // Check for control flow statements
        if let Some(stmt) = self.parse_control_flow_statement()? {
            return Ok(Some(stmt));
        }
        
        // if self.match_token(&[TokenType::OpenBrace]) {
        //     let break_stmt_node = BreakStmtNode::new();
        //     return Ok(Some(StatementType::BreakStmt { break_stmt_node }));
        // }
        // SuperString/backtick support removed

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    fn block_scope(&mut self, block_line: usize) -> Result<StatementType, ParseError> {
        let scope_name = &format!("block_scope_{}", self.stmt_idx);

        if self.is_building_symbol_table {
            let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
            self.arcanum
                .enter_scope(ParseScopeType::Block { block_scope_rcref });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                return Err(ParseError::new(&format!("Failed to set block scope '{}': {}", scope_name, err)));
            }
        }
        let ret = self.block(block_line);
        // exit block scope
        self.arcanum.exit_scope();
        ret
    }

    /* --------------------------------------------------------------------- */

    fn block(&mut self, block_line: usize) -> Result<StatementType, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);

        if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
            return Err(parse_error);
        }

        let block_stmt_node = BlockStmtNode::new(block_line, statements);
        let stmt_type = StatementType::BlockStmt { block_stmt_node };
        Ok(stmt_type)
    }

    /* --------------------------------------------------------------------- */

    // This method detects if an expression can be tested:
    // (a = 1) ? --- not testable
    // (a + b) ? --- not testable (TODO: review but think not
    // not sure about frame event and parts. for now yes.

    fn is_testable_expression(&self, expr_t: &ExprType) -> bool {
        match expr_t {
            AssignmentExprT { .. } => false,
            UnaryExprT { .. } => true,
            BinaryExprT { .. } => true,
            ExprListT { expr_list_node } => {
                if expr_list_node.exprs_t.len() != 1 {
                    return false;
                }

                let first_expr_t = expr_list_node.exprs_t.first().unwrap();
                self.is_testable_expression(first_expr_t)
            }
            _ => true,
        }
    }

    /* --------------------------------------------------------------------- */

    // REMOVED: Deprecated ternary test functions
    /*
    fn is_bool_test(&self) -> bool {
        self.peek().token_type == TokenType::BoolTestTrue
            || self.peek().token_type == TokenType::BoolTestFalse
    }
    */

    /* --------------------------------------------------------------------- */

    /*
    fn is_string_match_test(&self) -> bool {
        self.peek().token_type == TokenType::StringTest
    }
    */

    /* --------------------------------------------------------------------- */

    /*
    fn is_number_match_test(&self) -> bool {
        self.peek().token_type == TokenType::NumberTest
    }
    */

    /* --------------------------------------------------------------------- */

    /*
    fn is_enum_match_test(&self) -> bool {
        self.peek().token_type == TokenType::EnumTest
    }
    */

    /* --------------------------------------------------------------------- */

    // TODO
    // fn is_regex_test(&self) -> bool {
    //
    //     //panic!("not implemented")
    //     false
    // }

    /* --------------------------------------------------------------------- */

    // bool_test -> ('?' | '?!') bool_test_true_branch (':' bool_test_else_branch)? '::'

    fn bool_test(&mut self, _expr_t: ExprType) -> Result<BoolTestNode, ParseError> {
        let _is_negated: bool;

        // self.sync_tokens_from_error_context = vec![TokenType::ColonBar]; // Removed with ternary syntax

        // REMOVED: Deprecated ternary test syntax
        // '?' and '?!' tokens removed in v0.30
        return Err(ParseError::new("Ternary operators have been removed. Use if/elif/else statements instead."));

        // All code below is unreachable - ternary syntax removed
        /*
        let mut conditional_branches: Vec<BoolTestConditionalBranchNode> = Vec::new();

        let first_branch_node =
            match self.bool_test_conditional_branch_statements_scope(is_negated, expr_t) {
                Ok(branch_node) => branch_node,
                Err(parse_error) => return Err(parse_error),
            };

        conditional_branches.push(first_branch_node);

        // v0.30: Parent dispatch now handled by => $^ statements

        // (':' bool_test_else_branch)?
        let mut bool_test_else_node_opt: Option<BoolTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            bool_test_else_node_opt = Option::from(match self.bool_test_else_branch_scope() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // ':|'
        if let Err(parse_error) = self.consume(TokenType::ColonBar, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        Ok(BoolTestNode::new(
            conditional_branches,
            bool_test_else_node_opt,
        ))
        */
    }

    /* --------------------------------------------------------------------- */

    // bool_test_body -> statements* branch_terminator?

    fn bool_test_else_continue_branch(
        &mut self,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let _expr_t: ExprType;
        let result = self.expression();
        match result {
            Ok(expression_opt) => match expression_opt {
                Some(et) => {
                    _expr_t = et;
                }
                None => {
                    return Err(ParseError::new("TODO"));
                }
            },
            Err(parse_error) => return Err(parse_error),
        }

        let _is_negated: bool;

        // REMOVED: Deprecated ternary test syntax
        // '?' and '?!' tokens removed in v0.30
        let err_msg = "Ternary operators have been removed. Use if/elif/else statements instead.";
        self.error_at_current(&&err_msg);
        return Err(ParseError::new(err_msg));

        // Original code commented out:
        // self.bool_test_conditional_branch_statements_scope(is_negated, expr_t)
    }

    /* --------------------------------------------------------------------- */

    // bool_test_conditional_branch_statements -> statements* branch_terminator?

    fn bool_test_conditional_branch_statements_scope(
        &mut self,
        is_negated: bool,
        expr_t: ExprType,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let scope_name = &format!("bool_test_conditional_branch_{}", self.stmt_idx);
        if self.is_building_symbol_table {
            let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
            self.arcanum
                .enter_scope(ParseScopeType::Block { block_scope_rcref });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                return Err(ParseError::new(&format!("Failed to set bool test conditional branch scope '{}': {}", scope_name, err)));
            }
        }
        let ret = self.bool_test_conditional_branch_statements(is_negated, expr_t);
        // exit block scope
        self.arcanum.exit_scope();
        ret
    }

    /* --------------------------------------------------------------------- */

    // bool_test_conditional_branch_statements -> statements* branch_terminator?

    fn bool_test_conditional_branch_statements(
        &mut self,
        is_negated: bool,
        expr_t: ExprType,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_expr_opt) => Ok(BoolTestConditionalBranchNode::new(
                is_negated,
                expr_t,
                statements,
                branch_terminator_expr_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    fn bool_test_else_branch_scope(&mut self) -> Result<BoolTestElseBranchNode, ParseError> {
        let scope_name = &format!("bool_test_else_branch_scope_{}", self.stmt_idx);
        if self.is_building_symbol_table {
            let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
            self.arcanum
                .enter_scope(ParseScopeType::Block { block_scope_rcref });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                return Err(ParseError::new(&format!("Failed to set bool test else branch scope '{}': {}", scope_name, err)));
            }
        }
        let ret = self.bool_test_else_branch();
        // exit block scope
        self.arcanum.exit_scope();
        ret
    }

    /* --------------------------------------------------------------------- */

    // bool_test_else_branch -> statements* branch_terminator?

    fn bool_test_else_branch(&mut self) -> Result<BoolTestElseBranchNode, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_expr_opt) => Ok(BoolTestElseBranchNode::new(
                statements,
                branch_terminator_expr_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // branch_terminator -> '^' | ':>'

    // TODO: explore returning a TerminatorType rather than node
    fn branch_terminator(&mut self) -> Result<Option<TerminatorExpr>, ParseError> {
        // REMOVED: Old caret return syntax (^ and ^(value))
        // The caret syntax has been completely removed
        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // '^' '('
    //           return_expr -> expression ')'

    // fn return_expr(&mut self, expr_t:ExpressionType) -> Result<StringMatchTestNode,ParseError> {
    //
    // }

    /* --------------------------------------------------------------------- */

    // string_match_test -> '?~'  ('/' match_string ('|' match_string)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    // REMOVED: Deprecated ternary string match test function
    /*
    fn string_match_test(&mut self, expr_t: ExprType) -> Result<StringMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::StringTest, "Expected '?~'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<StringMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.string_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        // v0.30: Parent dispatch now handled by => $^ statements

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<StringMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.string_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) = self.consume(TokenType::ColonBar, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        Ok(StringMatchTestNode::new(
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }
    */

    /* --------------------------------------------------------------------- */

    // Match a string
    // string_match_test ->  ('~/' match_string ('|' match_string)* '/' (statement* branch_terminator?) ':>')+  '::'
    // Match an empty string
    // string_match_test ->  ('~//' (statement* branch_terminator?) ':>')+  '::'
    // Match null
    // string_match_test ->  ('!/!' (statement* branch_terminator?) ':>')+  '::'

    // REMOVED: string_match_test_match_branch - part of deprecated ternary syntax
    /*
    fn string_match_test_match_branch(
        &mut self,
    ) -> Result<StringMatchTestMatchBranchNode, ParseError> {
        let mut match_strings: Vec<String> = Vec::new();
        let string_match_t;

        if self.match_token(&[TokenType::StringMatchStart]) {
            // MatchString token contains any and all characters
            // scraped from the input stream until one of the match string
            // terminators '|' or '/' is found.
            if self.match_token(&[TokenType::MatchString]) {
                let match_string_tok = self.previous();
                let match_pattern_string = match_string_tok.lexeme.clone();
                match_strings.push(match_pattern_string);
            } else {
                return Err(ParseError::new("TODO"));
            }

            while self.match_token(&[TokenType::Pipe]) {
                if self.match_token(&[TokenType::MatchString]) {
                    let match_string_tok = self.previous();
                    let match_pattern_string = match_string_tok.lexeme.clone();
                    match_strings.push(match_pattern_string);
                } else {
                    return Err(ParseError::new("TODO"));
                }
            }

            let string_match_test_pattern_node = StringMatchTestPatternNode::new(match_strings);
            string_match_t = StringMatchType::MatchString {
                string_match_test_pattern_node,
            };

            if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
                return Err(parse_error);
            }
        } else if self.match_token(&[TokenType::MatchEmptyString]) {
            string_match_t = StringMatchType::MatchEmptyString;
        } else if self.match_token(&[TokenType::MatchNull]) {
            string_match_t = StringMatchType::MatchNullString;
        } else {
            let err_msg = &format!(
                "Expected string match '~/'  or null match '!/' token. Found '{}'.",
                self.current_token
            );
            self.error_at_current(err_msg.as_str());
            return Err(ParseError::new("TODO"));
        }

        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();

        match result {
            Ok(branch_terminator_t_opt) => Ok(StringMatchTestMatchBranchNode::new(
                string_match_t,
                statements,
                branch_terminator_t_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }
    */

    /* --------------------------------------------------------------------- */

    // match_test_else_branch -> statements* branch_terminator?

    fn string_match_test_else_branch(
        &mut self,
    ) -> Result<StringMatchTestElseBranchNode, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(StringMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // Filter and repackage expressions for the correct types in the context of
    // a list element e.g. x[0], zoo["lion"], bar[foo()] etc.

    fn return_assign_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        match self.expression() {
            Ok(Some(expr_t)) => {
                match expr_t {
                    // Matches a valid expression for list element e.g x[0]
                    ExprType::LiteralExprT { literal_expr_node } => {
                        Ok(Some(ExprType::LiteralExprT { literal_expr_node }))
                    }
                    ExprType::CallChainExprT {
                        call_chain_expr_node,
                    } => Ok(Some(ExprType::CallChainExprT {
                        call_chain_expr_node,
                    })),
                    ExprType::BinaryExprT { binary_expr_node } => {
                        Ok(Some(ExprType::BinaryExprT { binary_expr_node }))
                    }
                    ExprType::ActionCallExprT {
                        action_call_expr_node,
                    } => Ok(Some(ExprType::ActionCallExprT {
                        action_call_expr_node,
                    })),
                    ExprType::CallExprT { call_expr_node } => {
                        Ok(Some(ExprType::CallExprT { call_expr_node }))
                    }
                    ExprType::VariableExprT { var_node } => {
                        Ok(Some(ExprType::VariableExprT { var_node }))
                    }
                    ExprType::FrameEventExprT { frame_event_part } => {
                        Ok(Some(ExprType::FrameEventExprT { frame_event_part }))
                    }
                    ExprType::ExprListT { expr_list_node } => {
                        Ok(Some(ExprType::ExprListT { expr_list_node }))
                    }
                    ExprType::ListT { list_node } => {
                        Ok(Some(ExprType::ListT { list_node }))
                    }
                    ExprType::DictLiteralT { dict_literal_node } => {
                        Ok(Some(ExprType::DictLiteralT { dict_literal_node }))
                    }
                    ExprType::SetLiteralT { set_literal_node } => {
                        Ok(Some(ExprType::SetLiteralT { set_literal_node }))
                    }
                    ExprType::TupleLiteralT { tuple_literal_node } => {
                        Ok(Some(ExprType::TupleLiteralT { tuple_literal_node }))
                    }
                    _ => {
                        // Log error but pass expression through to complete parse.
                        // TODO: be more specific about the id of the list identifier.
                        let msg = &format!("Error - invalid expression type for return assigment.");
                        self.error_at_current(msg);
                        Ok(Some(expr_t))
                    }
                }
            }
            Ok(None) => {
                return Ok(None);
            }
            Err(err) => return Err(err),
        }
    }

    /* --------------------------------------------------------------------- */

    // Filter and repackage expressions for the correct types in the context of
    // a list element e.g. x[0], zoo["lion"], bar[foo()] etc.

    fn list_elem_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        match self.expression() {
            Ok(Some(expr_t)) => {
                match expr_t {
                    // Matches a valid expression for list element e.g x[0]
                    ExprType::LiteralExprT { literal_expr_node } => {
                        Ok(Some(ExprType::LiteralExprT { literal_expr_node }))
                    }
                    ExprType::CallChainExprT {
                        call_chain_expr_node,
                    } => Ok(Some(ExprType::CallChainExprT {
                        call_chain_expr_node,
                    })),
                    ExprType::BinaryExprT { binary_expr_node } => {
                        Ok(Some(ExprType::BinaryExprT { binary_expr_node }))
                    }
                    ExprType::ActionCallExprT {
                        action_call_expr_node,
                    } => Ok(Some(ExprType::ActionCallExprT {
                        action_call_expr_node,
                    })),
                    ExprType::CallExprT { call_expr_node } => {
                        Ok(Some(ExprType::CallExprT { call_expr_node }))
                    }
                    ExprType::VariableExprT { var_node } => {
                        Ok(Some(ExprType::VariableExprT { var_node }))
                    }
                    ExprType::FrameEventExprT { frame_event_part } => {
                        Ok(Some(ExprType::FrameEventExprT { frame_event_part }))
                    }
                    _ => {
                        // Log error but pass expression through to complete parse.
                        // TODO: be more specific about the id of the list identifier.
                        let msg = &format!("Error - invalid expression type for list element.");
                        self.error_at_current(msg);
                        Ok(Some(expr_t))
                    }
                }
            }
            Ok(None) => {
                return Ok(None);
            }
            Err(err) => return Err(err),
        }
    }

    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        self.assignment()
    }

    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn parse_lambda(&mut self) -> Result<Option<ExprType>, ParseError> {
        // Parse parameters (optional)
        let mut params = Vec::new();
        
        // Check if we have parameters before the colon
        if !self.check(TokenType::Colon) {
            loop {
                if self.check(TokenType::Identifier) {
                    let param_token = self.advance();
                    params.push(param_token.lexeme.clone());
                    
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        
        // Consume the colon
        self.consume(TokenType::Colon, "Expected ':' after lambda parameters")?;
        
        // Parse the body expression - use assignment() to allow nested lambdas
        let body_result = self.assignment()?;
        if let Some(body) = body_result {
            let lambda_expr_node = LambdaExprNode::new(self.previous().line, params, body);
            return Ok(Some(LambdaExprT { lambda_expr_node }));
        } else {
            return Err(ParseError::new("Expected expression as lambda body"));
        }
    }

    fn assignment(&mut self) -> Result<Option<ExprType>, ParseError> {
        self.assignment_or_lambda()
    }
    
    fn assignment_or_lambda(&mut self) -> Result<Option<ExprType>, ParseError> {
        // Check for lambda first (lowest precedence except for assignment)
        if self.match_token(&[TokenType::Lambda]) {
            return self.parse_lambda();
        }
        
        let mut l_value = match self.walrus() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        // v0.52: Check for multiple assignment targets (x, y, z = ...)
        let mut l_values = Vec::new();
        let mut is_multiple = false;
        let saved_l_value;  // Save the original l_value
        
        // v0.53: Only process comma-separated values as tuple/multiple assignment if not inside collection
        // Check if we have comma-separated targets
        if self.peek().token_type == TokenType::Comma && !self.is_parsing_collection {
            is_multiple = true;
            saved_l_value = l_value;  // Move l_value to saved
            l_values.push(saved_l_value);
            
            while self.match_token(&[TokenType::Comma]) {
                // Parse the next target
                match self.logical_or() {
                    Ok(Some(expr_type)) => {
                        l_values.push(expr_type);
                    }
                    Ok(None) => {
                        self.error_at_current("Expected expression after ',' in expression list");
                        return Err(ParseError::new("Expected expression after ','"));
                    }
                    Err(parse_error) => return Err(parse_error),
                }
            }
            
            // Check if this is an assignment or just a tuple/expression list
            if self.peek().token_type != TokenType::Equals {
                // Not an assignment - return as tuple
                return Ok(Some(ExprType::TupleLiteralT {
                    tuple_literal_node: TupleLiteralNode::new(self.previous().line, l_values),
                }));
            }
            
            // It's an assignment - create a dummy l_value for the assignment node
            let line = self.previous().line;
            l_value = ExprType::VariableExprT {
                var_node: VariableNode::new(
                    line,
                    IdentifierNode::new(
                        Token::new(TokenType::Identifier, "_multi".to_string(), TokenLiteral::None, line, 0, 6),
                        None,
                        IdentifierDeclScope::UnknownScope,
                        false,
                        line,
                    ),
                    IdentifierDeclScope::UnknownScope,
                    None,
                )
            };
        }

        // Check for assignment operators (including compound assignments)
        let assignment_op = if self.match_token(&[TokenType::Equals]) {
            Some(AssignmentOperator::Equals)
        } else if self.match_token(&[TokenType::PlusEqual]) {
            Some(AssignmentOperator::PlusEquals)
        } else if self.match_token(&[TokenType::DashEqual]) {
            Some(AssignmentOperator::MinusEquals)
        } else if self.match_token(&[TokenType::StarEqual]) {
            Some(AssignmentOperator::StarEquals)
        } else if self.match_token(&[TokenType::SlashEqual]) {
            Some(AssignmentOperator::SlashEquals)
        } else if self.match_token(&[TokenType::FloorDivideEqual]) {
            Some(AssignmentOperator::FloorDivideEquals)
        } else if self.match_token(&[TokenType::PercentEqual]) {
            Some(AssignmentOperator::PercentEquals)
        } else if self.match_token(&[TokenType::StarStarEqual]) {
            Some(AssignmentOperator::PowerEquals)
        } else if self.match_token(&[TokenType::AmpersandEqual]) {
            Some(AssignmentOperator::AndEquals)
        } else if self.match_token(&[TokenType::PipeEqual]) {
            Some(AssignmentOperator::OrEquals)
        } else if self.match_token(&[TokenType::LeftShiftEqual]) {
            Some(AssignmentOperator::LeftShiftEquals)
        } else if self.match_token(&[TokenType::RightShiftEqual]) {
            Some(AssignmentOperator::RightShiftEquals)
        } else if self.match_token(&[TokenType::CaretEqual]) {
            Some(AssignmentOperator::XorEquals)
        } else if self.match_token(&[TokenType::AtEqual]) {
            Some(AssignmentOperator::MatMulEquals)
        } else {
            None
        };
        
        if let Some(op) = assignment_op {
            // this changes the tokens generated for expression lists
            // like (a) and (a,b,c)
            self.is_parsing_rhs = true;

            let line = self.previous().line;
            // Check for lambda in assignment RHS
            // v0.52: Parse right-hand side, which might also be comma-separated
            let r_value = if self.match_token(&[TokenType::Lambda]) {
                match self.parse_lambda() {
                    Ok(Some(expr_type)) => {
                        self.is_parsing_rhs = false;
                        expr_type
                    }
                    Ok(None) => {
                        self.is_parsing_rhs = false;
                        return Ok(None);
                    }
                    Err(parse_error) => {
                        self.is_parsing_rhs = false;
                        return Err(parse_error);
                    }
                }
            } else {
                // Parse first expression on right side
                let first_rhs = match self.logical_or() {
                    Ok(Some(expr_type)) => expr_type,
                    Ok(None) => {
                        self.is_parsing_rhs = false;
                        return Ok(None);
                    }
                    Err(parse_error) => {
                        self.is_parsing_rhs = false;
                        return Err(parse_error);
                    }
                };
                
                // v0.53: Only wrap comma-separated values in tuple if not inside collection
                // Check if right side is also comma-separated (for tuple on RHS)
                if self.peek().token_type == TokenType::Comma && !self.is_parsing_collection {
                    let mut rhs_values = vec![first_rhs];
                    while self.match_token(&[TokenType::Comma]) {
                        match self.logical_or() {
                            Ok(Some(expr_type)) => {
                                rhs_values.push(expr_type);
                            }
                            Ok(None) => {
                                self.error_at_current("Expected expression after ',' in RHS");
                                self.is_parsing_rhs = false;
                                return Err(ParseError::new("Expected expression after ',' in RHS"));
                            }
                            Err(parse_error) => {
                                self.is_parsing_rhs = false;
                                return Err(parse_error);
                            }
                        }
                    }
                    self.is_parsing_rhs = false;
                    // Return as tuple for multiple RHS values
                    ExprType::TupleLiteralT {
                        tuple_literal_node: TupleLiteralNode::new(self.previous().line, rhs_values),
                    }
                } else {
                    self.is_parsing_rhs = false;
                    first_rhs
                }
            };

            let r_value_rc = Rc::new(r_value);

            // v0.52: Skip assign() for multiple assignment - handled differently
            if !is_multiple {
                match self.assign(&mut l_value, r_value_rc.clone()) {
                    Ok(()) => {}
                    Err(..) => {
                        // grammar is correct and error already logged. Continue
                    }
                }
            }

            if !r_value_rc.as_ref().is_valid_assignment_rvalue_expr_type() {
                let err_msg = &format!(
                    "rvalue expr type '{}' is not a valid assignment expression type.",
                    r_value_rc.as_ref().expr_type_name()
                );
                self.error_at_current(err_msg);
            }

            // v0.52: Create multiple assignment node if we have multiple targets
            let assignment_expr_node = if is_multiple {
                // Multiple assignment only supports simple equals
                if op != AssignmentOperator::Equals {
                    self.error_at_current("Multiple assignment only supports '=' operator");
                    return Err(ParseError::new("Multiple assignment only supports '=' operator"));
                }
                AssignmentExprNode::new_multiple(l_values, r_value_rc.clone(), line)
            } else if op == AssignmentOperator::Equals {
                AssignmentExprNode::new(l_value, r_value_rc.clone(), line)
            } else {
                AssignmentExprNode::new_with_op(l_value, r_value_rc.clone(), op, line)
            };
            return Ok(Some(AssignmentExprT {
                assignment_expr_node,
            }));
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn equality(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.bitwise_or() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual, TokenType::In, TokenType::Not, TokenType::Is]) {
            //           let line = self.previous().line;
            let operator_token = self.previous();
            let op_type = if operator_token.token_type == TokenType::Not {
                // Check if the next token is 'in' for 'not in' operator
                if self.check(TokenType::In) {
                    self.advance(); // consume the 'in' token
                    OperatorType::NotIn
                } else {
                    // It's a regular 'not' - this shouldn't happen in equality context
                    return Err(ParseError::new("Unexpected 'not' in comparison context"));
                }
            } else if operator_token.token_type == TokenType::Is {
                // Check if the next token is 'not' for 'is not' operator
                if self.check(TokenType::Not) {
                    self.advance(); // consume the 'not' token
                    OperatorType::IsNot
                } else {
                    OperatorType::Is
                }
            } else {
                self.get_operator_type(&operator_token.clone())
            };
            let r_value = match self.bitwise_or() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            // if !r_value.is_valid_binary_expr_type() {
            //     let err_msg = "rvalue expr is not a valid binary expression type.";
            //     self.error_at_current(err_msg);
            // }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn bitwise_or(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.bitwise_xor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Pipe]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.bitwise_xor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn bitwise_xor(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.bitwise_and() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Caret]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.bitwise_and() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }
        
        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn bitwise_and(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.comparison() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Ampersand]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.comparison() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn comparison(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.bitwise_shift() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[
            TokenType::GT,
            TokenType::GreaterEqual,
            TokenType::LT,
            TokenType::LessEqual,
        ]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.bitwise_shift() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn bitwise_shift(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.term() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::LeftShift, TokenType::RightShift]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.term() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn term(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.factor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Dash, TokenType::Plus, TokenType::Percent]) {
            let operator_token = self.previous().clone();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.factor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => {
                    let err_msg = format!(
                        "Expected binary expression. Found \"{} {}\".",
                        l_value.to_string(),
                        operator_token.lexeme
                    );
                    self.error_at_current(&err_msg);
                    let parse_error = ParseError::new(err_msg.as_str());
                    return Err(parse_error);
                }
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn factor(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.power() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::ForwardSlash, TokenType::Star, TokenType::FloorDivide, TokenType::At]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.power() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn power(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.unary_expression() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        // Right associative - use if instead of while
        if self.match_token(&[TokenType::StarStar]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            // Recursive call for right associativity
            let r_value = match self.power() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    // Walrus operator (:=) - assignment expression that returns value
    fn walrus(&mut self) -> Result<Option<ExprType>, ParseError> {
        // Check if we have an identifier followed by :=
        // This allows us to handle (n := expr) where n is a new variable
        if self.peek().token_type == TokenType::Identifier {
            let next_idx = self.current + 1;
            if next_idx < self.tokens.len() && self.tokens[next_idx].token_type == TokenType::Walrus {
                // This is a walrus expression with a simple identifier
                let id_token = self.advance().clone();
                self.advance(); // consume :=
                
                // Create a variable node for the identifier
                let id_node = IdentifierNode::new(
                    id_token.clone(),
                    None,
                    IdentifierDeclScope::UnknownScope,
                    false,
                    id_token.line,
                );
                let var_node = VariableNode::new(
                    id_node.line,
                    id_node,
                    IdentifierDeclScope::UnknownScope,
                    None,
                );
                let l_value = ExprType::VariableExprT { var_node };
                
                // Parse the right side
                let r_value = match self.logical_or() {
                    Ok(Some(expr_type)) => Rc::new(expr_type),
                    Ok(None) => {
                        self.error_at_current("Expected expression after ':=' operator");
                        return Err(ParseError::new("Expected expression after ':='"));
                    }
                    Err(parse_error) => return Err(parse_error),
                };
                
                // Create walrus assignment expression
                let assignment_expr = AssignmentExprNode::new(
                    l_value,
                    r_value,
                    self.previous().line,
                );
                
                // Return as walrus expression
                return Ok(Some(ExprType::WalrusExprT {
                    assignment_expr_node: assignment_expr,
                }));
            }
        }
        
        // Not a walrus expression, parse normally
        let l_value = match self.logical_or() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        // Check for walrus operator after complex expression (shouldn't happen in valid Python)
        if self.match_token(&[TokenType::Walrus]) {
            // Ensure l_value is a variable
            match &l_value {
                ExprType::VariableExprT { .. } => {
                    // Valid - proceed with walrus assignment
                }
                _ => {
                    self.error_at_current("Walrus operator (:=) requires a simple variable on the left side");
                    return Err(ParseError::new("Invalid walrus operator usage"));
                }
            }

            // Parse the right side (should not recursively call walrus)
            let r_value = match self.logical_or() {
                Ok(Some(expr_type)) => Rc::new(expr_type),
                Ok(None) => {
                    self.error_at_current("Expected expression after ':=' operator");
                    return Err(ParseError::new("Expected expression after ':='"));
                }
                Err(parse_error) => return Err(parse_error),
            };

            // Create walrus assignment expression
            let assignment_expr = AssignmentExprNode::new(
                l_value,
                r_value,
                self.previous().line,
            );

            // Return as walrus expression (assignment that yields its value)
            return Ok(Some(ExprType::WalrusExprT {
                assignment_expr_node: assignment_expr,
            }));
        }

        Ok(Some(l_value))
    }


    /* --------------------------------------------------------------------- */

    fn logical_or(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.logical_and() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Or]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.logical_and() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_and(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.equality() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::And]) {
            let operator_token = self.previous();
            let op_type = self.get_operator_type(&operator_token.clone());
            let r_value = match self.equality() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            if !l_value.is_valid_binary_expr_type() {
                let err_msg = "lvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }
            if !r_value.is_valid_binary_expr_type() {
                let err_msg = "rvalue expr is not a valid binary expression type.";
                self.error_at_current(err_msg);
            }

            let binary_expr_node = BinaryExprNode::new(self.previous().line, l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */
    //
    // fn decorated_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
    //     // match self.prefix_unary_expression() {
    //     match self.unary_expression() {
    //         Ok(Some(expr_t)) => {
    //             return Ok(Some(expr_t));
    //         }
    //         Ok(None) => match self.unary_expression() {
    //             Ok(Some(expr_t)) => return Ok(Some(expr_t)),
    //             Ok(None) => return Ok(None),
    //             Err(parse_err) => return Err(parse_err),
    //         },
    //         Err(parse_error) => return Err(parse_error),
    //     }
    // }
    //
    // /* --------------------------------------------------------------------- */
    //
    // fn prefix_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
    //     // if self.match_token(&[TokenType::PlusPlus]) {
    //     //     match self.unary_expression2() {
    //     //         Ok(Some(mut expr_t)) => {
    //     //             match expr_t {
    //     //                 ExprType::CallChainExprT {
    //     //                     ref mut call_chain_expr_node,
    //     //                 } => {
    //     //                     call_chain_expr_node.inc_dec = IncDecExpr::PreInc;
    //     //                 }
    //     //                 ExprType::LiteralExprT {
    //     //                     ref mut literal_expr_node,
    //     //                 } => {
    //     //                     literal_expr_node.inc_dec = IncDecExpr::PreInc;
    //     //                 }
    //     //                 // ExprType::ExprListT {ref mut expr_list_node} => {
    //     //                 //     expr_list_node.inc_dec = IncDecExpr::PreInc;
    //     //                 // }
    //     //                 ExprType::ExprListT {
    //     //                     ref mut expr_list_node,
    //     //                 } => {
    //     //                     //       expr_list_node.inc_dec = IncDecExpr::PreDec;
    //     //                     for expr_t in &mut expr_list_node.exprs_t {
    //     //                         match expr_t {
    //     //                             ExprType::CallChainExprT {
    //     //                                 ref mut call_chain_expr_node,
    //     //                             } => {
    //     //                                 call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                             }
    //     //                             ExprType::LiteralExprT {
    //     //                                 ref mut literal_expr_node,
    //     //                             } => {
    //     //                                 literal_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                             }
    //     //                             _ => {
    //     //                                 let err_msg = "Can not increment/decrement something that cannot be assigned.";
    //     //                                 self.error_at_current(err_msg);
    //     //                                 return Err(ParseError::new(err_msg));
    //     //                             }
    //     //                         }
    //     //                     }
    //     //                 }
    //     //                 _ => {}
    //     //             }
    //     //             return Ok(Some(expr_t));
    //     //         }
    //     //         Ok(None) => return Ok(None),
    //     //         Err(parse_err) => return Err(parse_err),
    //     //     }
    //     // } else if self.match_token(&[TokenType::DashDash]) {
    //     //     match self.unary_expression2() {
    //     //         Ok(Some(mut expr_t)) => {
    //     //             match expr_t {
    //     //                 ExprType::CallChainExprT {
    //     //                     ref mut call_chain_expr_node,
    //     //                 } => {
    //     //                     call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                 }
    //     //                 ExprType::LiteralExprT {
    //     //                     ref mut literal_expr_node,
    //     //                 } => {
    //     //                     literal_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                 }
    //     //                 ExprType::ExprListT {
    //     //                     ref mut expr_list_node,
    //     //                 } => {
    //     //                     //                          expr_list_node.inc_dec = IncDecExpr::PreDec;
    //     //                     for expr_t in &mut expr_list_node.exprs_t {
    //     //                         match expr_t {
    //     //                             ExprType::CallChainExprT {
    //     //                                 ref mut call_chain_expr_node,
    //     //                             } => {
    //     //                                 call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                             }
    //     //                             ExprType::LiteralExprT {
    //     //                                 ref mut literal_expr_node,
    //     //                             } => {
    //     //                                 literal_expr_node.inc_dec = IncDecExpr::PreDec;
    //     //                             }
    //     //                             _ => {
    //     //                                 let err_msg = "Can not increment/decrement something that cannot be assigned.";
    //     //                                 self.error_at_current(err_msg);
    //     //                                 return Err(ParseError::new(err_msg));
    //     //                             }
    //     //                         }
    //     //                     }
    //     //                 }
    //     //                 _ => {}
    //     //             }
    //     //             return Ok(Some(expr_t));
    //     //         }
    //     //         Ok(None) => return Ok(None),
    //     //         Err(parse_err) => return Err(parse_err),
    //     //     }
    //     // }
    //
    //     return self.postfix_unary_expression();
    // }
    //
    // /* --------------------------------------------------------------------- */
    //
    // fn postfix_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
    //     match self.unary_expression() {
    //         Ok(Some(CallChainExprT {
    //             call_chain_expr_node,
    //         })) => {
    //             let mut x = CallChainExprT {
    //                 call_chain_expr_node,
    //             };
    //             match self.post_inc_dec_expression(&mut x) {
    //                 Ok(_result) => {
    //                     return Ok(Some(x));
    //                 }
    //                 Err(err) => {
    //                     return Err(err);
    //                 }
    //             }
    //         }
    //         Ok(Some(expr_t)) => return Ok(Some(expr_t)),
    //         Err(parse_error) => return Err(parse_error),
    //         Ok(None) => return Ok(None),
    //     }
    // }

    /* --------------------------------------------------------------------- */

    // unary_expression -> TODO

    // Helper: Parse await expression
    fn parse_await_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::ExprType::AwaitExprT;
        
        let expr_result = self.unary_expression()?;
        if let Some(expr) = expr_result {
            let await_expr_node = AwaitExprNode::new(self.previous().line, expr);
            Ok(Some(AwaitExprT { await_expr_node }))
        } else {
            Err(ParseError::new("Expected expression after 'await'"))
        }
    }

    // v0.42: Parse yield expressions
    fn parse_yield_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::ExprType::{YieldExprT, YieldFromExprT};
        
        // Check for "yield from"
        if self.match_token(&[TokenType::From]) {
            // yield from expression
            let expr_result = self.unary_expression()?;
            if let Some(expr) = expr_result {
                let yield_from_expr_node = YieldFromExprNode::new(self.previous().line, expr);
                Ok(Some(YieldFromExprT { yield_from_expr_node }))
            } else {
                Err(ParseError::new("Expected expression after 'yield from'"))
            }
        } else {
            // regular yield or yield with value
            // Check if the next token could be the start of an expression
            // Common statement starts that aren't expressions: if, while, for, return, yield, etc.
            // Also check for block end
            let next_is_expr = !self.check(TokenType::If) 
                && !self.check(TokenType::While)
                && !self.check(TokenType::For)
                && !self.check(TokenType::Return_)
                && !self.check(TokenType::Yield)
                && !self.check(TokenType::CloseBrace)  // End of block
                && !self.is_at_end();
            
            let expr_result = if next_is_expr {
                self.assignment()?
            } else {
                None
            };
            
            let yield_expr_node = YieldExprNode::new(self.previous().line, expr_result);
            Ok(Some(YieldExprT { yield_expr_node }))
        }
    }

    // Helper: Parse unary operators (!, -)
    fn parse_unary_operator(&mut self) -> Result<Option<ExprType>, ParseError> {
        let token = self.previous();
        let mut operator_type = self.get_operator_type(&token.clone());
        if operator_type == OperatorType::Minus {
            // change this so the code gen doesn't have a space between the - and ID
            // -x rather than - x
            operator_type = OperatorType::Negated;
        }
        
        match self.unary_expression() {
            Ok(Some(x)) => {
                let unary_expr_node = UnaryExprNode::new(self.previous().line, operator_type, x);
                Ok(Some(UnaryExprT { unary_expr_node }))
            }
            Err(parse_error) => Err(parse_error),
            Ok(None) => Ok(None)
        }
    }

    // Helper: Parse special keywords (self, system, state, transition)
    fn parse_special_keywords(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::Transition]) {
            match self.transition_expr() {
                Ok(transition_expr_node) => {
                    return Ok(Some(TransitionExprT {
                        transition_expr_node,
                    }));
                }
                Err(parse_err) => return Err(parse_err),
            }
        } else if self.match_token(&[TokenType::Self_]) {
            // Frame v0.31: Handle explicit self.method() and self.variable syntax
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Found Self_ token in parse_special_keywords");
            }
            return self.parse_self_context();
        } else if self.match_token(&[TokenType::SystemReturn]) {
            // Frame v0.31: Handle system.return special variable
            return Ok(Some(self.create_system_return_variable()));
        } else if self.match_token(&[TokenType::System]) {
            // Bare 'system' keyword is not allowed - reserved for future use
            let err_msg = "The 'system' keyword is reserved. Only 'system.return' is currently supported for setting interface return values.";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        } else if self.match_token(&[TokenType::State]) {
            return self.parse_state_context();
        } else if self.match_token(&[TokenType::PipePipeLBracket]) {
            return self.parse_event_handler_param();
        } else if self.match_token(&[TokenType::PipePipeDot]) {
            return self.parse_event_handler_var();
        }

        Ok(None)
    }

    // Helper: Create system.return variable
    fn create_system_return_variable(&mut self) -> ExprType {
        let system_return_token = self.previous().clone();
        let line_number = system_return_token.line;
        
        let system_return_node = VariableNode::new(
            line_number,
            IdentifierNode::new(
                Token {
                    token_type: system_return_token.token_type,
                    lexeme: "system.return".to_string(),
                    literal: system_return_token.literal,
                    line: line_number,
                    start: system_return_token.start,
                    length: "system.return".len(),
                },
                None,
                IdentifierDeclScope::UnknownScope,
                false,
                line_number,
            ),
            IdentifierDeclScope::UnknownScope,
            None,
        );
        
        ExprType::VariableExprT { 
            var_node: system_return_node 
        }
    }

    // Helper: Parse state context ($[param] or $.var)
    fn parse_state_context(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::LBracket]) {
            if self.match_token(&[TokenType::Identifier]) {
                let id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::StateParamScope,
                    false,
                    self.previous().line,
                );
                let var_scope = id_node.scope.clone();
                let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);

                let var_node = VariableNode::new(id_node.line, id_node, var_scope, symbol_type_rcref_opt);
                if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                    return Err(parse_error);
                }
                Ok(Some(VariableExprT { var_node }))
            } else {
                self.error_at_current("Expected identifier.");
                Err(ParseError::new("TODO"))
            }
        } else if self.match_token(&[TokenType::Dot]) {
            if self.match_token(&[TokenType::Identifier]) {
                let id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::StateVarScope,
                    false,
                    self.previous().line,
                );
                let var_scope = id_node.scope.clone();
                let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
                let var_node = VariableNode::new(id_node.line, id_node, var_scope, symbol_type_rcref_opt);
                Ok(Some(VariableExprT { var_node }))
            } else {
                self.error_at_current("Expected identifier.");
                Err(ParseError::new("TODO"))
            }
        } else {
            self.error_at_current("Unexpected token.");
            Err(ParseError::new("TODO"))
        }
    }

    // Helper: Parse event handler parameter (||[param])
    fn parse_event_handler_param(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::Identifier]) {
            let id_node = IdentifierNode::new(
                self.previous().clone(),
                None,
                IdentifierDeclScope::EventHandlerParamScope,
                false,
                self.previous().line,
            );
            let var_scope = id_node.scope.clone();
            let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
            let var_node = VariableNode::new(id_node.line, id_node, var_scope, symbol_type_rcref_opt);
            
            if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                return Err(parse_error);
            }
            Ok(Some(VariableExprT { var_node }))
        } else {
            self.error_at_current("Expected identifier.");
            Err(ParseError::new("TODO"))
        }
    }

    // Helper: Parse event handler variable (||.var)
    fn parse_event_handler_var(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::Identifier]) {
            let id_tok = self.previous().clone();
            let id_node = IdentifierNode::new(
                id_tok,
                None,
                IdentifierDeclScope::EventHandlerVarScope,
                false,
                self.previous().line,
            );
            
            let var_scope = id_node.scope.clone();
            let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
            let var_node = VariableNode::new(id_node.line, id_node, var_scope, symbol_type_rcref_opt);
            Ok(Some(VariableExprT { var_node }))
        } else {
            self.error_at_current("Expected identifier.");
            Err(ParseError::new("TODO"))
        }
    }

    fn unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        // v0.42: Handle yield expressions
        if self.match_token(&[TokenType::Yield]) {
            return self.parse_yield_expression();
        }
        
        // v0.35: Handle await expressions
        if self.match_token(&[TokenType::Await]) {
            return self.parse_await_expression();
        }
        
        // Handle unary operators (!, -, ~)
        if self.match_token(&[TokenType::Not, TokenType::Dash, TokenType::Tilde]) {
            return self.parse_unary_operator();
        }

        // Check for nested groups or tuples
        // '(' ')' | '(' expr+ ')' | '(expr,)' | '(expr, expr, ...)'
        if self.match_token(&[TokenType::LParen]) {
            match self.expr_list_or_tuple() {
                Ok(Some(expr_type)) => return Ok(Some(expr_type)),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {
                    // v0.20: Allow empty expression lists '()' - treat as empty tuple
                    let _ = self.consume(TokenType::RParen, "Expected ')'");
                    // Return empty tuple for consistency with Python
                    return Ok(Some(TupleLiteralT {
                        tuple_literal_node: TupleLiteralNode::new(self.previous().line, Vec::new()),
                    }))
                }
            }
        }

        // Check for special keywords and transitions
        if let Some(special_expr) = self.parse_special_keywords()? {
            return Ok(Some(special_expr));
        }
        //
        //         // self.a
        //         if !self.match_token(&[TokenType::Identifier]) {
        //             let msg = &format!("Error - expected identifier.");
        //             self.error_at_current(msg);
        //             return Err(ParseError::new(msg));
        //         }
        //
        //         let identifier_name = self.previous().lexeme.clone();
        //
        //         if !self.is_building_symbol_table {
        //             if self.arcanum.lookup_system_symbol(&identifier_name).is_none() {
        //                 let err_msg = format!(
        //                     "Call to '{}' not found on self.",
        //                     identifier_name
        //                 );
        //                 self.error_at_current(&err_msg);
        //                 return Err(ParseError::new(&err_msg));
        //             }
        //         }
        //
        //         let call_chain_result = self.call(IdentifierDeclScope::UnknownScope);
        //
        //         match call_chain_result {
        //             Ok(call_chain_opt) => {
        //                 let system_type_expr_node =
        //                     SystemInstanceExprT::new(system_id_node, Box::new(call_chain_opt));
        //
        //                 return Ok(Some(SystemTypeExprT {
        //                     system_type_expr_node,
        //                 }));
        //             }
        //             Err(parse_err) => {
        //                 return Err(parse_err);
        //             }
        //         }
        //
        //     } else if self.match_token(&[TokenType::Identifier]) {
        //         // This is a call to a system Type. Only operations are accessible.
        //
        //         // TODO v.20: Need to fix #Foo as it is no longer valid syntax.
        //         // #Foo(...)  or #Foo.call(...)
        //         let system_id_node = IdentifierNode::new(
        //             self.previous().clone(),
        //             None,
        //             IdentifierDeclScope::UnknownScope,
        //             false,
        //             self.previous().line,
        //         );
        //
        //         if self.match_token(&[TokenType::Dot]) {
        //             // System static operation call
        //             // FooSystem.staticOperation()
        //
        //             if !self.match_token(&[TokenType::Identifier]) {
        //                 let msg = &format!("Error - expected identifier.");
        //                 self.error_at_current(msg);
        //                 return Err(ParseError::new(msg));
        //             }
        //
        //             let identifier_name = self.previous().lexeme.clone();
        //
        //             if !self.is_building_symbol_table {
        //                 if self.arcanum.lookup_operation(&identifier_name).is_none() {
        //                     let err_msg = format!(
        //                         "Call to '{}' not found on '{}' system.",
        //                         identifier_name, system_id_node
        //                     );
        //                     self.error_at_current(&err_msg);
        //                     return Err(ParseError::new(&err_msg));
        //                 }
        //             }
        //
        //             let call_chain_result = self.call(IdentifierDeclScope::UnknownScope);
        //
        //             match call_chain_result {
        //                 Ok(call_chain_opt) => {
        //                     let system_type_expr_node =
        //                         SystemTypeExprNode::new(system_id_node, Box::new(call_chain_opt));
        //
        //                     return Ok(Some(SystemTypeExprT {
        //                         system_type_expr_node,
        //                     }));
        //                 }
        //                 Err(parse_err) => {
        //                     return Err(parse_err);
        //                 }
        //             }
        //         } else if let Err(parse_error) = self.consume(TokenType::LParen, "Expected '('.") {
        //             return Err(parse_error);
        //         }
        //         let (system_start_state_args, start_enter_args, domain_args) =
        //             match self.system_arguments() {
        //                 Ok((system_start_state_args, start_enter_args, domain_args)) => {
        //                     (system_start_state_args, start_enter_args, domain_args)
        //                 }
        //                 Err(parse_err) => {
        //                     return Err(parse_err);
        //                 }
        //             };
        //
        //         let system_instance_expr_node = SystemInstanceExprNode::new(
        //             system_id_node,
        //             system_start_state_args,
        //             start_enter_args,
        //             domain_args,
        //         );
        //
        //         return Ok(Some(SystemInstanceExprT {
        //             system_instance_expr_node,
        //         }));
        //     } else {
        //
        //         // self reference
        //         let self_expr_node= SelfExprNode::new();
        //         return Ok(Some(SelfExprT {self_expr_node}));
        //     }



            // TODO 23/SEP/10 - I *think* this is dead code as all tests pass.            // else {
            //     // System reference
            //     //               scope = IdentifierDeclScope::System;
            //     let id_node = IdentifierNode::new(
            //         self.previous().clone(),
            //         None,
            //         IdentifierDeclScope::System,
            //         false,
            //         self.previous().line,
            //     );
            //     let var_scope = id_node.scope.clone();
            //     let var_node = VariableNode::new(id_node.line, id_node, var_scope, None);
            //     return Ok(Some(VariableExprT { var_node }));
            // }


        // @TODO need to determine if this is the best way to
        // deal w/ references. We can basically put & in front
        // of a wide range of syntax it doesn't apply to.
        let mut is_reference = false;
        if self.match_token(&[TokenType::Ampersand]) {
            is_reference = true;
        }

        // Handle cls keyword specially (for class methods)
        if self.match_token(&[TokenType::Cls]) {
            // cls is the first parameter in class methods
            // Treat it like a regular identifier and let the visitor handle the validation
            match self.call(IdentifierDeclScope::UnknownScope) {
                Ok(Some(VariableExprT { mut var_node })) => {
                    var_node.id_node.is_reference = is_reference;
                    return Ok(Some(VariableExprT { var_node }));
                }
                Ok(Some(CallExprT {
                    call_expr_node: method_call_expr_node,
                })) => {
                    return Ok(Some(CallExprT {
                        call_expr_node: method_call_expr_node,
                    }))
                }
                Ok(Some(CallChainExprT {
                    mut call_chain_expr_node,
                })) => {
                    // set the is_reference on first variable in the call chain
                    let call_chain_first_node_opt = call_chain_expr_node.call_chain.get_mut(0);
                    if let Some(call_chain_first_node) = call_chain_first_node_opt {
                        call_chain_first_node.setIsReference(is_reference);
                    }

                    let x = CallChainExprT {
                        call_chain_expr_node,
                    };
                    return Ok(Some(x));
                }
                _ => {}
            }
        }
        
        // Handle super keyword specially
        if self.match_token(&[TokenType::Super]) {
            // super can only be used in methods within a class that extends another class
            // For now, treat it like a regular identifier and let the visitor handle the validation
            match self.call(IdentifierDeclScope::UnknownScope) {
                Ok(Some(VariableExprT { mut var_node })) => {
                    var_node.id_node.is_reference = is_reference;
                    return Ok(Some(VariableExprT { var_node }));
                }
                Ok(Some(CallExprT {
                    call_expr_node: method_call_expr_node,
                })) => {
                    return Ok(Some(CallExprT {
                        call_expr_node: method_call_expr_node,
                    }))
                }
                Ok(Some(CallChainExprT {
                    mut call_chain_expr_node,
                })) => {
                    // set the is_reference on first variable in the call chain
                    let call_chain_first_node_opt = call_chain_expr_node.call_chain.get_mut(0);
                    if let Some(call_chain_first_node) = call_chain_first_node_opt {
                        call_chain_first_node.setIsReference(is_reference);
                    }

                    let x = CallChainExprT {
                        call_chain_expr_node,
                    };
                    return Ok(Some(x));
                }
                _ => {}
            }
        }
        
        // TODO: I think only identifier is allowed?
        if self.match_token(&[TokenType::Identifier]) {
            // let debug_is_building_symbol_table = self.is_building_symbol_table;
            // let debug_current_token = self.current_token.clone();
            // let debug_processed_tokens = self.processed_tokens.clone();
            // let x = self.arcanum.lookup_system_symbol()
            match self.call(IdentifierDeclScope::UnknownScope) {
                Ok(Some(VariableExprT { mut var_node })) => {
                    var_node.id_node.is_reference = is_reference;
                    return Ok(Some(VariableExprT { var_node }));
                }
                Ok(Some(CallExprT {
                    call_expr_node: method_call_expr_node,
                })) => {
                    return Ok(Some(CallExprT {
                        call_expr_node: method_call_expr_node,
                    }))
                }
                Ok(Some(ActionCallExprT {
                    action_call_expr_node,
                })) => {
                    return Ok(Some(ActionCallExprT {
                        action_call_expr_node,
                    }))
                }
                Ok(Some(CallChainExprT {
                    mut call_chain_expr_node,
                })) => {
                    // set the is_reference on first variable in the call chain
                    let call_chain_first_node_opt = call_chain_expr_node.call_chain.get_mut(0);
                    if let Some(call_chain_first_node) = call_chain_first_node_opt {
                        call_chain_first_node.setIsReference(is_reference);
                    }

                    let x = CallChainExprT {
                        call_chain_expr_node,
                    };
                    // self.post_inc_dec_expression(&mut x);
                    return Ok(Some(x));
                    // return Ok(Some(CallChainLiteralExprT {
                    //     call_chain_expr_node,
                    // }));
                }
                Ok(Some(EnumeratorExprT { enum_expr_node })) => {
                    return Ok(Some(EnumeratorExprT { enum_expr_node }))
                }
                Ok(Some(FunctionRefT { name })) => {
                    // v0.38: Function reference as value (first-class function)
                    return Ok(Some(FunctionRefT { name }))
                }
                Ok(Some(_)) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        // number | string | bool | null | nil
        match self.literal_expr() {
            Ok(Some(mut literal_expr_node)) => {
                literal_expr_node.is_reference = is_reference;
                
                // Check if there's a dot after the literal for method calls like "string".upper()
                if self.peek().token_type == TokenType::Dot {
                    // Create a temporary variable to hold the literal value
                    // This allows us to call methods on string/number literals
                    let temp_var_name = format!("__literal_temp_{}", self.peek().line);
                    let temp_id = IdentifierNode::new(
                        Token {
                            token_type: TokenType::Identifier,
                            lexeme: temp_var_name.clone(),
                            literal: TokenLiteral::None,
                            line: self.peek().line,
                            start: 0,
                            length: temp_var_name.len(),
                        },
                        None,
                        IdentifierDeclScope::UnknownScope,
                        false,
                        self.peek().line,
                    );
                    
                    // Create a variable node wrapping the literal
                    let _var_node = VariableNode::new(
                        temp_id.line,
                        temp_id,
                        IdentifierDeclScope::UnknownScope,
                        None,
                    );
                    
                    // Build a call chain starting with the literal as a pseudo-variable
                    let mut call_chain = std::collections::VecDeque::new();
                    
                    // Add the literal as a special node type that will be handled by the visitor
                    // We'll use CallChainNodeType::CallChainLiteralExprT to represent this
                    call_chain.push_back(CallChainNodeType::CallChainLiteralExprT { 
                        call_chain_literal_expr_node: CallChainLiteralExprNode::new(literal_expr_node.line, 
                            literal_expr_node.token_t.clone(),
                            literal_expr_node.value.clone(),
                        ),
                    });
                    
                    // Now consume the dot and parse the method call
                    while self.match_token(&[TokenType::Dot]) {
                        if !self.match_token(&[TokenType::Identifier]) {
                            let err_msg = "Expected identifier after '.'";
                            self.error_at_current(err_msg);
                            return Err(ParseError::new(err_msg));
                        }
                        
                        let method_id = IdentifierNode::new(
                            self.previous().clone(),
                            None,
                            IdentifierDeclScope::UnknownScope,
                            false,
                            self.previous().line,
                        );
                        
                        // Check if it's a method call (has parentheses) or property access
                        if self.match_token(&[TokenType::LParen]) {
                            // It's a method call
                            let expr_list_opt = match self.expr_list() {
                                Ok(Some(expr_list_t)) => {
                                    match expr_list_t {
                                        ExprListT { expr_list_node } => Some(expr_list_node),
                                        _ => None,
                                    }
                                }
                                Ok(None) => None,
                                Err(parse_error) => return Err(parse_error),
                            };
                            
                            // Note: expr_list() already consumes the RParen, so we don't need to do it again
                            
                            // Create a CallExprListNode from the expression list
                            let call_expr_list = if let Some(expr_list) = expr_list_opt {
                                CallExprListNode::new(expr_list.exprs_t)
                            } else {
                                CallExprListNode::new(Vec::new())
                            };
                            
                            let mut call_expr_node = CallExprNode::new(method_id.line, 
                                method_id,
                                call_expr_list,
                                None,  // No call chain continuation
                            );
                            
                            // v0.62: Perform semantic resolution if enabled
                            self.resolve_call_expr(&mut call_expr_node);
                            
                            call_chain.push_back(CallChainNodeType::UndeclaredCallT { 
                                call_node: call_expr_node 
                            });
                        } else {
                            // It's property access (rare for literals but possible)
                            let var_node = VariableNode::new(
                                method_id.line,
                                method_id,
                                IdentifierDeclScope::UnknownScope,
                                None,
                            );
                            call_chain.push_back(CallChainNodeType::VariableNodeT { var_node });
                        }
                    }
                    
                    // If we built a call chain, return it
                    if call_chain.len() > 1 {
                        let call_chain_expr_node = CallChainExprNode::new(self.previous().line, call_chain);
                        return Ok(Some(CallChainExprT { call_chain_expr_node }));
                    }
                }
                
                // No dot after literal, return as normal
                return Ok(Some(LiteralExprT { literal_expr_node }));
            }
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        // $$[+] | $$[-]
        match self.stack_operation() {
            Ok(Some(state_stack_op_node)) => {
                return Ok(Some(StateStackOperationExprT {
                    state_stack_op_node,
                }))
            }
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        // @ | @|| | @[x] | @^
        match self.frame_event_part(is_reference) {
            Ok(Some(frame_event_part)) => return Ok(Some(FrameEventExprT { frame_event_part })),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        if self.match_token(&[TokenType::LBracket]) {
            match self.list() {
                Ok(list_node) => return Ok(Some(ListT { list_node })),
                //                Ok(None) => self.error_at_current("Empty list '()' not allowed "), // continue
                //                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Dictionary or Set literals {key: value} or {1, 2, 3}
        if self.match_token(&[TokenType::OpenBrace]) {
            match self.dict_or_set_literal() {
                Ok(expr_type) => return Ok(Some(expr_type)),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // loop ...
        // match self.loop_expression() {
        //     Ok(Some(loop_types)) => return Ok(Some(LoopExprT {loop_types})),
        //     Err(parse_error) => return Err(parse_error),
        //     Ok(None) => {} // continue
        // }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    //

    fn stack_operation(&mut self) -> Result<Option<StateStackOperationNode>, ParseError> {
        if self.match_token(&[TokenType::StateStackOperationPush]) {
            let line = self.previous().line;  // v0.78.11: capture line for source mapping
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(line, StateStackOperationType::Push);
            return Ok(Some(ssot));
        } else if self.match_token(&[TokenType::StateStackOperationPop]) {
            let line = self.previous().line;  // v0.78.11: capture line for source mapping
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(line, StateStackOperationType::Pop);
            return Ok(Some(ssot));
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // Scope manager wrapper that ensures scope is always properly closed
    fn with_block_scope<F, R>(&mut self, scope_prefix: &str, parse_fn: F) -> Result<R, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<R, ParseError>
    {
        let scope_name = &format!("{}_{}", scope_prefix, self.stmt_idx);
        
        if self.is_building_symbol_table {
            let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
            self.arcanum
                .enter_scope(ParseScopeType::Block { block_scope_rcref });
        } else {
            if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                return Err(ParseError::new(&format!("Failed to set block scope '{}': {}", scope_name, err)));
            }
        }
        
        let result = parse_fn(self);
        self.arcanum.exit_scope();
        result
    }

    /* --------------------------------------------------------------------- */

    // Helper function to parse a single statement after colon (Python-style)
    fn parse_single_statement_block(&mut self, scope_prefix: &str) -> Result<BlockStmtNode, ParseError> {
        // For single statements, use the current position
        let block_line = self.peek().line;
        self.with_block_scope(scope_prefix, |parser| {
            // Check if next token is an open brace - this is not allowed after colon
            if parser.peek().token_type == TokenType::OpenBrace {
                parser.error_at_current("Block statements not allowed after ':'. Use either ':' for single statement or '{' for block.");
                return Err(ParseError::new("Block statements not allowed after ':'. Use either ':' for single statement or '{' for block."));
            }
            
            match parser.decl_or_stmt(IdentifierDeclScope::BlockVarScope) {
                Ok(Some(stmt)) => Ok(BlockStmtNode::new(block_line, vec![stmt])),
                Ok(None) => {
                    parser.error_at_current("Expected statement after ':'.");
                    Err(ParseError::new("Expected statement after ':'."))
                }
                Err(e) => Err(e)
            }
        })
    }

    /* --------------------------------------------------------------------- */

    // Helper function to parse a braced block { stmt* }
    fn parse_braced_block(&mut self, scope_prefix: &str) -> Result<BlockStmtNode, ParseError> {
        // The opening brace should have already been matched, get its line
        let block_line = self.previous().line;
        self.with_block_scope(scope_prefix, |parser| {
            let statements = parser.statements(IdentifierDeclScope::BlockVarScope);
            
            if let Err(parse_error) = parser.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }
            
            Ok(BlockStmtNode::new(block_line, statements))
        })
    }

    /* --------------------------------------------------------------------- */

    // Helper function to parse either a block { stmt* } or a single statement (legacy)
    fn parse_block_or_statement(&mut self, scope_prefix: &str) -> Result<BlockStmtNode, ParseError> {
        if self.match_token(&[TokenType::OpenBrace]) {
            // Parse block with braces
            let block_line = self.previous().line;  // v0.78.10: capture line for block mapping
            let scope_name = &format!("{}_{}", scope_prefix, self.stmt_idx);
            if self.is_building_symbol_table {
                let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
                self.arcanum
                    .enter_scope(ParseScopeType::Block { block_scope_rcref });
            } else {
                if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                    return Err(ParseError::new(&format!("Failed to set block scope '{}': {}", scope_name, err)));
                }
            }

            let statements = self.statements(IdentifierDeclScope::BlockVarScope);
            
            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                self.arcanum.exit_scope();
                return Err(parse_error);
            }
            self.arcanum.exit_scope();

            Ok(BlockStmtNode::new(block_line, statements))
        } else {
            // Parse single statement
            let stmt_line = self.peek().line;  // v0.78.10: capture line for single statement
            let scope_name = &format!("{}_{}", scope_prefix, self.stmt_idx);
            if self.is_building_symbol_table {
                let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
                self.arcanum
                    .enter_scope(ParseScopeType::Block { block_scope_rcref });
            } else {
                if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                    return Err(ParseError::new(&format!("Failed to set block scope '{}': {}", scope_name, err)));
                }
            }

            match self.decl_or_stmt(IdentifierDeclScope::BlockVarScope) {
                Ok(Some(decl_or_stmt)) => {
                    self.arcanum.exit_scope();
                    Ok(BlockStmtNode::new(stmt_line, vec![decl_or_stmt]))
                }
                Ok(None) => {
                    self.arcanum.exit_scope();
                    self.error_at_current("Expected statement after condition.");
                    Err(ParseError::new("Expected statement after condition."))
                }
                Err(e) => {
                    self.arcanum.exit_scope();
                    Err(e)
                }
            }
        }
    }

    /* --------------------------------------------------------------------- */

    fn try_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Parse the try block
        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected '{' after 'try'.");
            return Err(ParseError::new("Expected '{' after 'try'."));
        }

        let try_block = match self.parse_braced_block("try_block") {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        // Parse except clauses (optional - try/finally is allowed without except)
        let mut except_clauses = Vec::new();
        
        // Check if we have except clauses
        let has_except = self.match_token(&[TokenType::Except]);
        
        if has_except {
            loop {
            // Capture line number for source mapping (v0.78.7)
            let except_line = self.previous().line;
            
            // Parse exception specification (optional)
            let mut exception_types = None;
            let mut var_name = None;

            // Check if there's an exception type specified
            if self.check(TokenType::Identifier) || self.check(TokenType::LParen) {
                // Could be:
                // 1. except ValueError
                // 2. except ValueError as e
                // 3. except (ValueError, TypeError)
                // 4. except (ValueError, TypeError) as e
                // 5. except e  (just binding, no type)
                
                if self.match_token(&[TokenType::LParen]) {
                    // Multiple exception types
                    let mut types = Vec::new();
                    loop {
                        if self.peek().token_type == TokenType::Identifier {
                            types.push(self.peek().lexeme.clone());
                            self.advance();
                        } else {
                            break;
                        }
                        
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                    
                    if !self.match_token(&[TokenType::RParen]) {
                        self.error_at_current("Expected ')' after exception types.");
                        return Err(ParseError::new("Expected ')' after exception types."));
                    }
                    
                    if !types.is_empty() {
                        exception_types = Some(types);
                    }
                } else if self.peek().token_type == TokenType::Identifier {
                    // Check if next token is 'as' to determine if this is a type or just a binding
                    let ident = self.peek().lexeme.clone();
                    self.advance();
                    
                    if self.match_token(&[TokenType::As]) {
                        // This was an exception type, now get the variable name
                        exception_types = Some(vec![ident]);
                        
                        if self.peek().token_type == TokenType::Identifier {
                            var_name = Some(self.peek().lexeme.clone());
                            self.advance();
                        }
                    } else if self.check(TokenType::OpenBrace) {
                        // Just a variable binding (except e {})
                        var_name = Some(ident);
                    } else {
                        // Just an exception type (except ValueError {})
                        exception_types = Some(vec![ident]);
                    }
                }
                
                // Check for 'as' if we haven't processed it yet
                if exception_types.is_some() && var_name.is_none() && self.match_token(&[TokenType::As]) {
                    if self.peek().token_type == TokenType::Identifier {
                        var_name = Some(self.peek().lexeme.clone());
                        self.advance();
                    }
                }
            }

            // Parse the except block
            if !self.match_token(&[TokenType::OpenBrace]) {
                self.error_at_current("Expected '{' after except clause.");
                return Err(ParseError::new("Expected '{' after except clause."));
            }

            let except_block = match self.parse_braced_block("except_block") {
                Ok(block) => block,
                Err(e) => return Err(e),
            };

            except_clauses.push(ExceptClauseNode::new(
                except_line,  // v0.78.7: source map support
                exception_types,
                var_name,
                except_block,
            ));

            // Check for more except clauses
            if !self.match_token(&[TokenType::Except]) {
                break;
            }
            }
        }

        // Parse optional else block
        let else_block = if self.match_token(&[TokenType::Else]) {
            if !self.match_token(&[TokenType::OpenBrace]) {
                self.error_at_current("Expected '{' after 'else'.");
                return Err(ParseError::new("Expected '{' after 'else'."));
            }

            Some(match self.parse_braced_block("else_block") {
                Ok(block) => block,
                Err(e) => return Err(e),
            })
        } else {
            None
        };

        // Parse optional finally block
        let finally_block = if self.match_token(&[TokenType::Finally]) {
            if !self.match_token(&[TokenType::OpenBrace]) {
                self.error_at_current("Expected '{' after 'finally'.");
                return Err(ParseError::new("Expected '{' after 'finally'."));
            }

            Some(match self.parse_braced_block("finally_block") {
                Ok(block) => block,
                Err(e) => return Err(e),
            })
        } else {
            None
        };

        // Validate that we have at least except clauses or finally block
        if except_clauses.is_empty() && finally_block.is_none() {
            self.error_at_current("Try statement must have either 'except' clauses or 'finally' block.");
            return Err(ParseError::new("Try statement must have either 'except' clauses or 'finally' block."));
        }

        let try_stmt_node = TryStmtNode::new(self.previous().line, 
            try_block,
            except_clauses,
            else_block,
            finally_block,
        );

        Ok(Some(StatementType::TryStmt { try_stmt_node }))
    }

    fn raise_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Parse optional exception expression
        let exception_expr = match self.expression() {
            Ok(Some(expr)) => Some(expr),
            Ok(None) => None,  // bare 'raise' for re-raising
            Err(e) => return Err(e),
        };

        // Parse optional 'from' clause
        let from_expr = if self.match_token(&[TokenType::From]) {
            match self.expression() {
                Ok(Some(expr)) => Some(expr),
                Ok(None) => {
                    self.error_at_current("Expected expression after 'from'.");
                    return Err(ParseError::new("Expected expression after 'from'."));
                }
                Err(e) => return Err(e),
            }
        } else {
            None
        };

        let raise_stmt_node = RaiseStmtNode::new(self.previous().line, exception_expr, from_expr);

        Ok(Some(StatementType::RaiseStmt { raise_stmt_node }))
    }

    fn with_statement(&mut self, is_async: bool) -> Result<Option<StatementType>, ParseError> {
        // Parse the context expression (e.g., open("file.txt"))
        let context_expr = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected expression after 'with'.");
                return Err(ParseError::new("Expected expression after 'with'."));
            }
            Err(e) => return Err(e),
        };

        // Parse optional 'as' clause
        let target_var = if self.match_token(&[TokenType::As]) {
            if self.peek().token_type == TokenType::Identifier {
                let var_name = self.peek().lexeme.clone();
                self.advance();
                Some(var_name)
            } else {
                self.error_at_current("Expected identifier after 'as'.");
                return Err(ParseError::new("Expected identifier after 'as'."));
            }
        } else {
            None
        };

        // Parse the block
        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected '{' after with expression.");
            return Err(ParseError::new("Expected '{' after with expression."));
        }

        let with_block = match self.parse_braced_block("with_block") {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        let with_stmt_node = WithStmtNode::new(self.previous().line, 
            is_async,
            context_expr,
            target_var,
            with_block,
        );

        Ok(Some(StatementType::WithStmt { with_stmt_node }))
    }

    fn match_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Parse the match expression
        let match_expr = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected expression after 'match'.");
                return Err(ParseError::new("Expected expression after 'match'."));
            }
            Err(e) => return Err(e),
        };

        // Expect opening brace
        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected '{' after match expression.");
            return Err(ParseError::new("Expected '{' after match expression."));
        }

        let mut cases = Vec::new();

        // Parse cases until we hit the closing brace
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            // Expect 'case' keyword
            if !self.match_token(&[TokenType::Case]) {
                self.error_at_current("Expected 'case' in match statement.");
                return Err(ParseError::new("Expected 'case' in match statement."));
            }
            
            // Capture line number for source mapping (v0.78.7)
            let case_line = self.previous().line;

            // Parse the pattern
            let pattern = match self.parse_pattern() {
                Ok(pattern) => pattern,
                Err(e) => return Err(e),
            };

            // Parse optional guard clause (if expression)
            let guard = if self.match_token(&[TokenType::If]) {
                match self.expression() {
                    Ok(Some(expr)) => Some(expr),
                    Ok(None) => {
                        self.error_at_current("Expected guard expression after 'if'.");
                        return Err(ParseError::new("Expected guard expression after 'if'."));
                    }
                    Err(e) => return Err(e),
                }
            } else {
                None
            };

            // Expect opening brace for case body
            if !self.match_token(&[TokenType::OpenBrace]) {
                self.error_at_current("Expected '{' after case pattern.");
                return Err(ParseError::new("Expected '{' after case pattern."));
            }

            // Parse case body statements
            let mut statements = Vec::new();
            while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
                match self.decl_or_stmt(IdentifierDeclScope::UnknownScope) {
                    Ok(Some(stmt)) => statements.push(stmt),
                    Ok(None) => break,
                    Err(e) => return Err(e),
                }
            }

            // Expect closing brace for case body
            if !self.match_token(&[TokenType::CloseBrace]) {
                self.error_at_current("Expected '}' after case body.");
                return Err(ParseError::new("Expected '}' after case body."));
            }

            cases.push(CaseNode::new(case_line, pattern, guard, statements));
        }

        // Expect closing brace for match statement
        if !self.match_token(&[TokenType::CloseBrace]) {
            self.error_at_current("Expected '}' after match cases.");
            return Err(ParseError::new("Expected '}' after match cases."));
        }

        let match_stmt_node = MatchStmtNode::new(self.previous().line, match_expr, cases);
        Ok(Some(StatementType::MatchStmt { match_stmt_node }))
    }

    fn parse_literal(&mut self) -> Result<LiteralExprNode, ParseError> {
        let token = self.peek();
        if matches!(token.token_type, 
            TokenType::Number | 
            TokenType::String | 
            TokenType::FString |
            TokenType::RawString |
            TokenType::ByteString |
            TokenType::TripleQuotedString |
            TokenType::True | 
            TokenType::False | 
            TokenType::None_
        ) {
            let token_type = token.token_type;
            let lexeme = token.lexeme.clone();
            self.advance();
            Ok(LiteralExprNode::new(self.previous().line, token_type, lexeme))
        } else {
            Err(ParseError::new("Expected literal value"))
        }
    }

    fn parse_pattern(&mut self) -> Result<PatternNode, ParseError> {
        let mut pattern = self.parse_pattern_inner()?;
        
        // Check for OR patterns using 'or' keyword
        if self.match_token(&[TokenType::Or]) {
            let mut patterns = vec![pattern];
            patterns.push(self.parse_pattern_inner()?);
            
            // Continue collecting patterns connected by 'or'
            while self.match_token(&[TokenType::Or]) {
                patterns.push(self.parse_pattern_inner()?);
            }
            
            pattern = PatternNode::Or(patterns);
        }
        
        // Check if this pattern has an 'as' clause
        if self.match_token(&[TokenType::As]) {
            if self.peek().token_type != TokenType::Identifier {
                return Err(ParseError::new("Expected identifier after 'as' in pattern."));
            }
            let as_name = self.peek().lexeme.clone();
            self.advance();
            return Ok(PatternNode::As(Box::new(pattern), as_name));
        }
        
        Ok(pattern)
    }
    
    fn parse_pattern_inner(&mut self) -> Result<PatternNode, ParseError> {
        // Check for wildcard pattern
        if self.peek().lexeme == "_" && self.peek().token_type == TokenType::Identifier {
            self.advance();
            return Ok(PatternNode::Wildcard);
        }

        // Check for literal patterns
        if matches!(self.peek().token_type, TokenType::Number | TokenType::String | TokenType::True | TokenType::False | TokenType::None_) {
            let literal = self.parse_literal()?;
            return Ok(PatternNode::Literal(literal));
        }

        // Check for sequence patterns (list/tuple)
        if self.match_token(&[TokenType::LBracket]) {
            let mut patterns = Vec::new();
            if !self.check(TokenType::RBracket) {
                loop {
                    // Check for star pattern (*identifier)
                    if self.match_token(&[TokenType::Star]) {
                        if self.peek().token_type != TokenType::Identifier {
                            return Err(ParseError::new("Expected identifier after '*' in star pattern."));
                        }
                        let star_name = self.peek().lexeme.clone();
                        self.advance();
                        patterns.push(PatternNode::Star(star_name));
                    } else {
                        patterns.push(self.parse_pattern()?);
                    }
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            if !self.match_token(&[TokenType::RBracket]) {
                return Err(ParseError::new("Expected ']' after sequence pattern."));
            }
            return Ok(PatternNode::Sequence(patterns));
        }

        // Check for tuple pattern with parentheses
        if self.match_token(&[TokenType::LParen]) {
            let mut patterns = Vec::new();
            if !self.check(TokenType::RParen) {
                loop {
                    // Check for star pattern (*identifier)
                    if self.match_token(&[TokenType::Star]) {
                        if self.peek().token_type != TokenType::Identifier {
                            return Err(ParseError::new("Expected identifier after '*' in star pattern."));
                        }
                        let star_name = self.peek().lexeme.clone();
                        self.advance();
                        patterns.push(PatternNode::Star(star_name));
                    } else {
                        patterns.push(self.parse_pattern()?);
                    }
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            if !self.match_token(&[TokenType::RParen]) {
                return Err(ParseError::new("Expected ')' after tuple pattern."));
            }
            return Ok(PatternNode::Sequence(patterns));
        }

        // Check for mapping pattern (dict)
        if self.match_token(&[TokenType::OpenBrace]) {
            let mut mappings = Vec::new();
            if !self.check(TokenType::CloseBrace) {
                loop {
                    // Parse key (must be string literal or identifier)
                    let key = if self.peek().token_type == TokenType::String {
                        let s = self.peek().lexeme.clone();
                        self.advance();
                        // For string literals, use the content as-is (it already has quotes removed by scanner)
                        s
                    } else if self.peek().token_type == TokenType::Identifier {
                        let id = self.peek().lexeme.clone();
                        self.advance();
                        id
                    } else {
                        return Err(ParseError::new("Expected string or identifier as dictionary key in pattern."));
                    };

                    if !self.match_token(&[TokenType::Colon]) {
                        return Err(ParseError::new("Expected ':' after dictionary key in pattern."));
                    }

                    let value_pattern = self.parse_pattern()?;
                    mappings.push((key, value_pattern));

                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            if !self.match_token(&[TokenType::CloseBrace]) {
                return Err(ParseError::new("Expected '}' after mapping pattern."));
            }
            return Ok(PatternNode::Mapping(mappings));
        }

        // Default to capture pattern (identifier)
        if self.peek().token_type == TokenType::Identifier {
            let name = self.peek().lexeme.clone();
            self.advance();
            
            // Check for class pattern
            if self.match_token(&[TokenType::LParen]) {
                let mut args = Vec::new();
                if !self.check(TokenType::RParen) {
                    loop {
                        args.push(self.parse_pattern()?);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                if !self.match_token(&[TokenType::RParen]) {
                    return Err(ParseError::new("Expected ')' after class pattern arguments."));
                }
                return Ok(PatternNode::Class(name, args));
            }
            
            // DON'T check for OR pattern here - it needs to be handled differently
            // The pipe character conflicts with other uses in Frame
            
            return Ok(PatternNode::Capture(name));
        }

        self.error_at_current("Invalid pattern in match statement.");
        Err(ParseError::new("Invalid pattern in match statement."))
    }

    fn if_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let if_line = self.previous().line;  // Get line from 'if' token
        
        // Parse the condition expression
        let condition = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected condition after 'if'.");
                return Err(ParseError::new("Expected condition after 'if'."));
            }
            Err(e) => return Err(e),
        };

        // Parse the if block - check for colon (Python-style) or braces
        let if_block = if self.match_token(&[TokenType::Colon]) {
            // Python-style: if condition: statement
            self.parse_single_statement_block("if_block")
        } else if self.match_token(&[TokenType::OpenBrace]) {
            // Braced block: if condition { statements }
            self.parse_braced_block("if_block")
        } else {
            self.error_at_current("Expected ':' or '{' after if condition.");
            return Err(ParseError::new("Expected ':' or '{' after if condition."));
        };

        let if_block = match if_block {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        // Parse elif clauses
        let mut elif_clauses = Vec::new();
        while self.match_token(&[TokenType::Elif]) {
            let elif_line = self.previous().line;  // Get line from 'elif' token
            let elif_condition = match self.expression() {
                Ok(Some(expr)) => expr,
                Ok(None) => {
                    self.error_at_current("Expected condition after 'elif'.");
                    return Err(ParseError::new("Expected condition after 'elif'."));
                }
                Err(e) => return Err(e),
            };

            // Check for colon (Python-style) or braces
            let elif_block = if self.match_token(&[TokenType::Colon]) {
                self.parse_single_statement_block("elif_block")
            } else if self.match_token(&[TokenType::OpenBrace]) {
                self.parse_braced_block("elif_block")
            } else {
                self.error_at_current("Expected ':' or '{' after elif condition.");
                return Err(ParseError::new("Expected ':' or '{' after elif condition."));
            };

            let elif_block = match elif_block {
                Ok(block) => block,
                Err(e) => return Err(e),
            };

            elif_clauses.push(ElifClause {
                line: elif_line,
                condition: elif_condition,
                block: elif_block,
            });
        }

        // Parse optional else clause
        let else_block = if self.match_token(&[TokenType::Else]) {
            // Check for colon (Python-style) or braces
            let block = if self.match_token(&[TokenType::Colon]) {
                self.parse_single_statement_block("else_block")
            } else if self.match_token(&[TokenType::OpenBrace]) {
                self.parse_braced_block("else_block")
            } else {
                self.error_at_current("Expected ':' or '{' after else.");
                return Err(ParseError::new("Expected ':' or '{' after else."));
            };

            match block {
                Ok(block) => Some(block),
                Err(e) => return Err(e),
            }
        } else {
            None
        };

        let if_stmt_node = IfStmtNode::new(if_line, condition, if_block, elif_clauses, else_block);
        Ok(Some(StatementType::IfStmt { if_stmt_node }))
    }

    /* --------------------------------------------------------------------- */

    fn for_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Check if this is a C-style for loop by looking for var declaration followed by semicolon
        // or an assignment followed by semicolon
        let checkpoint = self.current;
        
        // Try to parse as C-style for loop first
        // for var i = 0; i < 10; i = i + 1 { ... }
        // for i = 0; i < 10; i = i + 1 { ... }
        
        let mut init_stmt = LoopFirstStmt::None;
        let mut _is_c_style = false;
        
        if self.match_token(&[TokenType::Var]) {
            // Try parsing var declaration
            match self.var_declaration(IdentifierDeclScope::LoopVarScope) {
                Ok(var_decl_t_rc_ref) => {
                    if self.match_token(&[TokenType::Semicolon]) {
                        // This is C-style for loop
                        _is_c_style = true;
                        init_stmt = LoopFirstStmt::VarDecl {
                            var_decl_node_rcref: var_decl_t_rc_ref,
                        };
                    } else {
                        // Check for 'in' - this is for-in loop
                        if self.match_token(&[TokenType::In]) {
                            // Continue with for-in loop parsing
                            self.current = checkpoint;
                            return self.for_in_statement();
                        } else {
                            self.error_at_current("Expected ';' for C-style loop or 'in' for iteration.");
                            return Err(ParseError::new("Invalid for loop syntax."));
                        }
                    }
                }
                Err(parse_error) => return Err(parse_error),
            }
        } else {
            // Check if this is a simple identifier followed by 'in' (for-in loop)
            // We need to check this BEFORE parsing as expression to avoid consuming 'in' as operator
            if self.check(TokenType::Identifier) {
                let next_token_idx = self.current + 1;
                if next_token_idx < self.tokens.len() && self.tokens[next_token_idx].token_type == TokenType::In {
                    // This is definitely a for-in loop with identifier
                    return self.for_in_statement();
                }
            }
            
            // Otherwise try to parse as expression (for C-style loop)
            let first_expr_result = self.expression();
            match first_expr_result {
                Ok(Some(expr_type)) => {
                    if self.match_token(&[TokenType::Semicolon]) {
                        // C-style for loop with expression init
                        _is_c_style = true;
                        init_stmt = match expr_type {
                            VariableExprT { var_node } => LoopFirstStmt::Var { var_node },
                            AssignmentExprT {
                                assignment_expr_node,
                            } => LoopFirstStmt::VarAssign {
                                assign_expr_node: assignment_expr_node,
                            },
                            _ => {
                                let err_msg = "Invalid initialization in C-style for loop.";
                                self.error_at_current(err_msg);
                                return Err(ParseError::new(err_msg));
                            }
                        };
                    } else if self.match_token(&[TokenType::In]) {
                        // This is for-in loop, reset and parse as for-in
                        self.current = checkpoint;
                        return self.for_in_statement();
                    } else {
                        self.error_at_current("Expected ';' for C-style loop or 'in' for iteration.");
                        return Err(ParseError::new("Invalid for loop syntax."));
                    }
                }
                Ok(None) => {
                    // Empty init clause in C-style loop
                    if self.match_token(&[TokenType::Semicolon]) {
                        _is_c_style = true;
                    } else {
                        self.error_at_current("Expected expression or variable declaration after 'for'.");
                        return Err(ParseError::new("Invalid for loop syntax."));
                    }
                }
                Err(err) => return Err(err),
            }
        }
        
        if _is_c_style {
            // Continue parsing C-style for loop
            // We've already consumed init and first semicolon
            // Now parse condition
            let condition_expr_t_opt = self.expression()?;
            
            if !self.match_token(&[TokenType::Semicolon]) {
                self.error_at_current("Expected ';' after for loop condition.");
                return Err(ParseError::new("Expected ';' after for loop condition."));
            }
            
            // Parse increment expression
            let inc_expr_t_opt = self.expression()?;
            
            // Parse block
            if !self.match_token(&[TokenType::OpenBrace]) {
                self.error_at_current("Expected '{' after for loop header.");
                return Err(ParseError::new("Expected '{' after for loop header."));
            }
            
            let statements = self.statements(IdentifierDeclScope::BlockVarScope);
            
            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }
            
            // Create a LoopForStmtNode using existing loop infrastructure
            let loop_for_stmt_node = LoopForStmtNode::new(self.previous().line, 
                Some(init_stmt),
                condition_expr_t_opt,
                inc_expr_t_opt,
                statements,
            );
            
            let loop_stmt_node = LoopStmtNode::new(self.previous().line, LoopStmtTypes::LoopForStmt {
                loop_for_stmt_node,
            });
            
            Ok(Some(StatementType::LoopStmt { loop_stmt_node }))
        } else {
            // Should not reach here as we handle for-in above
            self.current = checkpoint;
            self.for_in_statement()
        }
    }
    
    fn for_in_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let for_line = self.previous().line;  // Get line from 'for' token that was already matched
        let mut variable: Option<VariableNode> = None;
        let mut identifier: Option<IdentifierNode> = None;

        // Parse for var x in items or for x in items
        if self.match_token(&[TokenType::Var]) {
            // for var x in items
            match self.var_declaration(IdentifierDeclScope::LoopVarScope) {
                Ok(var_decl_t_rc_ref) => {
                    // Create a VariableNode from the VariableDeclNode
                    let var_decl = var_decl_t_rc_ref.borrow();
                    let id_node = IdentifierNode::new(
                        Token::new(
                            TokenType::Identifier,
                            var_decl.name.clone(),
                            TokenLiteral::None,
                            0, // line
                            0, // start
                            var_decl.name.len(),
                        ),
                        None,
                        IdentifierDeclScope::LoopVarScope,
                        false,
                        0,
                    );
                    let var_node = VariableNode::new(id_node.line, id_node, IdentifierDeclScope::LoopVarScope, None);
                    variable = Some(var_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        } else {
            // for x in items - expect identifier
            if self.match_token(&[TokenType::Identifier]) {
                let id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::LoopVarScope,
                    false,
                    self.previous().line,
                );
                identifier = Some(id_node);
            } else {
                self.error_at_current("Expected variable name after 'for'.");
                return Err(ParseError::new("Expected variable name after 'for'."));
            }
        }

        // Expect 'in' keyword
        if !self.match_token(&[TokenType::In]) {
            self.error_at_current("Expected 'in' after for loop variable.");
            return Err(ParseError::new("Expected 'in' after for loop variable."));
        }

        // Parse iterable expression
        let iterable = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected iterable expression after 'in'.");
                return Err(ParseError::new("Expected iterable expression after 'in'."));
            }
            Err(e) => return Err(e),
        };

        // Check if iterating over an enum type
        let mut is_enum_iteration = false;
        let mut enum_type_name = None;
        
        // eprintln!("DEBUG: Checking iterable for enum iteration");
        match &iterable {
            VariableExprT { ref var_node } => {
                // eprintln!("  Iterable is VariableExprT");
                // Check if this variable refers to an enum type
                let var_name = &var_node.id_node.name.lexeme;
                // eprintln!("  Variable name: {}", var_name);
                
                // Look up the symbol to see if it's an enum
                if let Some(symbol_type_rcref) = self.arcanum.lookup(var_name, &IdentifierDeclScope::UnknownScope) {
                    let symbol_type = symbol_type_rcref.borrow();
                    // eprintln!("  Found symbol type: {:?}", &*symbol_type);
                    if let SymbolType::EnumDeclSymbolT { .. } = &*symbol_type {
                        // eprintln!("  It's an enum! Setting is_enum_iteration=true");
                        is_enum_iteration = true;
                        enum_type_name = Some(var_name.clone());
                    }
                }
            }
            CallChainExprT { ref call_chain_expr_node } => {
                // eprintln!("  Iterable is CallChainExprT with {} nodes", call_chain_expr_node.call_chain.len());
                // Check if it's a single-node chain with an enum variable
                if call_chain_expr_node.call_chain.len() == 1 {
                    if let Some(first_node) = call_chain_expr_node.call_chain.front() {
                        match first_node {
                            CallChainNodeType::VariableNodeT { ref var_node } => {
                                let var_name = &var_node.id_node.name.lexeme;
                                // eprintln!("  Single VariableNodeT in chain: {}", var_name);
                                
                                // Check if the variable's symbol type is an enum
                                if let Some(ref symbol_type_opt) = var_node.symbol_type_rcref_opt {
                                    let symbol_type = symbol_type_opt.borrow();
                                    // eprintln!("  Symbol type from var_node: {:?}", &*symbol_type);
                                    if let SymbolType::EnumDeclSymbolT { .. } = &*symbol_type {
                                        // eprintln!("  It's an enum! Setting is_enum_iteration=true");
                                        is_enum_iteration = true;
                                        enum_type_name = Some(var_name.clone());
                                    }
                                }
                            }
                            _ => {
                                // eprintln!("  First node is not a VariableNodeT");
                            }
                        }
                    }
                }
            }
            _ => {
                // eprintln!("  Iterable is neither VariableExprT nor CallChainExprT");
            }
        }

        // Parse the for block - check for colon (Python-style) or braces
        let for_block = if self.match_token(&[TokenType::Colon]) {
            // Python-style: for x in items: statement
            self.parse_single_statement_block("for_block")
        } else if self.match_token(&[TokenType::OpenBrace]) {
            // Braced block: for x in items { statements }
            self.parse_braced_block("for_block")
        } else {
            self.error_at_current("Expected ':' or '{' after for iterable.");
            return Err(ParseError::new("Expected ':' or '{' after for iterable."));
        };

        let for_block = match for_block {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        // v0.51: Check for optional else clause
        let else_block_opt = if self.match_token(&[TokenType::Else]) {
            let else_block = if self.match_token(&[TokenType::Colon]) {
                // Python-style: else: statement
                self.parse_single_statement_block("for_else_block")
            } else if self.match_token(&[TokenType::OpenBrace]) {
                // Braced block: else { statements }
                self.parse_braced_block("for_else_block")
            } else {
                self.error_at_current("Expected ':' or '{' after 'else'.");
                return Err(ParseError::new("Expected ':' or '{' after 'else'."));
            };
            
            match else_block {
                Ok(block) => Some(block),
                Err(e) => return Err(e),
            }
        } else {
            None
        };

        let for_stmt_node = if let Some(else_block) = else_block_opt {
            if is_enum_iteration {
                let mut node = ForStmtNode::with_else(
                    for_line,
                    variable,
                    identifier,
                    iterable,
                    for_block,
                    else_block,
                );
                node.is_enum_iteration = true;
                node.enum_type_name = enum_type_name;
                node
            } else {
                ForStmtNode::with_else(for_line, variable, identifier, iterable, for_block, else_block)
            }
        } else if is_enum_iteration {
            ForStmtNode::new_enum_iteration(
                for_line,
                variable,
                identifier,
                iterable,
                for_block,
                enum_type_name.unwrap(),
            )
        } else {
            ForStmtNode::new(for_line, variable, identifier, iterable, for_block)
        };
        Ok(Some(StatementType::ForStmt { for_stmt_node }))
    }

    /* --------------------------------------------------------------------- */

    fn while_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let while_line = self.previous().line;  // Get line from 'while' token
        
        // Parse the condition expression
        let condition = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected condition after 'while'.");
                return Err(ParseError::new("Expected condition after 'while'."));
            }
            Err(e) => return Err(e),
        };

        // Parse the while block - check for colon (Python-style) or braces
        let while_block = if self.match_token(&[TokenType::Colon]) {
            // Python-style: while condition: statement
            self.parse_single_statement_block("while_block")
        } else if self.match_token(&[TokenType::OpenBrace]) {
            // Braced block: while condition { statements }
            self.parse_braced_block("while_block")
        } else {
            self.error_at_current("Expected ':' or '{' after while condition.");
            return Err(ParseError::new("Expected ':' or '{' after while condition."));
        };

        let while_block = match while_block {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        // v0.51: Check for optional else clause
        let while_stmt_node = if self.match_token(&[TokenType::Else]) {
            let else_block = if self.match_token(&[TokenType::Colon]) {
                // Python-style: else: statement
                self.parse_single_statement_block("while_else_block")
            } else if self.match_token(&[TokenType::OpenBrace]) {
                // Braced block: else { statements }
                self.parse_braced_block("while_else_block")
            } else {
                self.error_at_current("Expected ':' or '{' after 'else'.");
                return Err(ParseError::new("Expected ':' or '{' after 'else'."));
            };
            
            let else_block = match else_block {
                Ok(block) => block,
                Err(e) => return Err(e),
            };
            
            WhileStmtNode::with_else(while_line, condition, while_block, else_block)
        } else {
            WhileStmtNode::new(while_line, condition, while_block)
        };
        
        Ok(Some(StatementType::WhileStmt { while_stmt_node }))
    }

    /* --------------------------------------------------------------------- */

    // v0.50: Parse del statement - del target
    fn del_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Parse the target expression to delete
        let target = match self.expression() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                self.error_at_current("Expected target expression after 'del'.");
                return Err(ParseError::new("Expected target expression after 'del'."));
            }
            Err(e) => return Err(e),
        };

        // Create the del statement node
        let del_stmt_node = DelStmtNode::new(self.previous().line, target);
        Ok(Some(StatementType::DelStmt { del_stmt_node }))
    }

    /* --------------------------------------------------------------------- */

    // TODO - update other scopes to follow this patter so that
    // TODO - all return paths automatically pop scope.

    fn loop_statement_scope(&mut self) -> Result<Option<StatementType>, ParseError> {
        // for all loop types, push a symbol table for new scope
        self.is_loop_scope = true;
        if self.is_building_symbol_table {
            let scope_name = &format!("{}.{}", LoopStmtScopeSymbol::scope_name(), self.stmt_idx);
            let loop_stmt_scope_symbol_rcref =
                Rc::new(RefCell::new(LoopStmtScopeSymbol::new(scope_name)));
            self.arcanum.enter_scope(ParseScopeType::Loop {
                loop_scope_symbol_rcref: loop_stmt_scope_symbol_rcref,
            });
        } else {
            // give each loop in a scope a unique name
            let scope_name = &format!("{}.{}", LoopStmtScopeSymbol::scope_name(), self.stmt_idx);
            if let Err(err) = self.arcanum.set_parse_scope(scope_name) {
                return Err(ParseError::new(&format!("Failed to set loop scope '{}': {}", scope_name, err)));
            }
        }
        // parse loop
        let ret = self.loop_statement();
        // exit loop scope
        self.arcanum.exit_scope();
        self.is_loop_scope = false;
        ret
    }

    /* --------------------------------------------------------------------- */

    // loop { foo() }
    // loop var x = 0; x < 10; x++ { foo(x) }
    // loop var x in range(5) { foo(x) }
    // loop .. { foo() continue break }

    fn loop_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        // Deprecation warning for 'loop' keyword
        eprintln!("Warning: The 'loop' keyword is deprecated. Use 'for' or 'while' instead for better readability and Python compatibility.");
        
        if self.match_token(&[TokenType::OpenBrace]) {
            // loop { foo() }
            return self.loop_infinite_statement();
        }

        let mut init_stmt = LoopFirstStmt::None;

        if self.match_token(&[TokenType::Var]) {
            // loop var x:int = 0; ...
            match self.var_declaration(IdentifierDeclScope::LoopVarScope) {
                Ok(var_decl_t_rc_ref) => {
                    init_stmt = LoopFirstStmt::VarDecl {
                        var_decl_node_rcref: var_decl_t_rc_ref,
                    };
                }
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }
        } else {
            // loop y in foo() { bar(y) }
            let first_expr_result = self.expression();
            match first_expr_result {
                Ok(Some(expr_type)) => {
                    init_stmt = match expr_type {
                        VariableExprT { var_node } => LoopFirstStmt::Var { var_node },
                        AssignmentExprT {
                            assignment_expr_node,
                        } => LoopFirstStmt::VarAssign {
                            assign_expr_node: assignment_expr_node,
                        },
                        CallChainExprT {
                            call_chain_expr_node,
                        } => LoopFirstStmt::CallChain {
                            call_chain_expr_node,
                        },
                        _ => {
                            // TODO - improve error msg
                            let err_msg = format!("Invalid initial clause in loop.");
                            self.error_at_current(&err_msg);
                            let parse_error = ParseError::new(err_msg.as_str());
                            return Err(parse_error);
                        }
                    };
                    // if let AssignmentExprT {assignment_expr_node} = expr_type {
                    //     init_stmt = LoopFirstStmt::VarAssignInit {
                    //         assign_expr_node: assignment_expr_node
                    //     };
                    // }

                    // loop_first_expr_opt = Some(expr_type);
                }
                Ok(None) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }

        if self.match_token(&[TokenType::Semicolon]) {
            // loop var x:int = 0; ...
            // loop x = 0; ...
            return self.loop_for_statement(Some(init_stmt));
        }

        // loop var x:int in range(5) { foo(x) }
        // loop x in range(5) { foo(x) }
        if self.match_token(&[TokenType::In]) {
            return self.loop_in_statement(init_stmt);

        }

        return Err(ParseError::new("Unrecognized loop syntax."));
    }

    /* --------------------------------------------------------------------- */

    fn loop_infinite_statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);

        if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
            return Err(parse_error);
        }

        let loop_infinite_stmt_node = LoopInfiniteStmtNode::new(self.previous().line, statements);

        let loop_stmt_node = LoopStmtNode::new(self.previous().line, LoopStmtTypes::LoopInfiniteStmt {
            loop_infinite_stmt_node,
        });
        let stmt_type = StatementType::LoopStmt { loop_stmt_node };
        Ok(Some(stmt_type))
    }

    /* --------------------------------------------------------------------- */

    // TODO - restrict allowed expressions. Currently all can be used in any clause.
    fn loop_for_statement(
        &mut self,
        init_stmt: Option<LoopFirstStmt>,
    ) -> Result<Option<StatementType>, ParseError> {
        let mut test_expr_opt = Option::None;
        let mut inc_dec_expr_opt = Option::None;

        let second_expr_result = self.expression();
        match second_expr_result {
            Ok(Some(expr_type)) => {
                test_expr_opt = Some(expr_type);
            }
            Ok(None) => {}
            Err(err) => {
                return Err(err);
            }
        }
        if self.match_token(&[TokenType::Semicolon]) {}
        let third_expr_result = self.expression();
        match third_expr_result {
            Ok(Some(expr_type)) => {
                inc_dec_expr_opt = Some(expr_type);
            }
            Ok(None) => {}
            Err(err) => {
                return Err(err);
            }
        }

        // statements block
        if self.match_token(&[TokenType::OpenBrace]) {
            let statements = self.statements(IdentifierDeclScope::BlockVarScope);

            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }

            let loop_for_stmt_node =
                LoopForStmtNode::new(self.previous().line, init_stmt, test_expr_opt, inc_dec_expr_opt, statements);

            let loop_stmt_node =
                LoopStmtNode::new(self.previous().line, LoopStmtTypes::LoopForStmt { loop_for_stmt_node });
            let stmt_type = StatementType::LoopStmt { loop_stmt_node };
            return Ok(Some(stmt_type));
        } else {
            return Err(ParseError::new("Missing loop open brace '{'"));
        }
    }

    /* --------------------------------------------------------------------- */

    fn loop_in_statement(
        &mut self,
        loop_first_stmt: LoopFirstStmt,
    ) -> Result<Option<StatementType>, ParseError> {
        let iterable_expr;
        let second_expr_result = self.expression();
        match second_expr_result {
            Ok(Some(expr_type)) => {
                iterable_expr = Box::new(expr_type);
            }
            Ok(None) => {
                self.error_at_current("Expected loop iterable expression.");
                return Err(ParseError::new("Expected loop iterable expression."));
            }
            Err(err) => {
                return Err(err);
            }
        }

        // statements block
        if self.match_token(&[TokenType::OpenBrace]) {
            let statements = self.statements(IdentifierDeclScope::BlockVarScope);

            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }

            // let expr_t = match x {
            //     LoopFirstStmt::VarDecl {var_node} => {
            //         VariableExprT {v}
            //     }
            //     _ => {
            //         let err_msg = "Expected variable or var declaration.";
            //         self.error_at_current(&&err_msg);
            //         return Err(ParseError::new(err_msg));
            //     }
            // };
            let loop_in_stmt_node = LoopInStmtNode::new(self.previous().line, loop_first_stmt, iterable_expr, statements);

            let loop_stmt_node = LoopStmtNode::new(self.previous().line, LoopStmtTypes::LoopInStmt { loop_in_stmt_node });
            let stmt_type = StatementType::LoopStmt { loop_stmt_node };
            return Ok(Some(stmt_type));
        } else {
            return Err(ParseError::new("Missing loop open brace '{'"));
        }
    }

    /* --------------------------------------------------------------------- */

    // Parse FrameEvent "part" identifier:
    // $@||  - Event message
    // $@[p] - Event parameter
    // $@^   - Event return object/value

    fn frame_event_part(
        &mut self,
        is_reference: bool,
    ) -> Result<Option<FrameEventPart>, ParseError> {
        if !self.match_token(&[TokenType::DollarAt]) {
            return Ok(None);
        }

        // '@' '||'
        if self.match_token(&[TokenType::PipePipe]) {
            return Ok(Some(FrameEventPart::Message { is_reference }));
        }

        // '@' '[' identifier ']'
        if self.match_token(&[TokenType::LBracket]) {
            if self.match_token(&[TokenType::Identifier]) {
                let id_tok = self.previous().clone();

                if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                    return Err(parse_error);
                }

                // TODO!! must test for existence
                let param_symbol_rcref;
                let symbol_type_rcref_opt = self
                    .arcanum
                    .lookup(&id_tok.lexeme, &IdentifierDeclScope::UnknownScope);
                match symbol_type_rcref_opt {
                    Some(symbol_type_rcref) => {
                        let symbol_type = symbol_type_rcref.borrow();

                        match &*symbol_type {
                            SymbolType::EventHandlerParam {
                                event_handler_param_symbol_rcref,
                            } => {
                                param_symbol_rcref = event_handler_param_symbol_rcref.clone();
                            }
                            _ => {
                                self.error_at_current(&format!(
                                    "{} is not an event parameter.",
                                    id_tok.lexeme
                                ));
                                return Err(ParseError::new(""));
                            }
                        }
                    }
                    None => {
                        self.error_at_current(&format!(
                            "Unknown event parameter - {}.",
                            id_tok.lexeme
                        ));
                        return Err(ParseError::new(""));
                    }
                }

                return Ok(Some(FrameEventPart::Param {
                    param_symbol_rcref,
                    is_reference,
                }));
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }
        }

        // '@' '^'
        // if self.match_token(&[TokenType::Caret]) {
        //     return Ok(Some(FrameEventPart::Return { is_reference }));
        // }

        // @
        Ok(Some(FrameEventPart::Event { is_reference }))
    }

    /* --------------------------------------------------------------------- */
    //
    // Wrapper to convert result from expr_list() from ExprType to ExprListNode.
    //

    fn expr_list_node(&mut self) -> Result<Option<ExprListNode>, ParseError> {
        match self.expr_list() {
            Ok(Some(ExprListT { expr_list_node })) => {
                return Ok(Some(expr_list_node));
            }
            Err(parse_err) => {
                return Err(parse_err);
            }
            _ => {
                let err_msg = &format!("Invalid error type.");
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        }
    }

    /* --------------------------------------------------------------------- */

    // list -> '[' (list_comprehension | expression_list) ']'
    // list_comprehension -> expression 'for' identifier 'in' expression ('if' expression)?
    // expression_list -> (unpack_expr | expression) (',' (unpack_expr | expression))*
    // unpack_expr -> '*' expression

    // Parse dictionary or set literal: {key: value} or {1, 2, 3}
    fn dict_or_set_literal(&mut self) -> Result<ExprType, ParseError> {
        // v0.53: Set flag to indicate we're inside a collection literal
        let was_parsing_collection = self.is_parsing_collection;
        self.is_parsing_collection = true;
        
        // Handle empty {} - default to empty dict
        // v0.38: Use {,} for empty set (single comma inside)
        if self.peek().token_type == TokenType::CloseBrace {
            self.advance(); // consume '}'
            self.is_parsing_collection = was_parsing_collection;  // Restore flag
            return Ok(ExprType::DictLiteralT { 
                dict_literal_node: DictLiteralNode::new(self.previous().line, Vec::new())
            });
        }
        
        // v0.38: Check for {,} pattern for empty set
        if self.peek().token_type == TokenType::Comma {
            self.advance(); // consume ','
            if self.peek().token_type == TokenType::CloseBrace {
                self.advance(); // consume '}'
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Ok(ExprType::SetLiteralT {
                    set_literal_node: SetLiteralNode::new(self.previous().line, Vec::new())
                });
            } else {
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Err(ParseError::new("Expected '}' after ',' for empty set literal"));
            }
        }
        
        // Check for dict unpacking (**expr)
        if self.match_token(&[TokenType::StarStar]) {
            // This is a dict with unpacking
            let mut pairs = Vec::new();
            
            // Parse unpacked expression
            match self.expression() {
                Ok(Some(expr)) => {
                    // For dict unpacking, we only need to store it once as a key
                    // The visitor will recognize this pattern
                    let dict_unpack = DictUnpackExprNode::new(self.previous().line, expr);
                    pairs.push((
                        ExprType::DictUnpackExprT { dict_unpack_expr_node: dict_unpack },
                        ExprType::NilExprT  // Placeholder value for unpacking
                    ));
                }
                Ok(None) => {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected expression after '**'"))
                },
                Err(e) => {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(e)
                },
            };
            
            // Parse remaining items (could be more unpacking or regular pairs)
            while !self.match_token(&[TokenType::CloseBrace]) {
                if !self.match_token(&[TokenType::Comma]) {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected ',' or '}' in dictionary"));
                }
                
                // Allow trailing comma
                if self.peek().token_type == TokenType::CloseBrace {
                    self.advance();
                    break;
                }
                
                // Check for another unpacking
                if self.match_token(&[TokenType::StarStar]) {
                    match self.expression() {
                        Ok(Some(expr)) => {
                            let dict_unpack = DictUnpackExprNode::new(self.previous().line, expr);
                            pairs.push((
                                ExprType::DictUnpackExprT { dict_unpack_expr_node: dict_unpack },
                                ExprType::NilExprT  // Placeholder value for unpacking
                            ));
                        }
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(ParseError::new("Expected expression after '**'"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                } else {
                    // Regular key-value pair
                    // For keys, we don't want lambda expressions, so use equality() instead of expression()
                    let key = match self.equality() {
                        Ok(Some(expr)) => expr,
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                            return Err(ParseError::new("Expected key in dictionary"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                    
                    if !self.match_token(&[TokenType::Colon]) {
                        self.is_parsing_collection = was_parsing_collection;  // Restore flag
                        return Err(ParseError::new("Expected ':' after dictionary key"));
                    }
                    
                    // For values, we want full expressions including lambda
                    let value = match self.expression() {
                        Ok(Some(expr)) => expr,
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(ParseError::new("Expected value in dictionary"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                    
                    pairs.push((key, value));
                }
            }
            
            self.is_parsing_collection = was_parsing_collection;  // Restore flag
            return Ok(ExprType::DictLiteralT { 
                dict_literal_node: DictLiteralNode::new(self.previous().line, pairs)
            });
        }
        
        // Parse first element (regular, not unpacking)
        // Use equality() here to avoid parsing lambda as a key
        let first_expr = match self.equality() {
            Ok(Some(expr)) => expr,
            Ok(None) => {
                if self.peek().token_type == TokenType::CloseBrace {
                    // Empty dict
                    self.advance();
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Ok(ExprType::DictLiteralT { 
                        dict_literal_node: DictLiteralNode::new(self.previous().line, Vec::new())
                    });
                }
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Err(ParseError::new("Expected expression in literal"));
            }
            Err(e) => {
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Err(e)
            },
        };
        
        // Check if it's a dictionary (has colon) or set (has comma or closing brace)
        if self.match_token(&[TokenType::Colon]) {
            // It's a dictionary
            let mut pairs = Vec::new();
            
            // Parse first value
            let value = match self.expression() {
                Ok(Some(expr)) => expr,
                Ok(None) => {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected value after ':' in dictionary"))
                },
                Err(e) => {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(e)
                },
            };
            
            // Check if it's a dictionary comprehension (has 'for' keyword)
            if self.peek().token_type == TokenType::For {
                // It's a dictionary comprehension
                let comp_result = self.dict_comprehension(first_expr, value)?;
                // Consume the closing brace
                if !self.match_token(&[TokenType::CloseBrace]) {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected '}' after dictionary comprehension"));
                }
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Ok(comp_result);
            }
            
            pairs.push((first_expr, value));
            
            // Parse remaining pairs
            while !self.match_token(&[TokenType::CloseBrace]) {
                if !self.match_token(&[TokenType::Comma]) {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected ',' or '}' in dictionary"));
                }
                
                // Allow trailing comma
                if self.peek().token_type == TokenType::CloseBrace {
                    self.advance();
                    break;
                }
                
                // Check for dict unpacking
                if self.match_token(&[TokenType::StarStar]) {
                    // Dict unpacking in the middle
                    match self.expression() {
                        Ok(Some(expr)) => {
                            let dict_unpack = DictUnpackExprNode::new(self.previous().line, expr);
                            pairs.push((
                                ExprType::DictUnpackExprT { dict_unpack_expr_node: dict_unpack },
                                ExprType::NilExprT  // Placeholder value for unpacking
                            ));
                        }
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(ParseError::new("Expected expression after '**'"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                } else {
                    // Parse next key - don't allow lambda as key
                    let key = match self.equality() {
                        Ok(Some(expr)) => expr,
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                            return Err(ParseError::new("Expected key in dictionary"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                    
                    if !self.match_token(&[TokenType::Colon]) {
                        self.is_parsing_collection = was_parsing_collection;  // Restore flag
                        return Err(ParseError::new("Expected ':' after dictionary key"));
                    }
                    
                    // Parse value - allow full expressions including lambda
                    let value = match self.expression() {
                        Ok(Some(expr)) => expr,
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(ParseError::new("Expected value in dictionary"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // Restore flag
                            return Err(e)
                        },
                    };
                    
                    pairs.push((key, value));
                }
            }
            
            self.is_parsing_collection = was_parsing_collection;  // Restore flag
            Ok(ExprType::DictLiteralT { 
                dict_literal_node: DictLiteralNode::new(self.previous().line, pairs)
            })
        } else {
            // It's a set or set comprehension
            
            // Check if it's a set comprehension (has 'for' keyword after first expression)
            if self.peek().token_type == TokenType::For {
                // It's a set comprehension
                let comp_result = self.set_comprehension(first_expr)?;
                // Consume the closing brace
                if !self.match_token(&[TokenType::CloseBrace]) {
                    return Err(ParseError::new("Expected '}' after set comprehension"));
                }
                return Ok(comp_result);
            }
            
            // It's a regular set literal
            let mut elements = vec![first_expr];
            
            // Check for single element set
            if self.match_token(&[TokenType::CloseBrace]) {
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Ok(ExprType::SetLiteralT { 
                    set_literal_node: SetLiteralNode::new(self.previous().line, elements)
                });
            }
            
            // Parse remaining elements
            if !self.match_token(&[TokenType::Comma]) {
                self.is_parsing_collection = was_parsing_collection;  // Restore flag
                return Err(ParseError::new("Expected ',' or '}' in set literal"));
            }
            
            loop {
                // Allow trailing comma
                if self.peek().token_type == TokenType::CloseBrace {
                    self.advance();
                    break;
                }
                
                // Parse next element
                match self.expression() {
                    Ok(Some(expr)) => elements.push(expr),
                    Ok(None) => {
                        self.is_parsing_collection = was_parsing_collection;  // Restore flag
                        return Err(ParseError::new("Expected element in set"))
                    },
                    Err(e) => {
                        self.is_parsing_collection = was_parsing_collection;  // Restore flag
                        return Err(e)
                    },
                }
                
                if self.match_token(&[TokenType::CloseBrace]) {
                    break;
                }
                
                if !self.match_token(&[TokenType::Comma]) {
                    self.is_parsing_collection = was_parsing_collection;  // Restore flag
                    return Err(ParseError::new("Expected ',' or '}' in set"));
                }
            }
            
            self.is_parsing_collection = was_parsing_collection;  // Restore flag
            Ok(ExprType::SetLiteralT { 
                set_literal_node: SetLiteralNode::new(self.previous().line, elements)
            })
        }
    }

    fn list(&mut self) -> Result<ListNode, ParseError> {
        // v0.53: Set flag to indicate we're inside a collection literal
        let was_parsing_collection = self.is_parsing_collection;
        self.is_parsing_collection = true;
        
        // First, check if this is a list comprehension by looking ahead
        // We need to look for the pattern: expr 'for' ... 
        let checkpoint = self.current;
        let mut is_comprehension = false;
        
        // Try to parse first expression and check if 'for' follows
        if let Ok(Some(_first_expr)) = self.expression() {
            if self.peek().token_type == TokenType::For {
                is_comprehension = true;
            }
        }
        
        // Reset to start of list content
        self.current = checkpoint;
        
        if is_comprehension {
            // Parse as list comprehension
            let result = self.list_comprehension();
            // v0.53: Restore the original flag value before returning
            self.is_parsing_collection = was_parsing_collection;
            return result;
        }
        
        // Parse as regular list with possible unpacking
        let mut expressions: Vec<ExprType> = Vec::new();

        loop {
            if self.match_token(&[TokenType::RBracket]) {
                break;
            }
            
            // Check for unpacking operator
            if self.match_token(&[TokenType::Star]) {
                // Parse unpacked expression
                match self.expression() {
                    Ok(Some(expr)) => {
                        let unpack_node = UnpackExprNode::new(self.previous().line, expr);
                        expressions.push(ExprType::UnpackExprT { unpack_expr_node: unpack_node });
                    }
                    Ok(None) => {
                        return Err(ParseError::new("Expected expression after '*' operator"));
                    }
                    Err(parse_error) => return Err(parse_error),
                }
            } else {
                // Regular expression
                match self.expression() {
                    Ok(Some(expression)) => {
                        expressions.push(expression);
                    }
                    Ok(None) => break,
                    Err(parse_error) => return Err(parse_error),
                }
            }
            
            if self.peek().token_type == TokenType::RBracket {
                continue;
            }
            if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
                return Err(parse_error);
            }
        }

        // v0.53: Restore the original flag value
        self.is_parsing_collection = was_parsing_collection;
        Ok(ListNode::new(self.previous().line, expressions))
    }
    
    // Parse dictionary comprehension: {key: value for var in iterable if condition}
    fn dict_comprehension(&mut self, key_expr: ExprType, value_expr: ExprType) -> Result<ExprType, ParseError> {
        // Consume 'for' keyword
        if !self.match_token(&[TokenType::For]) {
            return Err(ParseError::new("Expected 'for' in dictionary comprehension"));
        }
        
        // Parse target variable(s) - could be k, v for unpacking
        let mut targets = Vec::new();
        
        // Parse first target
        if !self.match_token(&[TokenType::Identifier]) {
            return Err(ParseError::new("Expected identifier after 'for' in dictionary comprehension"));
        }
        targets.push(self.previous().lexeme.clone());
        
        // Check for comma (for multiple targets like k, v)
        if self.match_token(&[TokenType::Comma]) {
            if !self.match_token(&[TokenType::Identifier]) {
                return Err(ParseError::new("Expected identifier after ',' in dictionary comprehension"));
            }
            targets.push(self.previous().lexeme.clone());
        }
        
        // Join targets with comma for Python code generation
        let target = targets.join(", ");
        
        // Consume 'in' keyword
        if !self.match_token(&[TokenType::In]) {
            return Err(ParseError::new("Expected 'in' after variable in dictionary comprehension"));
        }
        
        // Parse iterable expression
        let iter = match self.expression() {
            Ok(Some(e)) => e,
            Ok(None) => return Err(ParseError::new("Expected iterable in dictionary comprehension")),
            Err(e) => return Err(e),
        };
        
        // Parse optional condition
        let condition = if self.match_token(&[TokenType::If]) {
            match self.expression() {
                Ok(Some(e)) => Some(e),
                Ok(None) => return Err(ParseError::new("Expected condition after 'if' in dictionary comprehension")),
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        
        // DO NOT consume closing brace - let dict_or_set_literal handle it
        
        let comprehension_node = DictComprehensionNode::new(self.previous().line, key_expr, value_expr, target, iter, condition);
        Ok(ExprType::DictComprehensionExprT { 
            dict_comprehension_node: comprehension_node 
        })
    }

    // Parse set comprehension: {expr for var in iterable if condition}
    fn set_comprehension(&mut self, expr: ExprType) -> Result<ExprType, ParseError> {
        // Consume 'for' keyword
        if !self.match_token(&[TokenType::For]) {
            return Err(ParseError::new("Expected 'for' in set comprehension"));
        }
        
        // Parse target variable
        if !self.match_token(&[TokenType::Identifier]) {
            return Err(ParseError::new("Expected identifier after 'for' in set comprehension"));
        }
        let target = self.previous().lexeme.clone();
        
        // Consume 'in' keyword
        if !self.match_token(&[TokenType::In]) {
            return Err(ParseError::new("Expected 'in' after variable in set comprehension"));
        }
        
        // Parse iterable expression
        let iter = match self.expression() {
            Ok(Some(e)) => e,
            Ok(None) => return Err(ParseError::new("Expected iterable in set comprehension")),
            Err(e) => return Err(e),
        };
        
        // Parse optional condition
        let condition = if self.match_token(&[TokenType::If]) {
            match self.expression() {
                Ok(Some(e)) => Some(e),
                Ok(None) => return Err(ParseError::new("Expected condition after 'if' in set comprehension")),
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        
        // DO NOT consume closing brace - let dict_or_set_literal handle it
        
        let comprehension_node = SetComprehensionNode::new(self.previous().line, expr, target, iter, condition);
        Ok(ExprType::SetComprehensionExprT { 
            set_comprehension_node: comprehension_node 
        })
    }

    // Parse list comprehension: [expr for var in iterable if condition]
    fn list_comprehension(&mut self) -> Result<ListNode, ParseError> {
        // Parse the expression part
        let expr = match self.expression() {
            Ok(Some(e)) => e,
            Ok(None) => return Err(ParseError::new("Expected expression in list comprehension")),
            Err(e) => return Err(e),
        };
        
        // Expect 'for'
        if !self.match_token(&[TokenType::For]) {
            return Err(ParseError::new("Expected 'for' in list comprehension"));
        }
        
        // Parse target variable
        let target = if self.peek().token_type == TokenType::Identifier {
            let token = self.advance();
            token.lexeme.clone()
        } else {
            return Err(ParseError::new("Expected identifier after 'for'"));
        };
        
        // Expect 'in'
        if !self.match_token(&[TokenType::In]) {
            return Err(ParseError::new("Expected 'in' after identifier in list comprehension"));
        }
        
        // Parse iterable expression
        let iter = match self.expression() {
            Ok(Some(e)) => e,
            Ok(None) => return Err(ParseError::new("Expected iterable expression after 'in'")),
            Err(e) => return Err(e),
        };
        
        // Optional 'if' condition
        let condition = if self.match_token(&[TokenType::If]) {
            match self.expression() {
                Ok(Some(e)) => Some(e),
                Ok(None) => return Err(ParseError::new("Expected condition after 'if'")),
                Err(e) => return Err(e),
            }
        } else {
            None
        };
        
        // Consume the closing bracket
        if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']' after list comprehension.") {
            return Err(parse_error);
        }
        
        // Create comprehension node and wrap it in a list
        let comprehension_node = ListComprehensionNode::new(self.previous().line, expr, target, iter, condition);
        let comp_expr = ExprType::ListComprehensionExprT { 
            list_comprehension_node: comprehension_node 
        };
        
        // Return as a ListNode with a single comprehension expression
        Ok(ListNode::new(self.previous().line, vec![comp_expr]))
    }

    /* --------------------------------------------------------------------- */

    // expr_list -> '(' expression* ')'

    fn expr_list(&mut self) -> Result<Option<ExprType>, ParseError> {
        // Set flag to prevent comma-separated values from being parsed as tuples
        let was_parsing_collection = self.is_parsing_collection;
        self.is_parsing_collection = true;
        
        let mut expressions: Vec<ExprType> = Vec::new();

        loop {
            if self.match_token(&[TokenType::RParen]) {
                break;
            }
            
            // Check for unpacking operator in function arguments
            if self.match_token(&[TokenType::Star]) {
                // Parse unpacked expression
                match self.expression() {
                    Ok(Some(expr)) => {
                        let unpack_node = UnpackExprNode::new(self.previous().line, expr);
                        expressions.push(ExprType::UnpackExprT { unpack_expr_node: unpack_node });
                    }
                    Ok(None) => {
                        self.is_parsing_collection = was_parsing_collection;
                        return Err(ParseError::new("Expected expression after '*' operator"));
                    }
                    Err(parse_error) => {
                        self.is_parsing_collection = was_parsing_collection;
                        return Err(parse_error);
                    }
                }
            } else {
                // Regular expression
                match self.expression() {
                    Ok(Some(expression)) => {
                        expressions.push(expression);
                    }
                    // should see a list of valid expressions until ')'
                    Ok(None) => {
                        self.is_parsing_collection = was_parsing_collection;
                        return Ok(None);
                    }
                    Err(parse_error) => {
                        self.is_parsing_collection = was_parsing_collection;
                        return Err(parse_error);
                    }
                }
            }
            
            if self.peek().token_type == TokenType::RParen {
                continue;
            }
            if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
                return Err(parse_error);
            }
        }

        // Restore the flag before returning
        self.is_parsing_collection = was_parsing_collection;
        
        if expressions.is_empty() {
            Ok(None)
        } else {
            let expr_list = ExprListT {
                expr_list_node: ExprListNode::new(expressions),
            };
            Ok(Some(expr_list))
        }
    }

    // Parse expression list or tuple - returns tuple if has trailing comma or multiple elements
    fn expr_list_or_tuple(&mut self) -> Result<Option<ExprType>, ParseError> {
        // v0.53: Set flag to indicate we're inside a collection literal (tuples)
        let was_parsing_collection = self.is_parsing_collection;
        self.is_parsing_collection = true;
        
        let mut expressions: Vec<ExprType> = Vec::new();
        let mut has_trailing_comma = false;

        // Handle empty case
        if self.peek().token_type == TokenType::RParen {
            self.advance();
            // Empty () is an empty tuple
            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
            return Ok(Some(TupleLiteralT {
                tuple_literal_node: TupleLiteralNode::new(self.previous().line, Vec::new()),
            }));
        }

        // Parse first expression
        match self.expression() {
            Ok(Some(expr)) => {
                // v0.42: Check for generator expression (expr for var in iterable)
                if self.peek().token_type == TokenType::For {
                    self.advance(); // consume 'for'
                    
                    // Parse target variable
                    let target = if self.peek().token_type == TokenType::Identifier {
                        let token = self.advance();
                        token.lexeme.clone()
                    } else {
                        return Err(ParseError::new("Expected identifier after 'for' in generator expression"));
                    };
                    
                    // Expect 'in'
                    if !self.match_token(&[TokenType::In]) {
                        return Err(ParseError::new("Expected 'in' after identifier in generator expression"));
                    }
                    
                    // Parse iterable expression
                    let iter = match self.expression() {
                        Ok(Some(e)) => e,
                        Ok(None) => return Err(ParseError::new("Expected iterable expression after 'in'")),
                        Err(e) => return Err(e),
                    };
                    
                    // Optional 'if' condition
                    let condition = if self.match_token(&[TokenType::If]) {
                        match self.expression() {
                            Ok(Some(e)) => Some(e),
                            Ok(None) => return Err(ParseError::new("Expected condition after 'if'")),
                            Err(e) => return Err(e),
                        }
                    } else {
                        None
                    };
                    
                    // Expect closing parenthesis
                    if !self.match_token(&[TokenType::RParen]) {
                        self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                        return Err(ParseError::new("Expected ')' after generator expression"));
                    }
                    
                    let generator_expr_node = GeneratorExprNode::new(self.previous().line, expr, target, iter, condition);
                    self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                    return Ok(Some(GeneratorExprT { generator_expr_node }));
                }
                
                expressions.push(expr);
            }
            Ok(None) => {
                // No expression, just close paren
                if self.match_token(&[TokenType::RParen]) {
                    self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                    return Ok(Some(TupleLiteralT {
                        tuple_literal_node: TupleLiteralNode::new(self.previous().line, Vec::new()),
                    }));
                }
                self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                return Err(ParseError::new("Expected expression in parentheses"));
            }
            Err(e) => {
                self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                return Err(e)
            },
        }

        // Check for comma (which makes it a tuple) or close paren
        if self.match_token(&[TokenType::RParen]) {
            // Single expression without comma - just a parenthesized expression
            if expressions.len() == 1 {
                self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                return Ok(Some(expressions.into_iter().next().unwrap()));
            }
        } else if self.match_token(&[TokenType::Comma]) {
            has_trailing_comma = true;
            
            // Parse remaining elements
            loop {
                // Check for trailing comma before close paren
                if self.peek().token_type == TokenType::RParen {
                    self.advance();
                    break;
                }
                
                // Check for unpacking in tuple
                if self.match_token(&[TokenType::Star]) {
                    match self.expression() {
                        Ok(Some(expr)) => {
                            let unpack_node = UnpackExprNode::new(self.previous().line, expr);
                            expressions.push(ExprType::UnpackExprT { unpack_expr_node: unpack_node });
                        }
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                            return Err(ParseError::new("Expected expression after '*'"))
                        },
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                            return Err(e)
                        },
                    }
                } else {
                    // Regular expression
                    match self.expression() {
                        Ok(Some(expr)) => expressions.push(expr),
                        Ok(None) => break,
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                            return Err(e)
                        },
                    }
                }
                
                if self.match_token(&[TokenType::RParen]) {
                    break;
                }
                
                if !self.match_token(&[TokenType::Comma]) {
                    self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
                    return Err(ParseError::new("Expected ',' or ')' in tuple"));
                }
            }
        } else {
            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
            return Err(ParseError::new("Expected ',' or ')' after expression"));
        }

        // If we have multiple expressions or a trailing comma, it's a tuple
        if expressions.len() > 1 || has_trailing_comma {
            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
            Ok(Some(TupleLiteralT {
                tuple_literal_node: TupleLiteralNode::new(self.previous().line, expressions),
            }))
        } else {
            // Single expression without comma - return the expression itself
            self.is_parsing_collection = was_parsing_collection;  // v0.53: Restore flag
            Ok(Some(expressions.into_iter().next().unwrap()))
        }
    }

    /* --------------------------------------------------------------------- */

    // expr_list -> '(' expression* ')'
    //
    // fn expr_list(&mut self) -> Result<Option<ExprType>, ParseError> {
    //     let mut expressions: Vec<ExprType> = Vec::new();
    //
    //     while !self.match_token(&[TokenType::RParen]) {
    //         match self.expression() {
    //             Ok(Some(expression)) => {
    //                 expressions.push(expression);
    //             }
    //             // should see a list of valid expressions until ')'
    //             Ok(None) => return Ok(None),
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //         if self.peek().token_type == TokenType::Comma {
    //             if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
    //                 return Err(parse_error);
    //             }
    //         }
    //     }
    //
    //     let expr_list = ExprListT {
    //         expr_list_node: ExprListNode::new(expressions),
    //     };
    //     Ok(Some(expr_list))
    // }

    /* --------------------------------------------------------------------- */
    //
    // fn post_inc_dec_expression(&mut self, expr_t: &mut ExprType) -> Result<(), ParseError> {
    //     let mut inc_dec = IncDecExpr::None;
    //
    //     if self.match_token(&[TokenType::PlusPlus]) {
    //         inc_dec = IncDecExpr::PostInc;
    //     } else if self.match_token(&[TokenType::DashDash]) {
    //         inc_dec = IncDecExpr::PostDec;
    //     }
    //
    //     match expr_t {
    //         ExprType::CallChainExprT {
    //             ref mut call_chain_expr_node,
    //         } => {
    //             call_chain_expr_node.inc_dec = inc_dec;
    //         }
    //         _ => {
    //             let err_msg = "Expression can not be auto incremented or decrecmented";
    //             self.error_at_current(err_msg);
    //             return Err(ParseError::new(err_msg));
    //         }
    //     }
    //
    //     return Ok(());
    // }

    /* --------------------------------------------------------------------- */

    // TODO: create a new return type that is narrowed to just the types this method returns.
    // TODO: change the return type to be CallChainLiteralExprT as it doesn't return anything else.

    // Immediately before this method is called, the parser matched an identifier. Calls have
    // precedence over variables or properties so try to parse a call or call chain and discover
    // variables or properties as a byproduct.
    // See https://craftinginterpreters.com/classes.html#properties-on-instances

    // This returns either a CallChainExprT or EnumeratorExprT or None.
    // TODO: create new enum to narrow the ExprTypes possible to return.

    fn call(
        &mut self,
        explicit_scope: IdentifierDeclScope,
    ) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::ExprType::CallChainExprT;
        let mut scope: IdentifierDeclScope = explicit_scope.clone();
        let mut call_chain: std::collections::VecDeque<CallChainNodeType> =
            std::collections::VecDeque::new();

        // let debug_id_token = self.previous().lexeme.clone();

        if self.previous().token_type == TokenType::Self_ {
            if self.match_token(&[TokenType::Dot]) {
                if self.match_token(&[TokenType::Identifier]) {

                    let id_node = IdentifierNode::new(
                        self.previous().clone(),
                        None,
                        scope.clone(),
                        false,
                        self.previous().line,
                    );

                    let node = match self.arcanum.get_system_symbol(id_node.name.lexeme.as_str()) {
                        Ok(SystemSymbolType::DomainSymbol {
                               domain_scope_symbol_rcref,
                           }) => {
                            let x = Some(domain_scope_symbol_rcref.clone());
                            let var_node = VariableNode::new(id_node.line, 
                                id_node,
                                IdentifierDeclScope::DomainBlockScope,
                                x,
                            );
                            CallChainNodeType::VariableNodeT { var_node }
                            // CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                        }
                        Ok(SystemSymbolType::OperationSymbol { .. }) => {
                            let operation_ref_expr_node =
                                OperationRefExprNode::new(id_node.line, id_node.name.lexeme.clone());
                            CallChainNodeType::OperationRefT {
                                operation_ref_expr_node,
                            }
                        }
                        Ok(SystemSymbolType::ActionSymbol { .. }) => {
                            // TODO!! Finish this. Add ActionRefT.
                            CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                        }
                        Ok(SystemSymbolType::InterfaceSymbol { .. }) => {
                            // TODO!! Finish this. Add InterfaceRefT.
                            CallChainNodeType::UndeclaredIdentifierNodeT { id_node }

                        }
                        Err(_err) => {
                            // For unrecognized identifiers, just pass them through as undeclared
                            // This allows calling built-in functions like print() or external libraries
                            CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                        }
                    };

                    call_chain.push_back(node);

                    // We've parsed the system field so now scope is unknown.

                    scope = IdentifierDeclScope::UnknownScope;
                    
                    // eprintln!("DEBUG: After self.identifier, current token: {:?}", self.peek());

                    // After parsing self.something, we need to continue the chain
                    // The main loop below will handle additional dots and method calls
                    // TODO: Add proper context tracking for static operations
                    // For now, we'll allow self.method() calls but document the need for validation
                    
                    if self.debug_mode {
                        // eprintln!("DEBUG: After self.property parsed, checking for LParen. Current token: {:?}", self.peek());
                    }
                    
                    // The method identifier was already added to call_chain, now convert it to a call
                    if self.match_token(&[TokenType::LParen]) {
                        // Remove the last node (which is the method identifier)
                        if let Some(last_node) = call_chain.pop_back() {
                            // Get the method name from the last node
                            let method_id = match last_node {
                                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                                    id_node
                                }
                                CallChainNodeType::OperationRefT { operation_ref_expr_node } => {
                                    // Create an identifier node from the operation ref
                                    let mut token = self.previous().clone();
                                    token.lexeme = operation_ref_expr_node.name.clone();
                                    IdentifierNode::new(
                                        token,
                                        None,
                                        scope.clone(),
                                        false,
                                        self.previous().line,
                                    )
                                }
                                _ => {
                                    // Put it back and continue to main loop
                                    call_chain.push_back(last_node);
                                    // Continue to main loop - create a dummy identifier
                                    IdentifierNode::new(
                                        self.previous().clone(),
                                        None,
                                        scope.clone(),
                                        false,
                                        self.previous().line,
                                    )
                                }
                            };
                            
                            // Now finish the call with the method identifier
                            let call_expr_node_result = self.finish_call(method_id);
                            match call_expr_node_result {
                                Ok(call_expr_node) => {
                                    // Determine if this is an interface or operation call
                                    let method_name = call_expr_node.get_name();
                                    
                                    // Check if it's an interface method
                                    if let Some(interface_method_symbol_rcref) = 
                                        self.arcanum.lookup_interface_method(&method_name) {
                                        let node = self.create_interface_method_call_node(
                                            call_expr_node,
                                            &interface_method_symbol_rcref,
                                            CallOrigin::External,
                                        );
                                        call_chain.push_back(node);
                                    } else if let Some(operation_symbol_rcref) = 
                                        self.arcanum.lookup_operation(&method_name) {
                                        let node = self.create_operation_call_node(
                                            call_expr_node,
                                            &operation_symbol_rcref,
                                        );
                                        call_chain.push_back(node);
                                    } else {
                                        // Not an interface method or operation, just an undeclared call
                                        call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                                            call_node: call_expr_node,
                                        });
                                    }
                                    
                                    // v0.37: Check if there's more to the chain (like .method() after self.property)
                                    // Continue parsing dots and method calls
                                    while self.match_token(&[TokenType::Dot]) {
                                        if !self.match_token(&[TokenType::Identifier]) {
                                            let err_msg = "Expected identifier after '.'";
                                            return Err(ParseError::new(err_msg));
                                        }
                                        
                                        let next_id = IdentifierNode::new(
                                            self.previous().clone(),
                                            None,
                                            IdentifierDeclScope::UnknownScope,
                                            false,
                                            self.previous().line,
                                        );
                                        
                                        // Check if it's a method call
                                        if self.match_token(&[TokenType::LParen]) {
                                            let call_expr = self.finish_call(next_id)?;
                                            call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                                                call_node: call_expr,
                                            });
                                        } else {
                                            // Just a property access
                                            call_chain.push_back(CallChainNodeType::UndeclaredIdentifierNodeT {
                                                id_node: next_id,
                                            });
                                        }
                                    }
                                    
                                    let call_chain_expr_node = CallChainExprNode::new(self.previous().line, call_chain);
                                    return Ok(Some(CallChainExprT {
                                        call_chain_expr_node,
                                    }));
                                }
                                Err(parse_error) => return Err(parse_error),
                            }
                        }
                    } else {
                        // v0.37: No immediate method call after self.property
                        // But we still need to check for further dots (e.g., self.processed_data.append)
                        // if self.debug_mode {
                        //     eprintln!("DEBUG: After self.property, checking for continuation. Current token: {:?}", self.peek());
                        // }
                        while self.match_token(&[TokenType::Dot]) {
                            if !self.match_token(&[TokenType::Identifier]) {
                                let err_msg = "Expected identifier after '.'";
                                return Err(ParseError::new(err_msg));
                            }
                            
                            let next_id = IdentifierNode::new(
                                self.previous().clone(),
                                None,
                                IdentifierDeclScope::UnknownScope,
                                false,
                                self.previous().line,
                            );
                            
                            // Check if it's a method call
                            if self.match_token(&[TokenType::LParen]) {
                                let call_expr = self.finish_call(next_id)?;
                                call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                                    call_node: call_expr,
                                });
                            } else {
                                // Just a property access
                                call_chain.push_back(CallChainNodeType::UndeclaredIdentifierNodeT {
                                    id_node: next_id,
                                });
                            }
                        }
                        
                        // IMPORTANT: Always return the call chain here, even if no further dots were found
                        // We've already parsed self.property and need to return that chain
                        let call_chain_expr_node = CallChainExprNode::new(self.previous().line, call_chain);
                        return Ok(Some(CallChainExprT {
                            call_chain_expr_node,
                        }));
                    }
                    // Continue to main loop if no property after self
                } else {
                    let err_msg = format!("Expected Identifier. Found '{}'.", self.previous().lexeme);
                    self.error_at_previous(&err_msg);
                    let parse_error =
                        ParseError::new(err_msg.as_str());
                    return Err(parse_error);
                }
            } else {
                let self_expr_node = SelfExprNode::new(self.previous().line);
                return Ok(Some(SelfExprT {self_expr_node}));
            }
        }


        let mut id_node = IdentifierNode::new(
            self.previous().clone(),
            None,
            scope.clone(),
            false,
            self.previous().line,
        );


        // Loop over the tokens looking for "callable" tokens (methods and identifiers)
        // separated by '.' and build the "call_chain".

        let mut is_first_node = true;
        loop {
            // test for a call. "id(..."
            // let debug_name = format!("id = {}" , id_node.name.lexeme.clone());

            if self.match_token(&[TokenType::LParen]) {
                // For simple function calls, we'll let the UndeclaredCallT handle everything
                
                // v0.30: Try the new simplified call chain method for external functions ONLY
                // Use V2 only for: External functions (first node, empty chain)
                // Do NOT use V2 for method calls as it breaks the object-method relationship
                if is_first_node && call_chain.is_empty() {
                    // Rewind the LParen token since build_call_chain_v2 expects to parse it
                    self.current -= 1;
                    // eprintln!("DEBUG: Calling build_call_chain_v2_with_existing from call() for: {}", id_node.name.lexeme);
                    let result = self.build_call_chain_v2_with_existing(id_node, call_chain);
                    return result;
                }
                
                let call_expr_node_result = self.finish_call(id_node);
                match call_expr_node_result {
                    Ok(call_expr_node) => {
                        if !self.is_building_symbol_table {
                            if !is_first_node {
                                // TODO - review if this can be factored out or simplified.
                                // This code determines if the call is a call to a system interface or operation.
                                // If so it sets interface_method_symbol_rcref_opt to Some() which
                                // will add an InterfaceMethodCallT rather than a UndeclaredCallT
                                // to the call chain.
                                //
                                // interface_method_symbol_rcref_opt is used to indicate this.

                                let mut interface_method_symbol_rcref_opt = None;
                                let mut operation_symbol_rcref_opt = None;
                                // get the previous node to see what kind it was
                                let call_chain_node_type_opt = call_chain.get(call_chain.len() - 1);
                                if let Some(call_chain_node_type) = call_chain_node_type_opt {
                                    match call_chain_node_type {
                                        CallChainNodeType::VariableNodeT { var_node } => {
                                            let value = var_node.get_value();

                                            match &*value {
                                                // Test if var_node value references a system.
                                                ExprType::SystemInstanceExprT { .. } => {
                                                    // Determine if call is to an interface method.
                                                    if let Some(interface_method_symbol_rcref) = 
                                                        self.arcanum.lookup_interface_method(call_expr_node.get_name()) {
                                                        interface_method_symbol_rcref_opt = Some(interface_method_symbol_rcref.clone());
                                                        // Validation is handled when we create the call chain node later
                                                    }
                                                    // now check if the call was to a system operation
                                                    if let Some(operation_symbol_rcref) = 
                                                        self.arcanum.lookup_operation(call_expr_node.get_name()) {
                                                        operation_symbol_rcref_opt = Some(operation_symbol_rcref.clone());
                                                        // Validation is handled when we create the call chain node later
                                                    }
                                                    if interface_method_symbol_rcref_opt.is_none()
                                                        && operation_symbol_rcref_opt.is_none()
                                                    {
                                                        let err_msg = format!("Call to '{}' not found on '{}' system.", call_expr_node.get_name(), var_node.get_name());
                                                        self.error_at_previous(&err_msg);
                                                    }
                                                }
                                                _ => {}
                                            }
                                            // match &var_node.symbol_type_rcref_opt {
                                            //     Some(symbol_t_rcref) => {
                                            //         let symbol_t = symbol_t_rcref.borrow();
                                            //         match &*symbol_t {
                                            //             SymbolType::System {
                                            //                 system_symbol_rcref,
                                            //             } => {
                                            //                 interface_method_symbol_rcref_opt =
                                            //                     system_symbol_rcref
                                            //                         .borrow()
                                            //                         .get_interface_method(
                                            //                             call_expr_node.get_name(),
                                            //                         );
                                            //                 if interface_method_symbol_rcref_opt
                                            //                     .is_none()
                                            //                 {
                                            //                     // this call is to an interface method but it doesn't
                                            //                     // exist on the system
                                            //                     let err_msg = &format!("Interface method {} not found on {}.", call_expr_node.get_name(), var_node.get_name());
                                            //                     self.error_at_current(err_msg);
                                            //                 }
                                            //             }
                                            //             _ => {}
                                            //         }
                                            //     }
                                            //     None => {}
                                            // }
                                        }
                                        _ => {}
                                    }
                                }

                                // Use helper functions to create the appropriate call chain node
                                let call_t = if let Some(interface_method_symbol_rcref) = interface_method_symbol_rcref_opt {
                                    self.create_interface_method_call_node(
                                        call_expr_node,
                                        &interface_method_symbol_rcref,
                                        CallOrigin::External,
                                    )
                                } else if let Some(operation_symbol_rcref) = operation_symbol_rcref_opt {
                                    self.create_operation_call_node(
                                        call_expr_node,
                                        &operation_symbol_rcref,
                                    )
                                } else {
                                    CallChainNodeType::UndeclaredCallT {
                                        call_node: call_expr_node,
                                    }
                                };

                                call_chain.push_back(call_t);
                            } else {
                                // is first or only node in a call chain. For simple external calls, just add as UndeclaredCallT
                                let _method_name = call_expr_node.identifier.name.lexeme.clone();
                                // eprintln!("DEBUG PARSER: Creating UndeclaredCallT for simple function call '{}'", method_name);
                                let call_t = CallChainNodeType::UndeclaredCallT {
                                    call_node: call_expr_node,
                                };
                                call_chain.push_back(call_t);
                                // eprintln!("DEBUG PARSER: Added UndeclaredCallT to call_chain, new length: {}", call_chain.len());
                                
                                // For now, skip the complex action/interface lookup logic for simple calls
                                /*
                                // Debug dump before symbol lookup
                                if std::env::var("FRAME_DEBUG").is_ok() {
                                    // eprintln!(">>> PARSER: About to lookup action '{}' in call chain", method_name);
                                    self.arcanum.debug_dump_arcanum();
                                }
                                
                                let action_decl_symbol_opt =
                                    self.arcanum.lookup_action(&method_name);

                                // eprintln!("DEBUG PARSER: Looking up action '{}', result: {:?}", method_name, action_decl_symbol_opt.is_some());
                                match action_decl_symbol_opt {
                                    Some(ads) => {
                                        // eprintln!("DEBUG PARSER: Found action, processing...");
                                        
                                        // SCOPE CHECK: Functions cannot call actions
                                        match self.arcanum.scope_context {
                                            ScopeContext::Function(_) => {
                                                // Functions cannot call actions - treat as undeclared call
                                                // eprintln!("DEBUG PARSER: In function scope, cannot call action '{}', treating as undeclared", method_name);
                                                let call_t = CallChainNodeType::UndeclaredCallT {
                                                    call_node: call_expr_node,
                                                };
                                                call_chain.push_back(call_t);
                                            }
                                            _ => {
                                                // In system or global context, can call actions
                                                // first node is an action

                                                let action_symbol_opt =
                                                    self.arcanum.lookup_action(&method_name);

                                                match action_symbol_opt {
                                                    Some(action_scope_symbol_rcref) => {
                                                        // TODO - factor out arg/param validation into a utility function.
                                                        // validate args/params

                                                        let action_scope_symbol =
                                                            action_scope_symbol_rcref.borrow();
                                                        let action_node_rcref = action_scope_symbol
                                                            .ast_node_opt
                                                            .as_ref()
                                                            .unwrap();
                                                        let parameter_node_vec_opt =
                                                            &action_node_rcref.borrow().params;
                                                        // check if difference in the existance of parameters
                                                        if (!parameter_node_vec_opt.is_none()
                                                            && call_expr_node
                                                                .call_expr_list
                                                                .exprs_t
                                                                .is_empty())
                                                            || (parameter_node_vec_opt.is_none()
                                                                && !call_expr_node
                                                                    .call_expr_list
                                                                    .exprs_t
                                                                    .is_empty())
                                                        {
                                                            let err_msg = format!("Incorrect number of arguments for action '{}'.", method_name);
                                                            self.error_at_previous(&err_msg);
                                                            // let parse_error =
                                                            //     ParseError::new(err_msg.as_str());
                                                            // return Err(parse_error);
                                                        } else {
                                                            // check parameter count equals argument count
                                                            match &parameter_node_vec_opt {
                                                                Some(symbol_params) => {
                                                                    if symbol_params.len()
                                                                        != call_expr_node
                                                                            .call_expr_list
                                                                            .exprs_t
                                                                            .len()
                                                                    {
                                                                        let err_msg = format!("Number of arguments does not match parameters for action '{}'.", method_name);
                                                                        self.error_at_previous(&err_msg);
                                                                    }
                                                                }
                                                                None => {}
                                                            }
                                                        }

                                                        let mut action_call_expr_node =
                                                            ActionCallExprNode::new(call_expr_node);
                                                        action_call_expr_node
                                                            .set_action_symbol(&Rc::clone(&ads));
                                                        call_chain.push_back(
                                                            CallChainNodeType::ActionCallT {
                                                                action_call_expr_node,
                                                            },
                                                        );
                                                    }
                                                    None => {
                                                        // first node is not an action or interface call.
                                                        let call_t = CallChainNodeType::UndeclaredCallT {
                                                            call_node: call_expr_node,
                                                        };
                                                        call_chain.push_back(call_t);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        // first node is not an action. see if it is an interface call
                                        let interface_method_symbol_opt =
                                            self.arcanum.lookup_interface_method(&method_name);

                                        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                                            eprintln!("DEBUG PARSER: Looking up interface method '{}', result: {:?}", method_name, interface_method_symbol_opt.is_some());
                                        }
                                        if let Some(interface_method_symbol) = interface_method_symbol_opt {
                                            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                                                eprintln!("DEBUG PARSER: Found interface method, processing...");
                                            }
                                            // first node is an interface call.
                                            if self.is_action_scope {
                                                // iface calls disallowed in actions.
                                                let err_msg = format!("Interface calls disallowed inside of actions.");
                                                self.error_at_current(&err_msg);
                                            }
                                            
                                            // Use helper to create interface method call with validation
                                            let node = self.create_interface_method_call_node(
                                                call_expr_node,
                                                &interface_method_symbol,
                                                CallOrigin::Internal,
                                            );
                                            call_chain.push_back(node);
                                        } else {
                                                // first node is not an action or interface call.
                                                let call_t = CallChainNodeType::UndeclaredCallT {
                                                    call_node: call_expr_node,
                                                };
                                                call_chain.push_back(call_t);
                                            }
                                        }
                                    }
                                }
                                */
                            }
                        }
                    }
                    _ => return Err(ParseError::new("TODO")),
                }
            } else {
                // Not a call so just an identifier which may be a variable
                // or a reference to a system function.

                if explicit_scope != IdentifierDeclScope::SystemScope {
                    // This is a very subtle but important part of the logic here.
                    // We do a lookup for the id to find what scope it is in, if any.
                    // If there is a scope, it is a variable and the scope will be
                    // added to the Variable node so the parser and
                    // visitors will know what kind of variable it is.
                    match self.get_identifier_scope(&id_node, &explicit_scope) {
                        Ok(id_decl_scope) => scope = id_decl_scope,
                        Err(err) => return Err(err),
                    }
                } else {
                    // TODO - validate this
                    // scope = explicit_scope.clone();
                }

                // Variables must be the first "node" in a get expression. See https://craftinginterpreters.com/classes.html#properties-on-instances.

                // let debug_name = &id_node.name.lexeme.clone();
                let node = if is_first_node {
                    // Variables, parameters and enums must be
                    // the first (or only) node in the call chain

                    let symbol_name = format!("{}", &id_node.name.lexeme);

                    // this needs to fail if we pass a symbol known to be on the system and the lookup fails.
                    // if scope == IdentifierDeclScope::SystemScope {
                    //     // TODO v0.20 - remove this as it is now implemented at the top of the fn
                    //     // match self.arcanum.get_system_symbol(id_node.name.lexeme.as_str()) {
                    //     //     Ok(SystemSymbolType::DomainSymbol {
                    //     //            domain_scope_symbol_rcref,
                    //     //        }) => {
                    //     //         let x = Some(domain_scope_symbol_rcref.clone());
                    //     //         let var_node = VariableNode::new(
                    //     //             id_node,
                    //     //             IdentifierDeclScope::DomainBlockScope,
                    //     //             x,
                    //     //         );
                    //     //         CallChainNodeType::VariableNodeT { var_node }
                    //     //         // CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                    //     //     }
                    //     //     Ok(SystemSymbolType::OperationSymbol { .. }) => {
                    //     //         let operation_ref_expr_node =
                    //     //             OperationRefExprNode::new(id_node.name.lexeme.clone());
                    //     //         CallChainNodeType::OperationRefT {
                    //     //             operation_ref_expr_node,
                    //     //         }
                    //     //     }
                    //     //     Ok(SystemSymbolType::ActionSymbol { .. }) => {
                    //     //         // TODO!! Finish this. Add ActionRefT.
                    //     //         CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                    //     //     }
                    //     //     Ok(SystemSymbolType::InterfaceSymbol { .. }) => {
                    //     //         // TODO!! Finish this. Add InterfaceRefT.
                    //     //         CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                    //     //     }
                    //     //     Err(err) => {
                    //     //         self.error_at_current(err.as_str());
                    //     //         CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                    //     //     }
                    //     // }
                    // } else {
                        // Lookup the symbol name in the arcanum and then return a node
                        // based on if this is a known or unknown symbol.

                        let symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>> =
                            self.arcanum.lookup(&symbol_name, &explicit_scope).clone();
                        let call_chain_node_t = match &symbol_type_rcref_opt {
                            // found symbol
                            Some(symbol_t) => {
                                match &*symbol_t.borrow() {
                                    // node is Enumeration decl
                                    SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                                        let enum_symbol = enum_symbol_rcref.borrow();

                                        let _enum_decl_node =
                                            enum_symbol.ast_node_opt.as_ref().unwrap().borrow();
                                        
                                        // Check if there's a dot for enum member access
                                        // If not, this is a reference to the enum type itself (e.g., for iteration)
                                        if !self.match_token(&[TokenType::Dot]) {
                                            // Return just the enum type identifier (for iteration over enum)
                                            let var_node = VariableNode::new(id_node.line, 
                                                id_node,
                                                IdentifierDeclScope::UnknownScope,
                                                symbol_type_rcref_opt.clone(),
                                            );
                                            CallChainNodeType::VariableNodeT { var_node }
                                        } else {
                                            // Enum member access (e.g., MenuOption.Start)
                                            if self.match_token(&[TokenType::Identifier]) {
                                            let enumerator_name = &self.previous().lexeme;
                                            let mut found_enumerator = false;
                                            for enum_decl_node in &enum_symbol
                                                .ast_node_opt
                                                .as_ref()
                                                .unwrap()
                                                .borrow()
                                                .enums
                                            {
                                                if *enumerator_name == enum_decl_node.name {
                                                    found_enumerator = true;
                                                    break;
                                                }
                                            }
                                            if !found_enumerator {
                                                let msg = &format!(
                                                    "Expected enumerator for {} - found {}.",
                                                    enum_symbol.name, enumerator_name,
                                                );
                                                self.error_at_current(msg);
                                                return Err(ParseError::new(msg));
                                            }

                                                // Create an EnumeratorExprNode for the enum member access
                                                // This is the proper way to represent enum member access in the AST
                                                // The visitor will then properly qualify the enum name with the system name
                                                let enum_expr_node = EnumeratorExprNode::new(self.previous().line, 
                                                    enum_symbol.name.clone(),
                                                    enumerator_name.clone(),
                                                );
                                                
                                                // Store this so we can return it as an EnumeratorExprT
                                                // We need to break out of the normal call chain processing
                                                // and return the enum expression directly
                                                call_chain.clear(); // Clear any nodes we've collected
                                                return Ok(Some(EnumeratorExprT { enum_expr_node }));
                                            } else {
                                                return Err(ParseError::new("Expected enum member name after '.'"));
                                            }
                                        }
                                    }
                                    // TODO!!! Need to figure out how parameters should work wrt
                                    // setting their values in assignments. Parameters are different
                                    // than variables as THEY ARE NOT INITIALIZED in a variable
                                    // declaration. So lumping them in with variables is likely a
                                    // problem.

                                    // #STATE_NODE_UPDATE_BUG - Not updating the symbol table in the semantic pass
                                    // with resolved AST value for variables caused a very subtle bug
                                    // that resulted in the node always being UndeclaredIdentifierNodeT
                                    // rather than being updadted to VariableNodeT in the semantic pass.
                                    // Search on #STATE_NODE_UPDATE_BUG  to see other areas that are
                                    // related to his particular problem.

                                    // TODO!! It is a general point of failure
                                    // and source of fragility for the compiler that the sympol table references
                                    // AST nodes from the semantic pass to resolve parameters and types.
                                    SymbolType::BlockVar { .. }
                                    | SymbolType::DomainVariable { .. }
                                    | SymbolType::StateVariable { .. }
                                    | SymbolType::EventHandlerVariable { .. }
                                    | SymbolType::ParamSymbol { .. }
                                    | SymbolType::StateParam { .. }
                                    | SymbolType::EventHandlerParam { .. } => {
                                        if self.match_token(&[TokenType::LBracket]) {
                                            // Check if this is a slice or regular index
                                            let bracket_result = self.parse_bracket_expression();
                                            match bracket_result {
                                                Ok(BracketExpressionType::Index(expr)) => {
                                                    let list_elem_node = ListElementNode::new(
                                                        id_node,
                                                        scope.clone(),
                                                        expr,
                                                    );
                                                    CallChainNodeType::ListElementNodeT {
                                                        list_elem_node,
                                                    }
                                                }
                                                Ok(BracketExpressionType::Slice { start, end, step }) => {
                                                    let slice_node = SliceNode {
                                            line: self.previous().line,
                                                        identifier: id_node,
                                                        scope: scope.clone(),
                                                        start_expr: start,
                                                        end_expr: end,
                                                        step_expr: step,
                                                    };
                                                    CallChainNodeType::SliceNodeT {
                                                        slice_node,
                                                    }
                                                }
                                                Err(err) => return Err(err),
                                            }
                                        } else {
                                            let var_node = VariableNode::new(id_node.line, 
                                                id_node,
                                                scope.clone(),
                                                (&symbol_type_rcref_opt).clone(),
                                            );
                                            CallChainNodeType::VariableNodeT { var_node }
                                        }
                                    }
                                    // SymbolType::ParamSymbol {..} |
                                    // SymbolType::StateParam {..} |
                                    // SymbolType::EventHandlerParam {..} => {
                                    //     // TODO - need to support passing Frame types.
                                    //     // See https://github.com/frame-lang/frame_transpiler/issues/151
                                    //     CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                                    // }
                                    _ => CallChainNodeType::UndeclaredIdentifierNodeT { id_node },
                                }
                            }
                            None => {
                                // Check if this could be an imported module (has :: after it)
                                // This allows us to support things like Math::PI, Module::function() 
                                if self.peek().token_type == TokenType::ColonColon {
                                    // This could be an imported module, allow it as an undeclared identifier
                                    // The visitor will handle the actual module resolution
                                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                                } else if self.match_token(&[TokenType::LBracket]) {
                                    // Check if this is a slice or regular index
                                    let bracket_result = self.parse_bracket_expression();
                                    match bracket_result {
                                        Ok(BracketExpressionType::Index(expr)) => {
                                            let list_elem_node = ListElementNode::new(
                                                id_node,
                                                scope.clone(),
                                                expr,
                                            );
                                            CallChainNodeType::ListElementNodeT { list_elem_node }
                                        }
                                        Ok(BracketExpressionType::Slice { start, end, step }) => {
                                            let slice_node = SliceNode {
                                        line: self.previous().line,
                                                identifier: id_node,
                                                scope: scope.clone(),
                                                start_expr: start,
                                                end_expr: end,
                                                step_expr: step,
                                            };
                                            CallChainNodeType::SliceNodeT {
                                                slice_node,
                                            }
                                        }
                                        Err(err) => return Err(err),
                                    }
                                } else {
                                    // v0.38: During both passes, allow undeclared identifiers here
                                    // They could be function references which we'll check later
                                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                                }
                            }
                        };

                        call_chain_node_t
                  //  }
                } else {
                    if self.match_token(&[TokenType::LBracket]) {
                        // Check if this is a slice or regular index
                        let bracket_result = self.parse_bracket_expression();
                        match bracket_result {
                            Ok(BracketExpressionType::Index(expr)) => {
                                let list_elem_node =
                                    ListElementNode::new(id_node, scope.clone(), expr);
                                CallChainNodeType::UndeclaredListElementT { list_elem_node }
                            }
                            Ok(BracketExpressionType::Slice { start, end, step }) => {
                                let slice_node = SliceNode {
                                    line: self.previous().line,
                                    identifier: id_node,
                                    scope: scope.clone(),
                                    start_expr: start,
                                    end_expr: end,
                                    step_expr: step,
                                };
                                CallChainNodeType::UndeclaredSliceT {
                                    slice_node,
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    } else {
                        CallChainNodeType::UndeclaredIdentifierNodeT { id_node }
                    }
                };

                call_chain.push_back(node);
            };

            // Check if the last node was a list element or slice and the next token is '(' or '['
            // This handles patterns like array[0](args), dict["key"](params), or dict["key1"]["key2"]
            loop {
                if let Some(last_node) = call_chain.back() {
                    match last_node {
                        CallChainNodeType::ListElementNodeT { .. } |
                        CallChainNodeType::SliceNodeT { .. } |
                        CallChainNodeType::UndeclaredListElementT { .. } |
                        CallChainNodeType::UndeclaredSliceT { .. } => {
                            // Check for consecutive bracket indexing first
                            if self.match_token(&[TokenType::LBracket]) {
                                // Another indexing operation on the result
                                let bracket_result = self.parse_bracket_expression();
                                match bracket_result {
                                    Ok(BracketExpressionType::Index(expr)) => {
                                        // Create a synthetic identifier for the chained indexing
                                        let synthetic_id = IdentifierNode::new(
                                            Token::new(
                                                TokenType::Identifier,
                                                "@chain_index".to_string(),
                                                TokenLiteral::None,
                                                self.previous().line,
                                                0,
                                                0
                                            ),
                                            None,
                                            IdentifierDeclScope::UnknownScope,
                                            false,
                                            self.previous().line,
                                        );
                                        let list_elem_node = ListElementNode::new(
                                            synthetic_id,
                                            scope.clone(),
                                            expr,
                                        );
                                        call_chain.push_back(CallChainNodeType::UndeclaredListElementT { 
                                            list_elem_node 
                                        });
                                        // Continue checking for more operations
                                        continue;
                                    }
                                    Ok(BracketExpressionType::Slice { start, end, step }) => {
                                        // Handle slice on the result
                                        let synthetic_id = IdentifierNode::new(
                                            Token::new(
                                                TokenType::Identifier,
                                                "@chain_slice".to_string(),
                                                TokenLiteral::None,
                                                self.previous().line,
                                                0,
                                                0
                                            ),
                                            None,
                                            IdentifierDeclScope::UnknownScope,
                                            false,
                                            self.previous().line,
                                        );
                                        let slice_node = SliceNode {
                                        line: self.previous().line,
                                            identifier: synthetic_id,
                                            scope: scope.clone(),
                                            start_expr: start,
                                            end_expr: end,
                                            step_expr: step,
                                        };
                                        call_chain.push_back(CallChainNodeType::UndeclaredSliceT {
                                            slice_node,
                                        });
                                        // Continue checking for more operations
                                        continue;
                                    }
                                    Err(err) => return Err(err),
                                }
                            } else if self.match_token(&[TokenType::LParen]) {
                                // The indexed value is being called as a function
                                // Create a synthetic call node for this
                                
                                // Parse arguments using expr_list() to avoid tuple creation
                                let args = match self.expr_list() {
                                    Ok(Some(ExprListT { expr_list_node })) => expr_list_node.exprs_t,
                                    Ok(Some(_)) => return Err(ParseError::new("Invalid expression list in indexed call")),
                                    Ok(None) => Vec::new(),
                                    Err(e) => return Err(e),
                                };
                                
                                // Create a call expression node for the indexed call
                                let call_expr_list = CallExprListNode::new(args);
                                let mut call_expr_node = CallExprNode::new(
                                    self.previous().line,  // Add line parameter
                                    IdentifierNode::new(
                                        Token::new(
                                            TokenType::Identifier, 
                                            "@indexed_call".to_string(), 
                                            TokenLiteral::None,
                                            self.previous().line,
                                            0,
                                            0
                                        ),
                                        None,
                                        IdentifierDeclScope::UnknownScope,
                                        false,
                                        self.previous().line,
                                    ),
                                    call_expr_list,
                                    None,  // No call chain for the synthetic call
                                );
                                
                                // v0.62: Perform semantic resolution if enabled
                                self.resolve_call_expr(&mut call_expr_node);
                                
                                // Add the call to the chain
                                call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                                    call_node: call_expr_node,
                                });
                            } else {
                                break;  // No more operations to handle
                            }
                        }
                        _ => break,  // No more operations on list element
                    }
                } else {
                    break;  // No more nodes in chain
                }
            }

            // Check for module separator :: or instance member .
            // Module access uses :: while instance members use .
            let is_module_access = self.match_token(&[TokenType::ColonColon]);
            let is_member_access = !is_module_access && self.match_token(&[TokenType::Dot]);
            
            if !is_module_access && !is_member_access {
                break;  // End of chain
            }

            if self.match_token(&[TokenType::Identifier]) {
                id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::UnknownScope,
                    false,
                    self.previous().line,
                );
            } else {
                return Err(ParseError::new("TODO"));
            }
            is_first_node = false;
        }

        
        // v0.38: Check if this is a single function reference (first-class function)
        // If we have a single UndeclaredIdentifierNodeT that's actually a function,
        // return it as FunctionRefT instead of CallChainExprT
        // Check in second pass only to avoid errors during symbol table building
        if call_chain.len() == 1 && !self.is_building_symbol_table {
            if let Some(CallChainNodeType::UndeclaredIdentifierNodeT { id_node }) = call_chain.get(0) {
                let function_symbol_opt = self.arcanum.lookup_function(&id_node.name.lexeme);
                if function_symbol_opt.is_some() {
                    // This is a function being used as a value - return FunctionRefT
                    return Ok(Some(ExprType::FunctionRefT {
                        name: id_node.name.lexeme.clone(),
                    }));
                }
            }
        }
        
        let call_chain_expr_node = CallChainExprNode::new(self.previous().line, call_chain);
        Ok(Some(CallChainExprT {
            call_chain_expr_node,
        }))
    }

    /* --------------------------------------------------------------------- */
    
    // v0.30 Improved call chain parsing method
    // This method provides a cleaner, more maintainable approach to parsing call chains

    // Helper function to validate interface method or operation arguments
    fn validate_call_arguments(
        &mut self,
        call_expr_node: &CallExprNode,
        params_opt: &Option<Vec<ParameterNode>>,
        call_type: &str,
    ) {
        let params_is_none = params_opt.is_none();
        let args_is_empty = call_expr_node.call_expr_list.exprs_t.is_empty();
        
        if (!params_is_none && args_is_empty) || (params_is_none && !args_is_empty) {
            let err_msg = format!(
                "Incorrect number of arguments for {} '{}'.",
                call_type,
                call_expr_node.get_name()
            );
            self.error_at_previous(&err_msg);
        } else if let Some(param_vec) = params_opt {
            if param_vec.len() != call_expr_node.call_expr_list.exprs_t.len() {
                let err_msg = format!(
                    "Expected {} arguments but got {}.",
                    param_vec.len(),
                    call_expr_node.call_expr_list.exprs_t.len()
                );
                self.error_at_previous(&err_msg);
            }
        }
    }

    // Helper function to create interface method call chain node
    fn create_interface_method_call_node(
        &mut self,
        call_expr_node: CallExprNode,
        interface_method_symbol_rcref: &Rc<RefCell<InterfaceMethodSymbol>>,
        origin: CallOrigin,
    ) -> CallChainNodeType {
        // Validate arguments
        let interface_method_symbol = interface_method_symbol_rcref.borrow();
        let interface_method_node_rcref = interface_method_symbol
            .ast_node_opt
            .as_ref()
            .unwrap();
        let parameter_node_vec_opt = &interface_method_node_rcref
            .borrow()
            .params;
        
        self.validate_call_arguments(&call_expr_node, parameter_node_vec_opt, "interface method");
        
        let mut interface_method_call_expr_node = InterfaceMethodCallExprNode::new(call_expr_node.line, 
            call_expr_node,
            origin,
        );
        interface_method_call_expr_node.set_interface_symbol(interface_method_symbol_rcref);
        
        CallChainNodeType::InterfaceMethodCallT {
            interface_method_call_expr_node,
        }
    }

    // Helper function to create operation call chain node
    fn create_operation_call_node(
        &mut self,
        call_expr_node: CallExprNode,
        operation_symbol_rcref: &Rc<RefCell<OperationScopeSymbol>>,
    ) -> CallChainNodeType {
        // Validate arguments
        let operation_symbol = operation_symbol_rcref.borrow();
        let operation_node_rcref = operation_symbol.ast_node_opt.as_ref().unwrap();
        let parameter_node_vec_opt = &operation_node_rcref.borrow().params;
        
        self.validate_call_arguments(&call_expr_node, parameter_node_vec_opt, "operation");
        
        let mut operation_call_expr_node = OperationCallExprNode::new(call_expr_node.line, call_expr_node);
        operation_call_expr_node.set_operation_symbol(operation_symbol_rcref);
        
        CallChainNodeType::OperationCallT {
            operation_call_expr_node,
        }
    }

    // Helper function to validate and create action call node
    fn create_action_call_node(
        &mut self,
        call_expr_node: CallExprNode,
        action_symbol_rcref: &Rc<RefCell<ActionScopeSymbol>>,
    ) -> CallChainNodeType {
        // Validate arguments
        let action_symbol = action_symbol_rcref.borrow();
        let action_decl_node_rcref = action_symbol.ast_node_opt.as_ref().unwrap();
        let parameter_node_vec_opt = &action_decl_node_rcref.borrow().params;
        
        self.validate_call_arguments(&call_expr_node, parameter_node_vec_opt, "action");
        
        let mut action_call_expr_node = ActionCallExprNode::new(call_expr_node.line, call_expr_node);
        action_call_expr_node.set_action_symbol(action_symbol_rcref);
        
        CallChainNodeType::ActionCallT {
            action_call_expr_node,
        }
    }

    // Helper function to parse and add a dot-separated continuation
    fn parse_dot_continuation(&mut self, call_chain: &mut std::collections::VecDeque<CallChainNodeType>) -> Result<bool, ParseError> {
        if !self.match_token(&[TokenType::Dot]) {
            return Ok(false);  // No continuation
        }
        
        if !self.match_token(&[TokenType::Identifier]) {
            return Err(ParseError::new("Expected identifier after '.'"));
        }
        
        let next_id = IdentifierNode::new(
            self.previous().clone(),
            None,
            IdentifierDeclScope::UnknownScope,
            false,
            self.previous().line,
        );
        
        // Check if this is a method call
        if self.match_token(&[TokenType::LParen]) {
            let call_expr_node = self.finish_call(next_id)?;
            call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                call_node: call_expr_node,
            });
        } else {
            call_chain.push_back(CallChainNodeType::UndeclaredIdentifierNodeT {
                id_node: next_id,
            });
        }
        
        Ok(true)  // Continuation found and processed
    }

    fn build_call_chain_v2(&mut self, base_id: IdentifierNode) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::CallChainNodeTypeV2;
        use std::collections::VecDeque;
        
        let mut chain: VecDeque<CallChainNodeTypeV2> = VecDeque::new();
        let mut current_id = base_id;
        
        
        // Parse the rest of the chain
        loop {
            if self.match_token(&[TokenType::LParen]) {
                // It's a call - for external functions, just create the call node
                let call_expr = self.finish_call(current_id)?;
                chain.push_back(CallChainNodeTypeV2::Call {
                    expr: call_expr,
                    target_type: self.determine_call_target_type_v2(&chain),
                });
                break; // Calls end the chain for now
            } else if self.match_token(&[TokenType::Dot]) {
                // It's a member access - continue parsing
                if !self.match_token(&[TokenType::Identifier]) {
                    let err_msg = format!("Expected identifier after '.'");
                    return Err(ParseError::new(&err_msg));
                }
                
                current_id = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::UnknownScope,
                    false,
                    self.previous().line,
                );
                
                chain.push_back(CallChainNodeTypeV2::Identifier {
                    name: current_id.name.lexeme.clone(),
                    scope: IdentifierScope::Unknown, // Will be resolved later
                    line: current_id.name.line,
                });
            } else {
                // No more chain elements - just an identifier
                break;
            }
        }
        
        
        // Convert to legacy CallChainExprNode for compatibility
        let legacy_chain = self.convert_v2_to_legacy_chain(chain)?;
        let call_chain_expr_node = CallChainExprNode::new(self.previous().line, legacy_chain);
        
        Ok(Some(CallChainExprT {
            call_chain_expr_node,
        }))
    }
    
    // Helper method to determine identifier scope using new enums
    fn determine_identifier_scope_v2(&self, _id_node: &IdentifierNode) -> IdentifierScope {
        // For now, return Unknown - this will be enhanced later
        IdentifierScope::Unknown
    }
    
    // Helper method to determine call target type
    fn determine_call_target_type_v2(&self, _chain: &VecDeque<CallChainNodeTypeV2>) -> CallTargetType {
        // For now, return Unknown - this will be enhanced later
        CallTargetType::Unknown
    }
    
    // Convert V2 chain to legacy chain for compatibility
    fn convert_v2_to_legacy_chain(&self, v2_chain: VecDeque<CallChainNodeTypeV2>) -> Result<VecDeque<CallChainNodeType>, ParseError> {
        let mut legacy_chain = VecDeque::new();
        
        for node in v2_chain {
            match node {
                CallChainNodeTypeV2::Identifier { name, line, .. } => {
                    // Create a basic IdentifierNode for the legacy system
                    let id_node = IdentifierNode::new(
                        Token::new(TokenType::Identifier, name.clone(), TokenLiteral::None, line, 0, name.len()),
                        None,
                        IdentifierDeclScope::UnknownScope,
                        false,
                        line,
                    );
                    legacy_chain.push_back(CallChainNodeType::UndeclaredIdentifierNodeT { id_node });
                }
                CallChainNodeTypeV2::Call { expr, .. } => {
                    // Always create UndeclaredCallT here - resolution happens in visitor
                    legacy_chain.push_back(CallChainNodeType::UndeclaredCallT { call_node: expr });
                }
                // For now, convert other types to basic equivalents
                CallChainNodeTypeV2::Variable { var_node } => {
                    legacy_chain.push_back(CallChainNodeType::VariableNodeT { var_node });
                }
                CallChainNodeTypeV2::InterfaceMethod { method_node } => {
                    legacy_chain.push_back(CallChainNodeType::InterfaceMethodCallT { 
                        interface_method_call_expr_node: method_node 
                    });
                }
                CallChainNodeTypeV2::Operation { op_node } => {
                    legacy_chain.push_back(CallChainNodeType::OperationCallT { 
                        operation_call_expr_node: op_node 
                    });
                }
                CallChainNodeTypeV2::Action { action_node } => {
                    legacy_chain.push_back(CallChainNodeType::ActionCallT { 
                        action_call_expr_node: action_node 
                    });
                }
                _ => {
                    // For unhandled types, create a placeholder
                }
            }
        }
        
        Ok(legacy_chain)
    }

    // v0.30: Modified build_call_chain_v2 that can append to existing call chain for method calls
    fn build_call_chain_v2_with_existing(&mut self, base_id: IdentifierNode, mut existing_chain: VecDeque<CallChainNodeType>) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::CallChainNodeTypeV2;
        
        // Debug dump when starting V2 call chain building
        if std::env::var("FRAME_DEBUG").is_ok() {
            eprintln!(">>> PARSER: Starting V2 call chain for '{}'", base_id.name.lexeme);
            self.arcanum.debug_dump_arcanum();
        }
        
        // Continue with regular call chain building
        let mut chain: VecDeque<CallChainNodeTypeV2> = VecDeque::new();
        let current_id = base_id;
        
        // Parse the call
        if self.match_token(&[TokenType::LParen]) {
            // It's a call - for method calls, just create the call node
            let call_expr = self.finish_call(current_id)?;
            chain.push_back(CallChainNodeTypeV2::Call {
                expr: call_expr,
                target_type: self.determine_call_target_type_v2(&chain),
            });
        }
        
        // Convert to legacy and append to existing chain
        let legacy_chain = self.convert_v2_to_legacy_chain(chain)?;
        for node in legacy_chain {
            existing_chain.push_back(node);
        }
        
        let call_chain_expr_node = CallChainExprNode::new(self.previous().line, existing_chain);
        Ok(Some(CallChainExprT {
            call_chain_expr_node,
        }))
    }

    /* --------------------------------------------------------------------- */

    fn get_identifier_scope(
        &mut self,
        identifier_node: &IdentifierNode,
        explicit_scope: &IdentifierDeclScope,
    ) -> Result<IdentifierDeclScope, ParseError> {
        let mut scope: IdentifierDeclScope = IdentifierDeclScope::UnknownScope;
        // find the symbol in the arcanum
        let symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>> = self
            .arcanum
            .lookup(&identifier_node.name.lexeme, explicit_scope);
        match &symbol_type_rcref_opt {
            Some(symbol_type_rcref) => {
                let symbol_type = symbol_type_rcref.borrow();
                match &*symbol_type {
                    SymbolType::DomainVariable {
                        domain_variable_symbol_rcref,
                    } => {
                        scope = domain_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::StateParam {
                        state_param_symbol_rcref,
                    } => {
                        scope = state_param_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::StateVariable {
                        state_variable_symbol_rcref,
                    } => {
                        scope = state_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerVariable {
                        event_handler_variable_symbol_rcref,
                    } => {
                        scope = event_handler_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerParam {
                        event_handler_param_symbol_rcref,
                    } => {
                        scope = event_handler_param_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerLocalScope { .. } => {
                        scope = IdentifierDeclScope::UnknownScope;
                    }
                    SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                        scope = enum_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::LoopVar {
                        loop_variable_symbol_rcref,
                    } => {
                        scope = loop_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::BlockVar {
                        block_variable_symbol_rcref,
                    } => {
                        scope = block_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::ModuleVariable {
                        module_variable_symbol_rcref,
                    } => {
                        scope = module_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerScope { .. } => {
                        // TODO - what??
                        // this will be a lookup for a varible that clashes with
                        // the name of an event. Disregard.
                        // scope = loop_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::System { .. } => {
                        scope = IdentifierDeclScope::UnknownScope;
                    }
                    SymbolType::FunctionScope { .. } => {
                        // v0.38: Functions can be used as values (first-class functions)
                        scope = IdentifierDeclScope::UnknownScope;
                    }
                    SymbolType::Module { .. } => {
                        // v0.57: Modules can be accessed with :: separator
                        scope = IdentifierDeclScope::UnknownScope;
                    }
                    _ => {
                        // scope = IdentifierDeclScope::None;
                        let msg =
                            &format!("Error - unknown scope identifier {}.", identifier_node.name);
                        self.error_at_current(msg);
                        return Err(ParseError::new(msg));

                        // return Err(ParseError::new(&format!("Error - unknown scope identifier {}.",identifier_node.name)));
                    }
                }
            }
            None => {}
        };

        if !self.is_building_symbol_table
            && *explicit_scope != IdentifierDeclScope::UnknownScope
            && *explicit_scope != scope
        {
            let msg = &format!(
                "Identifier {} - invalid scope identifier.",
                identifier_node.name.lexeme
            );
            self.error_at_current(msg);
            return Err(ParseError::new(msg));
        }

        Ok(scope)
    }

    /* --------------------------------------------------------------------- */

    fn finish_call(&mut self, identifer_node: IdentifierNode) -> Result<CallExprNode, ParseError> {
        let call_expr_list_node;
        match self.expr_list() {
            Ok(Some(ExprListT { expr_list_node })) => {
                // need to differentiate between regular expression lists and call expression lists
                // for formatting.
                call_expr_list_node = CallExprListNode::new(expr_list_node.exprs_t);
                //    call_expr_list_node = CallExprListT {call_expr_list_node};
            }
            Ok(Some(_)) => return Err(ParseError::new("Invalid call expression list.")),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => call_expr_list_node = CallExprListNode::new(Vec::new()),
        }

        // Determine call context based on identifier prefix
        // Note: "system." is no longer possible here as it's handled by SystemReturn token
        let call_context = if identifer_node.name.lexeme.starts_with("self.") {
            CallContextType::SelfCall
        } else {
            CallContextType::ExternalCall  // Default
        };

        let mut call_expr_node = CallExprNode::new_with_context(identifer_node.line, identifer_node, call_expr_list_node, None, call_context);
        
        // v0.62: Perform semantic resolution if enabled
        self.resolve_call_expr(&mut call_expr_node);
        
        //        let method_call_expression_type = ExpressionType::MethodCallExprType {method_call_expr_node};
        Ok(call_expr_node)
    }

    /* --------------------------------------------------------------------- */

    // literal_expression -> '(' expression* ')'

    fn literal_expr(&mut self) -> Result<Option<LiteralExprNode>, ParseError> {
        // TODO: move this vec to the scanner
        let literal_tokens = vec![
            // SuperString removed,
            TokenType::String,
            TokenType::FString,
            TokenType::RawString,
            TokenType::ByteString,
            TokenType::TripleQuotedString,
            TokenType::Number,
            TokenType::ComplexNumber,
            TokenType::True,
            TokenType::False,
            TokenType::None_,
        ];

        for literal_tok in literal_tokens {
            if self.match_token(&[literal_tok]) {
                return Ok(Some(LiteralExprNode::new(self.previous().line, 
                    literal_tok,
                    self.previous().lexeme.clone(),
                )));
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // state_context ->

    fn target_state(
        &mut self,
        enter_args_opt: Option<ExprListNode>,
        context_change_type: &str,
        is_transition: bool,
    ) -> Result<Option<TargetStateContextType>, ParseError> {
        if self.match_token(&[TokenType::StateStackOperationPop]) {
            if !is_transition {
                let err_msg = "State change disallowed to a popped state.";
                self.error_at_previous(&err_msg);
            } else if let Some(..) = enter_args_opt {
                let err_msg =
                    "Transition enter arguments disallowed when transitioning to a popped state.";
                self.error_at_previous(&err_msg);
            }
            Ok(Some(TargetStateContextType::StateStackPop {}))
        } else if self.match_token(&[TokenType::StateStackOperationPush]) {
            let err_msg =
                "Error - $$[+] is an invalid transition target. Try replacing with $$[-]. ";
            self.error_at_previous(&err_msg);
            return Err(ParseError::new(err_msg));
        } else {
            // parse state ref e.g. '$S1'
            if !self.match_token(&[TokenType::State]) {
                let err_msg = &format!("Expected target state, found {}.", self.current_token);
                self.error_at_current(&&err_msg);
                return Err(ParseError::new(err_msg));
            }

            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Missing state identifier.";
                self.error_at_current(&&err_msg);
                return Err(ParseError::new(err_msg));
            }

            let state_id = self.previous();
            let name = state_id.lexeme.clone();

            // v0.30: Block transitions to parent ($^) - only dispatch (=> $^) is allowed
            if name == "^" && is_transition {
                let err_msg = "Syntax error: '-> $^' is not allowed. Use '=> $^' for parent dispatch instead.";
                self.error_at_previous(err_msg);
                return Err(ParseError::new(err_msg));
            }

            if !self.is_building_symbol_table {
                if !self.arcanum.has_state(&*name) {
                    let err_msg = format!(
                        "{} target state ${} not declared.",
                        context_change_type, name
                    );
                    self.error_at_current(&*err_msg.clone());
                    return Err(ParseError::new(&*err_msg));
                }
            }

            let mut args_cnt: usize = 0;
            if let Some(expr_list_node) = &enter_args_opt {
                args_cnt = expr_list_node.exprs_t.len();
            }
            // For transitions, validate transition enter args equal target state enter params.
            // For state changes, validate that the target state doesn't have an enter event handler.
            if !self.is_building_symbol_table {
                match self.get_state_enter_eventhandler(name.as_str()) {
                    Some(enter_event_handler_rcref) => {
                        if is_transition {
                            let event_handler_node = enter_event_handler_rcref.borrow();
                            let event_symbol = event_handler_node.event_symbol_rcref.borrow();
                            // This error is only valid for transitions.
                            if args_cnt != event_symbol.get_param_count() {
                                let err_msg = &format!("Number of transition enter arguments not equal to number of event handler parameters.");
                                self.error_at_previous(err_msg.as_str());
                            }
                        } else {
                            // is state change
                            let err_msg = &format!(
                                "State change disallowed to states with enter eventhandler."
                            );
                            self.error_at_previous(err_msg.as_str());
                        }
                    }
                    None => {}
                }
            }

            // parse optional state argument list
            // '(' ')' | '(' expr ')'
            //
            let mut state_ref_args_opt = None;
            if self.match_token(&[TokenType::LParen]) {
                // Parse arguments directly to avoid tuple wrapping
                let mut args = Vec::new();
                
                // Handle empty argument list
                if self.peek().token_type == TokenType::RParen {
                    // expr_list() will consume the RParen
                }
                
                // Set flag to prevent comma-separated values from being parsed as tuples
                let was_parsing_collection = self.is_parsing_collection;
                self.is_parsing_collection = true;
                
                // Parse arguments directly like function call arguments
                while !self.check(TokenType::RParen) {
                    match self.expression() {
                        Ok(Some(expr)) => {
                            args.push(expr);
                            if !self.check(TokenType::RParen) {
                                if !self.match_token(&[TokenType::Comma]) {
                                    self.is_parsing_collection = was_parsing_collection;
                                    self.error_at_current("Expected ',' or ')' in state arguments");
                                    return Err(ParseError::new("Expected ',' or ')' in state arguments"));
                                }
                            }
                        }
                        Ok(None) => {
                            self.is_parsing_collection = was_parsing_collection;
                            self.error_at_current("Expected expression in state arguments");
                            return Err(ParseError::new("Expected expression in state arguments"));
                        }
                        Err(e) => {
                            self.is_parsing_collection = was_parsing_collection;
                            return Err(e);
                        }
                    }
                }
                
                // Restore the flag
                self.is_parsing_collection = was_parsing_collection;
                
                if !self.match_token(&[TokenType::RParen]) {
                    self.error_at_current("Expected ')' after state arguments");
                    return Err(ParseError::new("Expected ')' after state arguments"));
                }
                
                if !args.is_empty() {
                    state_ref_args_opt = Some(ExprListNode::new(args));
                }
                
            }

            if !self.is_building_symbol_table {
                // validate state arguments match transition target state parameters
                let state_rcref_opt = self.arcanum.get_state(name.as_str());
                match state_rcref_opt {
                    Some(state_symbol) => {
                        match &state_ref_args_opt {
                            Some(expr_list_node) => {
                                match &state_symbol.borrow().params_opt {
                                    Some(params) => {
                                        if params.len() != expr_list_node.exprs_t.len() {
                                            // Error - number of state params does not match number of expression arguments
                                            let err_msg = &format!("Transition target state arguments do not match {} state parameters.", name);
                                            self.error_at_current(err_msg.as_str());
                                            return Err(ParseError::new(err_msg));
                                        }
                                    }
                                    None => {
                                        if expr_list_node.exprs_t.len() != 0 {
                                            // Error - number of state params does not match number of expression arguments
                                            let err_msg = &format!("Transition target state arguments do not match {} state parameters.", name);
                                            self.error_at_current(err_msg.as_str());
                                            return Err(ParseError::new(err_msg));
                                        }
                                    }
                                }
                            }
                            None => {
                                if let Some(params) = &state_symbol.borrow().params_opt {
                                    if params.len() != 0 {
                                        // Error - number of state params does not match number of expression arguments
                                        let err_msg = &format!("Transition target state arguments do not match {} state parameters.", name);
                                        self.error_at_current(err_msg.as_str());
                                        return Err(ParseError::new(err_msg));
                                    }

                                    // Ok - state params matches number of arguments.
                                }
                            }
                        }
                    }
                    None => {
                        // There has been some parser error. The absense of a state
                        // in the arcanum should have been caught above first.
                        let err_msg = format!(
                            "{} target state ${} not declared.",
                            context_change_type, name
                        );
                        self.error_at_current(&*err_msg.clone());
                        return Err(ParseError::new(&*err_msg));
                    }
                }
            }

            let state_context_node = TargetStateContextNode::new(
                StateRefNode::new(name),
                state_ref_args_opt,
                enter_args_opt,
            );

            Ok(Some(TargetStateContextType::StateRef {
                state_context_node,
            }))
        }
    }

    /* --------------------------------------------------------------------- */

    fn transition(
        &mut self,
        exit_args_opt: Option<ExprListNode>,
    ) -> Result<TransitionStatementNode, ParseError> {
        if exit_args_opt.is_some() {
            // need exit args generated
            self.generate_exit_args = true;
        }

        if !self.is_building_symbol_table {
            let state_name = self.state_name_opt.as_ref().unwrap().clone();

            let exit_args_cnt = match &exit_args_opt {
                Some(expr_list_node) => expr_list_node.exprs_t.len(),
                None => 0,
            };

            match self.validate_transition_exit_params(&state_name, exit_args_cnt) {
                Ok(()) => {}
                Err(parse_err) => {
                    self.error_at_current(parse_err.error.as_str());
                }
            }
        }
        match self.transition_expr() {
            Ok(transition_expr_node) => {
                let transition_stmt_node =
                    TransitionStatementNode::new(transition_expr_node.line, transition_expr_node, exit_args_opt);
                Ok(transition_stmt_node)
            }
            Err(parse_err) => Err(parse_err),
        }
    }

    /* --------------------------------------------------------------------- */

    // transition : exitArgs '->' enterArgs transitionLabel stateRef stateArgs

    fn transition_expr(&mut self) -> Result<TransitionExprNode, ParseError> {
        self.generate_transition_state = true;

        if self.is_action_scope {
            let err_msg = format!("Transitions disallowed inside of actions.");
            self.error_at_current(&err_msg);
            let parse_error = ParseError::new(err_msg.as_str());
            return Err(parse_error);
        } else if self.is_function_scope {
            let err_msg = format!("Transitions disallowed inside of functions.");
            self.error_at_current(&err_msg);
            let parse_error = ParseError::new(err_msg.as_str());
            return Err(parse_error);
        }
        let eh_rc_refcell = self.current_event_symbol_opt.as_ref().unwrap().clone();
        let evt_symbol = eh_rc_refcell.borrow();

        if evt_symbol.is_exit_msg {
            self.error_at_current("Transition disallowed in exit event handler.")
        }

        let mut enter_msg_with_enter_args: bool = false;
        let mut enter_args_opt: Option<ExprListNode> = None;
        let mut label_opt: Option<String> = None;

        // enterArgs: '(' ')' | '(' expr ')'
        if self.match_token(&[TokenType::LParen]) {
            if evt_symbol.is_enter_msg {
                enter_msg_with_enter_args = true;
            }
            // need enter args generated
            self.generate_enter_args = true;
            match self.expr_list() {
                Ok(Some(ExprListT { expr_list_node })) => enter_args_opt = Some(expr_list_node),
                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
                Ok(None) => self
                    .error_at_current("Transition enter args expression cannot be an empty list."), // continue
            }
        }

        // transition label string
        if self.match_token(&[TokenType::String]) {
            label_opt = Some(self.previous().lexeme.clone());
        }

        // Transition dispatch
        // -> => $Next
        let mut forward_event = false;
        if self.match_token(&[TokenType::Dispatch]) {
            forward_event = true;
            if enter_msg_with_enter_args {
                // TODO - revisit this rule and document, update or remove.
                // Disallowed:
                // $S0
                //     |>| -> ("hi") => $S1 ^ --- This is ok
                //     |>| -> ("hi") => $$[-] ^ --- I think this should be illegal
                //                              --- as the enter args are on the compartment

                self.error_at_current(
                    "Transition dispatch disallowed in enter message with enter event parameters.",
                )
            }
        }

        // -> $S0
        // -> $$[-]
        let target_state_context_t;
        match self.target_state(enter_args_opt, "Transition", true) {
            Ok(Some(scn)) => target_state_context_t = scn,
            Ok(None) => return Err(ParseError::new("TODO")),
            Err(parse_error) => return Err(parse_error),
        }

        // this is so we can know to declare a StateContext at the
        // top of the event handler.
        self.event_handler_has_transition = true;

        // Use the line from the previous token (which should be from the target state)
        let transition_line = self.previous().line;
        let transition_expr_node =
            TransitionExprNode::new(transition_line, target_state_context_t, label_opt, forward_event);

        Ok(transition_expr_node)
    }

    /* --------------------------------------------------------------------- */

    // change_state : '->>' change_state_label state_ref
    //
    // fn change_state(&mut self) -> Result<Option<StatementType>, ParseError> {
    //     self.generate_change_state = true;
    //
    //     let mut label_opt: Option<String> = None;
    //
    //     // change_state label string
    //     if self.match_token(&[TokenType::String]) {
    //         label_opt = Some(self.previous().lexeme.clone());
    //     }
    //
    //     // check that we are not changing state out of a state with
    //     // an exit event handler
    //
    //     let state_name = &self.state_name_opt.as_ref().unwrap().clone();
    //     match &self.get_state_exit_eventhandler(state_name) {
    //         Some(..) => {
    //             let err_msg =
    //                 &format!("State change disallowed out of states with exit eventhandler.");
    //             self.error_at_current(err_msg.as_str());
    //         }
    //         None => {}
    //     }
    //     let state_context_t;
    //     match self.target_state(None, "State change", false) {
    //         Ok(Some(scn)) => state_context_t = scn,
    //         Ok(None) => return Err(ParseError::new("TODO")),
    //         Err(parse_error) => return Err(parse_error),
    //     }
    //
    //     Ok(Some(StatementType::ChangeStateStmt {
    //         change_state_stmt_node: ChangeStateStatementNode {
    //             state_context_t,
    //             label_opt,
    //         },
    //     }))
    // }

    /* --------------------------------------------------------------------- */

    // match_number_test -> '?#'  ('/' match_number_pattern  ('|' match_number_pattern)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    // REMOVED: Deprecated ternary number match test function
    /*
    fn number_match_test(&mut self, expr_t: ExprType) -> Result<NumberMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::NumberTest, "Expected '?#'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<NumberMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.number_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        // v0.30: DispatchToParentState (@:>) deprecated - use => $^ statements
        loop {
            // :> : :|
            if self.peek().token_type == TokenType::Colon
                || self.peek().token_type == TokenType::ColonBar
            {
                break;
            }
            match self.number_match_test_match_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<NumberMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.number_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) = self.consume(TokenType::ColonBar, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        Ok(NumberMatchTestNode::new(
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }
    */

    /* --------------------------------------------------------------------- */

    // number_match_test ->  ('/' match_number '/' (statement* branch_terminator?) ':>')+  '::'

    fn number_match_test_match_branch(
        &mut self,
    ) -> Result<NumberMatchTestMatchBranchNode, ParseError> {
        // NumberMatchStart token was removed in v0.31
        self.error_at_current("Number pattern matching has been removed. Use if/else statements instead.");
        return Err(ParseError::new("Number pattern matching removed"));
    }

    /* --------------------------------------------------------------------- */

    // number_match_test_else_branch -> statements* branch_terminator?

    fn number_match_test_else_branch(
        &mut self,
    ) -> Result<NumberMatchTestElseBranchNode, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(NumberMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // match_enum_test -> '?:' '(' enum_type ')  ('/' match_enum_pattern  ('|' match_enum_pattern)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    // REMOVED: Deprecated ternary enum match test function  
    /*
    fn enum_match_test(&mut self, expr_t: ExprType) -> Result<EnumMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::EnumTest, "Expected '?:'.") {
            return Err(parse_error);
        }

        if let Err(parse_error) = self.consume(TokenType::LParen, "Expected '('.") {
            return Err(parse_error);
        }

        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = format!("Expected enum type name. Found {}.", self.previous().lexeme);
            self.error_at_current(&err_msg);
            let parse_error = ParseError::new(err_msg.as_str());
            return Err(parse_error);
        }

        let mut enum_type_name = String::new();
        let mut enum_symbol_rcref_opt = None;

        if !self.is_building_symbol_table {
            //     // semantic pass
            //

            enum_type_name = self.previous().lexeme.clone();
            let enum_symbol_t_rcref_opt = self.arcanum.lookup(
                enum_type_name.as_str(),
                &IdentifierDeclScope::DomainBlockScope,
            );
            match enum_symbol_t_rcref_opt {
                None => {
                    let err_msg = &format!("Enumerated type '{}' does not exist.", enum_type_name);
                    self.error_at_current(err_msg);
                    let parse_error = ParseError::new(err_msg.as_str());
                    return Err(parse_error);
                }
                Some(symbol_t_rcref) => {
                    let symbol_t = symbol_t_rcref.borrow();
                    match &*symbol_t {
                        SymbolType::EnumDeclSymbolT {
                            enum_symbol_rcref: enum_symbol_rcref_local,
                        } => {
                            // ok - enum type symbol exists
                            // let enum_symbol = enum_symbol_rcref.borrow();
                            enum_symbol_rcref_opt = Some(enum_symbol_rcref_local.clone());
                        }
                        _ => {
                            let err_msg =
                                &format!("Enumerated type '{}' does not exist.", enum_type_name);
                            self.error_at_current(err_msg);
                            let parse_error = ParseError::new(err_msg.as_str());
                            return Err(parse_error);
                        }
                    }
                }
            }
        }

        if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<EnumMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.enum_match_test_match_branch(&enum_symbol_rcref_opt) {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        // v0.30: DispatchToParentState (@:>) deprecated - use => $^ statements
        loop {
            // :> : :|
            if self.peek().token_type == TokenType::Colon
                || self.peek().token_type == TokenType::ColonBar
            {
                break;
            }
            match self.enum_match_test_match_branch(&enum_symbol_rcref_opt) {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<EnumMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.enum_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) = self.consume(TokenType::ColonBar, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        Ok(EnumMatchTestNode::new(
            enum_type_name,
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }
    */

    /* --------------------------------------------------------------------- */

    // enum_match_test ->  ('/' match_enum '/' (statement* branch_terminator?) ':>')+  '::'

    fn enum_match_test_match_branch(
        &mut self,
        _enum_symbol_rcref_opt: &Option<Rc<RefCell<EnumSymbol>>>,
    ) -> Result<EnumMatchTestMatchBranchNode, ParseError> {
        // EnumMatchStart token was removed in v0.31
        // Enum pattern matching has been removed - use if/else instead
        let err_msg = "Enum pattern matching has been removed. Use if/else statements instead.";
        self.error_at_current(&err_msg);
        return Err(ParseError::new(err_msg));
    }

    /* --------------------------------------------------------------------- */

    // enum_match_test_else_branch -> statements* branch_terminator?

    fn enum_match_test_else_branch(&mut self) -> Result<EnumMatchTestElseBranchNode, ParseError> {
        let statements = self.statements(IdentifierDeclScope::BlockVarScope);
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(EnumMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */
    /* --------------------------------------------------------------------- */
    // helper functions

    /* --------------------------------------------------------------------- */

    pub fn get_arcanum(self) -> Arcanum {
        self.arcanum
    }

    /* --------------------------------------------------------------------- */
    
    // v0.62: Helper method to resolve call expression types during semantic analysis
    fn resolve_call_expr(&mut self, call_expr_node: &mut CallExprNode) {
        // Only perform resolution during second pass
        if self.is_building_symbol_table {
            return;
        }
        
        // Create a temporary semantic analyzer
        let mut analyzer = SemanticCallAnalyzer::new(&self.arcanum);
        
        // v0.63: Set context in the analyzer for accurate resolution
        // Check class context first (most specific for method calls)
        if let Some(ref class_name) = self.current_class_name {
            if self.debug_mode {
                eprintln!("DEBUG: Setting class context: {}", class_name);
            }
            analyzer.enter_class(class_name);
        } else if let Some(ref system_name) = self.current_system_name {
            if self.debug_mode {
                eprintln!("DEBUG: Setting system context: {}", system_name);
            }
            analyzer.enter_system(system_name);
        } else if let Some(ref function_name) = self.current_function_name {
            if self.debug_mode {
                eprintln!("DEBUG: Setting function context: {}", function_name);
            }
            analyzer.enter_function();
        } else {
            if self.debug_mode {
                eprintln!("DEBUG: No context set for resolution");
            }
        }
        
        // Check if we're in a static context
        if self.is_static_operation {
            analyzer.set_static_context(true);
        }
        
        // Resolve the call type
        let resolved_type = analyzer.resolve_call(call_expr_node);
        
        // Store the resolution in the AST node
        call_expr_node.resolved_type = Some(resolved_type);
        
        if self.debug_mode {
            eprintln!("DEBUG: Resolved call {} to {:?}", 
                call_expr_node.identifier.name.lexeme, 
                call_expr_node.resolved_type);
        }
    }

    /* --------------------------------------------------------------------- */

    // pub fn get_system_hierarchy(self) -> SystemHierarchy {
    //     self.system_hierarchy_opt.unwrap()
    // }

    /* --------------------------------------------------------------------- */

    pub fn get_all(self) -> (Arcanum, Option<SystemHierarchy>) {
        (self.arcanum, self.system_hierarchy_opt)
    }

    /* --------------------------------------------------------------------- */

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    /* --------------------------------------------------------------------- */

    pub fn get_errors(&self) -> String {
        self.errors.clone()
    }

    /* --------------------------------------------------------------------- */

    // Helper functions

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        // cache off comments
        while self.check(TokenType::PythonComment) || self.check(TokenType::MultiLineComment) {
            let comment = self.peek().clone();
            self.comments.push(comment);
            self.advance();
        }

        if self.check(TokenType::Error) {
            self.error_at_current("Unexpected token.");
            self.advance();
            return false;
        }

        for token_type in token_types {
            if self.check(*token_type) {
                //              println!("Consumed {:?}",token_type);
                if !self.is_at_end() {
                    self.advance();
                }

                return true;
            }
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
            self.current_tok_ref = &self.tokens[self.current];
            self.current_token = self.peek().lexeme.clone();
            self.processed_tokens.push(' ');
            self.processed_tokens.push_str(&self.peek().lexeme.clone());
            //            println!("Current token = {:?}",self.peek());
        }

        self.previous()
    }

    /* --------------------------------------------------------------------- */

    fn check(&self, token_type: TokenType) -> bool {
        let t = self.peek();
        if token_type == t.token_type {
            return true;
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    /* --------------------------------------------------------------------- */

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /* --------------------------------------------------------------------- */

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /* --------------------------------------------------------------------- */

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        self.error_at_current(message);
        Err(ParseError::new(message))
    }

    /* --------------------------------------------------------------------- */

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.tokens[self.current], message);
    }

    /* --------------------------------------------------------------------- */

    fn error_at_previous(&mut self, message: &str) {
        self.error_at(&self.tokens[self.current - 1], message);
    }

    /* --------------------------------------------------------------------- */

    // TODO: put the message in the ParseError
    fn error_at(&mut self, token: &Token, message: &str) {
        // TODO I've commented these out as I'm not sure why we would
        // want to return if already panicing. Howver, this has been
        // here a long time so preserving in case there is strange behavior.
        // if self.panic_mode {
        //     return;
        // }

        self.panic_mode = true;
        self.had_error = true;

        let mut error_msg = format!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => error_msg.push_str(" at end"),
            TokenType::Error => error_msg.push_str(&format!(" at '{}'", token.lexeme)),
            _ => error_msg.push_str(&format!(" at '{}'", token.lexeme)),
        }

        self.errors
            .push_str(&format!("{} : {}\n", error_msg, message));

        //        println!("{} : {}", error_msg, message);
        // TODO:?
        //       ParseError::new( /* error_msg */ )
    }

    /* --------------------------------------------------------------------- */

    fn synchronize(&mut self, sync_tokens: &[TokenType]) -> bool {
        self.panic_mode = false;

        if self.is_at_end() {
            return false;
        }

        // in case not advancing
        if self.last_sync_token_idx == self.current {
            self.advance();
        }

        self.last_sync_token_idx = self.current;

        while self.peek().token_type != TokenType::Eof {
            for sync_token in sync_tokens {
                let current_token_type = self.peek().token_type;
                if *sync_token == current_token_type {
                    return true;
                }
            }

            self.advance();
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn follows(&self, token: &Token, follows_vec: &[TokenType]) -> bool {
        for follows_token_type in follows_vec {
            if *follows_token_type == token.token_type {
                return true;
            }
        }

        let vec_comments = &vec![TokenType::PythonComment, TokenType::MultiLineComment];
        for comment_token_type in vec_comments {
            if *comment_token_type == token.token_type {
                return true;
            }
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn get_state_enter_eventhandler(
        &mut self,
        state_name: &str,
    ) -> Option<Rc<RefCell<EventHandlerNode>>> {
        let state_rcref_opt = self.arcanum.get_state(state_name);
        match state_rcref_opt {
            Some(state_symbol) => match &state_symbol.borrow().state_node_opt {
                Some(state_node_rcref) => {
                    let state_node = state_node_rcref.borrow();
                    if let Some(enter_event_handler_rcref) = &state_node.enter_event_handler_opt {
                        Some(enter_event_handler_rcref.clone())
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    /* --------------------------------------------------------------------- */

    // fn get_state_exit_eventhandler(
    //     &mut self,
    //     state_name: &str,
    // ) -> Option<Rc<RefCell<EventHandlerNode>>> {
    //     let state_rcref_opt = self.arcanum.get_state(state_name);
    //     match state_rcref_opt {
    //         Some(state_symbol) => match &state_symbol.borrow().state_node_opt {
    //             Some(state_node_rcref) => {
    //                 let state_node = state_node_rcref.borrow();
    //                 if let Some(exit_event_handler_rcref) = &state_node.exit_event_handler_opt {
    //                     Some(exit_event_handler_rcref.clone())
    //                 } else {
    //                     None
    //                 }
    //             }
    //             None => None,
    //         },
    //         None => None,
    //     }
    // }

    /* --------------------------------------------------------------------- */

    // This method abstractly handles "l_value = r_value" for all expression types.

    fn assign(
        &mut self,
        l_value: &mut ExprType,
        r_value_rc: Rc<ExprType>,
    ) -> Result<(), ParseError> {
        // Check if this is a self.variable assignment (domain variable)
        if let ExprType::CallChainExprT { call_chain_expr_node } = l_value {
            if call_chain_expr_node.call_chain.len() >= 2 {
                // Check if first element is "self"
                if let Some(CallChainNodeType::VariableNodeT { var_node }) = 
                    call_chain_expr_node.call_chain.get(0) {
                    if var_node.id_node.name.lexeme == "self" {
                        // This is a self.variable assignment - allow it
                        // Domain variable assignments are handled by the code generator
                        return Ok(());
                    }
                }
            }
        }
        
        // Now get the variable and assign new value
        // let debug_expr_name = l_value.expr_type_name();
        let name_opt = l_value.get_name();
        if name_opt.is_some() {
            // this is a variable so update value
            let l_value_name = name_opt.unwrap();
            
            // Special case for system.return
            if l_value_name == "system.return" {
                // Check if we're in a context where system.return is allowed
                if self.operation_scope_depth > 0 && self.is_static_operation {
                    let err_msg = "Operations marked with @staticmethod cannot use system.return";
                    self.error_at_previous(&err_msg);
                    return Err(ParseError::new(err_msg));
                }
                // system.return is valid, just return Ok
                return Ok(());
            }
            
            let symbol_t_opt = self
                .arcanum
                .lookup(l_value_name.as_str(), &IdentifierDeclScope::UnknownScope);
            match symbol_t_opt {
                Some(symbol_t_rcref) => {
                    let mut symbol_t = symbol_t_rcref.borrow_mut();
                    match symbol_t.assign(r_value_rc.clone()) {
                        Ok(()) => Ok(()),
                        Err(err_msg) => {
                            self.error_at_current(&err_msg);
                            Err(ParseError::new(err_msg))
                        }
                    }
                }
                None => {
                    let err_msg = &format!("Invalid l_value name {}", l_value_name);
                    self.error_at_previous(&err_msg);
                    Err(ParseError::new(err_msg))
                }
            }
        } else {
            // Undeclared Identifier so nothing to validate
            Ok(())
        }
    }

    /* --------------------------------------------------------------------- */

    // This method abstractly handles "l_value = r_value" for all expression types.

    fn validate_transition_exit_params(
        &mut self,
        state_name: &str,
        current_state_exit_arg_cnt: usize,
    ) -> Result<(), ParseError> {
        let state_symbol_rcref_opt = self.arcanum.get_state(&state_name);
        let state_symbol_rcref = match state_symbol_rcref_opt {
            Some(state_symbol) => state_symbol.clone(),
            None => return Err(ParseError::new(&format!("State {} not found.", state_name))),
        };

        let state_symbol = state_symbol_rcref.borrow();
        let state_node_rcref = state_symbol.state_node_opt.as_ref().unwrap();
        let state_node = state_node_rcref.borrow();
        let param_cnt = state_node.get_exit_param_count();
        if param_cnt != current_state_exit_arg_cnt {
            let err_msg = &format!("Transition exit args length not equal to exit handler parameter length for state ${}", state_name);
            Err(ParseError::new(err_msg))
        } else {
            Ok(())
        }
    }

    // ------------------------------------------------------------------ */
    pub fn get_operator_type(&mut self, operator_token: &Token) -> OperatorType {
        let op_type = OperatorType::get_operator_type(&operator_token.token_type);
        if op_type == OperatorType::Unknown {
            let err_msg = &format!("Unknown operator {}", operator_token.lexeme);
            self.error_at_current(err_msg);
        }

        op_type
    }

    /* --------------------------------------------------------------------- */
    // LEGB Scope-Aware Symbol Lookups
    /* --------------------------------------------------------------------- */

    /// Scope-aware symbol lookup that respects LEGB and scope isolation
    fn scope_aware_symbol_lookup(&self, name: &str) -> Option<Rc<RefCell<SymbolType>>> {
        // Use LEGB lookup first
        let symbol_opt = self.arcanum.legb_lookup(name);
        
        // Apply scope accessibility check
        match symbol_opt {
            Some(symbol_ref) => {
                let symbol = symbol_ref.borrow();
                if self.arcanum.is_symbol_accessible(&*symbol) {
                    Some(symbol_ref.clone())
                } else {
                    // Symbol exists but not accessible in current scope
                    None
                }
            }
            None => None
        }
    }

    /// Check if we can call an action in the current scope context
    fn can_call_action_in_current_scope(&self, action_name: &str) -> bool {
        // Functions should NOT be able to call system actions
        match self.arcanum.scope_context {
            ScopeContext::Function(_) => {
                // Functions cannot call actions
                false
            }
            ScopeContext::System(_) => {
                // Systems can call their own actions - verify through LEGB
                if let Some(symbol_ref) = self.scope_aware_symbol_lookup(action_name) {
                    matches!(*symbol_ref.borrow(), SymbolType::ActionScope { .. })
                } else {
                    false
                }
            }
            ScopeContext::Global => {
                // Module level cannot call actions directly
                false
            }
        }
    }

    /// Check if we can call an operation in the current scope context  
    fn can_call_operation_in_current_scope(&self, operation_name: &str) -> bool {
        // Functions should NOT be able to call system operations directly
        match self.arcanum.scope_context {
            ScopeContext::Function(_) => {
                // Functions cannot call system operations directly
                false
            }
            ScopeContext::System(_) => {
                // Systems can call their own operations - verify through LEGB
                if let Some(symbol_ref) = self.scope_aware_symbol_lookup(operation_name) {
                    matches!(*symbol_ref.borrow(), SymbolType::OperationScope { .. })
                } else {
                    false
                }
            }
            ScopeContext::Global => {
                // Module level cannot call operations directly
                false
            }
        }
    }
    
    // ===================== Frame v0.31 Explicit Self/System Support =====================
    
    /// Parse self, self.method() or self.variable syntax
    fn parse_self_context(&mut self) -> Result<Option<ExprType>, ParseError> {
        use crate::frame_c::ast::SelfExprNode;
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG parse_self_context: is_class_method={}, is_static_operation={}", 
                      self.is_class_method, self.is_static_operation);
        }
        
        // Check if we're in a static operation - if so, self is not allowed
        if self.is_static_operation {
            let err_msg = "Cannot use 'self' in a static operation (marked with @staticmethod)";
            self.error_at_previous(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // v0.45: Allow self in class instance methods
        // For class methods, we allow self to work similarly to system contexts
        
        // We've already consumed 'self' token
        if !self.match_token(&[TokenType::Dot]) {
            // Standalone 'self' - create a special variable node representing the system instance
            let self_token = Token {
                token_type: TokenType::Self_,
                lexeme: "self".to_string(),
                literal: TokenLiteral::None,
                line: self.previous().line,
                start: self.previous().start,
                length: 4,  // "self" is 4 characters
            };
            let id_node = IdentifierNode::new(
                self_token,
                None,
                IdentifierDeclScope::SystemScope,
                false,
                self.previous().line,
            );
            let var_node = VariableNode::new_with_self(id_node.line, 
                id_node,
                IdentifierDeclScope::SystemScope,
                None,  // Symbol will be resolved later
                true,  // is_self = true
            );
            return Ok(Some(ExprType::VariableExprT { var_node }));
        }
        
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected identifier after 'self.'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let identifier_token = self.previous().clone();
        let id_node = IdentifierNode::new(
            identifier_token.clone(),
            None,
            IdentifierDeclScope::SystemScope,
            false,
            identifier_token.line,
        );
        
        // Check if this is a method call or variable access
        if self.match_token(&[TokenType::LParen]) {
            // self.method() - parse as method call
            let params = match self.expr_list() {
                Ok(Some(ExprListT { expr_list_node })) => expr_list_node.exprs_t,
                Ok(None) => Vec::new(),  // Empty parameter list
                Ok(Some(_)) => Vec::new(),  // Other expression types - treat as empty
                Err(err) => return Err(err),
            };
            
            // NOTE: expr_list() already consumes the RParen token, so we don't need to consume it again
            
            let call_expr_list = CallExprListNode::new(params);
            let mut call_expr = CallExprNode::new_with_context(id_node.line, 
                id_node,
                call_expr_list,
                None,
                CallContextType::SelfCall,
            );
            
            // v0.62: Perform semantic resolution if enabled
            self.resolve_call_expr(&mut call_expr);
            
            return Ok(Some(ExprType::CallExprT { call_expr_node: call_expr }));
        } else {
            // self.variable - create a CallChainExprT to represent self.variable access
            // This is needed so that assignments can recognize self.variable patterns
            
            // First create the 'self' node using the proper SelfT variant
            let self_expr_node = SelfExprNode::new(self.previous().line);
            
            // Then create the variable node for the property
            let var_node = VariableNode::new(id_node.line, 
                id_node,
                IdentifierDeclScope::DomainBlockScope,
                None,
            );
            
            // Build the call chain starting with SelfT
            let mut call_chain = VecDeque::new();
            call_chain.push_back(CallChainNodeType::SelfT { self_expr_node });
            call_chain.push_back(CallChainNodeType::VariableNodeT { var_node });
            
            // v0.37: Check for further dots in self.property.method() patterns
            while self.match_token(&[TokenType::Dot]) {
                if !self.match_token(&[TokenType::Identifier]) {
                    let err_msg = "Expected identifier after '.'";
                    return Err(ParseError::new(err_msg));
                }
                
                let next_id = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::UnknownScope,
                    false,
                    self.previous().line,
                );
                
                // Check if it's a method call
                if self.match_token(&[TokenType::LParen]) {
                    let params = match self.expr_list() {
                        Ok(Some(ExprListT { expr_list_node })) => expr_list_node.exprs_t,
                        Ok(None) => Vec::new(),
                        Ok(Some(_)) => Vec::new(),
                        Err(err) => return Err(err),
                    };
                    
                    let call_expr_list = CallExprListNode::new(params);
                    let mut call_expr = CallExprNode::new(next_id.line, next_id, call_expr_list, None);
                    
                    // v0.62: Perform semantic resolution if enabled
                    self.resolve_call_expr(&mut call_expr);
                    
                    call_chain.push_back(CallChainNodeType::UndeclaredCallT {
                        call_node: call_expr,
                    });
                } else {
                    // Just a property access
                    call_chain.push_back(CallChainNodeType::UndeclaredIdentifierNodeT {
                        id_node: next_id,
                    });
                }
            }
            
            let call_chain_expr_node = CallChainExprNode::new(self.previous().line, call_chain);
            
            return Ok(Some(ExprType::CallChainExprT { call_chain_expr_node }));
        }
    }
    
    // ===================== Frame v0.34 Module Support =====================
    
    fn module_declaration(&mut self) -> Result<Rc<RefCell<ModuleNode>>, ParseError> {
        // We've already consumed 'module' keyword
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected module name after 'module'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let module_name = self.previous().lexeme.clone();
        
        if !self.match_token(&[TokenType::OpenBrace]) {
            let err_msg = "Expected '{' after module name";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // v0.34: Enter the named module scope in the symbol table
        // We need to do this in both passes so functions inside modules can be found
        if self.is_building_symbol_table {
            self.arcanum.enter_scope(ParseScopeType::NamedModule { 
                module_name: module_name.clone() 
            });
        } else {
            // Second pass: we need to navigate to the module scope
            if let Err(err) = self.arcanum.set_parse_scope(&module_name) {
                // Module not found - this shouldn't happen if first pass succeeded
                return Err(ParseError::new(&format!("Failed to enter module scope '{}': {}", module_name, err)));
            }
        }
        
        // Parse module contents
        let mut functions = Vec::new();
        let mut systems = Vec::new();
        let mut variables = Vec::new();
        let mut enums = Vec::new();
        let mut nested_modules = Vec::new();
        
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Async]) {
                // Check for async function in module
                if self.match_token(&[TokenType::Function]) {
                    let function = self.function_scope_async(true)?;
                    functions.push(function);
                } else {
                    return Err(ParseError::new("Expected 'fn' after 'async' keyword"));
                }
            } else if self.match_token(&[TokenType::Function]) {
                let function = self.function_scope_async(false)?;
                functions.push(function);
            } else if self.match_token(&[TokenType::System]) {
                let module_for_system = crate::frame_c::ast::Module::new(vec![]);
                let system = self.system_scope(Some(module_for_system), None, None)?;
                systems.push(system);
            } else if self.match_token(&[TokenType::Var]) || self.match_token(&[TokenType::Const]) {
                self.previous(); // Put the token back
                match self.var_declaration(IdentifierDeclScope::ModuleScope) {
                    Ok(var_decl) => variables.push(var_decl),
                    Err(err) => return Err(err),
                }
            } else if self.match_token(&[TokenType::Enum]) {
                match self.enum_decl() {
                    Ok(enum_decl) => enums.push(enum_decl),
                    Err(err) => return Err(err),
                }
            } else if self.match_token(&[TokenType::Module]) {
                let nested_module = self.module_declaration()?;
                nested_modules.push(nested_module);
            } else {
                let err_msg = format!("Unexpected token in module: {:?}", self.peek().token_type);
                self.error_at_current(&err_msg);
                return Err(ParseError::new(&err_msg));
            }
        }
        
        if !self.match_token(&[TokenType::CloseBrace]) {
            let err_msg = "Expected '}' after module body";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // v0.34: Exit the named module scope
        // Exit in both passes to maintain proper scope
        if self.is_building_symbol_table {
            self.arcanum.exit_scope();
        } else {
            self.arcanum.exit_scope();
        }
        
        Ok(Rc::new(RefCell::new(ModuleNode::new(
            module_name,
            functions,
            systems,
            variables,
            enums,
            nested_modules,
        ))))
    }
    
    // ===================== Frame v0.45 Class Support =====================
    
    fn class_declaration(&mut self) -> Result<Rc<RefCell<ClassNode>>, ParseError> {
        // We've already consumed 'class' keyword
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected class name after 'class'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let class_name = self.previous().lexeme.clone();
        let line = self.previous().line;
        
        // v0.63: Set current class context for semantic resolution
        self.current_class_name = Some(class_name.clone());
        if self.debug_mode {
            eprintln!("DEBUG: Set current_class_name to {}", class_name);
        }
        
        if !self.match_token(&[TokenType::OpenBrace]) {
            let err_msg = "Expected '{' after class name";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Enter class scope for parsing
        if self.is_building_symbol_table {
            self.arcanum.enter_scope(ParseScopeType::Class { 
                class_name: class_name.clone() 
            });
        } else {
            // Second pass: navigate to class scope
            if let Err(err) = self.arcanum.set_parse_scope(&class_name) {
                return Err(ParseError::new(&format!("Failed to enter class scope '{}': {}", class_name, err)));
            }
        }
        
        // Parse class members
        let mut methods = Vec::new();
        let mut static_methods = Vec::new();
        let mut instance_vars = Vec::new();
        let mut static_vars = Vec::new();
        let mut constructor = None;
        let mut has_explicit_init_annotation = false;
        
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            // Check for @staticmethod decorator
            let is_static = if self.check(TokenType::OuterAttributeOrDomainParams) {
                // Parse attribute to see if it's @staticmethod
                let saved_pos = self.current;
                if self.match_token(&[TokenType::OuterAttributeOrDomainParams]) {
                    if self.match_token(&[TokenType::Identifier]) {
                        let attr_name = self.previous().lexeme.clone();
                        if !self.match_token(&[TokenType::RBracket]) {
                            return Err(ParseError::new("Expected ']' after attribute"));
                        }
                        attr_name == "staticmethod"
                    } else {
                        // Rewind if not a simple identifier attribute
                        self.current = saved_pos;
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            
            // Check for @init decorator
            let is_init_annotated = if !is_static && self.check(TokenType::OuterAttributeOrDomainParams) {
                let saved_pos = self.current;
                if self.match_token(&[TokenType::OuterAttributeOrDomainParams]) {
                    if self.match_token(&[TokenType::Identifier]) {
                        let attr_name = self.previous().lexeme.clone();
                        if !self.match_token(&[TokenType::RBracket]) {
                            return Err(ParseError::new("Expected ']' after attribute"));
                        }
                        if attr_name == "init" {
                            has_explicit_init_annotation = true;
                            true
                        } else {
                            // Rewind if not init attribute
                            self.current = saved_pos;
                            false
                        }
                    } else {
                        // Rewind if not a simple identifier attribute
                        self.current = saved_pos;
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            
            if self.match_token(&[TokenType::Var, TokenType::Const]) {
                // Variable declaration (instance or static based on context/decorator)
                let _is_const = self.previous().token_type == TokenType::Const;
                let var_decl = self.var_declaration(if is_static { 
                    IdentifierDeclScope::ClassStaticScope 
                } else { 
                    IdentifierDeclScope::ClassInstanceScope 
                })?;
                
                if is_static {
                    static_vars.push(var_decl);
                } else {
                    instance_vars.push(var_decl);
                }
            } else if self.match_token(&[TokenType::Function]) {
                // Method declaration
                let method = self.parse_class_method(is_static, is_init_annotated)?;
                
                // Check if this is a constructor
                let method_ref = method.borrow();
                if is_init_annotated || (!has_explicit_init_annotation && method_ref.name == "init" && !is_static) {
                    if constructor.is_some() {
                        return Err(ParseError::new("Class can only have one constructor"));
                    }
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG: Setting constructor for class, statements count: {}", method_ref.statements.len());
                    }
                    drop(method_ref); // Release borrow
                    constructor = Some(method.clone());
                } else if is_static {
                    drop(method_ref); // Release borrow
                    static_methods.push(method);
                } else {
                    drop(method_ref); // Release borrow
                    methods.push(method);
                }
            } else {
                let err_msg = "Expected method or variable declaration in class";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        }
        
        if !self.match_token(&[TokenType::CloseBrace]) {
            let err_msg = "Expected '}' at end of class";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Exit class scope
        if self.is_building_symbol_table {
            self.arcanum.exit_scope();
        }
        
        // v0.63: Clear class context after parsing
        if self.debug_mode {
            eprintln!("DEBUG: Clearing current_class_name (was {:?})", self.current_class_name);
        }
        self.current_class_name = None;
        
        Ok(Rc::new(RefCell::new(ClassNode::new(
            class_name,
            None,  // parent - not supported in this old function
            Vec::new(),  // v0.58: No decorators in this old path
            methods,
            static_methods,
            Vec::new(),  // class_methods - not supported in this old function  
            Vec::new(),  // properties - not supported in this old function
            instance_vars,
            static_vars,
            constructor,
            line,
        ))))
    }
    
    // DEPRECATED: Old class method parser - doesn't support @classmethod
    // TODO: Remove this and use parse_class_method_v2 everywhere
    fn parse_class_method(&mut self, is_static: bool, is_constructor: bool) -> Result<Rc<RefCell<MethodNode>>, ParseError> {
        // We've already consumed 'fn' token
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected method name after 'fn'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let method_name = self.previous().lexeme.clone();
        let line = self.previous().line;
        
        // Set flag to indicate we're in a class method (for self support)
        let saved_is_class_method = self.is_class_method;
        let saved_is_static = self.is_static_operation;
        self.is_class_method = !is_static;
        self.is_static_operation = is_static;
        
        // Parse parameters
        if !self.match_token(&[TokenType::LParen]) {
            let err_msg = "Expected '(' after method name";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let params = self.parameters()?;
        
        if !self.match_token(&[TokenType::RParen]) {
            let err_msg = "Expected ')' after parameters";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Optional return type
        let type_opt = if self.match_token(&[TokenType::Colon]) {
            Some(self.type_decl()?)
        } else {
            None
        };
        
        // Parse method body
        if !self.match_token(&[TokenType::OpenBrace]) {
            let err_msg = "Expected '{' to begin method body";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Parse statements
        let mut statements = Vec::new();
        let mut explicit_return_expr_opt: Option<ExprType> = None;
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Starting to parse class method body, is_class_method={}", self.is_class_method);
        }
        
        while !self.check(TokenType::CloseBrace) && !self.is_at_end() {
            if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                eprintln!("DEBUG: Parsing statement in class method, current token: {:?}", self.peek());
            }
            
            // Check for explicit return statement as terminator
            if self.check(TokenType::Return_) {
                self.advance(); // consume 'return'
                
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Found return statement in class method, is_class_method={}", self.is_class_method);
                }
                
                // Parse the return expression
                explicit_return_expr_opt = self.expression()?;
                
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Parsed explicit return expression in class method");
                }
                
                // A return statement should be the last thing in the method
                break;
            }
            // Check for variable declarations
            else if self.check(TokenType::Var) || self.check(TokenType::Const) {
                self.match_token(&[TokenType::Var, TokenType::Const]);
                // Use EventHandlerVarScope for method local variables (similar to how event handlers work)
                let var_decl = self.var_declaration(IdentifierDeclScope::EventHandlerVarScope)?;
                let decl_or_stmt = DeclOrStmtType::VarDeclT { var_decl_t_rcref: var_decl };
                statements.push(decl_or_stmt);
            } else if let Some(stmt) = self.statement()? {
                let decl_or_stmt = DeclOrStmtType::StmtT { stmt_t: stmt };
                statements.push(decl_or_stmt);
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: Added statement to class method");
                }
            } else {
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG: statement() returned None in class method");
                }
            }
        }
        
        if !self.match_token(&[TokenType::CloseBrace]) {
            let err_msg = "Expected '}' at end of method body";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Create terminator - use explicit return expression if found, otherwise implicit empty return
        let terminator = TerminatorExpr::new(TerminatorType::Return, explicit_return_expr_opt, line);
        
        // Restore flags
        self.is_class_method = saved_is_class_method;
        self.is_static_operation = saved_is_static;
        
        Ok(Rc::new(RefCell::new(MethodNode::new(
            method_name,
            params,
            statements,
            terminator,
            type_opt,
            is_constructor,
            is_static,
            false,  // is_class - not supported in this old function
            line,
        ))))
    }
    
    // ===================== Frame v0.31 Import Support =====================
    
    /// Parse import statement: import module [as alias]
    fn parse_import_statement(&mut self) -> Result<ImportNode, ParseError> {
        // We've already consumed 'import' token
        let line = self.previous().line;
        
        // Check for destructuring import syntax: import { ... } from "..."
        if self.match_token(&[TokenType::OpenBrace]) {
            return self.parse_frame_selective_import(line);
        }
        
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected module name after 'import'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let mut module_name = self.previous().lexeme.clone();
        
        // Handle dotted module names (e.g., os.path, fsl.str)
        while self.match_token(&[TokenType::Dot]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected identifier after '.' in module name";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            module_name.push('.');
            module_name.push_str(&self.previous().lexeme);
        }
        
        // Check for Frame file import: import Utils from "./utils.frm"
        // Only check if we're still on the same line (not a new statement)
        if self.peek().token_type == TokenType::From && self.peek().line == line {
            self.advance(); // Consume the 'from' token
            return self.parse_frame_module_import(module_name, line);
        }
        
        // Check for 'as alias' syntax (Python imports)
        if self.match_token(&[TokenType::As]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected alias name after 'as'";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            let alias = self.previous().lexeme.clone();
            
            Ok(ImportNode::new(
                ImportType::Aliased { 
                    module: module_name, 
                    alias 
                },
                line
            ))
        } else {
            Ok(ImportNode::new(
                ImportType::Simple { 
                    module: module_name 
                },
                line
            ))
        }
    }
    
    /// Parse from import: from module import item1, item2 or from module import *
    fn parse_from_import_statement(&mut self) -> Result<ImportNode, ParseError> {
        // We've already consumed 'from' token
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected module name after 'from'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let mut module_name = self.previous().lexeme.clone();
        let line = self.previous().line;
        
        // Handle dotted module names (e.g., os.path, fsl.list)
        while self.match_token(&[TokenType::Dot]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected identifier after '.' in module name";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            module_name.push('.');
            module_name.push_str(&self.previous().lexeme);
        }
        
        if !self.match_token(&[TokenType::Import]) {
            let err_msg = "Expected 'import' after module name";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Check for wildcard import
        if self.match_token(&[TokenType::Star]) {
            return Ok(ImportNode::new(
                ImportType::FromImportAll { 
                    module: module_name 
                },
                line
            ));
        }
        
        // Parse list of imported items
        let mut items = Vec::new();
        
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected identifier or '*' after 'import'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        items.push(self.previous().lexeme.clone());
        
        
        // Parse additional items separated by commas
        while self.match_token(&[TokenType::Comma]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected identifier after ','";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            items.push(self.previous().lexeme.clone());
            
            // v0.34: Track specific FSL imports  
        }
        
        Ok(ImportNode::new(
            ImportType::FromImport { 
                module: module_name, 
                items 
            },
            line
        ))
    }
    
    /// Parse Frame module import: import Utils from "./utils.frm" [as alias]
    /// v0.57: Added for multi-file module system
    fn parse_frame_module_import(&mut self, module_name: String, line: usize) -> Result<ImportNode, ParseError> {
        // We've already consumed 'from' token, expect string literal for file path
        if !self.match_token(&[TokenType::String]) {
            let err_msg = "Expected string literal file path after 'from'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let file_path = self.extract_string_literal(&self.previous().lexeme);
        
        // Validate that this is a Frame file (.frm extension)
        if !file_path.ends_with(".frm") {
            let err_msg = "Frame module imports must reference .frm files";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Check for optional 'as alias' syntax
        if self.match_token(&[TokenType::As]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected alias name after 'as'";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            let alias = self.previous().lexeme.clone();
            
            Ok(ImportNode::new(
                ImportType::FrameModuleAliased { 
                    module_name, 
                    file_path, 
                    alias 
                },
                line
            ))
        } else {
            Ok(ImportNode::new(
                ImportType::FrameModule { 
                    module_name, 
                    file_path 
                },
                line
            ))
        }
    }
    
    /// Parse Frame selective import: import { add, multiply } from "./math.frm"
    /// v0.57: Added for multi-file module system
    fn parse_frame_selective_import(&mut self, line: usize) -> Result<ImportNode, ParseError> {
        // We've already consumed '{' token, parse list of imported items
        let mut items = Vec::new();
        
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected identifier after '{'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        items.push(self.previous().lexeme.clone());
        
        // Parse additional items separated by commas
        while self.match_token(&[TokenType::Comma]) {
            if !self.match_token(&[TokenType::Identifier]) {
                let err_msg = "Expected identifier after ','";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
            items.push(self.previous().lexeme.clone());
        }
        
        if !self.match_token(&[TokenType::CloseBrace]) {
            let err_msg = "Expected '}' after import list";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if !self.match_token(&[TokenType::From]) {
            let err_msg = "Expected 'from' after import list";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        if !self.match_token(&[TokenType::String]) {
            let err_msg = "Expected string literal file path after 'from'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let file_path = self.extract_string_literal(&self.previous().lexeme);
        
        // Validate that this is a Frame file (.frm extension)
        if !file_path.ends_with(".frm") {
            let err_msg = "Frame module imports must reference .frm files";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        Ok(ImportNode::new(
            ImportType::FrameSelective { 
                items, 
                file_path 
            },
            line
        ))
    }
    
    /// Extract string content from string literal token
    /// v0.57: Helper for Frame file import parsing
    fn extract_string_literal(&self, lexeme: &str) -> String {
        // Remove surrounding quotes from string literal
        if lexeme.len() >= 2 && lexeme.starts_with('"') && lexeme.ends_with('"') {
            lexeme[1..lexeme.len()-1].to_string()
        } else if lexeme.len() >= 2 && lexeme.starts_with('\'') && lexeme.ends_with('\'') {
            lexeme[1..lexeme.len()-1].to_string()
        } else {
            lexeme.to_string() // Return as-is if not quoted
        }
    }
    
    // v0.56: Parse type alias: type Name = type_expression
    fn parse_type_alias(&mut self) -> Result<TypeAliasNode, ParseError> {
        // We've already consumed 'type' token
        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = "Expected type alias name after 'type'";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        let alias_name = self.previous().lexeme.clone();
        let line = self.previous().line;
        
        // Expect '=' 
        if !self.match_token(&[TokenType::Equals]) {
            let err_msg = "Expected '=' after type alias name";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        // Parse the type expression
        // Capture everything on the same line as the type expression
        let mut type_expr = String::new();
        let mut bracket_depth = 0;
        
        // Collect tokens until end of line or next major statement
        while !self.is_at_end() {
            let token = self.peek();
            
            // Track bracket depth
            if token.token_type == TokenType::LBracket {
                bracket_depth += 1;
            } else if token.token_type == TokenType::RBracket {
                bracket_depth -= 1;
            }
            
            // Stop conditions:
            // 1. Found a comment (marks end of line)
            // 2. Found next line starting with keyword (only if brackets are balanced)
            if token.token_type == TokenType::PythonComment {
                break;
            }
            
            // If brackets are balanced and at depth 0, check for line end
            if bracket_depth == 0 {
                // Check if this token itself is start of new statement
                if matches!(token.token_type,
                    TokenType::Type | TokenType::Import | TokenType::From |
                    TokenType::Function | TokenType::Class | TokenType::System |
                    TokenType::Var | TokenType::Const | TokenType::Enum) {
                    break;
                }
                // Also check for 'type' keyword as identifier (since it's no longer a reserved keyword)
                if token.token_type == TokenType::Identifier && token.lexeme == "type" {
                    break;
                }
            }
            
            // Add the token to type expression with appropriate spacing
            // Add space after commas for readability
            if !type_expr.is_empty() && !type_expr.ends_with('[') && token.lexeme != "]" && token.lexeme != "[" {
                // Add space before token if it's not a bracket and previous char wasn't an opening bracket
                if token.lexeme != "," {
                    type_expr.push(' ');
                }
            }
            type_expr.push_str(&token.lexeme);
            if token.lexeme == "," {
                type_expr.push(' ');  // Add space after comma
            }
            self.advance();
            
            // After consuming the token, check if we should stop
            if bracket_depth < 0 {
                // Unbalanced brackets - stop
                break;
            }
        }
        
        if type_expr.is_empty() {
            let err_msg = "Expected type expression after '='";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }
        
        Ok(TypeAliasNode::new(alias_name, type_expr, line))
    }
    
    // v0.37: Analyze system runtime async requirements
    fn analyze_system_runtime_info(&mut self, system: &mut SystemNode) -> Result<(), ParseError> {
        // Create new runtime info
        let mut runtime_info = RuntimeInfo::new();
        runtime_info.kernel.system_ref = system.name.clone();
        runtime_info.router.system_ref = system.name.clone();
        
        // Check if the system has any async requirements
        let mut system_needs_async = false;
        let mut async_transitions: Vec<TransitionNode> = Vec::new();
        
        // Analyze the machine block
        if let Some(machine_block) = &system.machine_block_node_opt {
            // Process each state
            for state_rc in &machine_block.states {
                let state = state_rc.borrow();
                let state_name = state.name.clone();
                let mut state_needs_async = false;
                
                // Check event handlers
                for event_handler_rc in &state.evt_handlers_rcref {
                    let handler = event_handler_rc.borrow();
                    
                    // Check if this handler is async or contains await
                    if handler.is_async || self.handler_contains_await(&handler) {
                        state_needs_async = true;
                        system_needs_async = true;
                        
                        // Track async requirements for transitions
                        // Check if handler has transition statements
                        {
                            for stmt in &handler.statements {
                                if let DeclOrStmtType::StmtT { stmt_t } = stmt {
                                    if let StatementType::TransitionStmt { transition_statement_node } = stmt_t {
                                        // Get target state name
                                        let target_state_name = match &transition_statement_node.transition_expr_node.target_state_context_t {
                                            TargetStateContextType::StateRef { state_context_node } => state_context_node.state_ref_node.name.clone(),
                                            _ => continue,
                                        };
                                        
                                        // This async handler performs a transition
                                        async_transitions.push(TransitionNode::new(
                                            state_name.clone(),
                                            target_state_name.clone(),
                                            true,
                                            match &handler.msg_t {
                                                MessageType::CustomMessage { message_node } => message_node.name.clone(),
                                                MessageType::None => "unknown".to_string(),
                                            },
                                        ));
                                        
                                        // NOTE: We do NOT force exit/enter handlers to be async
                                        // They should only be async if they actually use await
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Check enter handler
                if let Some(enter_handler_rc) = &state.enter_event_handler_opt {
                    let handler = enter_handler_rc.borrow();
                    if handler.is_async || self.handler_contains_await(&handler) {
                        state_needs_async = true;
                        system_needs_async = true;
                    }
                }
                
                // Check exit handler
                if let Some(exit_handler_rc) = &state.exit_event_handler_opt {
                    let handler = exit_handler_rc.borrow();
                    if handler.is_async || self.handler_contains_await(&handler) {
                        state_needs_async = true;
                        system_needs_async = true;
                    }
                }
                
                // Add state dispatcher if state needs async
                if state_needs_async {
                    runtime_info.state_dispatchers.push(StateDispatcherNode::new(
                        state_name.clone(),
                        true,
                    ));
                }
            }
        }
        
        // Check interface methods for async
        if let Some(interface_block) = &system.interface_block_node_opt {
            for method in &interface_block.interface_methods {
                if method.borrow().is_async {
                    system_needs_async = true;
                }
            }
        }
        
        // Check actions for async
        if let Some(actions_block) = &system.actions_block_node_opt {
            for action in &actions_block.actions {
                if action.borrow().is_async {
                    system_needs_async = true;
                }
            }
        }
        
        // Set kernel and router async requirements
        runtime_info.kernel.is_async = system_needs_async;
        runtime_info.router.is_async = system_needs_async;
        runtime_info.transitions = async_transitions;
        
        // Store runtime info in the system
        system.runtime_info = Some(runtime_info);
        
        // Validate that async handlers are properly marked
        self.validate_async_handlers(system)?;
        
        Ok(())
    }
    
    // Check if an event handler contains await expressions
    fn handler_contains_await(&self, handler: &EventHandlerNode) -> bool {
        // Check statements for await expressions
        for stmt in &handler.statements {
            if self.statement_contains_await(stmt) {
                return true;
            }
        }
        false
    }
    
    // Check if a statement contains await expressions
    fn statement_contains_await(&self, stmt: &DeclOrStmtType) -> bool {
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        self.expr_stmt_contains_await(expr_stmt_t)
                    }
                    StatementType::TransitionStmt { .. } => false,
                    _ => false,
                }
            }
            DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                let var_decl = var_decl_t_rcref.borrow();
                // Check if the value_rc contains await
                self.expr_contains_await(&var_decl.value_rc)
            }
        }
    }
    
    fn expr_stmt_contains_await(&self, expr_stmt: &ExprStmtType) -> bool {
        match expr_stmt {
            ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                self.expr_contains_await(&assignment_stmt_node.assignment_expr_node.r_value_rc)
            }
            ExprStmtType::CallStmtT { call_stmt_node: _ } => {
                // Check if the call expression might contain await
                // This is a simplified check - just returns false for now
                false
            }
            ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node: _ } => {
                // Check each node in the chain for an expression that might contain await
                false
            }
            _ => false,
        }
    }
    
    fn expr_contains_await(&self, expr: &ExprType) -> bool {
        use crate::frame_c::ast::ExprType::*;
        match expr {
            AwaitExprT { .. } => true,
            CallChainExprT { .. } => {
                // For now, we don't check inside call chains deeply
                // This would require more complex traversal
                false
            }
            _ => false,
        }
    }
    
    // Validate that handlers using await are marked async
    fn validate_async_handlers(&self, system: &SystemNode) -> Result<(), ParseError> {
        if let Some(machine_block) = &system.machine_block_node_opt {
            for state_rc in &machine_block.states {
                let state = state_rc.borrow();
                let state_name = &state.name;
                
                // Check event handlers
                for handler_rc in &state.evt_handlers_rcref {
                    let handler = handler_rc.borrow();
                    if self.handler_contains_await(&handler) && !handler.is_async {
                        let handler_name = match &handler.msg_t {
                            MessageType::CustomMessage { message_node } => message_node.name.clone(),
                            MessageType::None => "unknown".to_string(),
                        };
                        return Err(ParseError::new(&format!(
                            "Event handler '{}' in state '{}' uses 'await' but is not marked 'async'",
                            handler_name, state_name
                        )));
                    }
                }
                
                // Check enter handler
                if let Some(enter_handler_rc) = &state.enter_event_handler_opt {
                    let handler = enter_handler_rc.borrow();
                    if self.handler_contains_await(&handler) && !handler.is_async {
                        return Err(ParseError::new(&format!(
                            "Enter handler '$>' in state '{}' uses 'await' but is not marked 'async'",
                            state_name
                        )));
                    }
                }
                
                // Check exit handler
                if let Some(exit_handler_rc) = &state.exit_event_handler_opt {
                    let handler = exit_handler_rc.borrow();
                    if self.handler_contains_await(&handler) && !handler.is_async {
                        return Err(ParseError::new(&format!(
                            "Exit handler '<$' in state '{}' uses 'await' but is not marked 'async'",
                            state_name
                        )));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    
    // Collect tokens until we hit one of the stop tokens (at the same nesting level)
    fn collect_tokens_until(&mut self, stop_tokens: &[TokenType]) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut paren_depth = 0;
        let mut bracket_depth = 0;
        let mut brace_depth = 0;
        
        while !self.is_at_end() {
            let token = self.peek();
            
            // Check if we should stop (only at depth 0)
            if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 {
                if stop_tokens.contains(&token.token_type) {
                    break;
                }
            }
            
            // Track nesting depth
            match token.token_type {
                TokenType::LParen => paren_depth += 1,
                TokenType::RParen => {
                    paren_depth -= 1;
                    if paren_depth < 0 {
                        break; // Mismatched parenthesis
                    }
                }
                TokenType::LBracket => bracket_depth += 1,
                TokenType::RBracket => {
                    bracket_depth -= 1;
                    if bracket_depth < 0 {
                        break; // This is our closing bracket
                    }
                }
                TokenType::OpenBrace => brace_depth += 1,
                TokenType::CloseBrace => {
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        break;
                    }
                }
                _ => {}
            }
            
            tokens.push(self.advance().clone());
        }
        
        tokens
    }
    
    // Parse a sequence of tokens as an expression
    fn parse_token_sequence_as_expr(&mut self, tokens: Vec<Token>) -> Result<Option<ExprType>, ParseError> {
        if tokens.is_empty() {
            return Ok(None);
        }
        
        // Save current position
        let saved_pos = self.current;
        let saved_tok_ref = self.current_tok_ref;
        
        // Create a temporary parser state for these tokens
        // We'll insert the tokens at the current position and parse them
        let mut temp_tokens = Vec::new();
        
        // Add tokens before current position
        for i in 0..saved_pos {
            temp_tokens.push(self.tokens[i].clone());
        }
        
        // Add our tokens to parse
        let parse_start = temp_tokens.len();
        for token in tokens {
            temp_tokens.push(token);
        }
        
        // Add an EOF token to ensure parsing stops
        temp_tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: TokenLiteral::None,
            line: self.peek().line,
            start: 0,
            length: 0,
        });
        
        // Store the original tokens reference
        let original_tokens = self.tokens;
        
        // Temporarily replace tokens
        // SAFETY: This is safe because we restore it before returning
        let temp_tokens_ref: &'a [Token] = unsafe {
            std::mem::transmute(temp_tokens.as_slice())
        };
        self.tokens = temp_tokens_ref;
        self.current = parse_start;
        
        // Parse the expression
        let result = self.expression();
        
        // Restore original state
        self.tokens = original_tokens;
        self.current = saved_pos;
        self.current_tok_ref = saved_tok_ref;
        
        result
    }
    
    // Parse bracket expressions to distinguish between indexing and slicing
    fn parse_bracket_expression(&mut self) -> Result<BracketExpressionType, ParseError> {
        // Simple decision tree approach
        
        // Check for :: (ColonColon) which in bracket context means slice with no start/end
        if self.check(TokenType::ColonColon) {
            // [::step] pattern - module separator :: is two colons in slice context
            self.advance(); // consume '::'
            
            let mut step_expr: Option<Box<ExprType>> = None;
            if !self.check(TokenType::RBracket) {
                let tokens = self.collect_tokens_until(&[TokenType::RBracket]);
                step_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
            }
            
            self.consume(TokenType::RBracket, "Expected ']'")?;
            
            return Ok(BracketExpressionType::Slice {
                start: None,
                end: None,
                step: step_expr,
            });
        }
        
        // Look at what comes first: expression, colon, or bracket
        if self.check(TokenType::Colon) {
            // Starts with colon: slice like [:end] or [:end:step] or [::step]
            self.advance(); // consume first ':'
            
            let mut end_expr: Option<Box<ExprType>> = None;
            let mut step_expr: Option<Box<ExprType>> = None;
            
            // Check if this is [::step] pattern (double colon at start)
            if self.check(TokenType::Colon) {
                // It's [::step] - skip end, go straight to step
                self.advance(); // consume second ':'
                if !self.check(TokenType::RBracket) {
                    let tokens = self.collect_tokens_until(&[TokenType::RBracket]);
                    step_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                }
            } else {
                // Parse end if not bracket
                if !self.check(TokenType::RBracket) {
                    let tokens = self.collect_tokens_until(&[TokenType::Colon, TokenType::RBracket]);
                    end_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                }
                
                // Check for step after end
                if self.match_token(&[TokenType::Colon]) {
                    if !self.check(TokenType::RBracket) {
                        let tokens = self.collect_tokens_until(&[TokenType::RBracket]);
                        step_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                    }
                }
            }
            
            self.consume(TokenType::RBracket, "Expected ']'")?;
            
            Ok(BracketExpressionType::Slice {
                start: None,
                end: end_expr,
                step: step_expr,
            })
            
        } else if self.check(TokenType::RBracket) {
            // Empty brackets - error
            let err_msg = "Empty brackets not allowed";
            self.error_at_current(err_msg);
            Err(ParseError::new(err_msg))
            
        } else {
            // Starts with expression
            let first_tokens = self.collect_tokens_until(&[TokenType::Colon, TokenType::ColonColon, TokenType::RBracket]);
            let first_expr = self.parse_token_sequence_as_expr(first_tokens)?;
            
            if self.check(TokenType::RBracket) {
                // Just an index: expr]
                self.advance(); // consume ']'
                
                if let Some(expr) = first_expr {
                    Ok(BracketExpressionType::Index(expr))
                } else {
                    // Empty brackets [] are not allowed for indexing
                    let err_msg = "Missing index expression";
                    self.error_at_previous(err_msg);
                    Err(ParseError::new(err_msg))
                }
                
            } else if self.check(TokenType::ColonColon) {
                // It's a slice with no end: expr::step
                self.advance(); // consume '::'
                
                let mut step_expr: Option<Box<ExprType>> = None;
                if !self.check(TokenType::RBracket) {
                    let tokens = self.collect_tokens_until(&[TokenType::RBracket]);
                    step_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                }
                
                self.consume(TokenType::RBracket, "Expected ']'")?;
                
                Ok(BracketExpressionType::Slice {
                    start: first_expr.map(Box::new),
                    end: None,
                    step: step_expr,
                })
                
            } else if self.check(TokenType::Colon) {
                // It's a slice: expr:...
                self.advance(); // consume ':'
                
                let mut end_expr: Option<Box<ExprType>> = None;
                let mut step_expr: Option<Box<ExprType>> = None;
                
                // Parse end if present
                if !self.check(TokenType::Colon) && !self.check(TokenType::RBracket) {
                    let tokens = self.collect_tokens_until(&[TokenType::Colon, TokenType::RBracket]);
                    end_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                }
                
                // Check for step
                if self.match_token(&[TokenType::Colon]) {
                    if !self.check(TokenType::RBracket) {
                        let tokens = self.collect_tokens_until(&[TokenType::RBracket]);
                        step_expr = self.parse_token_sequence_as_expr(tokens)?.map(Box::new);
                    }
                }
                
                self.consume(TokenType::RBracket, "Expected ']'")?;
                
                Ok(BracketExpressionType::Slice {
                    start: first_expr.map(Box::new),
                    end: end_expr,
                    step: step_expr,
                })
                
            } else {
                let err_msg = format!("Expected ':' or ']' but found '{}'", self.peek().lexeme);
                self.error_at_current(&err_msg);
                Err(ParseError::new(&err_msg))
            }
        }
    }
    
    // Classify what type of call this is based on the method name
    fn classify_call_type(&self, method_name: &str) -> CallType {
        // Check interface methods first (highest priority in Frame)
        if self.arcanum.lookup_interface_method(method_name).is_some() {
            return CallType::Interface;
        }
        
        // Check actions
        if self.arcanum.lookup_action(method_name).is_some() {
            return CallType::Action;
        }
        
        // Check operations
        if self.arcanum.lookup_operation(method_name).is_some() {
            return CallType::Operation;
        }
        
        // Check module-level functions
        if self.arcanum.lookup_function(method_name).is_some() {
            return CallType::Function;
        }
        
        // Unknown type (could be external function like print())
        CallType::Unknown
    }
    
    // Parse function/method arguments using token collection for better handling of nested expressions
    fn parse_arguments_with_collection(&mut self) -> Result<ExprListNode, ParseError> {
        let mut args = Vec::new();
        
        // If immediate closing paren, no arguments
        if self.check(TokenType::RParen) {
            self.advance();
            return Ok(ExprListNode::new(args));
        }
        
        // Collect and parse arguments
        loop {
            // Collect tokens for this argument (until comma or closing paren)
            let arg_tokens = self.collect_tokens_until(&[TokenType::Comma, TokenType::RParen]);
            
            // Parse the collected tokens as an expression
            if let Some(expr) = self.parse_token_sequence_as_expr(arg_tokens)? {
                args.push(expr);
            }
            
            // Check what stopped us
            if self.match_token(&[TokenType::Comma]) {
                // More arguments to come
                if self.check(TokenType::RParen) {
                    // Trailing comma before closing paren
                    self.advance();
                    break;
                }
                continue;
            } else if self.match_token(&[TokenType::RParen]) {
                // End of arguments
                break;
            } else {
                return Err(ParseError::new("Expected ',' or ')' in argument list"));
            }
        }
        
        Ok(ExprListNode::new(args))
    }

    // ===================== Frame v0.58 Class Decorators =====================
    
    fn parse_class_decorator(&mut self) -> Result<String, ParseError> {
        // Consume the @ symbol
        if !self.match_token(&[TokenType::At]) {
            return Err(ParseError::new("Expected '@' for decorator"));
        }
        
        // Get the decorator name
        if !self.match_token(&[TokenType::Identifier]) {
            self.error_at_current("Expected decorator name after '@'");
            return Err(ParseError::new("Expected decorator name after '@'"));
        }
        
        let mut decorator_str = format!("@{}", self.previous().lexeme);
        
        // Check for decorator arguments (optional)
        if self.match_token(&[TokenType::LParen]) {
            decorator_str.push('(');
            
            // For decorator arguments, just collect the raw tokens until closing paren
            // This preserves the exact Python syntax for pass-through
            let mut depth = 1;
            let mut arg_tokens = Vec::new();
            
            while depth > 0 && !self.is_at_end() {
                let token = self.peek();
                
                if token.token_type == TokenType::LParen {
                    depth += 1;
                } else if token.token_type == TokenType::RParen {
                    depth -= 1;
                    if depth == 0 {
                        break;  // Don't include the final closing paren
                    }
                }
                
                // Collect the token for pass-through
                arg_tokens.push(token.lexeme.clone());
                self.advance();
            }
            
            // Join all tokens with appropriate spacing
            decorator_str.push_str(&arg_tokens.join(""));
            
            if !self.match_token(&[TokenType::RParen]) {
                self.error_at_current("Expected ')' after decorator arguments");
                return Err(ParseError::new("Expected ')' after decorator arguments"));
            }
            decorator_str.push(')');
        }
        
        Ok(decorator_str)
    }
    
    fn class_decl_with_decorators(&mut self, decorators: Vec<String>) -> Result<Rc<RefCell<ClassNode>>, ParseError> {
        // We've already consumed 'class' keyword, now delegate to existing class_decl
        // but we need to modify it to accept decorators
        
        // Get the class using existing logic
        let class_node = self.class_decl()?;
        
        // Update the class node with decorators
        {
            let mut class = class_node.borrow_mut();
            class.decorators = decorators;
        }
        
        Ok(class_node)
    }
    
    // Helper function to convert expression to string for decorator pass-through
    fn expr_to_string(&self, expr: &ExprType) -> String {
        // Simplified expression to string conversion
        // In a full implementation, this would be more comprehensive
        match expr {
            ExprType::LiteralExprT { literal_expr_node } => {
                // Handle different literal types
                match literal_expr_node.token_t {
                    TokenType::String | TokenType::RawString => format!("\"{}\"", literal_expr_node.value),
                    _ => literal_expr_node.value.clone(),
                }
            }
            ExprType::VariableExprT { var_node } => var_node.id_node.name.lexeme.clone(),
            ExprType::NilExprT => "None".to_string(),
            _ => "...".to_string(), // Placeholder for complex expressions
        }
    }
}

// Helper enum for bracket expression parsing
enum BracketExpressionType {
    Index(ExprType),
    Slice {
        start: Option<Box<ExprType>>,
        end: Option<Box<ExprType>>,
        step: Option<Box<ExprType>>,
    },
}
