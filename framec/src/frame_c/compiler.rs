use crate::frame_c::config::FrameConfig;
use crate::frame_c::parser::*;
use crate::frame_c::scanner::*;
use crate::frame_c::symbol_table::*;
use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::cpp_visitor::CppVisitor;
use crate::frame_c::visitors::cs_visitor::CsVisitor;
use crate::frame_c::visitors::cs_visitor_for_bob::CsVisitorForBob;
use crate::frame_c::visitors::gdscript_3_2_visitor::GdScript32Visitor;
use crate::frame_c::visitors::java_8_visitor::Java8Visitor;
use crate::frame_c::visitors::javascript_visitor::JavaScriptVisitor;
use crate::frame_c::visitors::plantuml_visitor::PlantUmlVisitor;
use crate::frame_c::visitors::python_visitor::PythonVisitor;
use crate::frame_c::visitors::rust_visitor::RustVisitor;
use crate::frame_c::visitors::smcat_visitor::SmcatVisitor;
use exitcode::USAGE;
use std::fs;
use std::path::{Path, PathBuf};

/* --------------------------------------------------------------------- */

static IS_DEBUG: bool = false;
static FRAMEC_VERSION: &str = "emitted from framec_v0.7.0";

/* --------------------------------------------------------------------- */

pub struct Exe {}

impl Exe {
    /* --------------------------------------------------------------------- */

    pub fn new() -> Exe {
        Exe {}
    }

    pub fn debug_print(msg: &str) {
        if !IS_DEBUG {
            return;
        }

        println!("{}", msg);
    }

    /* --------------------------------------------------------------------- */

    pub fn run_file(
        &self,
        config_path: &Option<PathBuf>,
        input_path: &Path,
        output_format: String,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                Exe::debug_print(&(&content).to_string());
                self.run(config_path, content, output_format)
            }
            Err(err) => {
                let error_msg = format!("Error reading input file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }

    pub fn run(
        &self,
        config_path: &Option<PathBuf>,
        content: String,
        mut output_format: String,
    ) -> Result<String, RunError> {
        // NOTE!!! There is a bug w/ the CLion debugger when a variable (maybe just String type)
        // isn't initialized under some circumstances. Basically the debugger
        // stops debugging or doesn't step and it looks like it hangs. To avoid
        // this you have to initialize the variable, but the compiler then complains
        // about the unused assignment. This can be squelched with `#[allow(unused_assignments)]`
        // but I've reported it to JetBrains and want it fixed. So when you are
        // debugging here, just uncomment the next line and then comment it back
        // when checking in.
        // let mut output= String::new();

        let output;
        let scanner = Scanner::new(content);

        let (has_errors, errors, tokens) = scanner.scan_tokens();
        if has_errors {
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors);
            return Err(run_error);
        }

        for token in &tokens {
            Exe::debug_print(&format!("{:?}", token));
        }

        let mut arcanum = Arcanum::new();
        let mut comments = Vec::new();
        // NOTE: This block is to remove references to symbol_table and comments
        {
            let mut syntactic_parser = Parser::new(&tokens, &mut comments, true, arcanum);
            syntactic_parser.parse();
            if syntactic_parser.had_error() {
                let mut errors = "Terminating with errors.\n".to_string();
                errors.push_str(&syntactic_parser.get_errors());
                let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
                return Err(run_error);
            }
            arcanum = syntactic_parser.get_arcanum();
        }

        let mut comments2 = comments.clone();
        let mut semantic_parser = Parser::new(&tokens, &mut comments2, false, arcanum);
        let system_node = semantic_parser.parse();
        if semantic_parser.had_error() {
            let mut errors = "Terminating with errors.\n".to_string();
            errors.push_str(&semantic_parser.get_errors());
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
            return Err(run_error);
        }

        let generate_enter_args = semantic_parser.generate_enter_args;
        let generate_exit_args = semantic_parser.generate_exit_args;
        let generate_state_context = semantic_parser.generate_state_context;
        let generate_state_stack = semantic_parser.generate_state_stack;
        let generate_change_state = semantic_parser.generate_change_state;
        let generate_transition_state = semantic_parser.generate_transition_state;

        // check for local config.yaml if no path specified
        let mut local_config_path = config_path;
        let config_yaml = PathBuf::from("config.yaml");
        let some_config_yaml = Some(config_yaml.clone());
        if local_config_path.is_none() && config_yaml.exists() {
            local_config_path = &some_config_yaml;
        }

        // load configuration
        let config = match FrameConfig::load(local_config_path, &system_node) {
            Ok(cfg) => cfg,
            Err(err) => {
                let run_error = RunError::new(frame_exitcode::CONFIG_ERR, &err.to_string());
                return Err(run_error);
            }
        };

        match &system_node.attributes_opt {
            Some(attributes) => {
                if let Some(language) = attributes.get("language") {
                    output_format = language.value.clone();
                }
            }
            None => {}
        }
        if output_format == "javascript" {
            let mut visitor = JavaScriptVisitor::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "cpp" {
            let mut visitor = CppVisitor::new(
                semantic_parser.get_arcanum(),
                config,
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "c_sharp_bob" {
            let mut visitor = CsVisitorForBob::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "c_sharp" {
            let mut visitor = CsVisitor::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "gdscript" {
            let mut visitor = GdScript32Visitor::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "java_8" {
            let mut visitor = Java8Visitor::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "python_3" {
            let mut visitor = PythonVisitor::new(
                semantic_parser.get_arcanum(),
                generate_exit_args,
                generate_enter_args || generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "plantuml" {
            let (arcanum, system_hierarchy) = semantic_parser.get_all();
            let mut visitor = PlantUmlVisitor::new(
                arcanum,
                system_hierarchy,
                generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "rust" {
            let mut visitor = RustVisitor::new(
                FRAMEC_VERSION,
                config,
                semantic_parser.get_arcanum(),
                generate_enter_args,
                generate_exit_args,
                generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "smcat" {
            let mut visitor = SmcatVisitor::new(
                FRAMEC_VERSION,
                config,
                semantic_parser.get_system_hierarchy(),
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        // } else if output_format == "xstate" {
        //     let mut visitor = XStateVisitor::new(semantic_parser.get_arcanum()
        //                                        , generate_exit_args
        //                                        , generate_state_context
        //                                        , generate_state_stack
        //                                        , generate_change_state
        //                                        , generate_transition_state
        //                                        , FRAMEC_VERSION
        //                                        , comments);
        //     visitor.run(&system_node);
        //     return visitor.get_code();
        } else {
            let error_msg = &format!("Error - unrecognized output format {}.", output_format);
            let run_error = RunError::new(USAGE, error_msg);
            return Err(run_error);
        }

        Ok(output)

        // let mut graphviz_visitor = GraphVizVisitor::new(semantic_parser.get_arcanum(), comments);
        // graphviz_visitor.run(&system_node);
        // println!("{}", graphviz_visitor.code);
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::new()
    }
}
