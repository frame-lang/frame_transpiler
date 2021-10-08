use super::parser::*;
use super::scanner::*;
use super::symbol_table::*;
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
extern crate yaml_rust;
use self::yaml_rust::Yaml;
use std::fs;
use std::path::{Path, PathBuf};
use yaml_rust::YamlLoader;
//use crate::frame_c::visitors::xtate_visitor::XStateVisitor;

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

    /// Load a configuration from the file at the given path.
    pub fn load_config_file(&self, path: &Path) -> Result<Yaml, RunError> {
        match fs::read_to_string(path) {
            Ok(content) => self.parse_config_yaml(&content),
            Err(err) => {
                let error_msg = format!(
                    "Error reading config file {}: {}",
                    path.to_str().unwrap(),
                    err
                );
                Err(RunError::new(
                    frame_exitcode::MISSING_CONFIG_ERR,
                    &*error_msg,
                ))
            }
        }
    }

    /// Load the `default_config.yaml` file.
    pub fn load_default_config_file(&self) -> Result<Yaml, RunError> {
        let content = include_str!("default_config.yaml");
        self.parse_config_yaml(&content.to_string())
    }

    /// Write out the `default_config.yaml` file as a new `config.yaml` file in
    /// the current working directory.
    pub fn write_default_config_file(&self) -> Result<(), RunError> {
        let contents = include_str!("default_config.yaml");
        match fs::write("config.yaml", contents) {
            Ok(_) => Ok(()),
            Err(err) => {
                let error_msg = format!("Error writing config.yaml: {}", err);
                let run_error = RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*error_msg);
                Err(run_error)
            }
        }
    }

    /// Parse the contents of a configuration file.
    fn parse_config_yaml(&self, content: &String) -> Result<Yaml, RunError> {
        let yaml = YamlLoader::load_from_str(content.as_str());
        match yaml {
            Ok(yaml_vec) => Ok(yaml_vec[0].clone()),
            Err(err) => {
                let error_msg = format!("Error parsing default_config.yaml: {}", err);
                let run_error = RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*error_msg);
                Err(run_error)
            }
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn run_file(
        &self,
        local_config: &Option<PathBuf>,
        input_path: &Path,
        output_format: String,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                Exe::debug_print(&format!("{}", &content));
                self.run(local_config, content, output_format)
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
        local_config: &Option<PathBuf>,
        content: String,
        mut output_format: String,
    ) -> Result<String, RunError> {
        // load default config
        let default_config_yaml = self.load_default_config_file()?;

        // load local config, if provided
        // TODO: should instead pass an Option<&Yaml> into visitors
        let mut local_config_yaml = default_config_yaml.clone();
        if let Some(path) = local_config {
            match self.load_config_file(path) {
                Ok(yaml) => {
                    local_config_yaml = yaml;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

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
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors.clone());
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
                let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors.clone());
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
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors.clone());
            return Err(run_error);
        }

        let generate_enter_args = semantic_parser.generate_enter_args;
        let generate_exit_args = semantic_parser.generate_exit_args;
        let generate_state_context = semantic_parser.generate_state_context;
        let generate_state_stack = semantic_parser.generate_state_stack;
        let generate_change_state = semantic_parser.generate_change_state;
        let generate_transition_state = semantic_parser.generate_transition_state;

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
                &default_config_yaml, // TODO pass in local_config_yaml too
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
            // let x = (&semantic_parser).get_arcanum();
            // semantic_parser = semantic_parser.into_inner();
            // let y = (&semantic_parser).get_system_hierarchy();
            let (x, y) = semantic_parser.get_all();
            let mut visitor = PlantUmlVisitor::new(
                x,
                y,
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
            let attributes = &system_node.attributes_opt;
            let mut visitor = RustVisitor::new(
                semantic_parser.get_arcanum(),
                &default_config_yaml,
                &local_config_yaml,
                attributes,
                generate_enter_args,
                generate_exit_args,
                generate_state_context,
                generate_state_stack,
                generate_change_state,
                generate_transition_state,
                FRAMEC_VERSION,
                comments,
            );
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "smcat" {
            let (x, y) = semantic_parser.get_all();
            let mut visitor = SmcatVisitor::new(x, y, FRAMEC_VERSION, comments);
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
