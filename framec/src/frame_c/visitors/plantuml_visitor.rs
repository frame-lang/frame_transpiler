#![allow(non_snake_case)]

use super::super::ast::*;
use super::super::symbol_table::*;
// use super::super::symbol_table::SymbolType::*;
use super::super::visitors::*;
use super::super::scanner::{Token,TokenType};
use crate::frame_c::utils::{SystemHierarchy};
// use yaml_rust::{YamlLoader, Yaml};

pub struct PlantUmlVisitor {
    compiler_version:String,
    pub code:String,
    pub dent:usize,
    pub current_state_name_opt:Option<String>,
    current_event_ret_type:String,
    arcanium:Arcanum,
    symbol_config:SymbolConfig,
    // current_comment_idx:usize,
    first_event_handler:bool,
    system_name:String,
    first_state_name:String,
    generate_exit_args:bool,
    // generate_state_context:bool,
    // generate_state_stack:bool,
    //generate_change_state:bool,
 //   generate_transition_state:bool,
    states:String,
    transitions:String,
    system_hierarchy:SystemHierarchy,
    event_handler_msg:String,
}

impl PlantUmlVisitor {

    //* --------------------------------------------------------------------- *//

    pub fn new(   arcanium:Arcanum
                  , system_hierarchy:SystemHierarchy
                  , generate_exit_args:bool
                  , _generate_state_context:bool
                  , _generate_state_stack:bool
                  , _generate_change_state:bool
                  , _generate_transition_state:bool
                  , compiler_version:&str
                  , _comments:Vec<Token>) -> PlantUmlVisitor {

        // These closures are needed to do the same actions as add_code() and newline()
        // when inside a borrowed self reference as they modify self.
        // let mut add_code_cl = |target:&mut String, s:&String| target.push_str(s);
        // let mut newline_cl = |target:&mut String, s:&String, d:usize| {
        //     target.push_str(&*format!("\n{}",(0..d).map(|_| "\t").collect::<String>()));
        // };

        PlantUmlVisitor {
            compiler_version:compiler_version.to_string(),
            code:String::from(""),
            dent:0,
            current_state_name_opt:None,
            current_event_ret_type:String::new(),
            arcanium,
            symbol_config:SymbolConfig::new(),
            // current_comment_idx:0,
            first_event_handler:true,
            system_name:String::new(),
            first_state_name:String::new(),
            generate_exit_args,
            // generate_state_context,
            // generate_state_stack,
            // generate_change_state,
            // generate_transition_state,
            states:String::new(),
            transitions:String::new(),
            system_hierarchy,
            event_handler_msg:String::new(),
        }
    }

    //* --------------------------------------------------------------------- *//
    //
    // fn get_variable_type(&self,symbol_type:&SymbolType) -> String {
    //     let var_type = match &*symbol_type {
    //         DomainVariableSymbolT { domain_variable_symbol_rcref } => {
    //             match &domain_variable_symbol_rcref.borrow().var_type {
    //                 Some(x) => x.get_type_str(),
    //                 None => String::from("<?>"),
    //             }
    //         },
    //         StateParamSymbolT { state_param_symbol_rcref } => {
    //             match &state_param_symbol_rcref.borrow().param_type {
    //                 Some(x) => x.get_type_str(),
    //                 None => String::from("<?>"),
    //             }
    //         },
    //         StateVariableSymbolT { state_variable_symbol_rcref } => {
    //             match &state_variable_symbol_rcref.borrow().var_type {
    //                 Some(x) => x.get_type_str(),
    //                 None => String::from("<?>"),
    //             }                    },
    //         EventHandlerParamSymbolT { event_handler_param_symbol_rcref } => {
    //             match &event_handler_param_symbol_rcref.borrow().param_type {
    //                 Some(x) => x.get_type_str(),
    //                 None => String::from("<?>"),
    //             }
    //         },
    //         EventHandlerVariableSymbolT { event_handler_variable_symbol_rcref } => {
    //             match &event_handler_variable_symbol_rcref.borrow().var_type {
    //                 Some(x) => x.get_type_str(),
    //                 None => String::from("<?>"),
    //             }
    //         },
    //
    //         _ => panic!("TODO"),
    //     };
    //
    //     return var_type;
    // }


    //* --------------------------------------------------------------------- *//

    pub fn get_code(&self) -> String {
        self.code.clone()
    }


    //* --------------------------------------------------------------------- *//

    fn generate_states(&self, node_name:&String, is_system_node:bool, indent:usize, output:&mut String) {
 //       let state_name = &node.name;
        let mut actual_indent = indent;
        if !is_system_node{
  //          output.push_str(&*format!("\n{}",indent));
            actual_indent += 1;
            output.push_str(&format!("{}state {} {{\n",self.specifiy_dent(indent),node_name));
        }
        let node = self.system_hierarchy.get_node(node_name).unwrap();
        for child_node_name in &node.children {
            let child_node = self.system_hierarchy.get_node(&child_node_name).unwrap();
            self.generate_states(&child_node.name, false,actual_indent, output);
        }
        if !is_system_node{
            output.push_str(&format!("{}}}\n",self.specifiy_dent(indent)));
        }
    }

    // fn debug_print_states(&mut self, node_rc_ref:&Rc<RefCell<Node>>) {
    //     let state_name = node_rc_ref.borrow().name.clone();
    //     //println!(&format!("state {} \n", state_name));
    //     println!("state {} \n", state_name);
    //     for child_rc_ref in &node_rc_ref.borrow().children {
    //         let child_node = child_rc_ref.borrow();
    //         let child_state_name = child_rc_ref.borrow().name.clone();
    //         self.debug_print_states(child_rc_ref);
    //     }
    //     //      self.states.push_str(&format!("}}\n"));
    // }
    //* --------------------------------------------------------------------- *//

    fn format_variable_expr(&self, variable_node:&VariableNode) -> String {
        let code = String::new();

        match variable_node.scope {
            IdentifierDeclScope::DomainBlock => {
                // code.push_str(&format!("this.{}",variable_node.id_node.name.lexeme));
            },
            IdentifierDeclScope::StateParam => {
                // let var_node = variable_node;
                // let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                // let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                // let var_symbol = var_symbol_rcref.borrow();
                // let var_type = self.get_variable_type(&*var_symbol);
                //
                // code.push_str(&format!("({}) _pStateContext_.getStateArg(\"{}\")"
                //                        ,var_type
                //                        ,variable_node.id_node.name.lexeme));
            },
            IdentifierDeclScope::StateVar => {
                // let var_node = variable_node;
                // let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                // let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                // let var_symbol = var_symbol_rcref.borrow();
                // let var_type = self.get_variable_type(&*var_symbol);
                //
                // code.push_str(&format!("({}) _pStateContext_.getStateVar(\"{}\")"
                //                        ,var_type
                //                        ,variable_node.id_node.name.lexeme));

            }
            IdentifierDeclScope::EventHandlerParam => {
                // let var_node = variable_node;
                // let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                // let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                // let var_symbol = var_symbol_rcref.borrow();
                // let var_type = self.get_variable_type(&*var_symbol);
                //
                // code.push_str(&format!("({}) e.Parameters[\"{}\"]"
                //                        ,var_type
                //                        ,variable_node.id_node.name.lexeme));

            },
            IdentifierDeclScope::EventHandlerVar => {
                // code.push_str(&format!("{}",variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::None => {
                // TODO: Explore labeling Variables as "extern" scope
                // code.push_str(&format!("{}",variable_node.id_node.name.lexeme));
            },            // Actions?
            _ => panic!("Illegal scope."),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_parameter_list(&mut self,params:&Vec<ParameterNode>) {
        let mut separator = "";
        for param in params {
            self.add_code(&format!("{}", separator));
            let param_type: String = match &param.param_type_opt {
                Some(ret_type) => ret_type.get_type_str(),
                None => String::from("<?>"),
            };
            self.add_code(&format!("{} {}", param_type, param.param_name));
            separator = ",";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_action_name(&mut self,action_name:&String) -> String {
        return format!("{}_do",action_name)
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self,system_node:&SystemNode) {
        system_node.accept( self);
    }

    //* --------------------------------------------------------------------- *//

    fn add_code(&mut self, s:&str)  {
        self.code.push_str(&*format!("{}",s));
    }

    //* --------------------------------------------------------------------- *//

    fn newline(&mut self)  {
        self.code.push_str(&*format!("\n{}",self.dent()));
    }

    //* --------------------------------------------------------------------- *//

    fn newline_to_string(&mut self, output:&mut String)  {
        output.push_str(&*format!("\n{}",self.dent()));
    }

    //* --------------------------------------------------------------------- *//

    fn dent(&self) -> String {
        return (0..self.dent).map(|_| "    ").collect::<String>()
    }

    //* --------------------------------------------------------------------- *//

    fn specifiy_dent(&self,dent:usize) -> String {
        return (0..dent).map(|_| "    ").collect::<String>()
    }

    //* --------------------------------------------------------------------- *//

    fn indent(&mut self) {
        self.dent += 1;
    }

    //* --------------------------------------------------------------------- *//

    fn outdent(&mut self) {
        self.dent -= 1;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_decl_stmts(&mut self, decl_stmt_types:&Vec<DeclOrStmtType>) {
        for decl_stmt_t in decl_stmt_types.iter() {
            match decl_stmt_t {
                DeclOrStmtType::VarDeclT {..}
                => {
          //          let variable_decl_node = var_decl_t_rc_ref.borrow();
          //          variable_decl_node.accept(self);
                },
                DeclOrStmtType::StmtT {stmt_t} => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            match expr_stmt_t {
                                // ExprStmtType::ActionCallStmtT { action_call_stmt_node }
                                // => action_call_stmt_node.accept(self),                        // // TODO
                                // ExprStmtType::CallStmtT { call_stmt_node }
                                // => call_stmt_node.accept(self),
                                // ExprStmtType::CallChainLiteralStmtT { call_chain_literal_stmt_node }
                                // => call_chain_literal_stmt_node.accept(self),
                                // ExprStmtType::AssignmentStmtT { assignment_stmt_node }
                                // => assignment_stmt_node.accept(self),
                                // ExprStmtType::VariableStmtT { variable_stmt_node }
                                // => variable_stmt_node.accept(self),
                                _ => {}
                            }
                        },
                        StatementType::TransitionStmt { transition_statement } => {
                            transition_statement.accept(self);
                        }
                        StatementType::TestStmt { test_stmt_node } => {
                            test_stmt_node.accept(self);
                        },
                        StatementType::StateStackStmt {state_stack_operation_statement_node} => {
                            state_stack_operation_statement_node.accept(self);
                        },
                        StatementType::ChangeStateStmt {change_state_stmt} => {
                            change_state_stmt.accept(self);
                        },
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

    // fn generate_machinery(&mut self, system_node: &SystemNode) {
    //     self.newline();
    //     self.newline();
    //     self.add_code(&format!("//=========== Machinery and Mechanisms ===========//"));
    //     self.newline();
    //     if let Some(first_state) = system_node.get_first_state() {
    //         self.newline();
    //         self.add_code(&format!("private delegate void FrameState(FrameEvent e);"));
    //         self.newline();
    //         self.add_code(&format!("private FrameState _state_;"));
    //         self.newline();
    //         if self.generate_state_context {
    //             self.add_code(&format!("private StateContext _stateContext_;"));
    //         }
    //         self.newline();
    //         self.newline();
    //         if self.generate_transition_state {
    //             if self.generate_state_context {
    //                 if self.generate_exit_args {
    //                     self.add_code(&format!("private void _transition_(FrameState newState,FrameEventParams exitArgs, StateContext stateContext) {{"));
    //                 } else {
    //                     self.add_code(&format!("private void _transition_(FrameState newState, StateContext stateContext) {{"));
    //                 }
    //             } else {
    //                 if self.generate_exit_args {
    //                     self.add_code(&format!("private void _transition_(FrameState newState,FrameEventParams exitArgs) {{"));
    //                 } else {
    //                     self.add_code(&format!("private void _transition_(FrameState newState) {{"));
    //                 }
    //             }
    //             self.indent();
    //             self.newline();
    //             if self.generate_exit_args {
    //                 self.add_code(&format!("FrameEvent exitEvent = new FrameEvent(\"<\",exitArgs);"));
    //             } else {
    //                 self.add_code(&format!("FrameEvent exitEvent = new FrameEvent(\"<\",null);"));
    //             }
    //             self.newline();
    //             self.add_code(&format!("_state_(exitEvent);"));
    //             self.newline();
    //             self.add_code(&format!("_state_ = newState;"));
    //             self.newline();
    //             if self.generate_state_context {
    //                 self.add_code(&format!("_stateContext_ = stateContext;"));
    //                 self.newline();
    //                 self.add_code(&format!("FrameEvent enterEvent = new FrameEvent(\">\",_stateContext_.getEnterArgs());"));
    //                 self.newline();
    //             } else {
    //                 self.add_code(&format!("FrameEvent enterEvent = new FrameEvent(\">\",null);"));
    //                 self.newline();
    //             }
    //             self.add_code(&format!("_state_(enterEvent);"));
    //             self.outdent();
    //             self.newline();
    //             self.add_code(&format!("}}"));
    //         }
    //         if self.generate_state_stack {
    //             self.newline();
    //             self.newline();
    //             self.add_code(&format!("private Stack<StateContext> _stateStack_ = new Stack<StateContext>();"));
    //             self.newline();
    //             self.newline();
    //             self.add_code(&format!("private void _stateStack_push(StateContext stateContext) {{"));
    //             self.indent();
    //             self.newline();
    //             self.add_code(&format!("_stateStack_.Push(stateContext);"));
    //             self.outdent();
    //             self.newline();
    //             self.add_code(&format!("}}"));
    //             self.newline();
    //             self.newline();
    //             self.add_code(&format!("private StateContext _stateStack_pop() {{"));
    //             self.indent();
    //             self.newline();
    //             self.add_code(&format!("StateContext stateContext =  _stateStack_.back();"));
    //             self.newline();
    //             self.add_code(&format!("return _stateStack_.Pop();"));
    //             self.outdent();
    //             self.newline();
    //             self.add_code(&format!("}}"));
    //         }
    //         if self.generate_change_state {
    //             self.newline();
    //             self.newline();
    //             self.add_code(&format!("private void _changeState_(newState) {{"));
    //             self.indent();
    //             self.newline();
    //             self.add_code(&format!("_state_ = newState;"));
    //             self.outdent();
    //             self.newline();
    //             self.add_code(&format!("}}"));
    //         }
    //     }
    // }

    //* --------------------------------------------------------------------- *//

    fn generate_comment(&mut self,_line:usize) {

        // can't use self.newline() or self.add_code() due to double borrow.
//         let mut generated_comment = false;
//         while self.current_comment_idx < self.comments.len() &&
//             line >= self.comments[self.current_comment_idx].line {
//             let comment = &self.comments[self.current_comment_idx];
//             if comment.token_type == TokenType::SingleLineCommentTok {
//                 self.code.push_str(&*format!("  // {}",&comment.lexeme[3..]));
//                 self.code.push_str(&*format!("\n{}",(0..self.dent).map(|_| "    ").collect::<String>()));
//
//             } else {
//                 let len = &comment.lexeme.len() - 3;
//                 self.code.push_str(&*format!("/* {}",&comment.lexeme[3..len]));
//                 self.code.push_str(&*format!("*/"));
//
//             }
//
//             self.current_comment_idx += 1;
//             generated_comment = true;
//         }
//         if generated_comment {
// //            self.code.push_str(&*format!("\n{}",(0..self.dent).map(|_| "\t").collect::<String>()));
//         }
    }

    //* --------------------------------------------------------------------- *//

    // TODO
    fn generate_state_ref_change_state(&mut self, change_state_stmt_node: &ChangeStateStatementNode) {

        let target_state_name = match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef {state_context_node}
            => &state_context_node.state_ref_node.name,
            _ => panic!("TODO"),
        };

        self.newline();
        let mut current_state:String = "??".to_string();
        if let Some(state_name) = &self.current_state_name_opt {
            current_state = state_name.clone();
        }

        let label = match &change_state_stmt_node.label_opt {
            Some(label) => {
                let cleaned = str::replace(label, "|", "&#124;");
                format!(" : {}", cleaned.clone())
            },
            None => {
                format!(" : {}",self.event_handler_msg.clone())
            },
        };

        let transition_code = &format!("{} -[dashed]-> {}{}\n"
                                       ,current_state
                                       ,self.format_target_state_name(target_state_name)
                                       ,label);
        //       println!("{}", &transition_code);
        self.transitions.push_str(transition_code);
        // self.add_code(&format!("_changeState_({});", self.format_target_state_name(target_state_name)));
        // self.transitions.push_str(&format!("_changeState_({});", self.format_target_state_name(target_state_name)));

    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(&mut self, transition_statement: &TransitionStatementNode) {

        let target_state_name = match &transition_statement.target_state_context_t {
            StateContextType::StateRef {state_context_node} => {
                &state_context_node.state_ref_node.name
            },
            _ => panic!("TODO"),
        };

        let _state_ref_code = format!("{}",self.format_target_state_name(target_state_name));

        // self.newline();
        match &transition_statement.label_opt {
            Some(_label) => {
                // self.add_code(&format!("// {}", label));
                // self.newline();
            },
            None => {},
        }

        // if self.generate_state_context {
        //     self.add_code(&format!("StateContext stateContext = new StateContext({});", state_ref_code));
        //     self.newline();
        // }

        // -- Exit Arguments --

//         if let Some(exit_args) = &transition_statement.exit_args_opt {
//             if exit_args.exprs_t.len() > 0 {
//
//                 // Note - searching for event keyed with "State:<"
//                 // e.g. "S1:<"
//
//                 let mut msg: String = String::new();
//                 if let Some(state_name) = &self.current_state_name_opt {
//                     msg = state_name.clone();
//                 }
//                 msg.push_str(":");
//                 msg.push_str(&self.symbol_config.exit_msg_symbol);
//
//                 if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
//                     match &event_sym.borrow().params_opt {
//                         Some(event_params) => {
//                             if exit_args.exprs_t.len() != event_params.len() {
//                                 panic!("Fatal error: misaligned parameters to arguments.")
//                             }
//                             let mut param_symbols_it = event_params.iter();
//                             self.add_code("FrameEventParams exitArgs = new FrameEventParams();");
//                             self.newline();
//                             // Loop through the ARGUMENTS...
//                             for expr_t in &exit_args.exprs_t {
//                                 // ...and validate w/ the PARAMETERS
//                                 match param_symbols_it.next() {
//                                     Some(p) => {
//                                         let param_type = match &p.param_type {
//                                             Some(param_type) => param_type.clone(),
//                                             None => String::from("<?>"),
//                                         };
//                                         let mut expr = String::new();
//                                         expr_t.accept_to_string(self, &mut expr);
//                                         self.add_code(&format!("exitArgs[\"{}\"] = {};", p.name, expr));
//                                         self.newline();
//                                     },
//                                     None => panic!("Invalid number of arguments for \"{}\" event handler.", msg),
//                                 }
//                             }
//                         },
//                         None => panic!("Fatal error: misaligned parameters to arguments."),
//                     }
//                 } else {
//                     panic!("TODO");
//                 }
//             }
//         }
//
//         // -- Enter Arguments --
//
//         let enter_args_opt = match &transition_statement.target_state_context_t {
//             StateContextType::StateRef { state_context_node}
//             => &state_context_node.enter_args_opt,
//             StateContextType::StateStackPop {}
//             => &None,
//         };
//
//         if let Some(enter_args) = enter_args_opt {
//
//             // Note - searching for event keyed with "State:>"
//             // e.g. "S1:>"
//
//             let mut msg:String = String::from(target_state_name);
//             msg.push_str(":");
//             msg.push_str(&self.symbol_config.enter_msg_symbol);
//
//             if let Some(event_sym) = self.arcanium.get_event(&msg,&self.current_state_name_opt) {
//                 match &event_sym.borrow().params_opt {
//                     Some(event_params) => {
//                         if enter_args.exprs_t.len() != event_params.len() {
//                             panic!("Fatal error: misaligned parameters to arguments.")
//                         }
//                         let mut param_symbols_it =  event_params.iter();
//                         for expr_t in &enter_args.exprs_t {
//                             match param_symbols_it.next() {
//                                 Some(p) => {
//                                     let param_type = match &p.param_type {
//                                         Some(param_type) => param_type.clone(),
//                                         None => String::from("<?>"),
//                                     };
//                                     let mut expr = String::new();
//                                     expr_t.accept_to_string(self,&mut expr);
//                                     self.add_code(&format!("stateContext.addEnterArg(\"{}\",{});", p.name, expr));
//                                     self.newline();
//                                 },
//                                 None => panic!("Invalid number of arguments for \"{}\" event handler.",msg),
//                             }
//                         }
//                     },
//                     None => panic!("Invalid number of arguments for \"{}\" event handler.",msg),
//                 }
//             } else {
//                 panic!("TODO");
//             }
//         }
//
//         // -- State Arguments --
//
//         let target_state_args_opt = match &transition_statement.target_state_context_t {
//             StateContextType::StateRef { state_context_node}
//             => &state_context_node.state_ref_args_opt,
//             StateContextType::StateStackPop {}
//             => &Option::None,
//         };
// //
//         if let Some(state_args) = target_state_args_opt {
// //            let mut params_copy = Vec::new();
//             if let Some(state_sym) = self.arcanium.get_state(&target_state_name) {
//                 match &state_sym.borrow().params_opt {
//                     Some(event_params) => {
//                         let mut param_symbols_it = event_params.iter();
//                         // Loop through the ARGUMENTS...
//                         for expr_t in &state_args.exprs_t {
//                             // ...and validate w/ the PARAMETERS
//                             match param_symbols_it.next() {
//
//                                 Some(param_symbol_rcref) => {
//                                     let param_symbol = param_symbol_rcref.borrow();
//                                     let param_type = match &param_symbol.param_type {
//                                         Some(param_type) => param_type.clone(),
//                                         None => String::from("<?>"),
//                                     };
//                                     let mut expr = String::new();
//                                     expr_t.accept_to_string(self, &mut expr);
//                                     self.add_code(&format!("stateContext.addStateArg(\"{}\",{});", param_symbol.name, expr));
//                                     self.newline();
//                                 },
//                                 None => panic!("Invalid number of arguments for \"{}\" state parameters.", target_state_name),
//                             }
// //
//                         }
//                     },
//                     None => {}
//                 }
//             } else {
//                 panic!("TODO");
//             }
//         } // -- State Arguments --
//
//         // -- State Variables --
//
//         let target_state_rcref_opt = self.arcanium.get_state(&target_state_name);
//
//         match target_state_rcref_opt {
//             Some(q) => {
// //                target_state_vars = "stateVars".to_string();
//                 if let Some(state_symbol_rcref) = self.arcanium.get_state(&q.borrow().name) {
//                     let state_symbol = state_symbol_rcref.borrow();
//                     let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
//                     // generate local state variables
//                     if state_node.vars.is_some() {
// //                        let mut separator = "";
//                         for var_rcref in state_node.vars.as_ref().unwrap() {
//                             let var = var_rcref.borrow();
//                             let var_type = match &var.type_opt {
//                                 Some(var_type) => var_type.clone(),
//                                 None => String::from("<?>"),
//                             };
//                             let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
//                             let mut expr_code = String::new();
//                             expr_t.accept_to_string(self,&mut expr_code);
//                             self.add_code(&format!("stateContext.addStateVar(\"{}\",{});", var.name, expr_code));
//                             self.newline();
//                         }
//                     }
//                 }
//             },
//             None => {
// //                code = target_state_vars.clone();
//             },
//         }
//
//         if self.generate_state_context {
//             if self.generate_exit_args {
//                 self.add_code(&format!("_transition_({},exitArgs,stateContext);",self.format_target_state_name(target_state_name)));
//             } else {
//                 self.add_code(&format!("_transition_({},stateContext);",self.format_target_state_name(target_state_name)));
//             }
//         } else {
//             if self.generate_exit_args {
//                 self.add_code(&format!("_transition_({},exitArgs);",self.format_target_state_name(target_state_name)));
//             } else {
//                 self.add_code(&format!("_transition_({});",self.format_target_state_name(target_state_name)));
//             }
//         }
        let mut current_state:String = "??".to_string();
        if let Some(state_name) = &self.current_state_name_opt {
            current_state = state_name.clone();
        }


        let label = match &transition_statement.label_opt {
            Some(label) => {
                let cleaned = str::replace(label, "|", "&#124;");
                format!(" : {}", cleaned.clone())
            },
            None => {
                format!(" : {}",self.event_handler_msg.clone())
            },
        };

        let transition_code = &format!("{} --> {}{}\n"
                                       ,current_state
                                       ,self.format_target_state_name(target_state_name)
                                       ,label);
 //       println!("{}", &transition_code);
        self.transitions.push_str(transition_code);
    }

    //* --------------------------------------------------------------------- *//

    fn format_target_state_name(&self,state_name:&str) -> String {
        format!("{}",state_name)
    }

    //* --------------------------------------------------------------------- *//

    // NOTE!!: it is *currently* disallowed to send state or event arguments to a state stack pop target
    // So currently this method just sets any exitArgs and pops the context from the state stack.

    fn generate_state_stack_pop_transition(&mut self, transition_statement: &TransitionStatementNode) {

        self.newline();
        match &transition_statement.label_opt {
            Some(_label) => {
                // self.add_code(&format!("// {}", label));
                // self.newline();
            },
            None => {},
        }

        // -- Exit Arguments --

        if let Some(exit_args) = &transition_statement.exit_args_opt {
            if exit_args.exprs_t.len() > 0 {

                // Note - searching for event keyed with "State:<"
                // e.g. "S1:<"

                let mut msg: String = String::new();
                if let Some(state_name) = &self.current_state_name_opt {
                    msg = state_name.clone();
                }
                // msg.push_str(":");
                // msg.push_str(&self.symbol_config.exit_msg_symbol);

                if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
                    match &event_sym.borrow().params_opt {
                        Some(event_params) => {
                            if exit_args.exprs_t.len() != event_params.len() {
                                panic!("Fatal error: misaligned parameters to arguments.")
                            }
                            let mut param_symbols_it = event_params.iter();
                            // self.add_code("FrameEventParams exitArgs = new FrameEventParams();");
                            // self.newline();
                            // Loop through the ARGUMENTS...
                            for _expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let _param_type = match &p.param_type_opt {
                                            Some(param_type) => param_type.get_type_str(),
                                            None => String::from("<?>"),
                                        };
                                        // let mut expr = String::new();
                                        // expr_t.accept_to_string(self, &mut expr);
                                        // self.add_code(&format!("exitArgs[\"{}\"] = {};", p.name, expr));
                                        // self.newline();
                                    },
                                    None => panic!("Invalid number of arguments for \"{}\" event handler.", msg),
                                }
                            }
                        },
                        None => panic!("Fatal error: misaligned parameters to arguments."),
                    }
                } else {
                    panic!("TODO");
                }
            }
        }

        // self.add_code(&format!("StateContext stateContext = _stateStack_pop();"));
        // self.newline();
        
        let label = match &transition_statement.label_opt {
            Some(label) => {
                let cleaned = str::replace(label, "|", "&#124;");
                format!(" : {}", cleaned.clone())
            },
            None => {
                format!(" : {}",self.event_handler_msg.clone())
            },
        };
        self.transitions.push_str(&format!("{} --> [H*]{}\n",&self.current_state_name_opt.as_ref().unwrap(),label));

    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for PlantUmlVisitor {

    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) -> AstVisitorReturnType {
        self.system_name = system_node.name.clone();
        let _ = self.compiler_version.clone(); // hack to shut the compiler up
        // self.add_code(&format!("// {}",self.compiler_version));
        // self.newline();
        self.add_code(&format!("@startuml\n"));
        // self.indent();
        // self.newline();
//        self.add_code(&format!("public FrameController self;"));
//         self.newline();
//         self.newline();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match (&system_node).get_first_state() {
            Some(x) => {
                self.first_state_name = x.borrow().name.clone();
                self.transitions.push_str(&format!("[*] --> {}\n",self.first_state_name));
 //               self.has_states = true;
            },
            None => {},
        }

        // generate constructor
        // self.add_code(&format!("public {}Base() {{",system_node.name));
        // self.indent();
        // self.newline();
        // self.add_code(&format!("self = this;"));
        // self.newline();
        // self.add_code(&format!("_state_ = _s{}_;",self.first_state_name));
        // if self.generate_state_context {
        //     self.newline();
        //     self.add_code(&format!("_stateContext_ = new StateContext(_s{}_);", self.first_state_name));
        //     if has_states {
        //         if let Some(state_symbol_rcref) = self.arcanium.get_state(&self.first_state_name) {
        //             //   self.newline();
        //             let state_symbol = state_symbol_rcref.borrow();
        //             let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
        //             // generate local state variables
        //             if state_node.vars.is_some() {
        //                 for var_rcref in state_node.vars.as_ref().unwrap() {
        //                     let var = var_rcref.borrow();
        //                     let var_type = match &var.type_opt {
        //                         Some(var_type) => var_type.clone(),
        //                         None => String::from("<?>"),
        //                     };
        //                     let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
        //                     let mut expr_code = String::new();
        //                     expr_t.accept_to_string(self, &mut expr_code);
        //                     self.add_code(&format!("_stateContext_.addStateVar(\"{}\",{});", var.name, expr_code));
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // self.outdent();
        // self.newline();
        // self.add_code(&format!("}}"));
        // self.newline();
        // end of generate constructor

        // if let Some(interface_block_node) = &system_node.interface_block_node_opt {
        //     interface_block_node.accept(self);
        // }

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        // if let Some(actions_block_node) = &system_node.actions_block_node_opt {
        //     actions_block_node.accept(self);
        // }
        //
        // if let Some(domain_block_node) = &system_node.domain_block_node_opt {
        //     domain_block_node.accept(self);
        // }

//         if has_states {
// //            self.generate_machinery(system_node);
//         }

        // TODO: formatting
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        // self.outdent();
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        self.add_code(&self.states.clone());
        self.add_code(&self.transitions.clone());
        self.add_code("@enduml");
        // self.newline();


        AstVisitorReturnType::SystemNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_messages_enum(&mut self, _interface_block_node: &InterfaceBlockNode) -> AstVisitorReturnType {
        panic!("Error - visit_frame_messages_enum() only used in Rust.");

        // AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_parameters(&mut self, _interface_block_node: &InterfaceBlockNode) -> AstVisitorReturnType {
        panic!("visit_interface_parameters() not valid for target language.");

        // AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node(&mut self, _interface_method_call_expr_node:&InterfaceMethodCallExprNode) -> AstVisitorReturnType {


        // TODO: review this return as I think it is a nop.
        AstVisitorReturnType::InterfaceMethodCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(&mut self, _interface_method_call_expr_node:&InterfaceMethodCallExprNode, _output:&mut String) -> AstVisitorReturnType {


        // TODO: review this return as I think it is a nop.
        AstVisitorReturnType::InterfaceMethodCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_block_node(&mut self, interface_block_node: &InterfaceBlockNode) -> AstVisitorReturnType {
        self.newline();
        self.add_code("//===================== Interface Block ===================//");
        self.newline();

        for interface_method_node_rcref in &interface_block_node.interface_methods {
            let interface_method_node = interface_method_node_rcref.borrow();
            interface_method_node.accept(self);
        }
        AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_node(&mut self, interface_method_node: &InterfaceMethodNode) -> AstVisitorReturnType {

        self.newline();
        let return_type = match &interface_method_node.return_type_opt {
            Some(ret) => ret.get_type_str(),
            None => "void".to_string(),
        };

        // see if an alias exists.
        let method_name_or_alias: &String;

        match &interface_method_node.alias {
            Some(alias_message_node) => {
                method_name_or_alias = &alias_message_node.name;
            },
            None => {
                method_name_or_alias = &interface_method_node.name;
            }
        }

        self.add_code(&format!("public {} {}(",return_type, interface_method_node.name));

        match &interface_method_node.params {
            Some (params)
            =>  self.format_parameter_list(params).clone(),
            None => {},
        }

        self.add_code(") {");
        self.indent();
        let params_param_code;
        if interface_method_node.params.is_some() {
            params_param_code = String::from("parameters");
            self.newline();
            self.add_code("FrameEventParams parameters = new FrameEventParams();");
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        let pname = &param.param_name;
                        self.newline();
                        self.add_code(&format!("parameters[\"{}\"] = {};\n", pname, pname));
                    }
                },
                None => {}
            }
        } else {
            params_param_code = String::from("null");
        }

        self.newline();
        self.add_code(&format!("FrameEvent e = new FrameEvent(\"{}\",{});", method_name_or_alias,params_param_code));
        self.newline();
        self.add_code(&format!("_state_(e);"));

        match &interface_method_node.return_type_opt {
            Some(return_type) => {
                self.newline();
                self.add_code(&format!("return ({}) e.Return;",return_type.get_type_str()));
            },
            None => {}
        }

        self.outdent(); self.newline();
        self.add_code(&format!("}}"));
        self.newline();

        AstVisitorReturnType::InterfaceMethodNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) -> AstVisitorReturnType {
        // self.newline();
        // self.newline();
        // self.add_code("//===================== Machine Block ===================//");
        // self.newline();
        // self.newline();

        let mut output = String::new();
        let sys_name = self.system_name.clone();
        let _system_node = self.system_hierarchy.get_system_node().unwrap();
        self.generate_states(&sys_name, true,0, &mut output);
        self.states = output;

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }

        AstVisitorReturnType::MachineBlockNode {}
    }



    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, actions_block_node: &ActionsBlockNode) -> AstVisitorReturnType {
        self.newline();
        self.newline();
        self.add_code("//===================== Actions Block ===================//");
        self.newline();
        self.newline();

        for action_decl_node_rcref in &actions_block_node.actions {
            let action_decl_node = action_decl_node_rcref.borrow();
            action_decl_node.accept(self);
        }

        AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node_rust_trait(&mut self, _: &ActionsBlockNode) -> AstVisitorReturnType {
        panic!("Error - visit_action_node_rust_trait() not implemented.");

        // AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_node_rust_impl(&mut self, _: &ActionsBlockNode) -> AstVisitorReturnType {
        panic!("Error - visit_actions_node_rust_impl() not implemented.");

        // AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_block_node(&mut self, domain_block_node: &DomainBlockNode) -> AstVisitorReturnType {
        self.newline();
        self.newline();
        self.add_code("//===================== Domain Block ===================//");
        self.newline();
        self.newline();

        let mut newline = false;
        for variable_decl_node_rcref in &domain_block_node.member_variables {
            let variable_decl_node = variable_decl_node_rcref.borrow();

            if newline {
                self.newline();
            }
            variable_decl_node.accept(self);
            newline = true;
        }

        AstVisitorReturnType::DomainBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) -> AstVisitorReturnType {

        // self.generate_comment(state_node.line);
        self.current_state_name_opt = Some(state_node.name.clone());
        // self.newline();
        // self.newline();
        // self.states.push_str(&format!("state {} {{\n", state_node.name));
        // self.states.push_str(&format!("}}\n"));
        // self.indent();

 //       println!("current state = {}", &state_node.name);

        let _state_symbol = match self.arcanium.get_state(&state_node.name) {
            Some(state_symbol) => state_symbol,
            None => panic!("TODO"),
        };

        self.first_event_handler = true; // context for formatting

        if state_node.evt_handlers_rcref.len() > 0 {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }

        match &state_node.dispatch_opt {
            Some(_dispatch) => {
        //        dispatch.accept(self);
            },
            None => {},
        }

        // self.outdent();
        // self.newline();
        // self.add_code("}");

        self.current_state_name_opt = None;
        AstVisitorReturnType::StateNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) -> AstVisitorReturnType {
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
//         self.newline();
//         self.generate_comment(evt_handler_node.line);
// //        let mut generate_final_close_paren = true;
        if let MessageType::CustomMessage {message_node} = &evt_handler_node.msg_t {
            self.event_handler_msg = format!("&#124;{}&#124;",message_node.name).to_string();
        } else { // AnyMessage ( ||* )
            self.event_handler_msg = "&#124;&#124;*".to_string();
        }
//         self.generate_comment(evt_handler_node.line);
//
//         self.indent();
        if let MessageType::CustomMessage {message_node} = &evt_handler_node.msg_t {

            let (_msg,_,_) = EventSymbol::get_event_msg(&self.symbol_config, &Some(evt_handler_node.state_name.clone()), &message_node.name);

            // Note: this is a bit convoluted as we cant use self.add_code() inside the
            // if statements as it is a double borrow (sigh).

            let params_code: Vec<String> = Vec::new();

            // NOW add the code. Sheesh.
            for param_code in params_code {
                self.newline();
                self.add_code(&param_code);
            }
        }


        // Generate statements
        self.visit_decl_stmts(&evt_handler_node.statements);

        let terminator_node = &evt_handler_node.terminator_node;
        terminator_node.accept(self);
        // self.outdent();
        //
        // self.newline();
        // self.add_code(&format!("}}"));

        // this controls formatting here
        self.first_event_handler = false;
        self.current_event_ret_type = String::new();

        AstVisitorReturnType::EventHandlerNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(&mut self, _evt_handler_terminator_node: &TerminatorExpr) -> AstVisitorReturnType {
        // self.newline();
        // match &evt_handler_terminator_node.terminator_type {
        //     TerminatorType::Return => {
        //         match &evt_handler_terminator_node.return_expr_t_opt {
        //             Some(expr_t) => {
        //                 self.add_code(&format!("e.Return = "));
        //                 expr_t.accept(self);
        //                 self.newline();
        //                 self.add_code("return;");
        //                 self.newline();
        //             },
        //             None => self.add_code("return;"),
        //         }
        //
        //     },
        //     TerminatorType::Continue => self.add_code("break;"),
        // }

        AstVisitorReturnType::EventHandlerTerminatorNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, method_call_statement: &CallStmtNode) -> AstVisitorReturnType {
        self.newline();
        method_call_statement.call_expr_node.accept(self);
        self.add_code(&format!(";"));

        AstVisitorReturnType::CallStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) -> AstVisitorReturnType {

        if let Some(call_chain) = &method_call.call_chain {

            for callable in call_chain {
                callable.callable_accept(self);
                self.add_code(&format!("."));
            }
        }

        self.add_code(&format!("{}(", method_call.identifier.name.lexeme));

        method_call.call_expr_list.accept(self);

        self.add_code(&format!(")"));


        AstVisitorReturnType::CallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node_to_string(&mut self, method_call: &CallExprNode, output:&mut String) -> AstVisitorReturnType {

        if let Some(call_chain) = &method_call.call_chain {

            for callable in call_chain {
                callable.callable_accept(self);
                output.push_str(&format!("."));
            }
        }

        output.push_str(&format!("{}(", method_call.identifier.name.lexeme));

        method_call.call_expr_list.accept_to_string(self, output);

        output.push_str(&format!(")"));

        AstVisitorReturnType::CallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node(&mut self, _action_call: &ActionCallExprNode) -> AstVisitorReturnType {

     //   let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        // self.add_code(&format!("{}(", action_name));

     //   action_call.expr_list.accept(self);

        // self.add_code(&format!(")"));

        AstVisitorReturnType::ActionCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(&mut self, action_call: &ActionCallExprNode, output:&mut String) -> AstVisitorReturnType {

        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        output.push_str(&format!("{}(",action_name));

        action_call.call_expr_list.accept_to_string(self, output);

        output.push_str(&format!(")"));

        AstVisitorReturnType::ActionCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node(&mut self, call_expr_list: &CallExprListNode) -> AstVisitorReturnType {

        let mut separator = "";
        self.add_code(&format!("("));

        for expr in &call_expr_list.exprs_t {

            self.add_code(&format!("{}",separator));
            expr.accept(self);
            separator = ",";
        }

        self.add_code(&format!(")"));

        AstVisitorReturnType::CallExprListNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node_to_string(&mut self, call_expr_list: &CallExprListNode, output:&mut String) -> AstVisitorReturnType {

        let mut separator = "";
        output.push_str(&format!("("));

        for expr in &call_expr_list.exprs_t {

            output.push_str(&format!("{}",separator));
            expr.accept_to_string(self, output);
            separator = ",";
        }

        output.push_str(&format!(")"));

        AstVisitorReturnType::CallExprListNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) -> AstVisitorReturnType {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
        self.add_code(&format!(";"));

        AstVisitorReturnType::ActionCallStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(&mut self, transition_statement: &TransitionStatementNode) -> AstVisitorReturnType {

        match &transition_statement.target_state_context_t {
            StateContextType::StateRef {..}
            => self.generate_state_ref_transition(transition_statement),
            StateContextType::StateStackPop {}
            => self.generate_state_stack_pop_transition(transition_statement),
        };

        AstVisitorReturnType::CallStatementNode {}
    }


    //* --------------------------------------------------------------------- *//

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) -> AstVisitorReturnType {
        self.add_code(&format!("{}", state_ref.name));

        AstVisitorReturnType::StateRefNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_change_state_statement_node(&mut self, change_state_stmt_node:&ChangeStateStatementNode) -> AstVisitorReturnType {

        match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { ..}
            => self.generate_state_ref_change_state(change_state_stmt_node),
            StateContextType::StateStackPop {}
            => panic!("TODO - not implemented"),
        };

        AstVisitorReturnType::ChangeStateStmtNode {}
    }

    //* --------------------------------------------------------------------- *//

    // TODO: ??
    fn visit_parameter_node(&mut self, _parameter_node: &ParameterNode) -> AstVisitorReturnType {

        // self.add_code(&format!("{}",parameter_node.name));

        AstVisitorReturnType::ParameterNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) -> AstVisitorReturnType {
        self.newline();
        self.add_code(&format!("_s{}_(e);", dispatch_node.target_state_ref.name));
        self.generate_comment(dispatch_node.line);
        self.newline();

        AstVisitorReturnType::DispatchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_test_statement_node(&mut self, test_stmt_node: &TestStatementNode) -> AstVisitorReturnType {

        match &test_stmt_node.test_t {
            TestType::BoolTest {bool_test_node}  => {
                bool_test_node.accept(self);
            },
            TestType::StringMatchTest {string_match_test_node} => {
                string_match_test_node.accept(self);
            },
            TestType::NumberMatchTest {number_match_test_node} => {
                number_match_test_node.accept(self);
            },
        }

        AstVisitorReturnType::TestStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_node(&mut self, bool_test_node:&BoolTestNode) -> AstVisitorReturnType {

        // self.newline();
        for branch_node in &bool_test_node.conditional_branch_nodes {
            // if branch_node.is_negated {
            //     self.add_code(&format!("{}(!(",if_or_else_if));
            // } else {
            //     self.add_code(&format!("{}(",if_or_else_if));
            // }

            branch_node.expr_t.accept(self);

            // if branch_node.is_negated {
            //     self.add_code(&format!(")"));
            // }
            // self.add_code(&format!(") {{"));
            // self.indent();

            branch_node.accept(self);

            // self.outdent(); self.newline();
            // self.add_code(&format!("}}"));
            //
            // if_or_else_if = " else if ";
        }

        // (':' bool_test_else_branch)?
        if let Some(bool_test_else_branch_node) = &bool_test_node.else_branch_node_opt {
            bool_test_else_branch_node.accept(self);
        }

        AstVisitorReturnType::BoolTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_statement_node(&mut self, method_call_chain_literal_stmt_node:&CallChainLiteralStmtNode) -> AstVisitorReturnType {

        self.newline();
        method_call_chain_literal_stmt_node.call_chain_literal_expr_node.accept(self);
        self.add_code(&format!(";"));
        AstVisitorReturnType::CallChainLiteralStmtNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node(&mut self, _method_call_chain_expression_node: &CallChainLiteralExprNode) -> AstVisitorReturnType {
        // TODO: maybe put this in an AST node

        // let mut separator = "";
        //
        // for node in &method_call_chain_expression_node.call_chain {
        //     self.add_code(&format!("{}",separator));
        //     match &node {
        //         CallChainLiteralNodeType::IdentifierNodeT { id_node }=> {
        //             self.add_code(&format!("{}",id_node.name.lexeme));
        //         },
        //         CallChainLiteralNodeType::CallT {call}=> {
        //             call.accept(self);
        //         },
        //     }
        //     separator = ".";
        // }

        AstVisitorReturnType::CallChainLiteralExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node_to_string(&mut self, _method_call_chain_expression_node:&CallChainLiteralExprNode, _output:&mut String) -> AstVisitorReturnType {
        panic!("TODO");
        // AstVisitorReturnType::CallChainLiteralExprNode {}
    }


    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_conditional_branch_node(&mut self, bool_test_true_branch_node:&BoolTestConditionalBranchNode) -> AstVisitorReturnType {

        self.visit_decl_stmts(&bool_test_true_branch_node.statements);

        // match &bool_test_true_branch_node.branch_terminator_expr_opt {
        //     Some(branch_terminator_expr) => {
        //         // self.newline();
        //         match &branch_terminator_expr.terminator_type {
        //             _Return => {
        //                 match &branch_terminator_expr.return_expr_t_opt {
        //                     Some(expr_t) => {
        //                         // self.add_code(&format!("e.Return = "));
        //                         expr_t.accept(self);
        //                         // self.add_code(";");
        //                         // self.newline();
        //                         // self.add_code("return;");
        //                     },
        //                     None => {
        //                         // self.add_code("return;")
        //                     },
        //                 }
        //             }
        //             _Continue => {
        //                 // self.add_code("break;");
        //             }
        //         }
        //     }
        //     None => {}
        // }

        AstVisitorReturnType::BoolTestConditionalBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_else_branch_node(&mut self, bool_test_else_branch_node:&BoolTestElseBranchNode) -> AstVisitorReturnType {

        // self.add_code(&format!(" else {{"));
        // self.indent();

        self.visit_decl_stmts(&bool_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        // match &bool_test_else_branch_node.branch_terminator_expr_opt {
        //     Some(branch_terminator_expr) => {
        //         // self.newline();
        //         match &bool_test_else_branch_node {
        //             Return => {
        //                 match &branch_terminator_expr.return_expr_t_opt {
        //                     Some(expr_t) => {
        //                         // self.add_code(&format!("e.Return = ",));
        //                         expr_t.accept(self);
        //                         // self.add_code(";");
        //                         // self.newline();
        //                         // self.add_code("return;");
        //                     },
        //                     None => {}
        //                     // self.add_code("return;"),
        //                 }
        //             }
        //             Continue => {
        //                 // self.add_code("break;");
        //             }
        //         }
        //     }
        //     None => {}
        // }

        // self.outdent();
        // self.newline();
        // self.add_code(&format!("}}"));

        AstVisitorReturnType::BoolTestElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_node(&mut self, string_match_test_node:&StringMatchTestNode) -> AstVisitorReturnType {

        // let mut if_or_else_if = "if";
        //
        // self.newline();
        for match_branch_node in &string_match_test_node.match_branch_nodes {
            // self.add_code(&format!("{} (", if_or_else_if));
            // TODO: use string_match_test_node.expr_t.accept(self) ?
            match &string_match_test_node.expr_t {
                ExprType::CallExprT { call_expr_node: method_call_expr_node }
                => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT { action_call_expr_node }
                => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT { call_chain_expr_node }
                => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node }
                => id_node.accept(self),

                _ => panic!("TODO"),
            }

            // TODO: use accept
            // self.add_code(&format!(" == \""));
            // match_branch_node.string_match_pattern_node.accept(self);
            // self.add_code(&format!("\") {{"));

            // let mut first_match = true;
            // for match_string in &match_branch_node.string_match_pattern_node.match_pattern_strings {
            //     if first_match {
            //         // self.add_code(&format!(" == \"{}\")",match_string));
            //         first_match = false;
            //     } else {
            //         // self.add_code(&format!(" || ("));
            //         match &string_match_test_node.expr_t {
            //             ExprType::CallExprT { call_expr_node: method_call_expr_node }
            //             => method_call_expr_node.accept(self),
            //             ExprType::ActionCallExprT { action_call_expr_node }
            //             => action_call_expr_node.accept(self),
            //             ExprType::CallChainLiteralExprT { call_chain_expr_node }
            //             => call_chain_expr_node.accept(self),
            //             ExprType::VariableExprT { var_node: id_node }
            //             => id_node.accept(self),
            //             _ => panic!("TODO"),
            //         }
            //         // self.add_code(&format!(" == \"{}\")",match_string));
            //     }
            // }
            // self.add_code(&format!(" {{"));
            // self.indent();

            match_branch_node.accept(self);

            // self.outdent(); self.newline();
            // self.add_code(&format!("}}"));
            //
            // if_or_else_if = " else if";
        }

        // (':' string_test_else_branch)?
        if let Some(string_match_else_branch_node) = &string_match_test_node.else_branch_node_opt {
            string_match_else_branch_node.accept(self);
        }

        AstVisitorReturnType::StringMatchTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_match_branch_node(&mut self, string_match_test_match_branch_node:&StringMatchTestMatchBranchNode) -> AstVisitorReturnType {

        self.visit_decl_stmts(&string_match_test_match_branch_node.statements);

        // match &string_match_test_match_branch_node.branch_terminator_expr_opt {
        //     Some(branch_terminator_expr) => {
        //         // self.newline();
        //         match &branch_terminator_expr.terminator_type {
        //             Return => {
        //                 match &branch_terminator_expr.return_expr_t_opt {
        //                     Some(expr_t) => {
        //                         // self.add_code(&format!("e.Return = "));
        //                         expr_t.accept(self);
        //                         // self.add_code(";");
        //                         // self.newline();
        //                         // self.add_code("return;");
        //                     },
        //                     None => {
        //
        //                     }
        //                         // self.add_code("return;"),
        //                 }
        //             }
        //             Continue => {
        //                 // self.add_code("break;");
        //
        //             }
        //         }
        //     }
        //     None => {}
        // }

        AstVisitorReturnType::StringMatchTestMatchBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_else_branch_node(&mut self, string_match_test_else_branch_node:&StringMatchTestElseBranchNode) -> AstVisitorReturnType {

        // self.add_code(&format!(" else {{"));
        // self.indent();

        self.visit_decl_stmts(&string_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        // match &string_match_test_else_branch_node.branch_terminator_expr_opt {
        //     Some(branch_terminator_expr) => {
        //         // self.newline();
        //         match &string_match_test_else_branch_node {
        //             Return => {
        //                 match &branch_terminator_expr.return_expr_t_opt {
        //                     Some(expr_t) => {
        //                         // self.add_code(&format!("e.Return = ",));
        //                         expr_t.accept(self);
        //                         // self.add_code(";");
        //                         // self.newline();
        //                         // self.add_code("return;");
        //                     },
        //                     None => {
        //                         // self.add_code("return;")
        //                     },
        //                 }
        //             }
        //             Continue => {
        //                 // self.add_code("break;");
        //
        //             }
        //         }
        //     }
        //     None => {}
        // }

        // self.outdent();
        // self.newline();
        // self.add_code(&format!("}}"));

        AstVisitorReturnType::StringMatchElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_pattern_node(&mut self, _string_match_test_else_branch_node:&StringMatchTestPatternNode) -> AstVisitorReturnType {

        // TODO
        panic!("todo");
//        AstVisitorReturnType::StringMatchTestPatternNode {}
    }

    //-----------------------------------------------------//

    fn visit_number_match_test_node(&mut self, number_match_test_node:&NumberMatchTestNode) -> AstVisitorReturnType {

        // let mut if_or_else_if = "if";

        self.newline();
        for match_branch_node in &number_match_test_node.match_branch_nodes {
            // self.add_code(&format!("{} (", if_or_else_if));
            match &number_match_test_node.expr_t {
                ExprType::CallExprT { call_expr_node: method_call_expr_node }
                => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT { action_call_expr_node }
                => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT { call_chain_expr_node }
                => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node }
                => id_node.accept(self),
                _ => panic!("TODO"),
            }

            let mut first_match = true;
            for _match_number in &match_branch_node.number_match_pattern_nodes {
                if first_match {
            //        self.add_code(&format!(" == {})",match_number.match_pattern_number));
                    first_match = false;
                } else {
                    // self.add_code(&format!(" || ("));
                    match &number_match_test_node.expr_t {
                        ExprType::CallExprT { call_expr_node: method_call_expr_node }
                        => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT { action_call_expr_node }
                        => action_call_expr_node.accept(self),
                        ExprType::CallChainLiteralExprT { call_chain_expr_node }
                        => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node }
                        => id_node.accept(self),
                        _ => panic!("TODO"),
                    }
            //        self.add_code(&format!(" == {})",match_number.match_pattern_number));
                }
            }

            // self.add_code(&format!(") {{"));
            // self.indent();

            match_branch_node.accept(self);

            // self.outdent(); self.newline();
            // self.add_code(&format!("}}"));

            //           self.indent();

            // if_or_else_if = " else if";
        }

        // (':' number_test_else_branch)?
        if let Some(number_match_else_branch_node) = &number_match_test_node.else_branch_node_opt {
            number_match_else_branch_node.accept(self);
        }


        AstVisitorReturnType::NumberMatchTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_match_branch_node(&mut self, number_match_test_match_branch_node:&NumberMatchTestMatchBranchNode) -> AstVisitorReturnType {

        self.visit_decl_stmts(&number_match_test_match_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        // match &number_match_test_match_branch_node.branch_terminator_expr_opt {
        //     Some(branch_terminator_expr) => {
        //         self.newline();
        //         match number_match_test_match_branch_node {
        //             Return => {
        //                 match &branch_terminator_expr.return_expr_t_opt {
        //                     Some(expr_t) => {
        //                         // self.add_code(&format!("e.Return = "));
        //                         expr_t.accept(self);
        //                         // self.add_code(";");
        //                         // self.newline();
        //                         self.add_code("return;");
        //                     },
        //                     None => {
        //                         // self.add_code("return;")
        //                     },
        //                 }
        //             }
        //             Continue => {
        //                 // self.add_code("break;");
        //
        //             }
        //         }
        //     }
        //     None => {}
        // }

        AstVisitorReturnType::NumberMatchTestMatchBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_else_branch_node(&mut self, number_match_test_else_branch_node:&NumberMatchTestElseBranchNode) -> AstVisitorReturnType {

        // self.add_code(&format!(" else {{"));
        // self.indent();

        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
    //     match &number_match_test_else_branch_node.branch_terminator_expr_opt {
    //         Some(branch_terminator_expr) => {
    //             self.newline();
    //             match number_match_test_else_branch_node {
    //                 Return => {
    //                     match &branch_terminator_expr.return_expr_t_opt {
    //                         Some(expr_t) => {
    //                             // self.add_code(&format!("e.Return = "));
    //                             expr_t.accept(self);
    //                             // self.add_code(";");
    //                             // self.newline();
    //                             // self.add_code("return;");
    //                         },
    //                         None => {
    //                             // self.add_code("return;")
    //                         },
    //                     }
    //                 }
    //                 Continue => {
    //                     // self.add_code("break;");
    //
    //                 }
    //             }
    //         }
    //         None => {}
    //     }

        // self.outdent();
        // self.newline();
        // self.add_code(&format!("}}"));

        AstVisitorReturnType::NumberMatchElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_pattern_node(&mut self, match_pattern_node:&NumberMatchTestPatternNode) -> AstVisitorReturnType {
        self.add_code(&format!("{}", match_pattern_node.match_pattern_number));

        AstVisitorReturnType::NumberMatchTestPatternNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node(&mut self, expr_list: &ExprListNode) -> AstVisitorReturnType {

        for expr in &expr_list.exprs_t {

            // self.add_code(&format!("{}",separator));
            expr.accept(self);
            // separator = ",";
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(&mut self, expr_list: &ExprListNode, output:&mut String) -> AstVisitorReturnType {

//        self.add_code(&format!("{}(e);\n",dispatch_node.target_state_ref.name));

        let mut separator = "";
        for expr in &expr_list.exprs_t {

            output.push_str(&format!("{}",separator));
            expr.accept_to_string(self, output);
            separator = ",";
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, literal_expression_node: &LiteralExprNode) -> AstVisitorReturnType {

        match &literal_expression_node.token_t {
            TokenType::NumberTok
            => self.add_code(&format!("{}", literal_expression_node.value)),
            TokenType::SuperStringTok
            => self.add_code(&format!("{}", literal_expression_node.value)),
            TokenType::StringTok
            => self.add_code(&format!("\"{}\"", literal_expression_node.value)),
            TokenType::TrueTok
            => self.add_code("true"),
            TokenType::FalseTok
            => self.add_code("false"),
            TokenType::NullTok
            => self.add_code("null"),
            TokenType::NilTok
            => self.add_code("null"),
            _ => panic!("TODO"),
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node_to_string(&mut self, literal_expression_node: &LiteralExprNode, output:&mut String) -> AstVisitorReturnType {

        // TODO: make a focused enum or the literals
        match &literal_expression_node.token_t {
            TokenType::NumberTok => {
                output.push_str(&format!("{}", literal_expression_node.value))
            },
            TokenType::StringTok => {
                output.push_str(&format!("\"{}\"", literal_expression_node.value));
            },
            TokenType::TrueTok => {
                output.push_str("true");
            },
            TokenType::FalseTok => {
                output.push_str("false");
            },
            TokenType::NilTok => {
                output.push_str("null");
            },
            TokenType::NullTok => {
                output.push_str("null");
            },
            _ => panic!("TODO"),
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node(&mut self, _identifier_node: &IdentifierNode) -> AstVisitorReturnType {

        panic!("Unexpected use of identifier.");

        // AstVisitorReturnType::IdentifierNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node_to_string(&mut self, _identifier_node: &IdentifierNode, _output:&mut String) -> AstVisitorReturnType {

        panic!("Unexpected use of identifier.");
        // AstVisitorReturnType::IdentifierNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node(&mut self, _state_stack_operation_node:&StateStackOperationNode) -> AstVisitorReturnType {

//        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateStackOperationNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node_to_string(&mut self, _state_stack_operation_node:&StateStackOperationNode, _output:&mut String) -> AstVisitorReturnType {

//        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateStackOperationNode {}
    }

    //* --------------------------------------------------------------------- *//
//
    fn visit_state_stack_operation_statement_node(&mut self, state_stack_op_statement_node:&StateStackOperationStatementNode) -> AstVisitorReturnType {

//        self.add_code(&format!("{}",identifier_node.name.lexeme));

        //       panic!("TODO: how is this used?");

        match state_stack_op_statement_node.state_stack_operation_node.operation_t {
            StateStackOperationType::Push => {
                // self.newline();
                // self.add_code(&format!("_stateStack_push_(_state_context_);"));
            },
            StateStackOperationType::Pop => {
                // self.add_code(&format!("let stateContext = _stateStack_pop_()"));
            }
        }
        AstVisitorReturnType::StateStackOperationStatementNode {}
    }
    //* --------------------------------------------------------------------- *//

    fn visit_state_context_node(&mut self, _state_context_node:&StateContextNode) -> AstVisitorReturnType {

        // TODO
//        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateContextNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_event_part(&mut self, frame_event_part:&FrameEventPart) -> AstVisitorReturnType {

        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event { is_reference: _is_reference } => self.add_code(&format!("e")),
            FrameEventPart::Message { is_reference: _is_reference }=> self.add_code(&format!("e._message")),
            FrameEventPart::Param { param_tok, is_reference: _is_reference } => self.add_code(&format!("e._params[\"{}\"]",param_tok.lexeme)),
            FrameEventPart::Return { is_reference: _is_reference } => self.add_code(&format!("e._return")),
        }

        AstVisitorReturnType::FrameEventExprType {}
    }

    //* --------------------------------------------------------------------- *//

    // TODO: this is not the right framemessage codegen
    fn visit_frame_event_part_to_string(&mut self, frame_event_part:&FrameEventPart, output:&mut String) -> AstVisitorReturnType {

        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event { is_reference: _is_reference } => output.push_str("e"),
            FrameEventPart::Message { is_reference: _is_reference } => output.push_str("e._message"),
            FrameEventPart::Param {param_tok, is_reference: _is_reference} => output.push_str(&format!("e._params[\"{}\"]",param_tok.lexeme)),
            FrameEventPart::Return { is_reference: _is_reference } => output.push_str("e._return"),
        }

        AstVisitorReturnType::FrameEventExprType {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_decl_node(&mut self, action_decl_node: &ActionNode) -> AstVisitorReturnType {

        self.newline();
        let action_ret_type:String = match &action_decl_node.type_opt {
            Some(ret_type) => ret_type.get_type_str(),
            None => String::from("void"),
        };

        let action_name = self.format_action_name(&action_decl_node.name);
        self.add_code(&format!("virtual {} {}(",action_ret_type, action_name));

        match &action_decl_node.params {
            Some (params)
            =>  self.format_parameter_list(params).clone(),
            None => {},
        }

        self.add_code(&format!(") {{}}"));

        AstVisitorReturnType::ActionDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, _action_decl_node: &ActionNode) -> AstVisitorReturnType {
        panic!("visit_action_impl_node() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) -> AstVisitorReturnType {

        self.visit_variable_decl_node(variable_decl_node);

        AstVisitorReturnType::VariableDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) -> AstVisitorReturnType {

        let var_type = match &variable_decl_node.type_opt {
            Some(x) => x.get_type_str(),
            None => String::from("<?>"),
        };
        let var_name =  &variable_decl_node.name;
        let var_init_expr = &variable_decl_node.initializer_expr_t_opt.as_ref().unwrap();
        self.newline();
        let mut code = String::new();
        var_init_expr.accept_to_string(self, &mut code);
        self.add_code( &format!("{} {} = {};",var_type,var_name, code));

        AstVisitorReturnType::VariableDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node(&mut self, variable_node: &VariableNode) -> AstVisitorReturnType {
        let code = self.format_variable_expr(variable_node);
        self.add_code(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node_to_string(&mut self, variable_node: &VariableNode, output:&mut String) -> AstVisitorReturnType {
        let code = self.format_variable_expr(variable_node);
        output.push_str(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_stmt_node(&mut self, variable_stmt_node: &VariableStmtNode) -> AstVisitorReturnType {
        // TODO: what is this line about?
        self.generate_comment(variable_stmt_node.get_line());
        self.newline();
        let code = self.format_variable_expr(&variable_stmt_node.var_node);
        self.add_code(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }


    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node(&mut self, assignment_expr_node: &AssignmentExprNode) -> AstVisitorReturnType {

        self.generate_comment(assignment_expr_node.line);
        self.newline();
        assignment_expr_node.l_value_box.accept(self);
        self.add_code(" = ");
        assignment_expr_node.r_value_box.accept(self);
        self.add_code(";");

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(&mut self, assignment_expr_node: &AssignmentExprNode, output:&mut String) -> AstVisitorReturnType {

        self.generate_comment(assignment_expr_node.line);
        self.newline();
        self.newline_to_string(output);
        assignment_expr_node.l_value_box.accept_to_string(self, output);
        output.push_str(" = ");
        assignment_expr_node.r_value_box.accept_to_string(self, output);
        output.push_str(";");

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(&mut self, assignment_stmt_node: &AssignmentStmtNode) -> AstVisitorReturnType {

        self.generate_comment(assignment_stmt_node.get_line());
        assignment_stmt_node.assignment_expr_node.accept(self);

        AstVisitorReturnType::AssignmentExprNode {}
    }


    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node(&mut self, _unary_expr_node: &UnaryExprNode) -> AstVisitorReturnType {

        AstVisitorReturnType::UnaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node_to_string(&mut self, _unary_expr_node: &UnaryExprNode, _output:&mut String) -> AstVisitorReturnType {

        AstVisitorReturnType::UnaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node(&mut self, _binary_expr_node: &BinaryExprNode) -> AstVisitorReturnType {


        AstVisitorReturnType::BinaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node_to_string(&mut self, _binary_expr_node: &BinaryExprNode, _output:&mut String) -> AstVisitorReturnType {


        AstVisitorReturnType::BinaryExprNode {}
    }


    //* --------------------------------------------------------------------- *//

    fn visit_operator_type(&mut self, _operator_type: &OperatorType) -> AstVisitorReturnType {


        AstVisitorReturnType::BinaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type_to_string(&mut self, _operator_type: &OperatorType, _output:&mut String) -> AstVisitorReturnType {


        AstVisitorReturnType::BinaryExprNode {}
    }
}



