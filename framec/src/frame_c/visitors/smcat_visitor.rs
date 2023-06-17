use crate::frame_c::ast::*;
use crate::frame_c::config::{FrameConfig, SmcatConfig};
use crate::frame_c::utils::SystemHierarchy;
use crate::frame_c::visitors::*;

fn indent_str(indent: usize) -> String {
    "  ".repeat(indent)
}

fn format_styling(style: &str) -> String {
    if style.is_empty() {
        String::new()
    } else {
        format!(" [{}]", style)
    }
}

pub struct SmcatVisitor {
    _compiler_version: &'static str,
    config: SmcatConfig,
    system_hierarchy: SystemHierarchy,
    current_state: Option<String>,
    current_handler: Option<String>,
    code: String,
}

impl SmcatVisitor {
    pub fn new(
        compiler_version: &'static str,
        config: FrameConfig,
        system_hierarchy: SystemHierarchy,
    ) -> SmcatVisitor {
        let smcat_config = config.codegen.smcat;
        SmcatVisitor {
            _compiler_version: compiler_version,
            config: smcat_config,
            system_hierarchy,
            current_state: None,
            current_handler: None,
            code: String::from(""),
        }
    }

    pub fn run(&mut self, system_node: &SystemNode) {
        system_node.accept(self);
    }

    fn add_code(&mut self, s: &str) {
        self.code.push_str(s);
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    fn generate_states(&self, node_name: &str, indent: usize, output: &mut String) {
        let node = self.system_hierarchy.get_node(node_name).unwrap();
        let mut child_iter = node.children.iter().peekable();
        let has_children = child_iter.peek().is_some();
        let indent_str = indent_str(indent);

        // add state
        let style = if has_children {
            &self.config.code.parent_state_node_style
        } else {
            &self.config.code.simple_state_node_style
        };
        output.push_str(&format!(
            "{}{}{}",
            indent_str,
            node_name,
            format_styling(style)
        ));

        // add children
        if has_children {
            output.push_str(" {\n");
        }
        while let Some(child_name) = child_iter.next() {
            let last_child = child_iter.peek().is_none();
            self.generate_states(child_name, indent + 1, output);
            output.push_str(&format!("{}\n", if last_child { ";" } else { "," }));
        }
        if has_children {
            output.push_str(&format!("{}}}", indent_str));
        }
    }

    fn visit_decl_stmts(&mut self, decl_stmt_types: &[DeclOrStmtType]) {
        for decl_stmt_t in decl_stmt_types.iter() {
            match decl_stmt_t {
                DeclOrStmtType::VarDeclT { .. } => {}
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        // TODO: do we need to worry about directly invoked handlers?
                        StatementType::ExpressionStmt { .. } => {}
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

                        StatementType::NoStmt => {}
                    }
                }
            }
        }
    }

    fn generate_transition(
        &mut self,
        source_name: &str,
        target_name: &str,
        style: &str,
        event_name: &str,
        label: Option<&String>,
    ) {
        self.add_code(&format!(
            "{} -> {}{} : \"  {}{}  \";\n",
            source_name,
            target_name,
            style,
            event_name,
            match label {
                Some(text) => format!("/ {}", text),
                None => String::new(),
            }
        ));
    }

    fn generate_state_ref_change_state(&mut self, change_state_stmt: &ChangeStateStatementNode) {
        let source_state = self.current_state.as_ref().unwrap().to_string();
        let target_state = match &change_state_stmt.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            StateContextType::StateStackPop {} => {
                panic!("TODO")
            }
        };
        let style = format_styling(&self.config.code.change_state_edge_style);
        let event = self.current_handler.as_ref().unwrap().clone();
        self.generate_transition(
            &source_state,
            target_state,
            &style,
            &event,
            change_state_stmt.label_opt.as_ref(),
        );
    }

    fn generate_state_ref_transition(&mut self, transition_stmt: &TransitionStatementNode) {
        let source_state = self.current_state.as_ref().unwrap().clone();
        let target_state = match &transition_stmt.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            StateContextType::StateStackPop {} => {
                panic!("TODO")
            }
        };
        let style = format_styling(&self.config.code.transition_edge_style);
        let event = self.current_handler.as_ref().unwrap().clone();
        self.generate_transition(
            &source_state,
            target_state,
            &style,
            &event,
            transition_stmt.label_opt.as_ref(),
        );
    }

    // TODO: Review if this is correct handling. At least with regular statecharts,
    // each state with children can have a separate history that's used to determine
    // initial child state on reentry to parent state
    fn generate_state_stack_pop_transition(
        &mut self,
        transition_statement: &TransitionStatementNode,
    ) {
        let event = self.current_handler.as_ref().unwrap().clone();
        let label = match &transition_statement.label_opt {
            Some(label) => label,
            None => &event,
        };
        // .deephistory suffix overrides target state label with H* and sets shape to
        // circle
        let transition = &format!(
            "{} -> H*.deephistory : {};\n",
            &self.current_state.as_ref().unwrap(),
            label
        );
        self.add_code(transition);
    }
}

impl AstVisitor for SmcatVisitor {
    fn visit_system_node(&mut self, system_node: &SystemNode) {
        // Generate the pointer to the initial state
        if system_node.get_first_state().is_some() {
            self.add_code("initial,\n");
        }
        // Generate the rest of the state machine
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }
    }

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) {
        let mut output = String::new();
        let system_name = &self.system_hierarchy.system_name;
        let system_node = self.system_hierarchy.get_node(system_name).unwrap();
        let mut state_iter = system_node.children.iter().peekable();
        while let Some(state_name) = state_iter.next() {
            let last_state = state_iter.peek().is_none();
            self.generate_states(state_name, 0, &mut output);
            output.push_str(&format!("{}\n", if last_state { ";" } else { "," }));
        }
        output.push('\n');
        self.add_code(&output);
        if let Some(first_state) = machine_block_node.get_first_state() {
            self.add_code(&format!(
                "initial -> {};\n",
                first_state.borrow().name.clone()
            ));
        }
        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }
    }

    fn visit_action_node_rust_trait(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_action_node_rust_trait() not implemented.");
    }

    fn visit_actions_node_rust_impl(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_actions_node_rust_impl() not implemented.");
    }

    fn visit_state_node(&mut self, state_node: &StateNode) {
        self.current_state = Some(state_node.name.clone());

        if !state_node.evt_handlers_rcref.is_empty() {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }

        match &state_node.dispatch_opt {
            Some(_dispatch) => {}
            None => {}
        }

        self.current_state = None;
    }

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) {
        // remember qualified event name
        let state_name = &self.current_state.as_ref().unwrap();
        let event_name =
            if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
                &message_node.name
            } else {
                "||*"
            };
        let mut qualified_event_name = String::new();
        if event_name == ">" || event_name == "<" {
            qualified_event_name.push_str(state_name);
            qualified_event_name.push(':');
        }
        qualified_event_name.push_str(event_name);
        self.current_handler = Some(qualified_event_name);

        // process statements, looking for transitions
        self.visit_decl_stmts(&evt_handler_node.statements);

        // forget event name
        self.current_handler = None;
    }

    fn visit_action_call_statement_node(&mut self, _action_call_stmt_node: &ActionCallStmtNode) {}

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

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) {
        self.add_code(&state_ref.name);
    }

    fn visit_change_state_statement_node(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_change_state(change_state_stmt_node)
            }
            StateContextType::StateStackPop {} => {
                // self.generate_state_stack_pop_change_state(transition_statement)
                panic!("change-state to state-stack pop not implemented");
            }
        };
    }

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

    fn visit_call_chain_literal_expr_node_to_string(
        &mut self,
        _method_call_chain_expression_node: &CallChainLiteralExprNode,
        _output: &mut String,
    ) {
        panic!("TODO");
    }

    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) {
        self.visit_decl_stmts(&bool_test_true_branch_node.statements);
    }

    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&bool_test_else_branch_node.statements);
    }

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

    fn visit_string_match_test_match_branch_node(
        &mut self,
        string_match_test_match_branch_node: &StringMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&string_match_test_match_branch_node.statements);
    }

    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&string_match_test_else_branch_node.statements);
    }

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

    fn visit_number_match_test_match_branch_node(
        &mut self,
        number_match_test_match_branch_node: &NumberMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&number_match_test_match_branch_node.statements);
    }

    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) {
        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);
    }
}
