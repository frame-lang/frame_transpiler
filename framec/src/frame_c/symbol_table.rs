use super::ast::*;
use crate::compiler::Exe;
use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// TODO: init from file
pub struct SymbolConfig {
    pub start_msg_symbol: String,
    pub stop_msg_symbol: String,
    pub enter_msg_symbol: String,
    pub exit_msg_symbol: String,
    pub save_msg_symbol: String,
    pub restore_msg_symbol: String,
}

impl SymbolConfig {
    pub fn new() -> SymbolConfig {
        SymbolConfig {
            start_msg_symbol: String::from(">>"),
            stop_msg_symbol: String::from("<<"),
            enter_msg_symbol: String::from(">"),
            exit_msg_symbol: String::from("<"),
            save_msg_symbol: String::from(">>>"),
            restore_msg_symbol: String::from("<<<"),
        }
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
    SystemScope {
        system_symbol: Rc<RefCell<SystemSymbol>>,
    },
    InterfaceBlockScope {
        interface_block_scope_symbol_rcref: Rc<RefCell<InterfaceBlockScopeSymbol>>,
    },
    // TODO:
    // InterfaceMethodDeclScope,
    MachineBlockScope {
        machine_scope_symbol_rcref: Rc<RefCell<MachineBlockScopeSymbol>>,
    },
    ActionsBlockScope {
        actions_block_scope_symbol_rcref: Rc<RefCell<ActionsBlockScopeSymbol>>,
    },
    DomainBlockScope {
        domain_block_scope_symbol_rcref: Rc<RefCell<DomainBlockScopeSymbol>>,
    },
    StateScope {
        state_symbol: Rc<RefCell<StateSymbol>>,
    },
    StateParamsScope {
        state_params_scope_symbol_rcref: Rc<RefCell<StateParamsScopeSymbol>>,
    },
    StateLocalScope {
        state_local_scope_symbol_rcref: Rc<RefCell<StateLocalScopeSymbol>>,
    },
    EventHandlerScope {
        event_handler_scope_symbol_rcref: Rc<RefCell<EventHandlerScopeSymbol>>,
    },
    EventHandlerParamsScope {
        event_handler_params_scope_symbol_rcref: Rc<RefCell<EventHandlerParamsScopeSymbol>>,
    },
    EventHandlerLocalScope {
        event_handler_local_scope_symbol_rcref: Rc<RefCell<EventHandlerLocalScopeSymbol>>,
    },
}

// This is what gets stored in the symbol tables
pub enum SymbolType {
    SystemSymbolT {
        system_symbol_ref: Rc<RefCell<SystemSymbol>>,
    },
    #[allow(dead_code)] // not dead. weird
    InterfaceBlockSymbolT {
        interface_block_symbol_rcref: Rc<RefCell<InterfaceBlockScopeSymbol>>,
    },
    // TODO: Add InterfaceMethod
    InterfaceMethodSymbolT {
        interface_method_symbol_rcref: Rc<RefCell<InterfaceMethodSymbol>>,
    },
    MachineBlockScopeSymbolT {
        machine_block_symbol_rcref: Rc<RefCell<MachineBlockScopeSymbol>>,
    },
    ActionsBlockScopeSymbolT {
        actions_block_symbol_rcref: Rc<RefCell<ActionsBlockScopeSymbol>>,
    },
    ActionDeclSymbolT {
        action_decl_symbol_rcref: Rc<RefCell<ActionDeclSymbol>>,
    },
    DomainBlockScopeSymbolT {
        domain_block_symbol_rcref: Rc<RefCell<DomainBlockScopeSymbol>>,
    },
    StateSymbolT {
        state_symbol_ref: Rc<RefCell<StateSymbol>>,
    },
    StateParamsScopeSymbolT {
        state_params_scope_rcref: Rc<RefCell<StateParamsScopeSymbol>>,
    },
    StateLocalScopeSymbolT {
        state_local_scope_struct_rcref: Rc<RefCell<StateLocalScopeSymbol>>,
    },
    EventHandlerScopeSymbolT {
        event_handler_scope_symbol: Rc<RefCell<EventHandlerScopeSymbol>>,
    },
    EventHandlerParamsScopeSymbolT {
        event_handler_params_scope_symbol_rcref: Rc<RefCell<EventHandlerParamsScopeSymbol>>,
    },
    EventHandlerLocalScopeSymbolT {
        event_handler_local_scope_rcref: Rc<RefCell<EventHandlerLocalScopeSymbol>>,
    },

    // Variable Symbol types
    DomainVariableSymbolT {
        domain_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    StateParamSymbolT {
        state_param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
    },
    StateVariableSymbolT {
        state_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
    EventHandlerParamSymbolT {
        event_handler_param_symbol_rcref: Rc<RefCell<ParameterSymbol>>,
    },
    EventHandlerVariableSymbolT {
        event_handler_variable_symbol_rcref: Rc<RefCell<VariableSymbol>>,
    },
}

impl Symbol for SymbolType {
    fn get_name(&self) -> String {
        match self {
            SymbolType::SystemSymbolT { system_symbol_ref } => {
                system_symbol_ref.borrow().get_name()
            }
            SymbolType::InterfaceBlockSymbolT {
                interface_block_symbol_rcref,
            } => interface_block_symbol_rcref.borrow().get_name(),
            SymbolType::InterfaceMethodSymbolT {
                interface_method_symbol_rcref,
            } => interface_method_symbol_rcref.borrow().get_name(),
            SymbolType::MachineBlockScopeSymbolT {
                machine_block_symbol_rcref,
            } => machine_block_symbol_rcref.borrow().get_name(),
            SymbolType::ActionsBlockScopeSymbolT {
                actions_block_symbol_rcref,
            } => actions_block_symbol_rcref.borrow().get_name(),
            SymbolType::DomainBlockScopeSymbolT {
                domain_block_symbol_rcref,
            } => domain_block_symbol_rcref.borrow().get_name(),
            SymbolType::StateSymbolT { state_symbol_ref } => state_symbol_ref.borrow().get_name(),
            SymbolType::StateParamsScopeSymbolT {
                state_params_scope_rcref,
            } => state_params_scope_rcref.borrow().get_name(),
            SymbolType::StateLocalScopeSymbolT {
                state_local_scope_struct_rcref: state_block_scope_struct_rcref,
            } => state_block_scope_struct_rcref.borrow().get_name(),
            SymbolType::DomainVariableSymbolT {
                domain_variable_symbol_rcref,
            } => domain_variable_symbol_rcref.borrow().get_name(),
            SymbolType::StateVariableSymbolT {
                state_variable_symbol_rcref,
            } => state_variable_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerScopeSymbolT {
                event_handler_scope_symbol,
            } => event_handler_scope_symbol.borrow().get_name(),
            SymbolType::EventHandlerParamsScopeSymbolT {
                event_handler_params_scope_symbol_rcref,
            } => event_handler_params_scope_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerParamSymbolT {
                event_handler_param_symbol_rcref,
            } => event_handler_param_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerVariableSymbolT {
                event_handler_variable_symbol_rcref,
            } => event_handler_variable_symbol_rcref.borrow().get_name(),
            SymbolType::ActionDeclSymbolT {
                action_decl_symbol_rcref,
            } => action_decl_symbol_rcref.borrow().get_name(),
            SymbolType::StateParamSymbolT {
                state_param_symbol_rcref,
            } => state_param_symbol_rcref.borrow().get_name(),
            SymbolType::EventHandlerLocalScopeSymbolT {
                event_handler_local_scope_rcref,
            } => event_handler_local_scope_rcref.borrow().get_name(),
        }
    }
}

impl ScopeSymbol for SymbolType {
    fn get_symbol_table(&self) -> Rc<RefCell<SymbolTable>> {
        match self {
            SymbolType::SystemSymbolT { system_symbol_ref } => {
                system_symbol_ref.borrow().get_symbol_table()
            }
            SymbolType::InterfaceBlockSymbolT {
                interface_block_symbol_rcref,
            } => interface_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::MachineBlockScopeSymbolT {
                machine_block_symbol_rcref,
            } => machine_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::ActionsBlockScopeSymbolT {
                actions_block_symbol_rcref,
            } => actions_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::ActionDeclSymbolT { .. } => {
                panic!("Fatal error - action decl symbol does not have a symbol table.")
            }
            // action_decl_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::DomainBlockScopeSymbolT {
                domain_block_symbol_rcref,
            } => domain_block_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::DomainVariableSymbolT { .. } => {
                panic!("Fatal error - domain variable symbol does not have a symbol table.")
            }
            //                => domain_variable_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::StateSymbolT { state_symbol_ref } => {
                state_symbol_ref.borrow().get_symbol_table()
            }
            SymbolType::StateParamsScopeSymbolT {
                state_params_scope_rcref,
            } => state_params_scope_rcref.borrow().get_symbol_table(),
            SymbolType::StateParamSymbolT { .. } => {
                panic!("Fatal error - state param symbol does not have a symbol table.")
            }
            //state_param_symbol_rcref.borrow().get_symbol_table(),
            SymbolType::StateLocalScopeSymbolT {
                state_local_scope_struct_rcref,
            } => state_local_scope_struct_rcref.borrow().get_symbol_table(),
            SymbolType::EventHandlerScopeSymbolT {
                event_handler_scope_symbol,
            } => event_handler_scope_symbol.borrow().get_symbol_table(),
            SymbolType::EventHandlerParamsScopeSymbolT {
                event_handler_params_scope_symbol_rcref,
            } => event_handler_params_scope_symbol_rcref
                .borrow()
                .get_symbol_table(),
            SymbolType::EventHandlerLocalScopeSymbolT {
                event_handler_local_scope_rcref,
            } => event_handler_local_scope_rcref.borrow().get_symbol_table(),
            _ => panic!("TODO"),
        }
    }

    fn get_symbol_table_for_symbol(&self, symbol_name: &str) -> Rc<RefCell<SymbolTable>> {
        match self {
            SymbolType::SystemSymbolT { system_symbol_ref } => system_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::MachineBlockScopeSymbolT {
                machine_block_symbol_rcref: machine_symbol_ref,
            } => machine_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::DomainBlockScopeSymbolT {
                domain_block_symbol_rcref: domain_symbol_ref,
            } => domain_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::StateSymbolT { state_symbol_ref } => state_symbol_ref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::StateParamsScopeSymbolT {
                state_params_scope_rcref,
            } => state_params_scope_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::StateLocalScopeSymbolT {
                state_local_scope_struct_rcref: state_block_scope_struct_rcref,
            } => state_block_scope_struct_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::EventHandlerParamsScopeSymbolT {
                event_handler_params_scope_symbol_rcref: event_handler_params_symbol_rcref,
            } => event_handler_params_symbol_rcref
                .borrow()
                .get_symbol_table_for_symbol(symbol_name),
            SymbolType::EventHandlerLocalScopeSymbolT {
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
            ParseScopeType::SystemScope { system_symbol } => {
                let name = system_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::SystemSymbolT {
                    system_symbol_ref: system_symbol,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::InterfaceBlockScope {
                interface_block_scope_symbol_rcref,
            } => {
                let name = interface_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::InterfaceBlockSymbolT {
                    interface_block_symbol_rcref: interface_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::MachineBlockScope {
                machine_scope_symbol_rcref: machine_symbol,
            } => {
                let name = machine_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::MachineBlockScopeSymbolT {
                    machine_block_symbol_rcref: machine_symbol,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::StateScope { state_symbol } => {
                let name = state_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateSymbolT {
                    state_symbol_ref: state_symbol,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::StateParamsScope {
                state_params_scope_symbol_rcref: state_params_scope,
            } => {
                let name = state_params_scope.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateParamsScopeSymbolT {
                    state_params_scope_rcref: state_params_scope,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::StateLocalScope {
                state_local_scope_symbol_rcref: state_block_scope_symbol_rcref,
            } => {
                let name = state_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateLocalScopeSymbolT {
                    state_local_scope_struct_rcref: state_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::EventHandlerScope {
                event_handler_scope_symbol_rcref,
            } => {
                let name = event_handler_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerScopeSymbolT {
                    event_handler_scope_symbol: event_handler_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::EventHandlerParamsScope {
                event_handler_params_scope_symbol_rcref,
            } => {
                let name = event_handler_params_scope_symbol_rcref
                    .borrow()
                    .name
                    .clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerParamsScopeSymbolT {
                    event_handler_params_scope_symbol_rcref:
                        event_handler_params_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::EventHandlerLocalScope {
                event_handler_local_scope_symbol_rcref: event_handler_block_scope_symbol_rcref,
            } => {
                let name = event_handler_block_scope_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerLocalScopeSymbolT {
                    event_handler_local_scope_rcref: event_handler_block_scope_symbol_rcref,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::ActionsBlockScope {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            } => {
                let name = actions_block_scope_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::ActionsBlockScopeSymbolT {
                    actions_block_symbol_rcref: actions_block_scope_symbol,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            ParseScopeType::DomainBlockScope {
                domain_block_scope_symbol_rcref: domain_symbol,
            } => {
                let name = domain_symbol.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::DomainBlockScopeSymbolT {
                    domain_block_symbol_rcref: domain_symbol,
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
        }
    }

    pub fn insert_symbol(&mut self, symbol_t: &SymbolType) {
        match symbol_t {
            SymbolType::DomainVariableSymbolT {
                domain_variable_symbol_rcref,
            } => {
                let name = domain_variable_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::DomainVariableSymbolT {
                    domain_variable_symbol_rcref: Rc::clone(domain_variable_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::StateParamSymbolT {
                state_param_symbol_rcref,
            } => {
                let name = state_param_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateParamSymbolT {
                    state_param_symbol_rcref: Rc::clone(state_param_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::StateLocalScopeSymbolT {
                state_local_scope_struct_rcref,
            } => {
                let name = state_local_scope_struct_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateLocalScopeSymbolT {
                    state_local_scope_struct_rcref: Rc::clone(state_local_scope_struct_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::StateVariableSymbolT {
                state_variable_symbol_rcref,
            } => {
                let name = state_variable_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::StateVariableSymbolT {
                    state_variable_symbol_rcref: Rc::clone(state_variable_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::EventHandlerParamSymbolT {
                event_handler_param_symbol_rcref,
            } => {
                let name = event_handler_param_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerParamSymbolT {
                    event_handler_param_symbol_rcref: Rc::clone(event_handler_param_symbol_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::EventHandlerLocalScopeSymbolT {
                event_handler_local_scope_rcref,
            } => {
                let name = event_handler_local_scope_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerLocalScopeSymbolT {
                    event_handler_local_scope_rcref: Rc::clone(event_handler_local_scope_rcref),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            SymbolType::EventHandlerVariableSymbolT {
                event_handler_variable_symbol_rcref,
            } => {
                let name = event_handler_variable_symbol_rcref.borrow().name.clone();
                let st_ref = Rc::new(RefCell::new(SymbolType::EventHandlerVariableSymbolT {
                    event_handler_variable_symbol_rcref: Rc::clone(
                        event_handler_variable_symbol_rcref,
                    ),
                }));
                self.symbols.insert(name, st_ref);
                ()
            }
            // TODO: Currently actions are just declared.
            // When actions have bodies then this should become a scope symbol.
            SymbolType::ActionDeclSymbolT {
                action_decl_symbol_rcref,
            } => {
                let name = action_decl_symbol_rcref.borrow().name.clone();
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::ActionDeclSymbolT {
                    action_decl_symbol_rcref: Rc::clone(action_decl_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                ()
            }
            SymbolType::InterfaceMethodSymbolT {
                interface_method_symbol_rcref,
            } => {
                let name = interface_method_symbol_rcref.borrow().name.clone();
                let symbol_type_rcref = Rc::new(RefCell::new(SymbolType::InterfaceMethodSymbolT {
                    interface_method_symbol_rcref: Rc::clone(interface_method_symbol_rcref),
                }));
                self.symbols.insert(name, symbol_type_rcref);
                ()
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
            let x = match domain_block_scope_symtype {
                Some(symbol_type) => symbol_type,
                None => return None,
            };
            let y = x.borrow();
            match &*y {
                SymbolType::DomainBlockScopeSymbolT {
                    domain_block_symbol_rcref,
                } => {
                    let domain_block_scope_symbol = domain_block_symbol_rcref.borrow();
                    let symbol_table = domain_block_scope_symbol.symtab_rcref.borrow();
                    match symbol_table.lookup_local(name) {
                        Some(a) => return Some(a.clone()),
                        None => return None,
                    }
                }
                _ => return None,
            }
        }

        if *search_scope == IdentifierDeclScope::None || *search_scope == self.identifier_decl_scope
        {
            let a = (self.symbols).get(name);
            match a {
                Some(aa) => return Some(Rc::clone(aa)),
                None => {}
            }
        }

        match &self.parent_symtab_rcref_opt {
            Some(b) => {
                let c = b.borrow();
                let d = c.lookup(name, search_scope);
                return match d {
                    Some(e) => Some(Rc::clone(&e)),
                    None => None,
                };
            }
            None => None,
        }
    }

    pub fn lookup_local(&self, name: &str) -> Option<Rc<RefCell<SymbolType>>> {
        let a = (self.symbols).get(name);
        match a {
            Some(aa) => return Some(aa.clone()),
            None => None,
        }
    }

    pub fn get_parent_symtab(&self) -> Option<Rc<RefCell<SymbolTable>>> {
        //     let x = self.parent_symtab_opt_ref;
        let x = self.parent_symtab_rcref_opt.as_ref()?;
        let y = Rc::clone(&x);
        Some(y)
    }
}

// TODO
impl fmt::Display for SymbolTable {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymbolTable {}", self.name)
    }
}

pub struct Arcanum {
    pub root_symtab: Rc<RefCell<SymbolTable>>,
    pub current_symtab: Rc<RefCell<SymbolTable>>,
    pub system_symbol_opt: Option<Rc<RefCell<SystemSymbol>>>,
    pub symbol_config: SymbolConfig,
    pub serializable: bool,
}

impl Arcanum {
    pub fn new() -> Arcanum {
        let st = SymbolTable::new(
            String::from("global"),
            None,
            IdentifierDeclScope::None,
            false,
        );
        let root_symbtab_rc = Rc::new(RefCell::new(st));

        Arcanum {
            current_symtab: Rc::clone(&root_symbtab_rc),
            root_symtab: root_symbtab_rc,
            system_symbol_opt: None,
            symbol_config: SymbolConfig::new(), // TODO
            serializable: false,
        }
    }

    pub fn debug_print_current_symbols(&self, symbol_table_rcref: Rc<RefCell<SymbolTable>>) {
        Exe::debug_print(&format!("<------------------->"));
        self.do_debug_print_current_symbols(symbol_table_rcref);
        Exe::debug_print(&format!("<------------------->"));
    }

    fn do_debug_print_current_symbols(&self, symbol_table_rcref: Rc<RefCell<SymbolTable>>) {
        let symbol_table = symbol_table_rcref.borrow();

        Exe::debug_print(&format!("---------------------"));
        Exe::debug_print(&format!("SymbolTable {}", symbol_table.name));
        for (key, _value) in &symbol_table.symbols {
            Exe::debug_print(&format!("{}", key));
        }

        match &symbol_table.parent_symtab_rcref_opt {
            Some(parent_symbol_table_rcref) => {
                self.do_debug_print_current_symbols(parent_symbol_table_rcref.clone());
            }
            None => return,
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

    // Actions are only declared in the -actions- block.
    // Get -actions-block- symtab from the system symbol and lookup.
    pub fn lookup_interface_method(
        &self,
        name: &str,
    ) -> Option<Rc<RefCell<InterfaceMethodSymbol>>> {
        let system_symbol_rcref = &self.system_symbol_opt.as_ref().unwrap();
        match &system_symbol_rcref.borrow().interface_block_symbol_opt {
            Some(interface_block_symbol_rcref) => {
                let interface_block_symbol = interface_block_symbol_rcref.borrow();
                let symbol_table = &interface_block_symbol.symtab_rcref.borrow();
                self.debug_print_current_symbols(interface_block_symbol.symtab_rcref.clone());
                match symbol_table.lookup(name, &IdentifierDeclScope::InterfaceBlock) {
                    Some(c) => {
                        let d = c.borrow();
                        return match &*d {
                            SymbolType::InterfaceMethodSymbolT {
                                interface_method_symbol_rcref,
                            } => Some(Rc::clone(&interface_method_symbol_rcref)),
                            _ => None,
                        };
                    }
                    None => None,
                }
            }
            None => return None,
        }
    }

    // Actions are only declared in the -actions- block.
    // Get -actions-block- symtab from the system symbol and lookup.
    pub fn lookup_action(&self, name: &str) -> Option<Rc<RefCell<ActionDeclSymbol>>> {
        let a = &self.system_symbol_opt.as_ref().unwrap();
        match &a.borrow().actions_block_symbol_opt {
            Some(actions_block_scope_symbol) => {
                let b = actions_block_scope_symbol.borrow();
                let x = &b.symtab_rcref.borrow();
                match x.lookup(name, &IdentifierDeclScope::ActionsBlock) {
                    Some(c) => {
                        let d = c.borrow();
                        return match &*d {
                            SymbolType::ActionDeclSymbolT {
                                action_decl_symbol_rcref: action_symbol_rcref,
                            } => Some(Rc::clone(&action_symbol_rcref)),
                            _ => None,
                        };
                    }
                    None => None,
                }
            }
            None => return None,
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn enter_scope(&mut self, scope_t: ParseScopeType) {
        // do scope specific actions
        match &scope_t {
            ParseScopeType::SystemScope {
                system_symbol: system_symbol_rcref,
            } => {
                // set parent symbol table
                let current_symbtab_rcref = Rc::clone(&self.current_symtab);
                system_symbol_rcref
                    .borrow_mut()
                    .set_parent_symtab(&current_symbtab_rcref);

                // cache the system symbol
                self.system_symbol_opt = Some(Rc::clone(system_symbol_rcref));
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
            ParseScopeType::InterfaceBlockScope {
                interface_block_scope_symbol_rcref,
            } => {
                // Attach MachineSymbol to SystemSymbol
                // TODO - figure out why borrow can't go in the Some()
                {
                    let x = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = x.borrow_mut();
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
            ParseScopeType::MachineBlockScope {
                machine_scope_symbol_rcref: machine_symbol_rcref,
            } => {
                // Attach MachineSymbol to SystemSymbol
                // TODO - figure out why borrow can't go in the Some()
                {
                    let x = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = x.borrow_mut();
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
            ParseScopeType::StateScope {
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
            ParseScopeType::StateParamsScope {
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
            ParseScopeType::StateLocalScope {
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
            ParseScopeType::EventHandlerScope {
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
            ParseScopeType::EventHandlerParamsScope {
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
            ParseScopeType::EventHandlerLocalScope {
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
            ParseScopeType::ActionsBlockScope {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            } => {
                {
                    let x = self.system_symbol_opt.as_ref().unwrap().as_ref();
                    let mut system_symbol = x.borrow_mut();
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
            ParseScopeType::DomainBlockScope {
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

    pub fn exit_parse_scope(&mut self) {
        let x = match self.current_symtab.borrow_mut().get_parent_symtab() {
            Some(symtab_ref) => symtab_ref,
            None => panic!("Fatal error - could not find parent symtab."),
        };

        Exe::debug_print(&format!(
            "Exit scope |{}|",
            self.current_symtab.borrow().name
        ));

        self.current_symtab = x;
        Exe::debug_print(&format!(
            "Returned to scope |{}|",
            self.current_symtab.borrow().name
        ));
    }

    fn get_next_symbol_table(
        &self,
        scope_name: &str,
        symtab_rcref: &Rc<RefCell<SymbolTable>>,
    ) -> Rc<RefCell<SymbolTable>> {
        let b = symtab_rcref.borrow();
        let c = &b.symbols;
        let d = c.get(scope_name);
        match d {
            Some(e) => {
                let f = e.borrow();
                let g = f.get_symbol_table();
                return Rc::clone(&g);
            }
            None => panic!("Fatal error - could not get next symbol table."),
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

                let x = states_symtab_rcref.borrow();
                let state_symbol_t = x.lookup(state_name, &IdentifierDeclScope::None);
                match state_symbol_t {
                    Some(symbol_t_ref) => {
                        let x = symbol_t_ref.borrow();

                        match &*x {
                            SymbolType::StateSymbolT { state_symbol_ref } => {
                                return Some(Rc::clone(state_symbol_ref))
                            }
                            _ => None,
                        }
                    }
                    _ => return None,
                }
            }
            None => return None,
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn declare_event(&mut self, event_symbol_rcref: Rc<RefCell<EventSymbol>>) {
        let msg = event_symbol_rcref.borrow().msg.clone();
        if msg == self.symbol_config.save_msg_symbol || msg == self.symbol_config.restore_msg_symbol
        {
            self.serializable = true;
        }
        let a = self.system_symbol_opt.as_ref().unwrap();
        let mut b = a.borrow_mut();
        b.events.insert(msg, Rc::clone(&event_symbol_rcref));
    }

    /* --------------------------------------------------------------------- */

    pub fn get_event_names(&self) -> Vec<String> {
        let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
        let system_symbol = system_symbol_rcref.borrow();
        //        let ret = system_symbol.events.iter().map(f).collect();
        let mut ret = Vec::new();
        for (k, _v) in system_symbol.events.iter() {
            ret.push(k.clone());
        }
        ret
    }

    /* --------------------------------------------------------------------- */

    // This method preferentially gets the interface name for a message if it
    // exists or returns the message name itself.

    pub fn get_interface_or_msg_from_msg(&self, msg: &str) -> Option<String> {
        let system_symbol_rcref = self.system_symbol_opt.as_ref().unwrap();
        let system_symbol = system_symbol_rcref.borrow();
        match system_symbol.events.get(&msg.to_string()) {
            Some(event_symbol_rcref) => {
                let event_symbol = event_symbol_rcref.borrow();
                match &event_symbol.interface_name_opt {
                    Some(interface_name) => Some(interface_name.clone()),
                    None => Some(event_symbol.msg.clone()),
                }
            }
            None => None,
        }
    }

    /* --------------------------------------------------------------------- */

    // This method preferentially gets the interface name for a message if it
    // exists or returns the message name itself.

    pub fn get_msg_from_interface_name(&self, interface_name: &String) -> String {
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
        match system_symbol.events.get(&cannonical_msg) {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }

    // pub fn get_event_ret_opt(&mut self, msg:&str, state_name_opt:&Option<String>) -> Option<TypeNode> {
    //     let a = self.get_event(msg,state_name_opt);
    //     let b = match a {
    //         Some(c) => c,
    //         None => return None,
    //     };
    //     let d = b.borrow();
    //     d.ret_type_opt.clone()
    //
    // }

    /* --------------------------------------------------------------------- */

    pub fn is_serializable(&self) -> bool {
        self.serializable
    }

    /* --------------------------------------------------------------------- */

    pub fn insert_symbol(&mut self, symbol_t: SymbolType) {
        let symbol_table = self.get_symbol_table_for_type(&symbol_t);
        symbol_table.borrow_mut().insert_symbol(&symbol_t);
    }

    /* --------------------------------------------------------------------- */

    // This method locates the proper symbol table for the system to insert the type into.
    // Typically this will be in the current symtab, but actions and domain objects
    // have other locations.
    // TODO: implement this for all symbol types!
    fn get_symbol_table_for_type(&self, symbol_t: &SymbolType) -> Rc<RefCell<SymbolTable>> {
        match symbol_t {
            SymbolType::StateParamSymbolT {
                state_param_symbol_rcref: _state_param_symbol_rcref,
            } => Rc::clone(&self.current_symtab),
            SymbolType::EventHandlerParamSymbolT {
                event_handler_param_symbol_rcref: _event_handler_param_symbol_rcref,
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
    pub domain_block_symbol_opt: Option<Rc<RefCell<DomainBlockScopeSymbol>>>,

    pub symbol_config: SymbolConfig,
}

impl SystemSymbol {
    pub fn new(name: String) -> SystemSymbol {
        SystemSymbol {
            name: name.clone(),
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                String::from(name.clone()),
                None,
                IdentifierDeclScope::None,
                true,
            ))),
            events: HashMap::new(),
            interface_block_symbol_opt: None,
            machine_block_symbol_opt: None,
            actions_block_symbol_opt: None,
            domain_block_symbol_opt: None,
            symbol_config: SymbolConfig::new(), // TODO
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
    }
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
            return Rc::clone(&e);
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
                IdentifierDeclScope::InterfaceBlock,
                false,
            ))),
        }
    }

    pub fn scope_name() -> &'static str {
        INTERFACE_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&symbol_table_for_symbol);
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
    pub ast_node: Option<Rc<RefCell<InterfaceMethodNode>>>,
}

impl InterfaceMethodSymbol {
    pub fn new(name: String) -> InterfaceMethodSymbol {
        InterfaceMethodSymbol {
            name,
            ast_node: None,
        }
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<InterfaceMethodNode>>) {
        self.ast_node = Some(Rc::clone(&ast_node));
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
    pub params_opt: Option<Vec<ParameterSymbol>>,
    pub ret_type_opt: Option<TypeNode>,
    pub is_enter_msg: bool,
    pub is_exit_msg: bool,
}

impl EventSymbol {
    pub fn new(
        symbol_config: &SymbolConfig,
        msg: &String,
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
            params_opt,
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
        msg: &String,
    ) -> (String, bool, bool) {
        let mut msg_name: String;
        let mut is_enter_msg = false;
        let mut is_exit_msg = false;
        if &symbol_config.enter_msg_symbol == msg {
            is_enter_msg = true;
            msg_name = state_name.as_ref().unwrap().clone();
            msg_name.push_str(":");
            msg_name.push_str(&symbol_config.enter_msg_symbol);
        } else if &symbol_config.exit_msg_symbol == msg {
            is_exit_msg = true;
            msg_name = state_name.as_ref().unwrap().clone();
            msg_name.push_str(":");
            msg_name.push_str(&symbol_config.exit_msg_symbol);
        } else {
            msg_name = msg.clone();
        }

        (msg_name, is_enter_msg, is_exit_msg)
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
                IdentifierDeclScope::None,
                false,
            ))),
        }
    }

    pub fn scope_name() -> &'static str {
        MACHINE_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
    pub event_handlers_opt: Option<Vec<String>>,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
    pub state_node: Option<Rc<RefCell<StateNode>>>,
    //    pub uses_enter_params:bool,
    requires_state_context: bool,
}

impl StateSymbol {
    pub fn new(state_name: &String, parent_symtab: Rc<RefCell<SymbolTable>>) -> StateSymbol {
        let st_rcref = SymbolTable::new(
            state_name.clone(),
            Some(Rc::clone(&parent_symtab)),
            IdentifierDeclScope::None,
            false,
        );
        StateSymbol {
            name: state_name.clone(),
            params_opt: None,
            event_handlers_opt: None,
            symtab_rcref: Rc::new(RefCell::new(st_rcref)),
            state_node: None,
            requires_state_context: false,
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
    }

    pub fn set_state_node(&mut self, state_node: Rc<RefCell<StateNode>>) {
        self.state_node = Some(state_node);
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
        let state_param_symbol = SymbolType::StateParamSymbolT {
            state_param_symbol_rcref: Rc::clone(&param_symbol_rcref),
        };

        return state_param_symbol;
        // this is wrong? as the param needs to be inserted in the
        // param symbol table maintained in arcanium.
        // TODO: does this symbol table need to be here???
        //        self.symtab_rcref.borrow_mut().insert_symbol(&state_param_symbol);
    }

    // pub fn add_event_handler(&mut self, event_symbol:&EventSymbol) {
    //     if event_symbol.requires_state_context() {
    //         self.requires_state_context = true
    //     }
    //     match &mut self.event_handlers_opt {
    //         Some(event_handlers) => {
    //             event_handlers.push(event_symbol.msg.clone());
    //         },
    //         None => {
    //             let mut eh_vec:Vec<String> = Vec::new();
    //             eh_vec.push(event_symbol.msg.clone());
    //             self.event_handlers_opt = Some(eh_vec);
    //         },
    //     }
    // }

    // pub fn get_state_param_scope_symbol(&self) -> Option<Rc<RefCell<StateParamsScopeSymbol>>> {
    //     let a =  &self.symtab_rcref;
    //     let c = a.borrow();
    //     let q = c.symbols.get("-state-parameters-");
    //     match q {
    //         Some(r) => {
    //             let s = r.borrow();
    //             match &*s {
    //                 SymbolType::StateParamsScopeSymbolT {state_params_scope_rcref} => {
    //                    Some(Rc::clone(state_params_scope_rcref))
    //                 },
    //                 _ => None,
    //             }
    //         },
    //         None => None,
    //     }
    // }
    //
    // pub fn get_state_local_scope_symbol(&self) -> Option<Rc<RefCell<StateLocalScopeSymbol>>> {
    //     let a = self.get_state_param_scope_symbol();
    //     match a {
    //         Some(b) => {
    //             let c = b.borrow();
    //             let d = &c.symtab_rcref;
    //             let e = &d.borrow().symbols;
    //             let f = e.get(StateLocalScopeSymbol::scope_name());
    //             match f {
    //                 Some(g) => {
    //                     let h = g.borrow();
    //                     match &*h {
    //                         SymbolType::StateLocalScopeSymbolT { state_local_scope_struct_rcref }
    //                         => {
    //                             Some(Rc::clone(state_local_scope_struct_rcref))
    //                         },
    //                         _ => None,
    //                     }
    //                 },
    //                 None => None,
    //             }
    //         },
    //         None => None,
    //     }
    // }

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
            return Rc::clone(&e);
        } else {
            panic!(
                "Fatal error - could not find symbol {} in state scope.",
                symbol_name
            );
        }
    }
}

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
            name: name,
            param_type_opt: param_type,
            scope,
        }
    }

    pub fn is_eq(&self, other: &ParameterNode) -> bool {
        if self.name != other.param_name {
            return false;
        }
        match &self.param_type_opt {
            Some(param_type) => match &other.param_type_opt {
                Some(other_param_type) => {
                    return &param_type.get_type_str() == &other_param_type.get_type_str();
                }
                None => return false,
            },
            None => match &other.param_type_opt {
                Some(_) => {
                    return false;
                }
                None => return true,
            },
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
                name.clone(),
                None,
                IdentifierDeclScope::StateParam,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        STATE_PARAMETERS_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
                IdentifierDeclScope::StateVar,
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
            Option::Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
        name: &String,
        event_symbol_rcref: Rc<RefCell<EventSymbol>>,
    ) -> EventHandlerScopeSymbol {
        EventHandlerScopeSymbol {
            name: name.clone(),
            event_symbol_rcref,
            symtab_rcref: Rc::new(RefCell::new(SymbolTable::new(
                name.clone(),
                None,
                IdentifierDeclScope::None,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt =
            Option::Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
                name.clone(),
                None,
                IdentifierDeclScope::EventHandlerParam,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        EVENT_HANDLER_PARAMETERS_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
        let event_handler_param_symbol = SymbolType::EventHandlerParamSymbolT {
            event_handler_param_symbol_rcref: Rc::clone(&param_symbol_rcref),
        };

        return event_handler_param_symbol;
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
            return Rc::clone(&e);
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
                IdentifierDeclScope::EventHandlerVar,
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
            Option::Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
                name.clone(),
                None,
                IdentifierDeclScope::ActionsBlock,
                false,
            ))),
        }
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
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
                name.clone(),
                None,
                IdentifierDeclScope::DomainBlock,
                false,
            ))),
        }
    }

    #[inline]
    pub fn scope_name() -> &'static str {
        DOMAIN_BLOCK_SCOPE_NAME
    }

    pub fn set_parent_symtab(&mut self, parent_symtab: &Rc<RefCell<SymbolTable>>) {
        self.symtab_rcref.borrow_mut().parent_symtab_rcref_opt = Some(Rc::clone(&parent_symtab));
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
            return Rc::clone(&e);
        } else {
            panic!(
                "Fatal error - could not find symbol {} in domain block scope.",
                symbol_name
            );
        }
    }
}

// ----------------------- //

pub struct ActionDeclSymbol {
    pub name: String,
    pub ast_node: Option<Rc<RefCell<ActionNode>>>,
}

impl ActionDeclSymbol {
    pub fn new(name: String) -> ActionDeclSymbol {
        ActionDeclSymbol {
            name,
            ast_node: None,
        }
    }

    pub fn set_ast_node(&mut self, ast_node: Rc<RefCell<ActionNode>>) {
        self.ast_node = Some(Rc::clone(&ast_node));
    }
}

impl Symbol for ActionDeclSymbol {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// ----------------------- //

pub struct ActionCallSymbol {
    pub name: String,
    pub ast_node: Option<Rc<RefCell<ActionCallExprNode>>>,
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
    pub ast_node: Option<Rc<RefCell<VariableDeclNode>>>,
}

impl VariableSymbol {
    pub fn new(
        name: String,
        var_type: Option<TypeNode>,
        scope: IdentifierDeclScope,
    ) -> VariableSymbol {
        VariableSymbol {
            name,
            var_type,
            scope,
            ast_node: None,
        }
    }

    // pub fn set_ast_node(&mut self, ast_node: VariableDeclNode) {
    //     self.ast_node = Some(Rc::new(RefCell::new(ast_node)));
    // }
}

impl Symbol for VariableSymbol {
    fn get_name(&self) -> String {
        self.name.clone() //String::from("domain")
    }
}
