use super::ast::*;
use crate::compiler::Exe;
use crate::frame_c::symbol_table::SystemSymbolType::{
    ActionSymbol, DomainSymbol, InterfaceSymbol, OperationSymbol,
};
use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// NOTES
// - Structures labeled "*ScopeSymbol" indicate support for a scope inside the symbol.

// TODO: init from file
pub struct SymbolConfig {
    // pub start_msg_symbol: String,
    // pub stop_msg_symbol: String,
    pub enter_msg_symbol: String,
    pub exit_msg_symbol: String,
    // pub save_msg_symbol: String,
    // pub restore_msg_symbol: String,
}

impl SymbolConfig {
    pub fn new() -> SymbolConfig {
        SymbolConfig {
            // start_msg_symbol: String::from(">>"),
            // stop_msg_symbol: String::from("<<"),
            enter_msg_symbol: String::from("$>"),
            exit_msg_symbol: String::from("<$"),
            // save_msg_symbol: String::from(">>>"),
            // restore_msg_symbol: String::from("<<<"),
        }
    }
}

impl Default for SymbolConfig {
    fn default() -> Self {
        SymbolConfig::new()
    }
}

pub trait Symbol {
    fn get_name(&self) -> String;
}

pub trait ScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>>;
    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>>;
}

pub enum ParseScopeType {
    Function {
        function_scope_symbol_rcref: Rc<RefCell<FunctionScopeSymbol>>,
    },
    System {
        system_symbol: Rc<RefCell<SystemSymbol>>,
    },
    InterfaceBlock {
        interface_block_scope_symbol_rcref: Rc<RefCell<InterfaceBlockScopeSymbol>>,
    },
    // TODO:
    // InterfaceMethodDeclScope,
    MachineBlock {
        machine_scope_symbol_rcref: Rc<RefCell<MachineBlockScopeSymbol>>,
    },
    ActionsBlock {
        actions_block_scope_symbol_rcref: Rc<RefCell<ActionsBlockScopeSymbol>>,
    },
    Action {
        action_scope_symbol_rcref: Rc<RefCell<ActionScopeSymbol>>,
    },
    OperationsBlock {
        operations_block_scope_symbol_rcref: Rc<RefCell<OperationsBlockScopeSymbol>>,
    },
    Operation {
        operation_scope_symbol_rcref: Rc<RefCell<OperationScopeSymbol>>,
    },
    DomainBlock {
        domain_block_scope_symbol_rcref: Rc<RefCell<DomainBlockScopeSymbol>>,
    },
    State {
        state_symbol: Rc<RefCell<StateSymbol>>,
    },
    StateParams {
        state_params_scope_symbol_rcref: Rc<RefCell<StateParamsScopeSymbol>>,
    },
    StateLocal {
        state_local_scope_symbol_rcref: Rc<RefCell<StateLocalScopeSymbol>>,
    },
    EventHandler {
        event_handler_scope_symbol_rcref: Rc<RefCell<EventHandlerScopeSymbol>>,
    },
    EventHandlerParams {
        event_handler_params_scope_symbol_rcref: Rc<RefCell<EventHandlerParamsScopeSymbol>>,
    },
    EventHandlerLocal {
        event_handler_local_scope_symbol_rcref: Rc<RefCell<EventHandlerLocalScopeSymbol>>,
    },
    Loop {
        loop_scope_symbol_rcref: Rc<RefCell<LoopStmtScopeSymbol>>,
    },
    Block {
        block_scope_rcref: Rc<RefCell<BlockScope>>,
    },
    Params {
        params_scope_symbol_rcref: Rc<RefCell<ParamsScopeSymbol>>,
    },
}

// This is what gets stored in the symbol tables
pub enum SymbolType {
    FunctionScope {
        function_symbol_ref: Rc<RefCell<FunctionScopeSymbol>>,
    },
    System {
        system_symbol_rcref: Rc<RefCell<SystemSymbol>>,
    },
    #[allow(dead_code)] // not dead. weird
    InterfaceBlock {
        interface_block_symbol_rcref: Rc<RefCell<InterfaceBlockScopeSymbol>>,
    },
    // TODO: Add InterfaceMethod
    InterfaceMethod {
        interface_method_symbol_rcref: Rc<RefCell<InterfaceMethodSymbol>>,
    },
    MachineBlockScope {
        machine_block_symbol_rcref: Rc<RefCell<MachineBlockScopeSymbol>>,
    },
    ActionsBlockScope {
        actions_block_symbol_rcref: Rc<RefCell<ActionsBlockScopeSymbol>>,
    },
    ActionScope {
        action_scope_symbol_rcref: Rc<RefCell<ActionScopeSymbol>>,
    },
    OperationsBlockScope {
        operations_block_symbol_rcref: Rc<RefCell<OperationsBlockScopeSymbol>>,
    },
    OperationScope {
        operation_scope_symbol_rcref: Rc<RefCell<OperationScopeSymbol>>,
    },
    DomainBlockScope {
        domain_block_symbol_rcref: Rc<RefCell<DomainBlockScopeSymbol>>,
    },
    State {
        state_symbol_ref: Rc<RefCell<StateSymbol>>,
    },
    StateParamsScope {
        state_params_scope_rcref: Rc<RefCell<StateParamsScopeSymbol>>,
    },
    StateLocalScope {
        state_local_scope_struct_rcref: Rc<RefCell<StateLocalScopeSymbol>>,
    },
    EventHandlerScope {
        event_handler_scope_symbol: Rc<RefCell<EventHandlerScopeSymbol>>,
    },
    EventHandlerParamsScope {
        event_handler_params_scope_symbol_rcref: Rc<RefCell<EventHandlerParamsScopeSymbol>>,
    },
    EventHandlerLocalScope {
        event_handler_local_scope_rcref: Rc<RefCell<EventHandlerLocalScopeSymbol>>,
    },

    EnumDeclSymbolT {
        enum_symbol_rcref: Rc<RefCell<EnumSymbol>>,
    },
    LoopStmtSymbol {
        loop_scope_symbol_rcref: Rc<RefCell<LoopStmtScopeSymbol>>,
    },
    BlockScope {
        block_scope_rcref: Rc<RefCell<BlockScope>>,
    },
    ParamsScope {
        params_scope_symbol_rcref: Rc<RefCell<ParamsScopeSymbol>>,
    },
    LoopVar {
        loop_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    BlockVar {
        block_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    DomainVariable {
        domain_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    StateVariable {
        state_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    EventHandlerVariable {
        event_handler_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },

    ParamSymbol {
        param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
    },
    // TODO: figure out if thse are really used anymore. I think the
    // IdentifierDeclScope::StateParam replaced this.
    StateParam {
        state_param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
    },
    EventHandlerParam {
        event_handler_param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
    },
}

impl SymbolType {
    pub fn assign(&mut self, r_value: Rc<ExprType>) -> Result<(), &str> {
        // let debug_type = self.debug_symbol_type_name();
        match self {
            SymbolType::BlockVar {
                block_variable_symbol_rcref,
            } => {
                let variable_symbol = block_variable_symbol_rcref.borrow_mut();
                let mut var_decl_node = variable_symbol.ast_node_rcref.borrow_mut();
                var_decl_node.value_rc = r_value;
                Ok(())
            }
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => {
                let variable_symbol = state_variable_symbol_rcref.borrow_mut();
                let mut var_decl_node = variable_symbol.ast_node_rcref.borrow_mut();
                var_decl_node.value_rc = r_value;
                Ok(())
            }
            SymbolType::LoopVar {
                loop_variable_symbol_rcref,
            } => {
                let variable_symbol = loop_variable_symbol_rcref.borrow_mut();
                let mut var_decl_node = variable_symbol.ast_node_rcref.borrow_mut();
                var_decl_node.value_rc = r_value;
                Ok(())
            }
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => {
                let variable_symbol = domain_variable_symbol_rcref.borrow_mut();
                let mut var_decl_node = variable_symbol.ast_node_rcref.borrow_mut();
                var_decl_node.value_rc = r_value;
                Ok(())
            }
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => {
                let variable_symbol = event_handler_variable_symbol_rcref.borrow_mut();
                let mut var_decl_node = variable_symbol.ast_node_rcref.borrow_mut();
                var_decl_node.value_rc = r_value;
                Ok(())
            }
            // TODO - this as part of param/var alignment
            // See https://github.com/frame-lang/frame_transpiler/issues/151
            SymbolType::EventHandlerParam { .. }
            | SymbolType::StateParam { .. }
            | SymbolType::ParamSymbol { .. } => Ok(()),
            _ => Err("Invalid l_value."),
        }
    }

    pub fn set_ast_node(
        &mut self,
        variable_decl_node_rcref: Rc<RefCell<VariableDeclNode>>,
    ) -> Result<(), &'static str> {
        match self {
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => {
                domain_variable_symbol_rcref
                    .borrow_mut()
                    .set_ast_node(variable_decl_node_rcref.clone());
            }
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => {
                //                    let a = state_variable_symbol_rcref.borrow();
                state_variable_symbol_rcref
                    .borrow_mut()
                    .set_ast_node(variable_decl_node_rcref.clone());
            }
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => {
                event_handler_variable_symbol_rcref
                    .borrow_mut()
                    .set_ast_node(variable_decl_node_rcref.clone());
            }
            SymbolType::LoopVar {
                loop_variable_symbol_rcref,
            } => {
                loop_variable_symbol_rcref
                    .borrow_mut()
                    .set_ast_node(variable_decl_node_rcref.clone());
            }
            SymbolType::BlockVar {
                block_variable_symbol_rcref,
            } => {
                block_variable_symbol_rcref
                    .borrow_mut()
                    .set_ast_node(variable_decl_node_rcref.clone());
            }
            _ => {
                let err_msg = "Unrecognized variable type.";
                return Err(err_msg);
            }
        }

        Ok(())
    }

    pub fn get_ast_node(&mut self) -> Result<Option<Rc<RefCell<VariableDeclNode>>>, &'static str> {
        match self {
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => Ok(Some(
                domain_variable_symbol_rcref
                    .borrow_mut()
                    .get_ast_node()
                    .clone(),
            )),
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => {
                //                    let a = state_variable_symbol_rcref.borrow();
                Ok(Some(
                    state_variable_symbol_rcref
                        .borrow_mut()
                        .get_ast_node()
                        .clone(),
                ))
            }
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => Ok(Some(
                event_handler_variable_symbol_rcref
                    .borrow_mut()
                    .get_ast_node()
                    .clone(),
            )),
            SymbolType::LoopVar {
                loop_variable_symbol_rcref,
            } => Ok(Some(
                loop_variable_symbol_rcref
                    .borrow_mut()
                    .get_ast_node()
                    .clone(),
            )),
            SymbolType::BlockVar {
                block_variable_symbol_rcref,
            } => Ok(Some(
                block_variable_symbol_rcref
                    .borrow_mut()
                    .get_ast_node()
                    .clone(),
            )),
            SymbolType::ParamSymbol { .. } => Ok(None),
            SymbolType::StateParam { .. } => Ok(None),
            SymbolType::EventHandlerParam { .. } => Ok(None),
            _ => {
                let err_msg = "Unrecognized variable type.";
                Err(err_msg)
            }
        }
    }

    /// Get the name of expression type we're looking at. Useful for debugging.
    pub fn debug_symbol_type_name(&self) -> &'static str {
        match self {
            SymbolType::FunctionScope { .. } => "FunctionScope",
            SymbolType::System { .. } => "System",
            SymbolType::InterfaceBlock { .. } => "InterfaceBlock",
            SymbolType::InterfaceMethod { .. } => "InterfaceMethod",
            SymbolType::MachineBlockScope { .. } => "MachineBlockScope",
            SymbolType::ActionsBlockScope { .. } => "ActionsBlockScope",
            SymbolType::ActionScope { .. } => "ActionScope",
            SymbolType::OperationsBlockScope { .. } => "OperationsBlockScope",
            SymbolType::OperationScope { .. } => "OperationBlockScope",
            SymbolType::DomainBlockScope { .. } => "DomainBlockScope",
            SymbolType::State { .. } => "State",
            SymbolType::StateParamsScope { .. } => "StateParamsScope",
            SymbolType::StateLocalScope { .. } => "StateLocalScope",
            SymbolType::EventHandlerScope { .. } => "EventHandlerScope",
            SymbolType::EventHandlerParamsScope { .. } => "EventHandlerParamsScope",
            SymbolType::EventHandlerLocalScope { .. } => "EventHandlerLocalScope",
            SymbolType::EnumDeclSymbolT { .. } => "EnumDeclSymbolT",
            SymbolType::LoopStmtSymbol { .. } => "LoopStmtSymbol",
            SymbolType::BlockScope { .. } => "BlockScope",
            SymbolType::ParamsScope { .. } => "ParamsScope",
            SymbolType::LoopVar { .. } => "LoopVar",
            SymbolType::BlockVar { .. } => "BlockVar",
            SymbolType::DomainVariable { .. } => "DomainVariable",
            SymbolType::EventHandlerVariable { .. } => "EventHandlerVariable",
            SymbolType::ParamSymbol { .. } => "ParamSymbol",
            SymbolType::StateParam { .. } => "StateParam",
            SymbolType::EventHandlerParam { .. } => "EventHandlerParam",
            SymbolType::StateVariable { .. } => "StateVariable",
        }
    }
}

impl Symbol for SymbolType {
    fn get_name(&self) -> String {
        match self {
            SymbolType::FunctionScope {
                function_symbol_ref,
            } => function_symbol_ref.borrow().get_name(),
            SymbolType::System {
                system_symbol_rcref: system_symbol_ref,
            } => system_symbol_ref.borrow().get_name(),
            SymbolType::InterfaceBlock {
                interface_block_symbol_rcref,
            } => interface_block_symbol_rcref.borrow().get_name(),
            SymbolType::InterfaceMethod {
                interface_method_symbol_rcref,
            } => interface_method_symbol_rcref.borrow().get_name(),
            SymbolType::MachineBlockScope {
                machine_block_symbol_rcref,
            } => machine_block_symbol_rcref.borrow().get_name(),
            SymbolType::ActionsBlockScope {
                actions_block_symbol_rcref,
            } => actions_block_symbol_rcref.borrow().get_name(),
            SymbolType::ActionScope {
                action_scope_symbol_rcref: action_symbol_rcref,
            } => action_symbol_rcref.borrow().get_name(),
            SymbolType::OperationsBlockScope {
                operations_block_symbol_rcref,
            } => operations_block_symbol_rcref.borrow().get_name(),
            SymbolType::OperationScope {
                operation_scope_symbol_rcref: operation_symbol_rcref,
            } => operation_symbol_rcref.borrow().get_name(),
            SymbolType::DomainBlockScope {
                domain_block_symbol_rcref,
            } => domain_block_symbol_rcref.borrow().get_name(),
            SymbolType::State { state_symbol_ref } => state_symbol_ref.borrow().get_name(),
            SymbolType::StateParamsScope {
                state_params_scope_rcref,
            } => state_params_scope_rcref.borrow().get_name(),
            SymbolType::StateLocalScope {
                state_local_scope_struct_rcref: state_block_scope_struct_rcref,
            } => state_block_scope_struct_rcref.borrow().get_name(),
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => domain_variable_symbol_rcref.borrow().get_name(),
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => state_variable_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerScope {
                event_handler_scope_symbol,
            } => event_handler_scope_symbol.borrow().get_name(),
            SymbolType::EventHandlerParamsScope {
                event_handler_params_scope_symbol_rcref,
            } => event_handler_params_scope_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerParam {
                event_handler_param_symbol_rcref,
            } => event_handler_param_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => event_handler_variable_symbol_rcref.borrow().get_name(),
            SymbolType::StateParam {
                state_param_symbol_rcref,
            } => state_param_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerLocalScope {
                event_handler_local_scope_rcref,
            } => event_handler_local_scope_rcref.borrow().get_name(),
            SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                enum_symbol_rcref.borrow().get_name()
            }
            SymbolType::LoopStmtSymbol {
                loop_scope_symbol_rcref,
            } => loop_scope_symbol_rcref.borrow().get_name(),
            SymbolType::LoopVar {
                loop_variable_symbol_rcref,
            } => loop_variable_symbol_rcref.borrow().get_name(),
            SymbolType::BlockScope { block_scope_rcref } => block_scope_rcref.borrow().get_name(),
            SymbolType::BlockVar {
                block_variable_symbol_rcref: block_var_rcref,
            } => block_var_rcref.borrow().get_name(),
            SymbolType::ParamsScope {
                params_scope_symbol_rcref,
            } => params_scope_symbol_rcref.borrow().get_name(),
            SymbolType::ParamSymbol { param_symbol_rcref } => {
                param_symbol_rcref.borrow().get_name()
            }
        }
    }
}

impl ScopeSymbol for SymbolType {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        match self {
            SymbolType::FunctionScope {
                function_symbol_ref,
            } => function_symbol_ref.borrow().get_symbol_table(),
            SymbolType::System {
                system_symbol_rcref: system_symbol_ref,
            } => system_symbol_ref.borrow().get_symbol_table(),
            SymbolType::InterfaceBlock {
                interface_block_symbol_rcref,
            } => interface_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::MachineBlockScope {
                machine_block_symbol_rcref,
            } => machine_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::ActionsBlockScope {
                actions_block_symbol_rcref,
            } => actions_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::ActionScope {
                action_scope_symbol_rcref,
            } => action_scope_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::OperationsBlockScope {
                operations_block_symbol_rcref,
            } => operations_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::OperationScope {
                operation_scope_symbol_rcref,
            } => operation_scope_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::DomainBlockScope {
                domain_block_symbol_rcref,
            } => domain_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::DomainVariable { .. } => {
                panic!("Fatal error - domain variable symbol does not have a symbol table.")
            }
            //                => domain_variable_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::State { state_symbol_ref } => state_symbol_ref.borrow().get_symbol_table(),
            SymbolType::StateParamsScope {
                state_params_scope_rcref,
            } => state_params_scope_rcref.borrow().get_symbol_table(),
            SymbolType::StateParam { .. } => {
                panic!("Fatal error - state param symbol does not have a symbol table.")
            }
            //state_param_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::StateLocalScope {
                state_local_scope_struct_rcref,
            } => state_local_scope_struct_rcref.borrow().get_symbol_table(),
            SymbolType::EventHandlerScope {
                event_handler_scope_symbol,
            } => event_handler_scope_symbol.borrow().get_symbol_table(),
            SymbolType::EventHandlerParamsScope {
                event_handler_params_scope_symbol_rcref,
            } => event_handler_params_scope_symbol_rcref
                .borrow()
                .get_symbol_table(),
            SymbolType::EventHandlerLocalScope {
                event_handler_local_scope_rcref,
            } => event_handler_local_scope_rcref.borrow().get_symbol_table(),
            SymbolType::LoopStmtSymbol {
                loop_scope_symbol_rcref,
            } => loop_scope_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::BlockScope { block_scope_rcref } => {
                block_scope_rcref.borrow().get_symbol_table()
            }
            SymbolType::BlockVar { .. } => {
                panic!("Fatal error - block variable symbol does not have a symbol table.")
            }
            // SymbolType::FunctionScope { .. } => {
            //     panic!("Fatal error - FunctionScope scope does not have a symbol table.")
            // }
            SymbolType::ParamsScope {
                params_scope_symbol_rcref,
            } => params_scope_symbol_rcref.borrow().get_symbol_table(),

            SymbolType::EnumDeclSymbolT { .. } => {
                panic!("Fatal error - EnumDeclSymbolT scope does not have a symbol table.")
            }
            SymbolType::LoopVar { .. } => {
                panic!("Fatal error - LoopVar scope does not have a symbol table.")
            }
            SymbolType::InterfaceMethod { .. } => {
                panic!("Fatal error - InterfaceMethod scope does not have a symbol table.")
            }
            SymbolType::StateVariable { .. } => {
                panic!("Fatal error - InterfaceMethod scope does not have a symbol table.")
            }
            SymbolType::EventHandlerVariable { .. } => {
                panic!("Fatal error - InterfaceMethod scope does not have a symbol table.")
            }
            SymbolType::ParamSymbol { .. } => {
                panic!("Fatal error - ParamSymbol scope does not have a symbol table.")
            }
            SymbolType::EventHandlerParam { .. } => {
                panic!("Fatal error - EventHandlerParam scope does not have a symbol table.")
            } // _ => {
              //     panic!("Could not find SymbolType. Giving up.")
              // }
        }
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        match self {
            SymbolType::System {
                system_symbol_rcref: system_symbol_ref,
            } => system_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::MachineBlockScope {
                machine_block_symbol_rcref: machine_symbol_ref,
            } => machine_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::DomainBlockScope {
                domain_block_symbol_rcref: domain_symbol_ref,
            } => domain_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::State { state_symbol_ref } => state_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::StateParamsScope {
                state_params_scope_rcref,
            } => state_params_scope_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::StateLocalScope {
                state_local_scope_struct_rcref: state_block_scope_struct_rcref,
            } => state_block_scope_struct_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::EventHandlerParamsScope {
                event_handler_params_scope_symbol_rcref: event_handler_params_symbol_rcref,
            } => event_handler_params_symbol_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::EventHandlerLocalScope {
                event_handler_local_scope_rcref: event_handler_block_scope_struct_rcref,
            } => event_handler_block_scope_struct_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            _ => panic!("TODO"),
        }
    }
}

pub struct SymbolTable {
    pub name: String,
    pub parent_symtab_rcref_opt: Option<Rc<RefCell<SymbolTable>>>,
    pub symbols: HashMap<String, Rc<RefCell<SymbolType>>>,
    pub identifier_decl_scope: IdentifierDeclScope,
    pub is_system_symtab: bool,
}

impl SymbolTable {
    pub fn new(
        name: String,
        parent: Option<Rc<RefCell<SymbolTable>>>,
        identifier_scope: IdentifierDeclScope,
        is_system_symtab: bool,
    ) -> SymbolTable {
        SymbolTable {
            name,
            parent_symtab_rcref_opt: parent,
            symbols: HashMap::new(),
            identifier_decl_scope: identifier_scope,
            is_system_symtab,
        }
    }

    pub fn insert_parse_scope(&mut self, scope_t: ParseScopeType) {
        match scope_t {
            ParseScopeType::Function {
                function_scope_symbol_rcref,
            } => {
                let name = function_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::FunctionScope {
                    function_symbol_ref: function_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::System { system_symbol } => {
                let name = system_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::System {
                    system_symbol_rcref: system_symbol,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::InterfaceBlock {
                interface_block_scope_symbol_rcref,
            } => {
                let name = interface_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::InterfaceBlock {
                    interface_block_symbol_rcref: interface_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::MachineBlock {
                machine_scope_symbol_rcref: machine_symbol,
            } => {
                let name = machine_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::MachineBlockScope {
                    machine_block_symbol_rcref: machine_symbol,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::State { state_symbol } => {
                let name = state_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::State {
                    state_symbol_ref: state_symbol,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::StateParams {
                state_params_scope_symbol_rcref: state_params_scope,
            } => {
                let name = state_params_scope.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateParamsScope {
                    state_params_scope_rcref: state_params_scope,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::StateLocal {
                state_local_scope_symbol_rcref: state_block_scope_symbol_rcref,
            } => {
                let name = state_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateLocalScope {
                    state_local_scope_struct_rcref: state_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::EventHandler {
                event_handler_scope_symbol_rcref,
            } => {
                let name = event_handler_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerScope {
                    event_handler_scope_symbol: event_handler_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::EventHandlerParams {
                event_handler_params_scope_symbol_rcref,
            } => {
                let name = event_handler_params_scope_symbol_rcref
                    .borrow()
                    .name
                    .clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerParamsScope {
                    event_handler_params_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::EventHandlerLocal {
                event_handler_local_scope_symbol_rcref: event_handler_block_scope_symbol_rcref,
            } => {
                let name = event_handler_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerLocalScope {
                    event_handler_local_scope_rcref: event_handler_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::ActionsBlock {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            } => {
                let name = actions_block_scope_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::ActionsBlockScope {
                    actions_block_symbol_rcref: actions_block_scope_symbol,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::Action {
                action_scope_symbol_rcref,
            } => {
                let name = action_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::ActionScope {
                    action_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::OperationsBlock {
                operations_block_scope_symbol_rcref: library_block_scope_symbol_rcref,
            } => {
                let name = library_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::OperationsBlockScope {
                    operations_block_symbol_rcref: library_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::Operation {
                operation_scope_symbol_rcref,
            } => {
                let name = operation_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::OperationScope {
                    operation_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::DomainBlock {
                domain_block_scope_symbol_rcref: domain_symbol,
            } => {
                let name = domain_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::DomainBlockScope {
                    domain_block_symbol_rcref: domain_symbol,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::Loop {
                loop_scope_symbol_rcref,
            } => {
                let name = loop_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::LoopStmtSymbol {
                    loop_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::Block { block_scope_rcref } => {
                let name = block_scope_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::BlockScope { block_scope_rcref }));
                self.symbols.insert(name, st_ref);
            }
            ParseScopeType::Params {
                params_scope_symbol_rcref,
            } => {
                let name = params_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::ParamsScope {
                    params_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
            }
        }
    }

    pub fn define(&mut self, symbol_t: &SymbolType) -> Result<(), String> {
        match symbol_t {
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => {
                let name = domain_variable_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::DomainVariable {
                    domain_variable_symbol_rcref: Rc::clone(domain_variable_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::StateParam {
                state_param_symbol_rcref,
            } => {
                let name = state_param_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::StateParam {
                    state_param_symbol_rcref: Rc::clone(state_param_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::StateLocalScope {
                state_local_scope_struct_rcref,
            } => {
                let name = state_local_scope_struct_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::StateLocalScope {
                    state_local_scope_struct_rcref: Rc::clone(state_local_scope_struct_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => {
                let name = state_variable_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::StateVariable {
                    state_variable_symbol_rcref: Rc::clone(state_variable_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::EventHandlerParam {
                event_handler_param_symbol_rcref,
            } => {
                let name = event_handler_param_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerParam {
                    event_handler_param_symbol_rcref: Rc::clone(event_handler_param_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::EventHandlerLocalScope {
                event_handler_local_scope_rcref,
            } => {
                let name = event_handler_local_scope_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerLocalScope {
                    event_handler_local_scope_rcref: Rc::clone(event_handler_local_scope_rcref),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => {
                let name = event_handler_variable_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerVariable {
                    event_handler_variable_symbol_rcref: Rc::clone(
                        event_handler_variable_symbol_rcref,
                    ),
                }));
                self.symbols.insert(name, st_ref);
                Ok(())
            }
            // TODO: Currently actions are just declared.
            // When actions have bodies then this should become a scope symbol.
            SymbolType::ActionScope {
                action_scope_symbol_rcref: action_symbol_rcref,
            } => {
                let name = action_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::ActionScope {
                    action_scope_symbol_rcref: Rc::clone(action_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            SymbolType::InterfaceMethod {
                interface_method_symbol_rcref,
            } => {
                let name = interface_method_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::InterfaceMethod {
                    interface_method_symbol_rcref: Rc::clone(interface_method_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                let name = enum_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::EnumDeclSymbolT {
                    enum_symbol_rcref: Rc::clone(enum_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            SymbolType::LoopVar {
                loop_variable_symbol_rcref,
            } => {
                let name = loop_variable_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::LoopVar {
                    loop_variable_symbol_rcref: Rc::clone(loop_variable_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            SymbolType::BlockVar {
                block_variable_symbol_rcref,
            } => {
                let name = block_variable_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::BlockVar {
                    block_variable_symbol_rcref: Rc::clone(block_variable_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            SymbolType::ParamSymbol { param_symbol_rcref } => {
                let name = param_symbol_rcref.borrow().name.clone();
                if self.symbols.get(&name[..]).is_some() {
                    let msg = format!("redeclaration of {}", name).to_string();
                    return Err(msg);
                }
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::ParamSymbol {
                    param_symbol_rcref: Rc::clone(param_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                Ok(())
            }
            _ => panic!("Fatal error - missing symbol type"),
        }
    }

    pub fn lookup(
        &self,
        name: &str,
        search_scope: &IdentifierDeclScope,
    ) -> Option<Rc<RefCell<SymbolType>>> {
        // if this is the symbol table for the system, then look in the domain symbol table to resolve the symbol.
        if self.is_system_symtab {
            let domain_block_scope_symtype =
                (self.symbols).get(DomainBlockScopeSymbol::scope_name());
            let symbol_type_rcref = match domain_block_scope_symtype {
                Some(symbol_type) => symbol_type,
                None => return None,
            };
            let symbol_type = symbol_type_rcref.borrow();
            match &*symbol_type {
                SymbolType::DomainBlockScope {
                    domain_block_symbol_rcref,
                } => {
                    let domain_block_scope_symbol = domain_block_symbol_rcref.borrow();
                    let symbol_table = domain_block_scope_symbol.symtab_rcref.borrow();
                    match symbol_table.lookup_local(name) {
                        Some(a) => {
                            return Some(a);
                        }
                        None => return None,
                    }
                }
                _ => return None,
            }
        }

        if *search_scope == IdentifierDeclScope::UnknownScope
            || *search_scope == self.identifier_decl_scope
        {
            if let Some(aa) = self.symbols.get(name) {
                return Some(Rc::clone(aa));
            }
        }

        match &self.parent_symtab_rcref_opt {
            Some(b) => {
                let c = b.borrow();
                let d = c.lookup(name, search_scope);
                d.map(|e| Rc::clone(&e))
            }
            None => None,
        }
    }

    pub fn lookup_local(&self, name: &str) -> Option<Rc<RefCell<SymbolType>>> {
        let a = (self.symbols).get(name);
        a.cloned()
    }

    pub fn get_parent_symtab(&self) -> Option<Rc<RefCell<SymbolTable>>> {
        let parent_symtab_rcref = self.parent_symtab_rcref_opt.as_ref()?;
        Some(Rc::clone(parent_symtab_rcref))
    }
}

// TODO
impl fmt::Display for SymbolTable {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymbolTable {}", self.name)
    }
}

pub enum SystemSymbolType {
    DomainSymbol {
        domain_scope_symbol_rcref: Rc<RefCell<SymbolType>>,
    },
    OperationSymbol {
        operation_scope_symbol_rcref: Rc<RefCell<OperationScopeSymbol>>,
    },
    ActionSymbol {
        action_scope_symbol_rcref: Rc<RefCell<ActionScopeSymbol>>,
    },
    InterfaceSymbol {
        interface_scope_symbol_rcref: Rc<RefCell<InterfaceMethodSymbol>>,
    },
}

// LEGB Scope Context
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeContext {
    Global,           // Module level
    Function(String), // Inside a specific function
    System(String),   // Inside a specific system
}

// LEGB Lookup Results  
pub enum LegbLookupResult {
    Function(Rc<RefCell<FunctionScopeSymbol>>),
    Action(Rc<RefCell<ActionScopeSymbol>>),
    Operation(Rc<RefCell<OperationScopeSymbol>>),
    Builtin(Rc<RefCell<FunctionScopeSymbol>>),
}

pub struct Arcanum {
    pub global_symtab: Rc<RefCell<SymbolTable>>,
    pub current_symtab: Rc<RefCell<SymbolTable>>,
    // LEGB Scoping
    pub builtin_symtab: Rc<RefCell<SymbolTable>>,  // Built-in functions (print, etc.)
    pub scope_context: ScopeContext,               // Current parsing context
    // Legacy single-system support (maintained for backward compatibility)
    pub system_symbol_opt: Option<Rc<RefCell<SystemSymbol>>>,
    // v0.30: Multi-entity collections
    pub function_symbols: Vec<Rc<RefCell<FunctionScopeSymbol>>>,
    pub system_symbols: Vec<Rc<RefCell<SystemSymbol>>>,
    pub symbol_config: SymbolConfig,
    pub serializable: bool,
}

impl Arcanum {
    pub fn new() -> Arcanum {
        // Create global (module) symbol table
        let st = SymbolTable::new(
            String::from("global"),
            None,
            IdentifierDeclScope::UnknownScope,
            false,
        );
        let global_symbtab_rc = Rc::new(RefCell::new(st));
        
        // Create built-in symbol table with Frame built-in functions
        let builtin_st = SymbolTable::new(
            String::from("builtins"),
            None,
            IdentifierDeclScope::UnknownScope,
            false,
        );
        let builtin_symbtab_rc = Rc::new(RefCell::new(builtin_st));
        
        // Add built-in functions
        Self::add_builtin_functions(&builtin_symbtab_rc);

        Arcanum {
            current_symtab: Rc::clone(&global_symbtab_rc),
            global_symtab: global_symbtab_rc,
            builtin_symtab: builtin_symbtab_rc,
            scope_context: ScopeContext::Global,
            system_symbol_opt: None, // Legacy compatibility
            function_symbols: Vec::new(), // v0.30: Multi-function support
            system_symbols: Vec::new(),   // v0.30: Multi-system support
            symbol_config: SymbolConfig::new(), // TODO
            serializable: false,
        }
    }

    /// Add built-in functions to the built-in symbol table
    fn add_builtin_functions(builtin_symtab: &Rc<RefCell<SymbolTable>>) {
        // Add print() built-in function
        // For now, we'll create a simple function symbol
        // TODO: Create proper BuiltinFunctionSymbol type if needed
        let print_symbol = SymbolType::FunctionScope {
            function_symbol_ref: Rc::new(RefCell::new(FunctionScopeSymbol::new("print".to_string()))),
        };
        builtin_symtab.borrow_mut().symbols.insert("print".to_string(), Rc::new(RefCell::new(print_symbol)));
    }

    pub fn debug_print_current_symbols(&self, symbol_table_rcref: Rc<RefCell<SymbolTable>>) {
        Exe::debug_print("<------------------->");
        self.do_debug_print_current_symbols(symbol_table_rcref);
        Exe::debug_print("<------------------->");
    }

    fn do_debug_print_current_symbols(&self, symbol_table_rcref: Rc<RefCell<SymbolTable>>) {
        let symbol_table = symbol_table_rcref.borrow();

        Exe::debug_print("---------------------");
        Exe::debug_print(&format!("SymbolTable {}", symbol_table.name));
        for key in symbol_table.symbols.keys() {
            Exe::debug_print(key);
        }

        if let Some(parent_symbol_table_rcref) = &symbol_table.parent_symtab_rcref_opt {
            self.do_debug_print_current_symbols(parent_symbol_table_rcref.clone());
        }
    }

    pub fn get_current_symtab(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.current_symtab)
    }

    pub fn get_current_identifier_scope(&self) -> IdentifierDeclScope {
        self.current_symtab.borrow().identifier_decl_scope.clone()
    }

    pub fn lookup(
        &self,
        name: &str,
        search_scope: &IdentifierDeclScope,
    ) -> Option<Rc<RefCell<SymbolType>>> {
        self.current_symtab.borrow().lookup(name, search_scope)
    }

    pub fn get_system_symbol(&self, name: &str) -> Result<SystemSymbolType,String> {
        if let Some(domain_scope_symbol_rcref) =
            self.lookup(name, &IdentifierDeclScope::DomainBlockScope)
        {
            Ok(DomainSymbol {
                domain_scope_symbol_rcref
            })
        } else if let Some(action_scope_symbol_rcref) = self.lookup_action(name) {
            Ok(ActionSymbol {
                action_scope_symbol_rcref,
            })
        } else if let Some(interface_scope_symbol_rcref) = self.lookup_interface_method(name) {
            Ok(InterfaceSymbol {
                interface_scope_symbol_rcref,
            })
        } else if let Some(operation_scope_symbol_rcref) = self.lookup_operation(name) {
            Ok(OperationSymbol {
                operation_scope_symbol_rcref,
            })
        } else {
            Err(String::from("Symbol not found in system."))
        }
    }

    // Interface methods are only declared in the -interface- block.
    // Get -interface-block- symtab from the system symbol and lookup.
    pub fn lookup_interface_method(
        &self,
        name: &str,
    ) -> Option<Rc<RefCell<InterfaceMethodSymbol>>> {
        let system_symbol_rcref_opt = &self.system_symbol_opt.as_ref();
        match system_symbol_rcref_opt {
            Some(system_symbol_rcref) => {
                match &system_symbol_rcref.borrow().interface_block_symbol_opt {
                    Some(interface_block_symbol_rcref) => {
                        let interface_block_symbol = interface_block_symbol_rcref.borrow();
                        let symbol_table = &interface_block_symbol.symtab_rcref.borrow();
                        self.debug_print_current_symbols(
                            interface_block_symbol.symtab_rcref.clone(),
                        );
                        match symbol_table.lookup(name, &IdentifierDeclScope::InterfaceBlockScope) {
                            Some(symbol_t_rcref) => {
                                let symbol_t = symbol_t_rcref.borrow();
                                match &*symbol_t {
                                    SymbolType::InterfaceMethod {
                                        interface_method_symbol_rcref,
                                    } => Some(Rc::clone(interface_method_symbol_rcref)),
                                    _ => None,
                                }
                            }
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    // Actions are only declared in the -actions- block.
    // Get -actions-block- symtab from the system symbol and lookup.
    #[allow(clippy::many_single_char_names)] // TODO
    pub fn lookup_action(&self, name: &str) -> Option<Rc<RefCell<ActionScopeSymbol>>> {
        let system_symbol_opt = &self.system_symbol_opt.as_ref();
        match system_symbol_opt {
            Some(system_symbol) => match &system_symbol.borrow().actions_block_symbol_opt {
                Some(actions_block_scope_symbol) => {
                    let actions_block_scope_symbol = actions_block_scope_symbol.borrow();
                    let symbol_table = &actions_block_scope_symbol.symtab_rcref.borrow();
                    match symbol_table.lookup(name, &IdentifierDeclScope::ActionsBlockScope) {
                        Some(symbol_table_rcref) => {
                            let symbol_t = symbol_table_rcref.borrow();
                            match &*symbol_t {
                                SymbolType::ActionScope {
                                    action_scope_symbol_rcref: action_symbol_rcref,
                                } => Some(Rc::clone(action_symbol_rcref)),
                                _ => None,
                            }
                        }
                        None => None,
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    // Operations are only declared in the -operations- block.
    // Get -operations-block- symtab from the system symbol and lookup.
    #[allow(clippy::many_single_char_names)] // TODO
    pub fn lookup_operation(&self, name: &str) -> Option<Rc<RefCell<OperationScopeSymbol>>> {
        let system_symbol_opt = &self.system_symbol_opt.as_ref();
        match system_symbol_opt {
            Some(system_symbol) => match &system_symbol.borrow().operations_block_symbol_opt {
                Some(operations_block_scope_symbol) => {
                    let operations_block_scope_symbol = operations_block_scope_symbol.borrow();
                    let symbol_table = &operations_block_scope_symbol.symtab_rcref.borrow();
                    match symbol_table.lookup(name, &IdentifierDeclScope::OperationsBlockScope) {
                        Some(symbol_table_rcref) => {
                            let symbol_t = symbol_table_rcref.borrow();
                            match &*symbol_t {
                                SymbolType::OperationScope {
                                    operation_scope_symbol_rcref: operation_symbol_rcref,
                                } => Some(Rc::clone(operation_symbol_rcref)),
                                _ => None,
                            }
                        }
                        None => None,
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    #[allow(clippy::many_single_char_names)] // TODO
    pub fn lookup_function(&self, name: &str) -> Option<Rc<RefCell<FunctionScopeSymbol>>> {
        let symbol_type_rcref_opt = self.global_symtab.borrow().lookup_local(name);

        match symbol_type_rcref_opt {
            Some(x) => {
                let y = x.borrow();
                match &*y {
                    SymbolType::FunctionScope {
                        function_symbol_ref,
                    } => Some(function_symbol_ref.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /* --------------------------------------------------------------------- */
    // LEGB Scoping Methods
    /* --------------------------------------------------------------------- */
    
    /// Set the current scope context (Function vs System)
    pub fn set_scope_context(&mut self, context: ScopeContext) {
        self.scope_context = context;
    }
    
    /// Get the current scope context
    pub fn get_scope_context(&self) -> &ScopeContext {
        &self.scope_context
    }

    /* --------------------------------------------------------------------- */
    
    // Debug Methods
    pub fn debug_dump_arcanum(&self) {
        if std::env::var("FRAME_DEBUG").is_ok() {
            eprintln!("\n=== ARCANUM DEBUG DUMP ===");
            self.debug_dump_scope_context();
            self.debug_dump_system_context();
            self.debug_dump_symbol_tables();
            self.debug_dump_entity_collections();
            eprintln!("=== END ARCANUM DUMP ===\n");
        }
    }
    
    fn debug_dump_scope_context(&self) {
        eprintln!("LEGB Scope Context: {:?}", self.scope_context);
    }
    
    fn debug_dump_system_context(&self) {
        match &self.system_symbol_opt {
            Some(system_symbol) => {
                let system_name = system_symbol.borrow().name.clone();
                eprintln!("Current System Context: Some({})", system_name);
            }
            None => eprintln!("Current System Context: None"),
        }
    }
    
    fn debug_dump_symbol_tables(&self) {
        eprintln!("--- Symbol Tables ---");
        eprintln!("Global SymTab: '{}'", self.global_symtab.borrow().name);
        eprintln!("Current SymTab: '{}'", self.current_symtab.borrow().name);
        eprintln!("Builtin SymTab: '{}' ({} symbols)", 
                  self.builtin_symtab.borrow().name,
                  self.builtin_symtab.borrow().symbols.len());
        
        // Dump current symbol table contents
        let current_symbols = &self.current_symtab.borrow().symbols;
        eprintln!("Current SymTab '{}' contains {} symbols:", 
                  self.current_symtab.borrow().name,
                  current_symbols.len());
        for (name, symbol_ref) in current_symbols {
            let symbol = symbol_ref.borrow();
            let symbol_type = match &*symbol {
                SymbolType::FunctionScope { .. } => "Function",
                SymbolType::ActionScope { .. } => "Action",
                SymbolType::OperationScope { .. } => "Operation",
                SymbolType::InterfaceMethod { .. } => "InterfaceMethod",
                SymbolType::System { .. } => "System",
                SymbolType::State { .. } => "State",
                SymbolType::DomainVariable { .. } => "DomainVariable",
                SymbolType::StateVariable { .. } => "StateVariable",
                SymbolType::EventHandlerVariable { .. } => "EventHandlerVariable",
                SymbolType::BlockVar { .. } => "BlockVariable",
                SymbolType::LoopVar { .. } => "LoopVariable",
                SymbolType::ParamSymbol { .. } => "Parameter",
                _ => "Other",
            };
            eprintln!("  '{}': {}", name, symbol_type);
        }
    }
    
    fn debug_dump_entity_collections(&self) {
        eprintln!("--- Entity Collections ---");
        eprintln!("Functions ({} total):", self.function_symbols.len());
        for (i, func_symbol) in self.function_symbols.iter().enumerate() {
            eprintln!("  [{}] '{}'", i, func_symbol.borrow().name);
        }
        
        eprintln!("Systems ({} total):", self.system_symbols.len());
        for (i, system_symbol) in self.system_symbols.iter().enumerate() {
            eprintln!("  [{}] '{}'", i, system_symbol.borrow().name);
        }
    }
    
    /// LEGB-aware symbol lookup that respects scope context
    /// L - Local (current function/system parameters, local vars)
    /// E - Enclosing (module-level functions, but NOT system internals unless in system context)
    /// G - Global (imported modules, global vars - not implemented yet)  
    /// B - Built-ins (print, etc.)
    pub fn legb_lookup_call(&self, name: &str) -> Option<LegbLookupResult> {
        // L - Local scope (handled by current_symtab in parser)
        // This is handled by the parser's local symbol table
        
        // E - Enclosing (module) scope  
        // Different behavior based on scope context
        match &self.scope_context {
            ScopeContext::Function(_) => {
                // In function context: only look for other functions, NOT system internals
                if let Some(function_symbol) = self.lookup_function(name) {
                    return Some(LegbLookupResult::Function(function_symbol));
                }
            }
            ScopeContext::System(_system_name) => {
                // In system context: look for actions/operations in current system first
                if let Some(action_symbol) = self.lookup_action(name) {
                    return Some(LegbLookupResult::Action(action_symbol));
                }
                if let Some(operation_symbol) = self.lookup_operation(name) {
                    return Some(LegbLookupResult::Operation(operation_symbol));
                }
                // Then look for functions
                if let Some(function_symbol) = self.lookup_function(name) {
                    return Some(LegbLookupResult::Function(function_symbol));
                }
            }
            ScopeContext::Global => {
                // At module level: look for functions only
                if let Some(function_symbol) = self.lookup_function(name) {
                    return Some(LegbLookupResult::Function(function_symbol));
                }
            }
        }
        
        // G - Global scope (not implemented yet)
        // TODO: Add support for imported modules and global variables
        
        // B - Built-ins  
        if let Some(builtin_symbol) = self.lookup_builtin(name) {
            return Some(LegbLookupResult::Builtin(builtin_symbol));
        }
        
        None
    }
    
    /// Lookup built-in functions
    pub fn lookup_builtin(&self, name: &str) -> Option<Rc<RefCell<FunctionScopeSymbol>>> {
        if let Some(symbol_rcref) = self.builtin_symtab.borrow().symbols.get(name) {
            let symbol = symbol_rcref.borrow();
            if let SymbolType::FunctionScope { function_symbol_ref } = &*symbol {
                return Some(Rc::clone(function_symbol_ref));
            }
        }
        None
    }

    /* --------------------------------------------------------------------- */
    // v0.30: Multi-entity accessor methods
    /* --------------------------------------------------------------------- */

    /// Get all functions in the module
    pub fn get_functions(&self) -> &Vec<Rc<RefCell<FunctionScopeSymbol>>> {
        &self.function_symbols
    }

    /// Get all systems in the module  
    pub fn get_systems(&self) -> &Vec<Rc<RefCell<SystemSymbol>>> {
        &self.system_symbols
    }

    /// Get function by name (v0.30 multi-entity support)
    pub fn get_function_by_name(&self, name: &str) -> Option<Rc<RefCell<FunctionScopeSymbol>>> {
        self.function_symbols.iter()
            .find(|func| func.borrow().name == name)
            .cloned()
    }

    /// Get system by name (v0.30 multi-entity support)  
    pub fn get_system_by_name(&self, name: &str) -> Option<Rc<RefCell<SystemSymbol>>> {
        self.system_symbols.iter()
            .find(|sys| sys.borrow().name == name)
            .cloned()
    }

    /// Check if module has multiple functions
    pub fn has_multiple_functions(&self) -> bool {
        self.function_symbols.len() > 1
    }

    /// Check if module has multiple systems
    pub fn has_multiple_systems(&self) -> bool {
        self.system_symbols.len() > 1
    }

    /// Get the primary system (first system for legacy compatibility)
    pub fn get_primary_system(&self) -> Option<Rc<RefCell<SystemSymbol>>> {
        self.system_symbols.first().cloned()
    }

    // v0.30: Multi-entity symbol lookup methods
    
    /// Search for an interface method across all systems in the module
    pub fn lookup_interface_method_in_all_systems(
        &self,
        name: &str,
    ) -> Option<(Rc<RefCell<InterfaceMethodSymbol>>, Rc<RefCell<SystemSymbol>>)> {
        for system_symbol_rcref in &self.system_symbols {
            match &system_symbol_rcref.borrow().interface_block_symbol_opt {
                Some(interface_block_symbol_rcref) => {
                    let interface_block_symbol = interface_block_symbol_rcref.borrow();
                    let symbol_table = &interface_block_symbol.symtab_rcref.borrow();
                    if let Some(symbol_t_rcref) = symbol_table.lookup(name, &IdentifierDeclScope::InterfaceBlockScope) {
                        let symbol_t = symbol_t_rcref.borrow();
                        if let SymbolType::InterfaceMethod { interface_method_symbol_rcref } = &*symbol_t {
                            return Some((Rc::clone(interface_method_symbol_rcref), Rc::clone(system_symbol_rcref)));
                        }
                    }
                }
                None => continue,
            }
        }
        None
    }

    /// Search for an action method across all systems in the module
    pub fn lookup_action_in_all_systems(
        &self,
        name: &str,
    ) -> Option<(Rc<RefCell<ActionScopeSymbol>>, Rc<RefCell<SystemSymbol>>)> {
        for system_symbol_rcref in &self.system_symbols {
            match &system_symbol_rcref.borrow().actions_block_symbol_opt {
                Some(actions_block_symbol_rcref) => {
                    let actions_block_symbol = actions_block_symbol_rcref.borrow();
                    let symbol_table = &actions_block_symbol.symtab_rcref.borrow();
                    if let Some(symbol_t_rcref) = symbol_table.lookup(name, &IdentifierDeclScope::ActionsBlockScope) {
                        let symbol_t = symbol_t_rcref.borrow();
                        if let SymbolType::ActionScope { action_scope_symbol_rcref } = &*symbol_t {
                            return Some((Rc::clone(action_scope_symbol_rcref), Rc::clone(system_symbol_rcref)));
                        }
                    }
                }
                None => continue,
            }
        }
        None
    }

    /// Search for an operation method across all systems in the module  
    pub fn lookup_operation_in_all_systems(
        &self,
        name: &str,
    ) -> Option<(Rc<RefCell<OperationScopeSymbol>>, Rc<RefCell<SystemSymbol>>)> {
        for system_symbol_rcref in &self.system_symbols {
            match &system_symbol_rcref.borrow().operations_block_symbol_opt {
                Some(operations_block_symbol_rcref) => {
                    let operations_block_symbol = operations_block_symbol_rcref.borrow();
                    let symbol_table = &operations_block_symbol.symtab_rcref.borrow();
                    if let Some(symbol_t_rcref) = symbol_table.lookup(name, &IdentifierDeclScope::OperationsBlockScope) {
                        let symbol_t = symbol_t_rcref.borrow();
                        if let SymbolType::OperationScope { operation_scope_symbol_rcref } = &*symbol_t {
                            return Some((Rc::clone(operation_scope_symbol_rcref), Rc::clone(system_symbol_rcref)));
                        }
                    }
                }
                None => continue,
            }
        }
        None
    }

    /// Search for a state across all systems in the module
    pub fn lookup_state_in_all_systems(
        &self,
        name: &str,
    ) -> Option<(Rc<RefCell<StateSymbol>>, Rc<RefCell<SystemSymbol>>)> {
        for system_symbol_rcref in &self.system_symbols {
            match &system_symbol_rcref.borrow().machine_block_symbol_opt {
                Some(machine_block_symbol_rcref) => {
                    let machine_block_symbol = machine_block_symbol_rcref.borrow();
                    let symbol_table = &machine_block_symbol.symtab_rcref.borrow();
                    if let Some(symbol_t_rcref) = symbol_table.lookup(name, &IdentifierDeclScope::UnknownScope) {
                        let symbol_t = symbol_t_rcref.borrow();
                        if let SymbolType::State { state_symbol_ref } = &*symbol_t {
                            return Some((Rc::clone(state_symbol_ref), Rc::clone(system_symbol_rcref)));
                        }
                    }
                }
                None => continue,
            }
        }
        None
    }

    /* --------------------------------------------------------------------- */

    pub fn enter_scope(&mut self, scope_t: ParseScopeType) {
        // do scope specific actions
        match &scope_t {
            ParseScopeType::Function {
                function_scope_symbol_rcref,
            } => {
                // LEGB: Set function scope context
                let function_name = function_scope_symbol_rcref.borrow().name.clone();
                self.scope_context = ScopeContext::Function(function_name.clone());
                
                // CRITICAL FIX: Clear system context when entering function scope
                // Functions should NOT have access to system internals (actions, operations)
                self.system_symbol_opt = None;
                
                // Debug dump after entering function scope
                if std::env::var("FRAME_DEBUG").is_ok() {
                    eprintln!(">>> ENTERED FUNCTION SCOPE: {}", function_name);
                    self.debug_dump_arcanum();
                }
                
                // set parent symbol table
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                function_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // v0.30: Add to multi-entity function collection
                self.function_symbols.push(Rc::clone(function_scope_symbol_rcref));

                // add new scope symbol to previous symbol table
                let function_scope_symbol_rcref_clone = Rc::clone(function_scope_symbol_rcref);
                // clone the Rc for the symbol table
                let function_symbol_symtab_rcref =
                    Rc::clone(&function_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // clone the Rc for the symbol table
                self.current_symtab = Rc::clone(&function_symbol_symtab_rcref);
            }
            ParseScopeType::System {
                system_symbol: system_symbol_rcref,
            } => {
                // LEGB: Set system scope context
                let system_name = system_symbol_rcref.borrow().name.clone();
                self.scope_context = ScopeContext::System(system_name.clone());
                
                // set parent symbol table
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                system_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // cache the system symbol (legacy compatibility)
                self.system_symbol_opt = Some(Rc::clone(system_symbol_rcref));
                
                // Debug dump after entering system scope (AFTER setting system_symbol_opt)
                if std::env::var("FRAME_DEBUG").is_ok() {
                    eprintln!(">>> ENTERED SYSTEM SCOPE: {}", system_name);
                    self.debug_dump_arcanum();
                }
                
                // v0.30: Add to multi-entity system collection
                self.system_symbols.push(Rc::clone(system_symbol_rcref));
                
                // add new scope symbol to previous symbol table
                let system_symbol_rcref_clone = Rc::clone(system_symbol_rcref);
                // clone the Rc for the symbol table
                let system_symbol_symtab_rcref =
                    Rc::clone(&system_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // clone the Rc for the symbol table
                self.current_symtab = Rc::clone(&system_symbol_symtab_rcref);
            }
            ParseScopeType::InterfaceBlock {
                interface_block_scope_symbol_rcref,
            } => {
                // Attach MachineSymbol to SystemSymbol
                // TODO - figure out why borrow can't go in the Some()
                {
                    let system_symbol_ref = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = system_symbol_ref.borrow_mut();
                    system_symbol.interface_block_symbol_opt =
                        Some(Rc::clone(interface_block_scope_symbol_rcref));
                }
                // current symtab should be the SystemSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                interface_block_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                let interface_scope_symbol_rcref_clone =
                    Rc::clone(interface_block_scope_symbol_rcref);
                let interface_scope_symbol_symtab_rcref =
                    Rc::clone(&interface_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // clone the Rc for the symbol table
                self.current_symtab = Rc::clone(&interface_scope_symbol_symtab_rcref);
            }
            ParseScopeType::MachineBlock {
                machine_scope_symbol_rcref: machine_symbol_rcref,
            } => {
                // Attach MachineSymbol to SystemSymbol
                // TODO - figure out why borrow can't go in the Some()
                {
                    let system_symbol_ref = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = system_symbol_ref.borrow_mut();
                    system_symbol.machine_block_symbol_opt = Some(Rc::clone(machine_symbol_rcref));
                }
                // current symtab should be the SystemSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                machine_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                let machine_symbol_rcref_clone = Rc::clone(machine_symbol_rcref);
                let machine_symbol_symtab_rcref =
                    Rc::clone(&machine_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // clone the Rc for the symbol table
                self.current_symtab = Rc::clone(&machine_symbol_symtab_rcref);
            }
            ParseScopeType::State {
                state_symbol: state_symbol_rcref,
            } => {
                // clone the Rc for the symbol table
                let state_symbol_rcref_clone = Rc::clone(state_symbol_rcref);
                let state_symbol_symtab_rcref =
                    Rc::clone(&state_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the MachineBlockSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                state_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new state's symbol table
                self.current_symtab = Rc::clone(&state_symbol_symtab_rcref);
            }
            ParseScopeType::StateParams {
                state_params_scope_symbol_rcref: state_params_rcref,
            } => {
                // clone the Rc for the symbol table
                let state_params_rcref_clone = Rc::clone(state_params_rcref);
                let state_params_symtab_rcref =
                    Rc::clone(&state_params_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the StateSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                state_params_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new state's symbol table
                self.current_symtab = Rc::clone(&state_params_symtab_rcref);
            }
            ParseScopeType::StateLocal {
                state_local_scope_symbol_rcref: state_block_rcref,
            } => {
                // clone the Rc for the symbol table
                let state_block_rcref_clone = Rc::clone(state_block_rcref);
                let state_block_symtab_rcref =
                    Rc::clone(&state_block_rcref_clone.borrow().symtab_rcref);

                // current symtab should be a StateSymbol or StateParamScope
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                state_block_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to state block symbol table
                self.current_symtab = Rc::clone(&state_block_symtab_rcref);
            }
            ParseScopeType::EventHandler {
                event_handler_scope_symbol_rcref,
            } => {
                // clone the Rc for the symbol table
                let event_handler_symbol_rcref_clone = Rc::clone(event_handler_scope_symbol_rcref);
                let event_handler_symbol_symtab_rcref =
                    Rc::clone(&event_handler_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the StateSymbol, StateParamsSymbol or StateBlockSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                event_handler_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&event_handler_symbol_symtab_rcref);
            }
            ParseScopeType::EventHandlerParams {
                event_handler_params_scope_symbol_rcref: event_handler_params_rcref,
            } => {
                // clone the Rc for the symbol table
                let event_handler_params_rcref_clone = Rc::clone(event_handler_params_rcref);
                let event_handler_params_symtab_rcref =
                    Rc::clone(&event_handler_params_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the EventHandlerSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                event_handler_params_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&event_handler_params_symtab_rcref);
            }
            ParseScopeType::EventHandlerLocal {
                event_handler_local_scope_symbol_rcref: event_handler_block_scope_symbol_rcref,
            } => {
                // clone the Rc for the symbol table
                let event_handler_block_scope_symbol_rcref_clone =
                    Rc::clone(event_handler_block_scope_symbol_rcref);
                let event_handler_block_scope_symbol_symtab_rcref = Rc::clone(
                    &event_handler_block_scope_symbol_rcref_clone
                        .borrow()
                        .symtab_rcref,
                );

                // current symtab should be the StateSymbol, StateParamsSymbol or StateBlockSymbol
                let current_symtab_rcref = Rc::clone(&self.current_symtab);
                event_handler_block_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&event_handler_block_scope_symbol_symtab_rcref);
            }
            ParseScopeType::ActionsBlock {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            } => {
                {
                    let system_symbol_ref = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = system_symbol_ref.borrow_mut();
                    system_symbol.actions_block_symbol_opt =
                        Some(Rc::clone(actions_block_scope_symbol));
                }

                // current symtab should be the SystemSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                actions_block_scope_symbol
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                let actions_block_scope_symbol_rcref_clone = Rc::clone(actions_block_scope_symbol);
                let actions_block_symbol_symtab_rcref =
                    Rc::clone(&actions_block_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                self.current_symtab = Rc::clone(&actions_block_symbol_symtab_rcref);
            }
            ParseScopeType::Action {
                action_scope_symbol_rcref,
            } => {
                let action_scope_symbol_rcref_clone = Rc::clone(action_scope_symbol_rcref);
                let action_scope_symbol_symtab_rcref =
                    Rc::clone(&action_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the ActionsBlockScopeSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                action_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // let action_scope_symbol_rcref_clone = Rc::clone(action_scope_symbol_rcref);
                //let action_symbol_symtab_rcref =
                //Rc::clone(&action_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                self.current_symtab = Rc::clone(&action_scope_symbol_symtab_rcref);
            }
            ParseScopeType::OperationsBlock {
                operations_block_scope_symbol_rcref,
            } => {
                {
                    let system_symbol_ref = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = system_symbol_ref.borrow_mut();
                    system_symbol.operations_block_symbol_opt =
                        Some(Rc::clone(operations_block_scope_symbol_rcref));
                }

                // current symtab should be the SystemSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                operations_block_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                let operations_block_scope_symbol_rcref_clone =
                    Rc::clone(operations_block_scope_symbol_rcref);
                let operations_block_symbol_symtab_rcref = Rc::clone(
                    &operations_block_scope_symbol_rcref_clone
                        .borrow()
                        .symtab_rcref,
                );

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                self.current_symtab = Rc::clone(&operations_block_symbol_symtab_rcref);
            }
            ParseScopeType::Operation {
                operation_scope_symbol_rcref,
            } => {
                let operation_scope_symbol_rcref_clone = Rc::clone(operation_scope_symbol_rcref);
                let operation_scope_symbol_symtab_rcref =
                    Rc::clone(&operation_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the ActionsBlockScopeSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                operation_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                self.current_symtab = Rc::clone(&operation_scope_symbol_symtab_rcref);
            }
            ParseScopeType::Loop {
                loop_scope_symbol_rcref,
            } => {
                let loop_scope_symbol_rcref_clone = Rc::clone(loop_scope_symbol_rcref);
                let loop_scope_symbol_symtab_rcref =
                    Rc::clone(&loop_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the ActionsBlockScopeSymbol
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                loop_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // let loop_scope_symbol_rcref_clone = Rc::clone(loop_scope_symbol_rcref);
                // let loop_symbol_symtab_rcref =
                //     Rc::clone(&loop_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                self.current_symtab = Rc::clone(&loop_scope_symbol_symtab_rcref);
            }
            ParseScopeType::DomainBlock {
                domain_block_scope_symbol_rcref,
            } => {
                // clone the Rc for the symbol table
                let domain_block_scope_symbol_rcref_clone =
                    Rc::clone(domain_block_scope_symbol_rcref);
                let domain_block_scope_symbol_symtab_rcref =
                    Rc::clone(&domain_block_scope_symbol_rcref_clone.borrow().symtab_rcref);

                // current symtab should be the StateSymbol, StateParamsSymbol or StateBlockSymbol
                let current_symtab_rcref = Rc::clone(&self.current_symtab);
                domain_block_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&domain_block_scope_symbol_symtab_rcref);
            }
            ParseScopeType::Block { block_scope_rcref } => {
                // clone the Rc for the symbol table
                let block_scope_rcref_rcref_clone = Rc::clone(block_scope_rcref);
                let block_scope_symtab_rcref =
                    Rc::clone(&block_scope_rcref_rcref_clone.borrow().symtab_rcref);

                let current_symtab_rcref = Rc::clone(&self.current_symtab);
                block_scope_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&block_scope_symtab_rcref);
            }
            ParseScopeType::Params {
                params_scope_symbol_rcref,
            } => {
                // clone the Rc for the symbol table
                let params_scope_symbol_rcref_clone = Rc::clone(params_scope_symbol_rcref);
                let params_scope_symtab_rcref =
                    Rc::clone(&params_scope_symbol_rcref_clone.borrow().symtab_rcref);

                let current_symtab_rcref = Rc::clone(&self.current_symtab);
                params_scope_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symtab_rcref);

                // add new scope symbol to previous symbol table
                self.current_symtab.borrow_mut().insert_parse_scope(scope_t);
                // update current symbol table to new event_handler's symbol table
                self.current_symtab = Rc::clone(&params_scope_symtab_rcref);
            }
        }

        Exe::debug_print(&format!(
            "Enter scope |{}|",
            self.current_symtab.borrow().name
        ));
        //    println!("Enter scope |{}|",self.current_symtab.borrow().name);
    }

    /* --------------------------------------------------------------------- */

    // This is used in the semantic pass to set the previously built scope from the symbol table.
    pub fn set_parse_scope(&mut self, scope_name: &str) {
        Exe::debug_print(&format!("Setting parse scope = |{}|.", scope_name));
        self.current_symtab = self.get_next_symbol_table(scope_name, &self.current_symtab);
    }

    /* --------------------------------------------------------------------- */

    pub fn exit_scope(&mut self) {
        let symbol_table_rcref = match self.current_symtab.borrow_mut().get_parent_symtab() {
            Some(symtab_ref) => symtab_ref,
            None => panic!("Fatal error - could not find parent symtab."),
        };

        Exe::debug_print(&format!(
            "Exit scope |{}|",
            self.current_symtab.borrow().name
        ));

        self.current_symtab = symbol_table_rcref;
        
        // LEGB: Reset to Global (module) scope context when exiting function/system scopes
        self.scope_context = ScopeContext::Global;
        
        // Debug dump after exiting scope
        if std::env::var("FRAME_DEBUG").is_ok() {
            eprintln!(">>> EXITED SCOPE - Back to Global");
            self.debug_dump_arcanum();
        }
        
        Exe::debug_print(&format!(
            "Returned to scope |{}|",
            self.current_symtab.borrow().name
        ));
    }

    #[allow(clippy::many_single_char_names)] // TODO
    fn get_next_symbol_table(
        &self,
        scope_name: &str,
        symtab_rcref: &Rc<RefCell<SymbolTable>>,
    ) -> Rc<RefCell<SymbolTable>> {
        let symbol_table = symtab_rcref.borrow();
        let symbols_map = &symbol_table.symbols;
        let symbol_t_rcref_opt = symbols_map.get(scope_name);
        match symbol_t_rcref_opt {
            Some(symbol_t_rcref) => {
                let symbol_t = symbol_t_rcref.borrow();
                let symbol_table_rcref = symbol_t.get_symbol_table();
                Rc::clone(&symbol_table_rcref)
            }
            None => {
                panic!("Fatal error - could not get next symbol table.")
            }
        }
    }

    /* --------------------------------------------------------------------- */

    // Get the system symbol, retrieve the machine block symbol and then find the state.

    pub fn get_state(&mut self, state_name: &str) -> Option<Rc<RefCell<StateSymbol>>> {
        match &self.system_symbol_opt {
            Some(system_symbol_rcref) => {
                let system_symbol_rcref2 = system_symbol_rcref.borrow();
                //      let state_name = system_symbol_ref2.name.clone();
                let states_symtab_rcref: Rc<RefCell<SymbolTable>>;
                match &system_symbol_rcref2.machine_block_symbol_opt {
                    Some(machine_symbol) => {
                        let m = machine_symbol.borrow();
                        states_symtab_rcref = Rc::clone(&m.symtab_rcref);
                    }
                    None => return None,
                }

                let symbol_table = states_symtab_rcref.borrow();
                let state_symbol_t =
                    symbol_table.lookup(state_name, &IdentifierDeclScope::UnknownScope);
                match state_symbol_t {
                    Some(symbol_t_ref) => {
                        let symbol_type = symbol_t_ref.borrow();

                        match &*symbol_type {
                            SymbolType::State { state_symbol_ref } => {
                                Some(Rc::clone(state_symbol_ref))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }

    /* --------------------------------------------------------------------- */

    // Get the system symbol, retrieve the machine block symbol and then find the state.

    pub fn has_state(&mut self, state_name: &str) -> bool {
        self.get_state(state_name).is_some()
    }

    /* --------------------------------------------------------------------- */

    pub fn declare_event(&mut self, event_symbol_rcref: Rc<RefCell<EventSymbol>>) {
        let msg = event_symbol_rcref.borrow().msg.clone();
        // if msg == self.symbol_config.save_msg_symbol || msg == self.symbol_config.restore_msg_symbol
        // {
        //     self.serializable = true;
        // }
        let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
        let mut system_symbol_rcref_mut = system_symbol_rcref.borrow_mut();
        system_symbol_rcref_mut
            .events
            .insert(msg, Rc::clone(&event_symbol_rcref));
    }

    /* --------------------------------------------------------------------- */

    // pub fn get_event_names(&self) -> Vec<String> {
    //     let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
    //     let system_symbol = system_symbol_rcref.borrow();
    //     let mut ret = Vec::new();
    //     for (k, _v) in system_symbol.events.iter() {
    //         ret.push(k.clone());
    //     }
    //     ret
    // }

    /// Get all action names from the action block.
    // pub fn get_action_names(&self) -> Vec<String> {
    //     let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
    //     let system_symbol = system_symbol_rcref.borrow();
    //     let mut result = Vec::new();
    //     if let Some(action_block_rcref) = &system_symbol.actions_block_symbol_opt {
    //         let action_block = action_block_rcref.borrow();
    //         let action_symbol_table = action_block.symtab_rcref.borrow();
    //         for action in action_symbol_table.symbols.keys() {
    //             result.push(action.clone());
    //         }
    //     }
    //     result
    // }

    /* --------------------------------------------------------------------- */

    // This method preferentially gets the interface name for a message if it
    // exists or returns the message name itself.

    // pub fn get_interface_or_msg_from_msg(&self, msg: &str) -> Option<String> {
    //     let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
    //     let system_symbol = system_symbol_rcref.borrow();
    //     match system_symbol.events.get(&msg.to_string()) {
    //         Some(event_symbol_rcref) => {
    //             let event_symbol = event_symbol_rcref.borrow();
    //             match &event_symbol.interface_name_opt {
    //                 Some(interface_name) => Some(interface_name.clone()),
    //                 None => Some(event_symbol.msg.clone()),
    //             }
    //         }
    //         None => None,
    //     }
    // }

    /* --------------------------------------------------------------------- */
    // This method preferentially gets the interface name for a message if it
    // exists or returns the message name itself.

    #[allow(dead_code)]
    pub fn get_msg_from_interface_name(&self, interface_name: &str) -> String {
        let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
        let system_symbol = system_symbol_rcref.borrow();
        for (_k, v) in system_symbol.events.iter() {
            let event_symbol = v.borrow();
            let event_symbol_interface_name_opt = &event_symbol.interface_name_opt;
            match event_symbol_interface_name_opt {
                Some(event_symbol_interface_name) => {
                    if interface_name.eq(event_symbol_interface_name) {
                        return event_symbol.msg.clone();
                    }
                }
                None => return interface_name.to_string(),
            }
        }

        // message didn't match any
        interface_name.to_string()
    }

    /* --------------------------------------------------------------------- */

    // This is for Rust which can't/won't deal with arbitrary message names.
    //
    // pub fn get_event_names_by_interface(&self) -> Vec<String> {
    //
    // }
    //
    /* --------------------------------------------------------------------- */

    pub fn get_event(
        &mut self,
        msg: &str,
        state_name_opt: &Option<String>,
    ) -> Option<Rc<RefCell<EventSymbol>>> {
        let cannonical_msg; // need to init as there is some weird bug that hangs the debugger
        if state_name_opt.is_some()
            && (self.symbol_config.enter_msg_symbol == msg
                || self.symbol_config.exit_msg_symbol == msg)
        {
            cannonical_msg = format!("{}:{}", state_name_opt.as_ref().unwrap(), msg);
        } else {
            cannonical_msg = msg.to_string();
        }
        let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
        let system_symbol = system_symbol_rcref.borrow_mut();
        system_symbol.events.get(&cannonical_msg).map(Rc::clone)
    }

    /* --------------------------------------------------------------------- */

    // pub fn is_serializable(&self) -> bool {
    //     self.serializable
    // }

    /* --------------------------------------------------------------------- */

    pub fn insert_symbol(&mut self, symbol_t: SymbolType) -> Result<(), String> {
        let symbol_table = self.get_symbol_table_for_type(&symbol_t);
        let result = symbol_table.borrow_mut().define(&symbol_t);
        result
    }

    /* --------------------------------------------------------------------- */

    // This method locates the proper symbol table for the system to insert the type into.
    // Typically this will be in the current symtab, but actions and domain objects
    // have other locations.
    // TODO: implement this for all symbol types!
    fn get_symbol_table_for_type(&self, symbol_t: &SymbolType) -> Rc<RefCell<SymbolTable>> {
        match symbol_t {
            SymbolType::StateParam {
                state_param_symbol_rcref: _state_param_symbol_rcref,
            } => Rc::clone(&self.current_symtab),
            SymbolType::EventHandlerParam {
                event_handler_param_symbol_rcref: _event_handler_param_symbol_rcref,
            } => Rc::clone(&self.current_symtab),
            SymbolType::ParamSymbol {
                param_symbol_rcref: _param_symbol_rcref,
            } => Rc::clone(&self.current_symtab),
            _ => panic!("TODO"),
        }
    }
}

pub struct SystemSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
    pub events: HashMap<String, Rc<RefCell<EventSymbol>>>,
    pub interface_block_symbol_opt: Option<Rc<RefCell<InterfaceBlockScopeSymbol>>>,
    pub machine_block_symbol_opt: Option<Rc<RefCell<MachineBlockScopeSymbol>>>,
    pub actions_block_symbol_opt: Option<Rc<RefCell<ActionsBlockScopeSymbol>>>,
    pub operations_block_symbol_opt: Option<Rc<RefCell<OperationsBlockScopeSymbol>>>,
    pub domain_block_symbol_opt: Option<Rc<RefCell<DomainBlockScopeSymbol>>>,
    pub symbol_config: SymbolConfig,
    // pub ast_node_opt: Option<Rc<RefCell<SystemNode>>>, // TODO??
    pub start_state_params_cnt: usize,
    pub state_enter_params_cnt: usize,
    pub domain_params_cnt: usize,
}

impl SystemSymbol {
    pub fn new(name: String) -> SystemSymbol {
        SystemSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::UnknownScope,
                true,
            ))),
            events: HashMap::new(),
            interface_block_symbol_opt: None,
            machine_block_symbol_opt: None,
            actions_block_symbol_opt: None,
            operations_block_symbol_opt: None,
            domain_block_symbol_opt: None,
            symbol_config: SymbolConfig::new(), // TODO
            // ast_node_opt: Option::None, // TODO
            start_state_params_cnt: 0,
            state_enter_params_cnt: 0,
            domain_params_cnt: 0,
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }

    pub fn get_interface_method(&self, name: &str) -> Option<Rc<RefCell<InterfaceMethodSymbol>>> {
        match &self.interface_block_symbol_opt {
            Some(interface_block_symbol_rcref) => {
                let interface_block_symbol = interface_block_symbol_rcref.borrow();
                let symbol_table = &interface_block_symbol.symtab_rcref.borrow();
                match symbol_table.lookup(name, &IdentifierDeclScope::InterfaceBlockScope) {
                    Some(c) => {
                        let symbol_t = c.borrow();
                        match &*symbol_t {
                            SymbolType::InterfaceMethod {
                                interface_method_symbol_rcref,
                            } => Some(Rc::clone(interface_method_symbol_rcref)),
                            _ => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
    // pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<SystemNode>>) {
    //     self.ast_node_opt = Some(Rc::clone(&ast_node));
    // }
}

impl Symbol for SystemSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for SystemSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in system scope.",
                symbol_name
            );
        }
    }
}

//-----------------------------------------------------//

const INTERFACE_SCOPE_NAME: &str = "-interface-block-";

pub struct InterfaceBlockScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl InterfaceBlockScopeSymbol {
    pub fn new() -> InterfaceBlockScopeSymbol {
        let name = InterfaceBlockScopeSymbol::scope_name();
        InterfaceBlockScopeSymbol {
            name: name.to_string(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::InterfaceBlockScope,
                false,
            ))),
        }
    }

    pub fn scope_name() -> &'static str {
        INTERFACE_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for InterfaceBlockScopeSymbol {
    fn default() -> Self {
        InterfaceBlockScopeSymbol::new()
    }
}

impl Symbol for InterfaceBlockScopeSymbol {
    fn get_name(&self) -> String {
        String::from("-interface-")
    }
}

impl ScopeSymbol for InterfaceBlockScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let symbol_table = self.symtab_rcref.borrow();
        let symbol_type_rcref_opt = symbol_table.symbols.get(symbol_name);
        if let Some(symbol_type_rcref) = symbol_type_rcref_opt {
            let symbol_type = symbol_type_rcref.borrow();
            let symbol_table_for_symbol = symbol_type.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&symbol_table_for_symbol)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in interface block scope.",
                symbol_name
            );
        }
    }
}

pub struct InterfaceMethodSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<InterfaceMethodNode>>>,
}

impl InterfaceMethodSymbol {
    pub fn new(name: String) -> InterfaceMethodSymbol {
        InterfaceMethodSymbol {
            name,
            ast_node_opt: None,
        }
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<InterfaceMethodNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

impl Symbol for InterfaceMethodSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// TODO: make iface a scope? currently the event object has all of the types
// impl ScopeSymbol for InterfaceMethodSymbol {
//
//     fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
//         Rc::clone(&self.symtab_rcref)
//     }
//
//     fn get_symbol_table_for_symbol(&self,symbol_name:&str) -> Rc<RefCell<SymbolTable>> {
//         let a = self.symtab_rcref.borrow();
//         let b = a.symbols.get(symbol_name);
//         if let Some(c) = b {
//             let d = c.borrow();
//             let e = d.get_symbol_table_for_symbol(symbol_name);
//             return Rc::clone(&e);
//         } else {
//             panic!("Fatal error - could not find symbol {} in interface method scope.", symbol_name);
//         }
//     }
// }

//-----------------------------------------------------//

pub struct EventSymbol {
    pub msg: String,
    pub interface_name_opt: Option<String>,
    pub event_symbol_params_opt: Option<Vec<ParameterSymbol>>,
    pub ret_type_opt: Option<TypeNode>,
    pub is_enter_msg: bool,
    pub is_exit_msg: bool,
}

impl EventSymbol {
    pub fn new(
        symbol_config: &SymbolConfig,
        msg: &str,
        interface_name_opt: Option<String>,
        params_opt: Option<Vec<ParameterSymbol>>,
        ret_type_opt: Option<TypeNode>,
        state_name_opt: Option<String>,
    ) -> EventSymbol {
        let (msg_name, is_enter_msg, is_exit_msg) =
            EventSymbol::get_event_msg(symbol_config, &state_name_opt, msg);

        EventSymbol {
            msg: msg_name,
            interface_name_opt,
            event_symbol_params_opt: params_opt,
            ret_type_opt,
            is_enter_msg,
            is_exit_msg,
        }
    }

    // pub fn requires_state_context(&self) -> bool {
    //     self.is_enter_msg && self.params_opt.is_some()
    // }

    pub fn get_event_msg(
        symbol_config: &SymbolConfig,
        state_name: &Option<String>,
        msg: &str,
    ) -> (String, bool, bool) {
        let mut msg_name: String;
        let mut is_enter_msg = false;
        let mut is_exit_msg = false;
        if symbol_config.enter_msg_symbol == msg {
            is_enter_msg = true;
            msg_name = state_name.as_ref().unwrap().clone();
            msg_name.push(':');
            msg_name.push_str(&symbol_config.enter_msg_symbol);
        } else if symbol_config.exit_msg_symbol == msg {
            is_exit_msg = true;
            msg_name = state_name.as_ref().unwrap().clone();
            msg_name.push(':');
            msg_name.push_str(&symbol_config.exit_msg_symbol);
        } else {
            msg_name = msg.to_string();
        }

        (msg_name, is_enter_msg, is_exit_msg)
    }

    pub fn get_param_count(&self) -> usize {
        if let Some(param_symbol_vec) = &self.event_symbol_params_opt {
            param_symbol_vec.len()
        } else {
            0
        }
    }
}

//-----------------------------------------------------//

const MACHINE_SCOPE_NAME: &str = "-machine-block-";

pub struct MachineBlockScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl MachineBlockScopeSymbol {
    pub fn new() -> MachineBlockScopeSymbol {
        let name = MachineBlockScopeSymbol::scope_name();
        MachineBlockScopeSymbol {
            name: name.to_string(),

            // TODO: Check if the IdentifierDeclScope should be set. It has been working but...
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn scope_name() -> &'static str {
        MACHINE_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for MachineBlockScopeSymbol {
    fn default() -> Self {
        MachineBlockScopeSymbol::new()
    }
}

impl Symbol for MachineBlockScopeSymbol {
    fn get_name(&self) -> String {
        String::from("-machine-")
    }
}

impl ScopeSymbol for MachineBlockScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in machine block scope.",
                symbol_name
            );
        }
    }
}

//-----------------------------------------------------//

pub struct StateSymbol {
    pub name: String,
    pub params_opt: Option<Vec<Rc<RefCell<ParameterSymbol>>>>,
    //   pub event_handlers_opt: Option<Vec<String>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
    pub state_node_opt: Option<Rc<RefCell<StateNode>>>,
    //    pub uses_enter_params:bool,
    requires_state_context: bool,
}

impl StateSymbol {
    pub fn new(state_name: &str, parent_symtab: Rc<RefCell<SymbolTable>>) -> StateSymbol {
        let st_rcref = SymbolTable::new(
            state_name.to_string(),
            Some(Rc::clone(&parent_symtab)),
            IdentifierDeclScope::UnknownScope,
            false,
        );
        StateSymbol {
            name: state_name.to_string(),
            params_opt: None,
            //   event_handlers_opt: None,
            symtab_rcref: Rc::new(RefCell::new(st_rcref)),
            state_node_opt: None,
            requires_state_context: false,
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }

    pub fn set_state_node(&mut self, state_node: Rc<RefCell<StateNode>>) {
        self.state_node_opt = Some(state_node);
    }

    // pub fn requires_state_context(&self) -> bool {
    //     self.requires_state_context
    // }

    pub fn add_parameter(
        &mut self,
        name: String,
        param_type: Option<TypeNode>,
        scope: IdentifierDeclScope,
    ) -> SymbolType {
        self.requires_state_context = true;

        let params;
        match &self.params_opt {
            Some(_) => {}
            None => {
                params = Vec::new();
                self.params_opt = Some(params);
            }
        }

        let param_symbol = ParameterSymbol::new(name, param_type, scope);
        let param_symbol_rcref = Rc::new(RefCell::new(param_symbol));
        self.params_opt
            .as_mut()
            .expect("Unable to add parameter")
            .push(Rc::clone(&param_symbol_rcref));

        // add to symbol table

        SymbolType::StateParam {
            state_param_symbol_rcref: Rc::clone(&param_symbol_rcref),
        }
        // this is wrong? as the param needs to be inserted in the
        // param symbol table maintained in arcanium.
        // TODO: does this symbol table need to be here???
        //        self.symtab_rcref.borrow_mut().insert_symbol(&state_param_symbol);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in state scope.",
                symbol_name
            );
        }
    }
}

// TODO - reconcile Parameters and Variable Node/Symbol differences.
#[derive(PartialEq)]
pub struct ParameterSymbol {
    pub name: String,
    pub param_type_opt: Option<TypeNode>,
    pub scope: IdentifierDeclScope,
}

impl ParameterSymbol {
    pub fn new(
        name: String,
        param_type: Option<TypeNode>,
        scope: IdentifierDeclScope,
    ) -> ParameterSymbol {
        ParameterSymbol {
            name,
            param_type_opt: param_type,
            scope,
        }
    }

    // pub fn set_ast_node(&mut self, ast_node_rcref: Rc<RefCell<ParameterNode>>) {
    //     self.ast_node_rcref = ast_node_rcref;
    // }
    //
    // pub fn get_ast_node(&mut self) -> Rc<RefCell<ParameterNode>> {
    //     self.ast_node_rcref.clone()
    // }

    pub fn is_eq(&self, other: &ParameterNode) -> bool {
        if self.name != other.param_name {
            return false;
        }
        match &self.param_type_opt {
            Some(param_type) => match &other.param_type_opt {
                Some(other_param_type) => {
                    param_type.get_type_str() == other_param_type.get_type_str()
                }
                None => false,
            },
            None => other.param_type_opt.is_none(),
        }
    }
}

impl Symbol for ParameterSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// -----------------------

// TODO: figure out how to namespace this so as to not have to suffix w/ Struct.

const STATE_PARAMETERS_SCOPE_NAME: &str = "-state-parameters-";

pub struct StateParamsScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl StateParamsScopeSymbol {
    pub fn new() -> StateParamsScopeSymbol {
        let name = String::from(STATE_PARAMETERS_SCOPE_NAME);
        StateParamsScopeSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::StateParamScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        STATE_PARAMETERS_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for StateParamsScopeSymbol {
    fn default() -> Self {
        StateParamsScopeSymbol::new()
    }
}

impl Symbol for StateParamsScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone() //String::from("domain")
    }
}

impl ScopeSymbol for StateParamsScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in state parameters scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

// TODO: figure out how to namespace this so as to not have to suffix w/ Struct.

const PARAMETERS_SCOPE_NAME: &str = "-parameters-";

pub struct ParamsScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl ParamsScopeSymbol {
    pub fn new() -> ParamsScopeSymbol {
        let name = String::from(PARAMETERS_SCOPE_NAME);
        ParamsScopeSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::StateParamScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        PARAMETERS_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for ParamsScopeSymbol {
    fn default() -> Self {
        ParamsScopeSymbol::new()
    }
}

impl Symbol for ParamsScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone() //String::from("domain")
    }
}

impl ScopeSymbol for ParamsScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in state parameters scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

// TODO: figure out how to namespace this so as to not have to suffix w/ Struct.

pub const STATE_LOCAL_SCOPE_NAME: &str = "-state-local-";

pub struct StateLocalScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl StateLocalScopeSymbol {
    pub fn new() -> StateLocalScopeSymbol {
        StateLocalScopeSymbol {
            name: STATE_LOCAL_SCOPE_NAME.to_string(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                STATE_LOCAL_SCOPE_NAME.to_string(),
                None,
                IdentifierDeclScope::StateVarScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        STATE_LOCAL_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }
}

impl Default for StateLocalScopeSymbol {
    fn default() -> Self {
        StateLocalScopeSymbol::new()
    }
}

impl Symbol for StateLocalScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for StateLocalScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in state local scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

pub struct EventHandlerScopeSymbol {
    pub name: String,
    pub event_symbol_rcref: Rc<RefCell<EventSymbol>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl EventHandlerScopeSymbol {
    pub fn new(
        name: &str,
        event_symbol_rcref: Rc<RefCell<EventSymbol>>,
    ) -> EventHandlerScopeSymbol {
        EventHandlerScopeSymbol {
            name: name.to_string(),
            event_symbol_rcref,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }
}

impl Symbol for EventHandlerScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for EventHandlerScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in event handler scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

// TODO: figure out how to namespace this so as to not have to suffix w/ Struct.

const EVENT_HANDLER_PARAMETERS_SCOPE_NAME: &str = "-event-handler-parameters-";

pub struct EventHandlerParamsScopeSymbol {
    pub name: String,
    //    pub declared_params_opt:Option<Vec<ParameterSymbol>>,
    pub event_symbol: Rc<RefCell<EventSymbol>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl EventHandlerParamsScopeSymbol {
    pub fn new(event_symbol: Rc<RefCell<EventSymbol>>) -> EventHandlerParamsScopeSymbol {
        let name = String::from(EVENT_HANDLER_PARAMETERS_SCOPE_NAME);
        EventHandlerParamsScopeSymbol {
            name: name.clone(),
            event_symbol,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::EventHandlerParamScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        EVENT_HANDLER_PARAMETERS_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }

    pub fn add_parameter(
        &mut self,
        name: String,
        param_type: Option<TypeNode>,
        scope: IdentifierDeclScope,
    ) -> SymbolType {
        // NOTE: the parameters here are the ones found in the decl of the event handler.
        // The event itself may have others, but they must be declared here in order to be
        // in the scope chain.

        let param_symbol = ParameterSymbol::new(name, param_type, scope);
        let param_symbol_rcref = Rc::new(RefCell::new(param_symbol));

        // add to symbol table

        SymbolType::EventHandlerParam {
            event_handler_param_symbol_rcref: Rc::clone(&param_symbol_rcref),
        }
    }
}

impl Symbol for EventHandlerParamsScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for EventHandlerParamsScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in event handler params scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

// TODO: figure out how to namespace this so as to not have to suffix w/ Struct.

pub const EVENT_HANDLER_LOCAL_SCOPE_NAME: &str = "-event-handler-local-";

pub struct EventHandlerLocalScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl EventHandlerLocalScopeSymbol {
    pub fn new() -> EventHandlerLocalScopeSymbol {
        EventHandlerLocalScopeSymbol {
            name: EVENT_HANDLER_LOCAL_SCOPE_NAME.to_string(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                EVENT_HANDLER_LOCAL_SCOPE_NAME.to_string(),
                None,
                IdentifierDeclScope::EventHandlerVarScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        EVENT_HANDLER_LOCAL_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }
}

impl Default for EventHandlerLocalScopeSymbol {
    fn default() -> Self {
        EventHandlerLocalScopeSymbol::new()
    }
}

// TODO is this used?
impl Symbol for EventHandlerLocalScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for EventHandlerLocalScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in event handler local scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

const ACTIONS_BLOCK_SCOPE_NAME: &str = "-actions-block-";

pub struct ActionsBlockScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl ActionsBlockScopeSymbol {
    pub fn new() -> ActionsBlockScopeSymbol {
        let name = String::from(ACTIONS_BLOCK_SCOPE_NAME);
        ActionsBlockScopeSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::ActionsBlockScope,
                false,
            ))),
        }
    }
    #[inline]
    pub fn scope_name() -> &'static str {
        ACTIONS_BLOCK_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for ActionsBlockScopeSymbol {
    fn default() -> Self {
        ActionsBlockScopeSymbol::new()
    }
}

// TODO: figure out how to do this (see machine to compare)
impl Symbol for ActionsBlockScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for ActionsBlockScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in actions block scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

const OPERATIONS_BLOCK_SCOPE_NAME: &str = "-operations-block-";

pub struct OperationsBlockScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl OperationsBlockScopeSymbol {
    pub fn new() -> OperationsBlockScopeSymbol {
        let name = String::from(OPERATIONS_BLOCK_SCOPE_NAME);
        OperationsBlockScopeSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::OperationsBlockScope,
                false,
            ))),
        }
    }
    #[inline]
    pub fn scope_name() -> &'static str {
        OPERATIONS_BLOCK_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for OperationsBlockScopeSymbol {
    fn default() -> Self {
        OperationsBlockScopeSymbol::new()
    }
}

// TODO: figure out how to do this (see machine to compare)
impl Symbol for OperationsBlockScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for OperationsBlockScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in actions block scope.",
                symbol_name
            );
        }
    }
}

// -----------------------

const DOMAIN_BLOCK_SCOPE_NAME: &str = "-domain-block-";

pub struct DomainBlockScopeSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl DomainBlockScopeSymbol {
    pub fn new() -> DomainBlockScopeSymbol {
        let name = String::from(DOMAIN_BLOCK_SCOPE_NAME);
        DomainBlockScopeSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name,
                None,
                IdentifierDeclScope::DomainBlockScope,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        DOMAIN_BLOCK_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(parent_symtab));
    }
}

impl Default for DomainBlockScopeSymbol {
    fn default() -> Self {
        DomainBlockScopeSymbol::new()
    }
}

// TODO: figure out how to do this (see machine to compare)
impl Symbol for DomainBlockScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone() //String::from("domain")
    }
}

impl ScopeSymbol for DomainBlockScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in domain block scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

pub struct ActionScopeSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<ActionNode>>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl ActionScopeSymbol {
    pub fn new(name: String) -> ActionScopeSymbol {
        ActionScopeSymbol {
            name: name.to_string(),
            ast_node_opt: None,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<ActionNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

// TODO - can Symbol and ScopeSymbol impls use a default implementation?
impl Symbol for ActionScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for ActionScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in action scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

pub struct OperationScopeSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<OperationNode>>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl OperationScopeSymbol {
    pub fn new(name: String) -> OperationScopeSymbol {
        OperationScopeSymbol {
            name: name.to_string(),
            ast_node_opt: None,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<OperationNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

// TODO - can Symbol and ScopeSymbol impls use a default implementation?
impl Symbol for OperationScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for OperationScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in action scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

const LOOP_SCOPE_NAME: &str = "loop";

pub struct LoopStmtScopeSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<LoopStmtNode>>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl LoopStmtScopeSymbol {
    pub fn new(name: &String) -> LoopStmtScopeSymbol {
        LoopStmtScopeSymbol {
            name: name.clone(),
            ast_node_opt: None,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                // TODO: Should this use the name passed in?
                String::from("loop"),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn scope_name() -> &'static str {
        LOOP_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<LoopStmtNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

// TODO - can Symbol and ScopeSymbol impls use a default implementation?
impl Symbol for LoopStmtScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for LoopStmtScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in Loop scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

pub struct FunctionScopeSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<FunctionNode>>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl FunctionScopeSymbol {
    pub fn new(name: String) -> FunctionScopeSymbol {
        FunctionScopeSymbol {
            name: name.to_string(),
            ast_node_opt: None,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.to_string(),
                None,
                IdentifierDeclScope::UnknownScope,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<FunctionNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

// TODO - can Symbol and ScopeSymbol impls use a default implementation?
impl Symbol for FunctionScopeSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for FunctionScopeSymbol {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in action scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

pub struct ActionCallSymbol {
    pub name: String,
    pub ast_node_opt: Option<Rc<RefCell<ActionCallExprNode>>>,
}

impl ActionCallSymbol {
    // pub fn new(name:String) -> ActionCallSymbol {
    //     ActionCallSymbol {
    //         name,
    //         ast_node:None,
    //     }
    // }

    // pub fn set_ast_node(&mut self, ast_node:Rc<RefCell<ActionCallExprNode>>) {
    //     self.ast_node = Some(Rc::clone(&ast_node));
    // }
}

impl Symbol for ActionCallSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// ----------------------- //

pub struct VariableSymbol {
    pub name: String,
    pub var_type: Option<TypeNode>,
    pub scope: IdentifierDeclScope,
    pub ast_node_rcref: Rc<RefCell<VariableDeclNode>>,
}

impl VariableSymbol {
    pub fn new(
        name: String,
        var_type: Option<TypeNode>,
        scope: IdentifierDeclScope,
        ast_node_rcref: Rc<RefCell<VariableDeclNode>>,
    ) -> VariableSymbol {
        VariableSymbol {
            name,
            var_type,
            scope,
            ast_node_rcref,
        }
    }

    pub fn set_ast_node(&mut self, ast_node_rcref: Rc<RefCell<VariableDeclNode>>) {
        self.ast_node_rcref = ast_node_rcref;
    }

    pub fn get_ast_node(&mut self) -> Rc<RefCell<VariableDeclNode>> {
        self.ast_node_rcref.clone()
    }
}

// impl VariableSymbol {
//     fn get_decl_initializer_expr() ->
// }

impl Symbol for VariableSymbol {
    fn get_name(&self) -> String {
        self.name.clone() //String::from("domain")
    }
}

// ----------------------- //

pub struct EnumSymbol {
    pub name: String,
    pub scope: IdentifierDeclScope,
    pub ast_node_opt: Option<Rc<RefCell<EnumDeclNode>>>,
}

impl EnumSymbol {
    pub fn new(name: String, scope: IdentifierDeclScope) -> EnumSymbol {
        EnumSymbol {
            name,
            scope,
            ast_node_opt: None,
        }
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<EnumDeclNode>>) {
        self.ast_node_opt = Some(Rc::clone(&ast_node));
    }
}

impl Symbol for EnumSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// -----------------------

pub struct BlockScope {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
}

impl BlockScope {
    pub fn new(name: &String) -> BlockScope {
        BlockScope {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.clone(),
                None,
                IdentifierDeclScope::BlockVarScope,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(parent_symtab));
    }
}

// TODO - can Symbol and ScopeSymbol impls use a default implementation?
impl Symbol for BlockScope {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ScopeSymbol for BlockScope {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        Rc::clone(&self.symtab_rcref)
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        let a = self.symtab_rcref.borrow();
        let b = a.symbols.get(symbol_name);
        if let Some(c) = b {
            let d = c.borrow();
            let e = d.get_symbol_table_for_symbol(symbol_name);
            Rc::clone(&e)
        } else {
            panic!(
                "Fatal error - could not find symbol {} in scope.",
                symbol_name
            );
        }
    }
}
