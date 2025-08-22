use crate::frame_c::config::FrameConfig;
use crate::frame_c::parser::*;
use crate::frame_c::scanner::*;
use crate::frame_c::symbol_table::*;
use crate::frame_c::utils::{frame_exitcode, RunError};
use crate::frame_c::visitors::python_visitor::PythonVisitor;
use crate::frame_c::visitors::graphviz_visitor::GraphVizVisitor;

use exitcode::USAGE;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::Read;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::rc::Rc;

// Re-export this enum here since it's part of the interface for the run functions. The definition
// lives with visitors since adding a new visitor requires extending the enum and its trait impls.
use crate::frame_c::ast::{AttributeNode, FrameModule};
pub use crate::frame_c::visitors::TargetLanguage;
use std::convert::TryFrom;

/* --------------------------------------------------------------------- */

static IS_DEBUG: bool = false;
static FRAMEC_VERSION: &str = "Emitted from framec_v0.30.0";

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

    /// Run the Frame compiler on a Frame specification loaded from a file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Optional path to a configuration YAML file.
    ///
    /// * `input_path` - Path to the file containing the Frame specification.
    ///
    /// * `target_language` - The target language to compile the specification to. This may be
    ///   `None` if the `language` attribute is defined in the specification itself.
    pub fn run_file(
        &self,
        config_path: &Option<PathBuf>,
        input_path: &Path,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        match fs::read_to_string(input_path) {
            Ok(content) => {
                Exe::debug_print(&content);
                self.run(config_path, input_path.to_str(), content, target_language)
            }
            Err(err) => {
                let error_msg = format!("Error reading input file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn run_stdin(
        &self,
        config_path: &Option<PathBuf>,
        target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        match stdin.read_to_string(&mut buffer) {
            Ok(_size) => {
                Exe::debug_print(&buffer);
                self.run(config_path, None, buffer, target_language)
            }
            Err(err) => {
                let error_msg = format!("Error reading input file: {}", err);
                let run_error = RunError::new(exitcode::NOINPUT, &*error_msg);
                Err(run_error)
            }
        }
    }

    /* --------------------------------------------------------------------- */

    /// Run the Frame compiler on a Frame specification passed as a `String`.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Optional path to a configuration YAML file.
    ///
    /// * `input_path_str` - Path to the file containing the Frame specification, as a `&str`.
    ///   This value is used for metadata in some backends, but the file path is not verified or
    ///   loaded. This argument may be `None` if the Frame specification does not exist on the file
    ///   system, for example, if it was obtained from standard input or the online framepiler.
    ///
    /// * `content` - The Frame specification.
    ///
    /// * `target_language` - The target language to compile the specification to. This may be
    ///   `None` if the `language` attribute is defined in the specification itself.

    pub fn run(
        &self,
        config_path: &Option<PathBuf>,
        _input_path_str: Option<&str>,
        content: String,
        mut target_language: Option<TargetLanguage>,
    ) -> Result<String, RunError> {
        // NOTE!!! There is a bug w/ the CLion debugger when a variable (maybe just String type)
        // isn't initialized under some circumstances. Basically the debugger
        // stops debugging or doesn't step and it looks like it hangs. To avoid
        // this you have to initialize the variable, but the compiler then complains
        // about the unused assignment. This can be squelched with `#[allow(unused_assignments)]`
        // but I've reported it to JetBrains and want it fixed. So when you are
        // debugging here, just uncomment the next line and then comment it back
        // when checking in.

        let mut hasher = Sha256::new();
        hasher.update(&content);
        // let sha256 = &format!("{:x}", hasher.finalize());

        let output;
        //        let mut output= String::new(); ^^^^ See above! ^^^^

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

            panic::set_hook(Box::new(|_info| {
                // prevent std output from panics.
            }));
            // catch and suppress panics
            let _result = panic::catch_unwind(AssertUnwindSafe(|| {
                syntactic_parser.parse();
            }));

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

        // TODO: this doesn't capture any panics like syntactic_parser above.
        // Need to figure how to implement.
        let frame_module = semantic_parser.parse();
        let system_node = frame_module.get_primary_system();
        if semantic_parser.had_error() {
            let mut errors = "Terminating with errors.\n".to_string();
            errors.push_str(&semantic_parser.get_errors());
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &errors);
            return Err(run_error);
        }

        // let generate_enter_args = semantic_parser.generate_enter_args;
        // let generate_exit_args = semantic_parser.generate_exit_args;
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

        // check for language attribute override in spec specifying target language

        match &system_node.system_attributes_opt {
            Some(attributes) => {
                if let Some(attr_node) = attributes.get("language") {
                    match attr_node {
                        AttributeNode::MetaNameValueStr { attr } => {
                            if let Ok(result) = TargetLanguage::try_from(attr.value.as_str()) {
                                target_language = Some(result);
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }

        match target_language {
            None => {
                let run_error = RunError::new(USAGE, "No target language specified.");
                return Err(run_error);
            }
            Some(lang) => match lang {
                TargetLanguage::Graphviz => {
                    let (arcanum, system_hierarchy_opt) = semantic_parser.get_all();
                    // If there was no system in the spec then don't run the visitor.
                    if let Some(system_hierarchy) = system_hierarchy_opt {
                        let mut visitor = GraphVizVisitor::new(
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
                    } else {
                        output = String::from(
                            "digraph structs { node [shape=plaintext] \
                                                    struct1 [label=\"No System\"]; \
                                                  }",
                        );
                    }
                }
                TargetLanguage::Python3 => {
                    let mut visitor = PythonVisitor::new(
                        semantic_parser.get_arcanum(),
                        // generate_exit_args,
                        // generate_enter_args || generate_state_context,
                        generate_state_stack,
                        generate_change_state,
                        // generate_transition_state,
                        FRAMEC_VERSION,
                        comments,
                        config,
                    );
                    let system_node_rcref = Rc::new(RefCell::new(system_node));
                    visitor.run(system_node_rcref);
                    output = visitor.get_code();
                }
            },
        }

        Ok(output)
    }
}

impl Default for Exe {
    fn default() -> Self {
        Exe::new()
    }
}
