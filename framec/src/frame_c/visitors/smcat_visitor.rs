use crate::frame_c::ast::*;
use crate::frame_c::scanner::Token;
use crate::frame_c::symbol_table::*;
use crate::frame_c::utils::SystemHierarchy;
use crate::frame_c::visitors::*;

pub struct SmcatVisitor {
    _compiler_version: String,
    pub code: String,
    pub dent: usize,
    pub current_state_name_opt: Option<String>,
    current_event_ret_type: String,
    arcanium: Arcanum,
    symbol_config: SymbolConfig,
    first_event_handler: bool,
    system_name: String,
    first_state_name: String,
    states: String,
    transitions: String,
    system_hierarchy: SystemHierarchy,
    event_handler_msg: String,
}

impl SmcatVisitor {
    //* --------------------------------------------------------------------- *//

    pub fn new(
        arcanium: Arcanum,
        system_hierarchy: SystemHierarchy,
        compiler_version: &str,
        _comments: Vec<Token>,
    ) -> SmcatVisitor {
        SmcatVisitor {
            _compiler_version: compiler_version.to_string(),
            code: String::from(""),
            dent: 0,
            current_state_name_opt: None,
            current_event_ret_type: String::new(),
            arcanium,
            symbol_config: SymbolConfig::new(),
            first_event_handler: true,
            system_name: String::new(),
            first_state_name: String::new(),
            states: String::new(),
            transitions: String::new(),
            system_hierarchy,
            event_handler_msg: String::new(),
        }
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    //* --------------------------------------------------------------------- *//

    fn generate_states(
        &self,
        node_name: &str,
        is_system_node: bool,
        indent: usize,
        output: &mut String,
    ) {
        let mut actual_indent = indent;
        let node = self.system_hierarchy.get_node(node_name).unwrap();
        let has_children = !(&node.children.is_empty());
        if !is_system_node {
            actual_indent += 1;
            if has_children {
                output.push_str(&format!("{}{} {{\n", self.specifiy_dent(indent), node_name));
            } else {
                output.push_str(&format!("{}{}, \n", self.specifiy_dent(indent), node_name));
            }
        }

        for child_node_name in &node.children {
            let child_node = self.system_hierarchy.get_node(child_node_name).unwrap();
            self.generate_states(&child_node.name, false, actual_indent, output);
        }

        // change last coma to semicolon
        if has_children {
            if let Some(location) = output.rfind(',') {
                output.replace_range(location..location + 1, ";")
            }
        }

        if !is_system_node && has_children {
            output.push_str(&format!("{}}},\n", self.specifiy_dent(indent)));
        }
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node: &SystemNode) {
        system_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn add_code(&mut self, s: &str) {
        self.code.push_str(s);
    }

    //* --------------------------------------------------------------------- *//

    fn newline(&mut self) {
        self.code.push_str(&*format!("\n{}", self.dent()));
    }

    //* --------------------------------------------------------------------- *//

    fn dent(&self) -> String {
        (0..self.dent).map(|_| "    ").collect::<String>()
    }

    //* --------------------------------------------------------------------- *//

    fn specifiy_dent(&self, dent: usize) -> String {
        (0..dent).map(|_| "    ").collect::<String>()
    }

    //* --------------------------------------------------------------------- *//

    fn visit_decl_stmts(&mut self, decl_stmt_types: &[DeclOrStmtType]) {
        for decl_stmt_t in decl_stmt_types.iter() {
            match decl_stmt_t {
                DeclOrStmtType::VarDeclT { .. } => {}
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t: _ } => {}
                        StatementType::TransitionStmt {
                            transition_statement,
                        } => {
                            transition_statement.accept(self);
                        }
                        StatementType::TestStmt { test_stmt_node } => {
                            test_stmt_node.accept(self);
                        }
                        StatementType::StateStackStmt {
                            state_stack_operation_statement_node,
                        } => {
                            state_stack_operation_statement_node.accept(self);
                        }
                        StatementType::ChangeStateStmt { change_state_stmt } => {
                            change_state_stmt.accept(self);
                        }
                        StatementType::NoStmt => {
                            // TODO
                            panic!("todo");
                        }
                    }
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_change_state(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        let target_state_name = match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => panic!("TODO"),
        };

        let mut current_state: String = "??".to_string();
        if let Some(state_name) = &self.current_state_name_opt {
            current_state = state_name.clone();
        }

        let label = match &change_state_stmt_node.label_opt {
            Some(label) => {
                format!("{};", label.clone())
            }
            None => {
                format!("{};", self.event_handler_msg.clone())
            }
        };

        let transition_code = &format!(
            "{} => {} [color=\"grey\"] : {}\n",
            current_state,
            self.format_target_state_name(target_state_name),
            label
        );
        self.transitions.push_str(transition_code);
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(&mut self, transition_statement: &TransitionStatementNode) {
        let target_state_name = match &transition_statement.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => panic!("TODO"),
        };

        let _state_ref_code = self.format_target_state_name(target_state_name);

        match &transition_statement.label_opt {
            Some(_label) => {}
            None => {}
        }

        let mut current_state: String = "??".to_string();
        if let Some(state_name) = &self.current_state_name_opt {
            current_state = state_name.clone();
        }

        let label = match &transition_statement.label_opt {
            Some(label) => label.clone(),
            None => self.event_handler_msg.clone(),
        };

        let transition_code = &format!(
            "{} => {} : {};\n",
            current_state,
            self.format_target_state_name(target_state_name),
            label
        );
        self.transitions.push_str(transition_code);
    }

    //* --------------------------------------------------------------------- *//

    fn format_target_state_name(&self, state_name: &str) -> String {
        state_name.to_string()
    }

    //* --------------------------------------------------------------------- *//
    // TODO: Review if this is correct handling. At least with regular statecharts,
    // each state with children can have a separate history that's used to determine
    // initial child state on reentry to parent state
    fn generate_state_stack_pop_transition(
        &mut self,
        transition_statement: &TransitionStatementNode,
    ) {
        let label = match &transition_statement.label_opt {
            Some(label) => label,
            None => &self.event_handler_msg,
        };
        // .deephistory suffix overrides target state label with H* and sets shape to
        // circle
        self.transitions.push_str(&format!(
            "{} => H*.deephistory : {};\n",
            &self.current_state_name_opt.as_ref().unwrap(),
            label
        ));
    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for SmcatVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        if let Some(first_state) = system_node.get_first_state() {
            self.first_state_name = first_state.borrow().name.clone();
            self.add_code("initial,\n");
            self.transitions
                .push_str(&format!("initial => \"{}\";\n", self.first_state_name));
        }

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        self.add_code(&self.states.clone());
        self.newline();
        self.add_code(&self.transitions.clone());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_messages_enum(&mut self, _interface_block_node: &InterfaceBlockNode) {
        panic!("Error - visit_frame_messages_enum() only used in Rust.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_parameters(&mut self, _interface_block_node: &InterfaceBlockNode) {
        panic!("visit_interface_parameters() not valid for target language.");
    }

    fn visit_interface_method_call_expression_node(
        &mut self,
        _interface_method_call_expr_node: &InterfaceMethodCallExprNode,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        _interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_block_node(&mut self, _interface_block_node: &InterfaceBlockNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_node(&mut self, _interface_method_node: &InterfaceMethodNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) {
        let mut output = String::new();
        let sys_name = self.system_name.clone();
        let _system_node = self.system_hierarchy.get_system_node().unwrap();
        self.generate_states(&sys_name, true, 0, &mut output);
        self.states = output;

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, _actions_block_node: &ActionsBlockNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_action_node_rust_trait(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_action_node_rust_trait() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_node_rust_impl(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_actions_node_rust_impl() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_block_node(&mut self, _domain_block_node: &DomainBlockNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) {
        self.current_state_name_opt = Some(state_node.name.clone());

        let _state_symbol = match self.arcanium.get_state(&state_node.name) {
            Some(state_symbol) => state_symbol,
            None => panic!("TODO"),
        };

        self.first_event_handler = true; // context for formatting

        if !state_node.evt_handlers_rcref.is_empty() {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }

        match &state_node.dispatch_opt {
            Some(_dispatch) => {}
            None => {}
        }

        self.current_state_name_opt = None;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) {
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            self.event_handler_msg = format!("|{}|", message_node.name);
        } else {
            // AnyMessage ( ||* )
            self.event_handler_msg = "||*".to_string();
        }
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            let (_msg, _, _) = EventSymbol::get_event_msg(
                &self.symbol_config,
                &Some(evt_handler_node.state_name.clone()),
                &message_node.name,
            );
        }

        // Generate statements
        self.visit_decl_stmts(&evt_handler_node.statements);

        // this controls formatting here
        self.first_event_handler = false;
        self.current_event_ret_type = String::new();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(
        &mut self,
        _evt_handler_terminator_node: &TerminatorExpr,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, _method_call_statement: &CallStmtNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, _method_call: &CallExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node_to_string(
        &mut self,
        _method_call: &CallExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node(&mut self, _action_call: &ActionCallExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        _action_call: &ActionCallExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node(&mut self, _call_expr_list: &CallExprListNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node_to_string(
        &mut self,
        _call_expr_list: &CallExprListNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, _action_call_stmt_node: &ActionCallStmtNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(&mut self, transition_statement: &TransitionStatementNode) {
        match &transition_statement.target_state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_transition(transition_statement)
            }
            StateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_transition(transition_statement)
            }
        };
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) {
        self.add_code(&state_ref.name);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_change_state_statement_node(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_change_state(change_state_stmt_node)
            }
            StateContextType::StateStackPop {} => panic!("TODO - not implemented"),
        };
    }

    //* --------------------------------------------------------------------- *//

    fn visit_parameter_node(&mut self, _parameter_node: &ParameterNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, _dispatch_node: &DispatchNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_test_statement_node(&mut self, test_stmt_node: &TestStatementNode) {
        match &test_stmt_node.test_t {
            TestType::BoolTest { bool_test_node } => {
                bool_test_node.accept(self);
            }
            TestType::StringMatchTest {
                string_match_test_node,
            } => {
                string_match_test_node.accept(self);
            }
            TestType::NumberMatchTest {
                number_match_test_node,
            } => {
                number_match_test_node.accept(self);
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_node(&mut self, bool_test_node: &BoolTestNode) {
        for branch_node in &bool_test_node.conditional_branch_nodes {
            branch_node.expr_t.accept(self);
            branch_node.accept(self);
        }

        // (':' bool_test_else_branch)?
        if let Some(bool_test_else_branch_node) = &bool_test_node.else_branch_node_opt {
            bool_test_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_statement_node(
        &mut self,
        _method_call_chain_literal_stmt_node: &CallChainLiteralStmtNode,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node(
        &mut self,
        _method_call_chain_expression_node: &CallChainLiteralExprNode,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node_to_string(
        &mut self,
        _method_call_chain_expression_node: &CallChainLiteralExprNode,
        _output: &mut String,
    ) {
        panic!("TODO");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) {
        self.visit_decl_stmts(&bool_test_true_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&bool_test_else_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    // Used in event string matching transitions
    fn visit_string_match_test_node(&mut self, string_match_test_node: &StringMatchTestNode) {
        for match_branch_node in &string_match_test_node.match_branch_nodes {
            // TODO: use string_match_test_node.expr_t.accept(self) ?
            match &string_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT {
                    call_chain_expr_node,
                } => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),

                _ => panic!("TODO"),
            }

            match_branch_node.accept(self);
        }

        // (':' string_test_else_branch)?
        if let Some(string_match_else_branch_node) = &string_match_test_node.else_branch_node_opt {
            string_match_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_match_branch_node(
        &mut self,
        string_match_test_match_branch_node: &StringMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&string_match_test_match_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&string_match_test_else_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_pattern_node(
        &mut self,
        _string_match_test_else_branch_node: &StringMatchTestPatternNode,
    ) {
        panic!("todo");
    }

    //-----------------------------------------------------//

    fn visit_number_match_test_node(&mut self, number_match_test_node: &NumberMatchTestNode) {
        for match_branch_node in &number_match_test_node.match_branch_nodes {
            // self.add_code(&format!("{} (", if_or_else_if));
            match &number_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT {
                    call_chain_expr_node,
                } => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                _ => panic!("TODO"),
            }

            let mut first_match = true;
            for _match_number in &match_branch_node.number_match_pattern_nodes {
                if first_match {
                    first_match = false;
                } else {
                    match &number_match_test_node.expr_t {
                        ExprType::CallExprT {
                            call_expr_node: method_call_expr_node,
                        } => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT {
                            action_call_expr_node,
                        } => action_call_expr_node.accept(self),
                        ExprType::CallChainLiteralExprT {
                            call_chain_expr_node,
                        } => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                        _ => panic!("TODO"),
                    }
                }
            }

            match_branch_node.accept(self);
        }

        if let Some(number_match_else_branch_node) = &number_match_test_node.else_branch_node_opt {
            number_match_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_match_branch_node(
        &mut self,
        number_match_test_match_branch_node: &NumberMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&number_match_test_match_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_pattern_node(
        &mut self,
        _match_pattern_node: &NumberMatchTestPatternNode,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node(&mut self, _expr_list: &ExprListNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(
        &mut self,
        _expr_list: &ExprListNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, _literal_expression_node: &LiteralExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node_to_string(
        &mut self,
        _literal_expression_node: &LiteralExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node(&mut self, _identifier_node: &IdentifierNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node_to_string(
        &mut self,
        _identifier_node: &IdentifierNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node_to_string(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_statement_node(
        &mut self,
        _state_stack_op_statement_node: &StateStackOperationStatementNode,
    ) {
    }
    //* --------------------------------------------------------------------- *//

    fn visit_state_context_node(&mut self, _state_context_node: &StateContextNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_frame_event_part(&mut self, _frame_event_part: &FrameEventPart) {}

    //* --------------------------------------------------------------------- *//

    fn visit_frame_event_part_to_string(
        &mut self,
        _frame_event_part: &FrameEventPart,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_decl_node(&mut self, _action_decl_node: &ActionNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, _action_decl_node: &ActionNode) {
        panic!("visit_action_impl_node() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, _variable_decl_node: &VariableDeclNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, _variable_decl_node: &VariableDeclNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node(&mut self, _variable_node: &VariableNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node_to_string(
        &mut self,
        _variable_node: &VariableNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_stmt_node(&mut self, _variable_stmt_node: &VariableStmtNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node(&mut self, _assignment_expr_node: &AssignmentExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(
        &mut self,
        _assignment_expr_node: &AssignmentExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(&mut self, _assignment_stmt_node: &AssignmentStmtNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node(&mut self, _unary_expr_node: &UnaryExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node_to_string(
        &mut self,
        _unary_expr_node: &UnaryExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node(&mut self, _binary_expr_node: &BinaryExprNode) {}

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node_to_string(
        &mut self,
        _binary_expr_node: &BinaryExprNode,
        _output: &mut String,
    ) {
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type(&mut self, _operator_type: &OperatorType) {}

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type_to_string(
        &mut self,
        _operator_type: &OperatorType,
        _output: &mut String,
    ) {
    }
}
