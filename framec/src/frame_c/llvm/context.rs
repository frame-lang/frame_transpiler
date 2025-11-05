use crate::frame_c::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::utils::{sanitize_identifier, to_pascal_case};
use super::value::{infer_value_kind_from_type, DomainField, ValueKind};

pub(super) struct SystemEmitContext {
    pub(super) system_name: String,
    pub(super) sanitized_name: String,
    pub(super) struct_name: String,
    pub(super) start_state_index: i32,
    pub(super) start_state_name: String,
    pub(super) state_map: HashMap<String, i32>,
    pub(super) states: Vec<StateEntry>,
    pub(super) domain_fields: Vec<DomainField>,
    pub(super) actions: Vec<ActionEntry>,
    pub(super) action_lookup: HashMap<String, usize>,
    pub(super) interface_methods: Vec<InterfaceMethodInfo>,
    interface_message_lookup: HashMap<String, usize>,
}

pub(super) struct MethodNames {
    pub(super) method_ident: String,
    pub(super) fn_name: String,
}

#[derive(Clone)]
pub(super) struct SystemSummary {
    pub(super) raw_name: String,
    pub(super) sanitized_name: String,
    pub(super) struct_name: String,
    pub(super) align: usize,
    pub(super) domain_fields: Vec<DomainField>,
    domain_field_lookup: HashMap<String, usize>,
    interface_param_lookup: HashMap<String, Vec<ValueKind>>,
}

impl SystemSummary {
    pub(super) fn init_fn(&self) -> String {
        format!("@{}__init", self.sanitized_name)
    }

    pub(super) fn deinit_fn(&self) -> String {
        format!("@{}__deinit", self.sanitized_name)
    }

    pub(super) fn method_fn(&self, method: &str) -> String {
        format!("@{}__{}", self.sanitized_name, sanitize_identifier(method))
    }

    pub(super) fn domain_field(&self, name: &str) -> Option<&DomainField> {
        self.domain_field_lookup
            .get(name)
            .and_then(|index| self.domain_fields.get(*index))
    }

    pub(super) fn interface_params(&self, name: &str) -> Option<&[ValueKind]> {
        self.interface_param_lookup
            .get(name)
            .map(|params| params.as_slice())
    }
}

#[derive(Clone)]
pub(super) struct MainLocal {
    pub(super) ptr: String,
    pub(super) system: SystemSummary,
}

pub(super) struct MainScope {
    locals: HashMap<String, MainLocal>,
    order: Vec<String>,
}

impl MainScope {
    pub(super) fn new() -> Self {
        MainScope {
            locals: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub(super) fn insert(&mut self, name: String, local: MainLocal) {
        self.order.push(name.clone());
        self.locals.insert(name, local);
    }

    pub(super) fn get(&self, name: &str) -> Option<&MainLocal> {
        self.locals.get(name)
    }

    pub(super) fn drop_order(&self) -> impl DoubleEndedIterator<Item = &String> + '_ {
        self.order.iter()
    }
}

#[derive(Clone)]
pub(super) struct StateEntry {
    pub(super) name: String,
    pub(super) handlers: HashMap<String, Rc<RefCell<EventHandlerNode>>>,
    pub(super) parent_state_name: Option<String>,
    pub(super) enter_handler: Option<Rc<RefCell<EventHandlerNode>>>,
    pub(super) exit_handler: Option<Rc<RefCell<EventHandlerNode>>>,
    pub(super) state_params: Vec<StateParam>,
    pub(super) enter_params: Vec<StateParam>,
}

#[derive(Clone)]
pub(super) struct ActionEntry {
    pub(super) name: String,
    pub(super) fn_name: String,
    pub(super) node: Rc<RefCell<ActionNode>>,
    pub(super) params: Vec<ActionParam>,
}

#[derive(Clone)]
pub(super) struct ActionParam {
    pub(super) name: String,
    pub(super) kind: ValueKind,
}

#[derive(Clone)]
pub(super) struct StateParam {
    pub(super) name: String,
    pub(super) kind: ValueKind,
}

#[derive(Clone)]
pub(super) struct InterfaceMethodInfo {
    pub(super) name: String,
    pub(super) message: String,
    pub(super) fn_name: String,
    pub(super) params: Vec<StateParam>,
}

impl SystemEmitContext {
    pub(super) fn new(system: &SystemNode) -> Option<Self> {
        let machine = system.machine_block_node_opt.as_ref()?;
        if machine.states.is_empty() {
            return None;
        }

        let sanitized = sanitize_identifier(&system.name);
        let struct_ident = to_pascal_case(&sanitized);
        let struct_name = format!("%{}System", struct_ident);

        let mut state_map = HashMap::new();
        let mut states = Vec::new();

        for (idx, state_rc) in machine.states.iter().enumerate() {
            let state = state_rc.borrow();
            let mut handlers = HashMap::new();

            for handler_rc in &state.evt_handlers_rcref {
                let handler = handler_rc.borrow();
                if let MessageType::CustomMessage { message_node } = &handler.msg_t {
                    handlers.insert(message_node.name.clone(), handler_rc.clone());
                }
            }

            let parent_state_name = state
                .dispatch_opt
                .as_ref()
                .map(|dispatch| dispatch.target_state_ref.name.clone());

            state_map.insert(state.name.clone(), idx as i32);
            let state_params = state
                .params_opt
                .as_ref()
                .map(|params| {
                    params
                        .iter()
                        .map(|param| StateParam {
                            name: param.param_name.clone(),
                            kind: infer_value_kind_from_type(param.param_type_opt.as_ref()),
                        })
                        .collect()
                })
                .unwrap_or_default();

            let enter_params = state
                .enter_event_handler_opt
                .as_ref()
                .map(|handler_rc| {
                    let handler = handler_rc.borrow();
                    let symbol = handler.event_symbol_rcref.borrow();
                    symbol
                        .event_symbol_params_opt
                        .as_ref()
                        .map(|params| {
                            params
                                .iter()
                                .map(|param| StateParam {
                                    name: param.name.clone(),
                                    kind: infer_value_kind_from_type(param.param_type_opt.as_ref()),
                                })
                                .collect()
                        })
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            states.push(StateEntry {
                name: state.name.clone(),
                handlers,
                parent_state_name,
                enter_handler: state.enter_event_handler_opt.clone(),
                exit_handler: state.exit_event_handler_opt.clone(),
                state_params,
                enter_params,
            });
        }

        let start_state_name = machine
            .get_first_state()
            .map(|state_rc| state_rc.borrow().name.clone())
            .or_else(|| states.first().map(|state| state.name.clone()))
            .unwrap_or_else(String::new);
        let start_state_index = state_map.get(&start_state_name).copied().unwrap_or(0);

        let mut domain_fields = Vec::new();
        if let Some(domain_block) = &system.domain_block_node_opt {
            for var_decl_rc in &domain_block.member_variables {
                let var_decl = var_decl_rc.borrow();
                if let Some(field) = DomainField::from_var_decl(&var_decl) {
                    domain_fields.push(field);
                }
            }
        }
        for (idx, field) in domain_fields.iter_mut().enumerate() {
            field.struct_index = idx + 1;
        }

        let mut actions = Vec::new();
        let mut action_lookup = HashMap::new();
        if let Some(actions_block) = &system.actions_block_node_opt {
            for action_rc in &actions_block.actions {
                let action = action_rc.borrow();
                if !action.is_implemented {
                    continue;
                }
                let fn_name = format!(
                    "@{}__action_{}",
                    sanitized,
                    sanitize_identifier(&action.name)
                );
                let mut params = Vec::new();
                if let Some(param_nodes) = &action.params {
                    for param in param_nodes {
                        let kind = infer_value_kind_from_type(param.param_type_opt.as_ref());
                        params.push(ActionParam {
                            name: param.param_name.clone(),
                            kind,
                        });
                    }
                }
                let entry = ActionEntry {
                    name: action.name.clone(),
                    fn_name,
                    node: Rc::clone(action_rc),
                    params,
                };
                action_lookup.insert(entry.name.clone(), actions.len());
                actions.push(entry);
            }
        }

        let mut interface_methods = Vec::new();
        let mut interface_message_lookup = HashMap::new();
        if let Some(interface_block) = &system.interface_block_node_opt {
            for method_rc in &interface_block.interface_methods {
                let method = method_rc.borrow();
                let method_ident = sanitize_identifier(&method.name);
                let fn_name = format!("@{}__{}", sanitized, method_ident);
                let message = method
                    .alias
                    .as_ref()
                    .map(|msg| msg.name.clone())
                    .unwrap_or_else(|| method.name.clone());
                let params = method
                    .params
                    .as_ref()
                    .map(|params| {
                        params
                            .iter()
                            .map(|param| StateParam {
                                name: param.param_name.clone(),
                                kind: infer_value_kind_from_type(param.param_type_opt.as_ref()),
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let index = interface_methods.len();
                interface_message_lookup.insert(message.clone(), index);
                interface_methods.push(InterfaceMethodInfo {
                    name: method.name.clone(),
                    message,
                    fn_name,
                    params,
                });
            }
        }

        Some(SystemEmitContext {
            system_name: system.name.clone(),
            sanitized_name: sanitized,
            struct_name,
            start_state_index,
            start_state_name,
            state_map,
            states,
            domain_fields,
            actions,
            action_lookup,
            interface_methods,
            interface_message_lookup,
        })
    }

    pub(super) fn summary(&self) -> SystemSummary {
        let domain_fields = self.domain_fields.clone();
        let mut domain_field_lookup = HashMap::new();
        for (idx, field) in domain_fields.iter().enumerate() {
            domain_field_lookup.insert(field.name.clone(), idx);
        }
        let mut interface_param_lookup = HashMap::new();
        for method in &self.interface_methods {
            let kinds = method
                .params
                .iter()
                .map(|param| param.kind)
                .collect::<Vec<_>>();
            interface_param_lookup.insert(method.name.clone(), kinds);
        }
        SystemSummary {
            raw_name: self.system_name.clone(),
            sanitized_name: self.sanitized_name.clone(),
            struct_name: self.struct_name.clone(),
            align: self.struct_alignment(),
            domain_fields,
            domain_field_lookup,
            interface_param_lookup,
        }
    }

    pub(super) fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    pub(super) fn actions_iter(&self) -> impl Iterator<Item = &ActionEntry> {
        self.actions.iter()
    }

    pub(super) fn action(&self, name: &str) -> Option<&ActionEntry> {
        self.action_lookup
            .get(name)
            .and_then(|index| self.actions.get(*index))
    }

    pub(super) fn struct_fields(&self) -> Vec<String> {
        let mut fields = Vec::with_capacity(3 + self.domain_fields.len());
        fields.push("i32".to_string());
        for field in &self.domain_fields {
            fields.push(field.field_type.llvm_type().to_string());
        }
        fields.push("ptr".to_string()); // runtime kernel
        fields.push("ptr".to_string()); // current compartment
        fields
    }

    pub(super) fn struct_alignment(&self) -> usize {
        if self
            .domain_fields
            .iter()
            .any(|field| field.field_type.needs_eight_byte_align())
        {
            8
        } else {
            4
        }
    }

    pub(super) fn start_state_name(&self) -> &str {
        self.start_state_name.as_str()
    }

    pub(super) fn domain_field(&self, name: &str) -> Option<&DomainField> {
        self.domain_fields.iter().find(|field| field.name == name)
    }

    pub(super) fn runtime_field_index(&self) -> usize {
        1 + self.domain_fields.len()
    }

    pub(super) fn compartment_field_index(&self) -> usize {
        2 + self.domain_fields.len()
    }

    pub(super) fn method_names(&self, method: &InterfaceMethodNode) -> MethodNames {
        let method_ident = sanitize_identifier(&method.name);
        let fn_name = format!("@{}__{}", self.sanitized_name, method_ident);
        MethodNames {
            method_ident,
            fn_name,
        }
    }

    pub(super) fn state_label(&self, method_ident: &str, state_name: &str) -> String {
        format!(
            "{}_{}_state_{}",
            self.sanitized_name,
            method_ident,
            sanitize_identifier(state_name)
        )
    }

    pub(super) fn state(&self, index: usize) -> &StateEntry {
        &self.states[index]
    }

    pub(super) fn state_enter_fn(&self, state_name: &str) -> String {
        format!(
            "@{}__state_{}_enter",
            self.sanitized_name,
            sanitize_identifier(state_name)
        )
    }

    pub(super) fn state_exit_fn(&self, state_name: &str) -> String {
        format!(
            "@{}__state_{}_exit",
            self.sanitized_name,
            sanitize_identifier(state_name)
        )
    }

    pub(super) fn transition_target_index(
        &self,
        transition: &TransitionStatementNode,
    ) -> Option<i32> {
        match &transition.transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => self
                .state_map
                .get(&state_context_node.state_ref_node.name)
                .copied(),
            TargetStateContextType::StateStackPop {} => None,
            TargetStateContextType::StateStackPush {} => None,
        }
    }

    pub(super) fn state_index(&self, name: &str) -> Option<i32> {
        self.state_map.get(name).copied()
    }

    pub(super) fn interface_methods(&self) -> &[InterfaceMethodInfo] {
        &self.interface_methods
    }

    pub(super) fn interface_method_by_message(
        &self,
        message: &str,
    ) -> Option<&InterfaceMethodInfo> {
        self.interface_message_lookup
            .get(message)
            .and_then(|index| self.interface_methods.get(*index))
    }
}

impl StateEntry {
    pub(super) fn state_param(&self, name: &str) -> Option<&StateParam> {
        self.state_params.iter().find(|param| param.name == name)
    }

    pub(super) fn enter_param(&self, name: &str) -> Option<&StateParam> {
        self.enter_params.iter().find(|param| param.name == name)
    }
}
