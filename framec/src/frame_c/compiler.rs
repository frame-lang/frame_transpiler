use super::scanner::*;
use super::parser::*;
use super::symbol_table::*;
use crate::frame_c::visitors::javascript_visitor::JavaScriptVisitor;
use crate::frame_c::visitors::cpp_visitor::CppVisitor;
use crate::frame_c::visitors::cs_visitor_for_bob::CsVisitorForBob;
use crate::frame_c::visitors::cs_visitor::CsVisitor;
use crate::frame_c::visitors::plantuml_visitor::PlantUmlVisitor;
use crate::frame_c::visitors::python_visitor::PythonVisitor;
use crate::frame_c::visitors::gdscript_3_2_visitor::GdScript32Visitor;
use crate::frame_c::visitors::java_8_visitor::Java8Visitor;
use crate::frame_c::visitors::rust_visitor::RustVisitor;
use crate::frame_c::utils::{RunError, frame_exitcode};
use exitcode::USAGE;
//use crate::frame_c::visitors::xtate_visitor::XStateVisitor;

/* --------------------------------------------------------------------- */

static IS_DEBUG:bool = false;
static FRAMEC_VERSION:&str = "emitted from framec_v0.4.0";


/* --------------------------------------------------------------------- */

pub struct Exe {
}

impl Exe {

    /* --------------------------------------------------------------------- */


    pub fn new() -> Exe {
        Exe {
        }
    }

    pub fn debug_print(msg:&str) {
        if !IS_DEBUG {
            return;
        }

        println!("{}", msg);
    }


    /* --------------------------------------------------------------------- */

    pub fn run(&self, contents:String, mut output_format:String) -> Result<String,RunError> {
        let output;

        let scanner = Scanner::new(contents);
        let (has_errors,errors,tokens) = scanner.scan_tokens();
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
                let mut errors =  "Terminating with errors.\n".to_string();
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
            let mut errors =  "Terminating with errors.\n".to_string();
            errors.push_str(&semantic_parser.get_errors());
            let run_error = RunError::new(frame_exitcode::PARSE_ERR, &*errors.clone());
            return Err(run_error);
        }

        let generate_exit_args = semantic_parser.generate_exit_args;
        let generate_state_context= semantic_parser.generate_state_context;
        let generate_state_stack = semantic_parser.generate_state_stack;
        let generate_change_state = semantic_parser.generate_change_state;
        let generate_transition_state = semantic_parser.generate_transition_state;


        match &system_node.attributes_opt {
            Some(attributes) => {
                if let Some(language) = attributes.get("language"){
                    output_format = language.value.clone();
                }
            },
            None => {},
        }
        if output_format == "javascript" {
            let mut visitor = JavaScriptVisitor::new(semantic_parser.get_arcanum()
                                                                                , generate_exit_args
                                                                                , generate_state_context
                                                                                , generate_state_stack
                                                                                , generate_change_state
                                                                                , generate_transition_state
                                                                                ,  FRAMEC_VERSION
                                                                                , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "cpp" {
            let mut visitor = CppVisitor::new(semantic_parser.get_arcanum()
                                                  , generate_exit_args
                                                  , generate_state_context
                                                  , generate_state_stack
                                                  , generate_change_state
                                                  , generate_transition_state
                                                  ,  FRAMEC_VERSION
                                                  , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "c_sharp_bob" {
            let mut visitor = CsVisitorForBob::new(semantic_parser.get_arcanum()
                                                   , generate_exit_args
                                                   , generate_state_context
                                                   , generate_state_stack
                                                   , generate_change_state
                                                   , generate_transition_state
                                                   , FRAMEC_VERSION
                                                   , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "c_sharp" {
            let mut visitor = CsVisitor::new(semantic_parser.get_arcanum()
                                             , generate_exit_args
                                             , generate_state_context
                                             , generate_state_stack
                                             , generate_change_state
                                             , generate_transition_state
                                             , FRAMEC_VERSION
                                             , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "gdscript" {
            let mut visitor = GdScript32Visitor::new(semantic_parser.get_arcanum()
                                                     , generate_exit_args
                                                     , generate_state_context
                                                     , generate_state_stack
                                                     , generate_change_state
                                                     , generate_transition_state
                                                     ,FRAMEC_VERSION
                                                     , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "java_8" {
            let mut visitor = Java8Visitor::new(semantic_parser.get_arcanum()
                                                   , generate_exit_args
                                                   , generate_state_context
                                                   , generate_state_stack
                                                   , generate_change_state
                                                   , generate_transition_state
                                                   ,FRAMEC_VERSION
                                                   , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "python_3" {
            let mut visitor = PythonVisitor::new(semantic_parser.get_arcanum()
                                                   , generate_exit_args
                                                   , generate_state_context
                                                   , generate_state_stack
                                                   , generate_change_state
                                                   , generate_transition_state
                                                   ,FRAMEC_VERSION
                                                   , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "plantuml" {
            // let x = (&semantic_parser).get_arcanum();
            // semantic_parser = semantic_parser.into_inner();
            // let y = (&semantic_parser).get_system_hierarchy();
            let (x,y) = semantic_parser.get_all();
            let mut visitor = PlantUmlVisitor::new(
                                                  x
                                                , y
                                                , generate_exit_args
                                                , generate_state_context
                                                , generate_state_stack
                                                , generate_change_state
                                                , generate_transition_state
                                                ,FRAMEC_VERSION
                                                , comments);
            visitor.run(&system_node);
            output = visitor.get_code();
        } else if output_format == "rust" {
            let mut visitor = RustVisitor::new(semantic_parser.get_arcanum()
                                             , generate_exit_args
                                             , generate_state_context
                                             , generate_state_stack
                                             , generate_change_state
                                             , generate_transition_state
                                             , FRAMEC_VERSION
                                             , comments);
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
            let error_msg = &format!("Error - unrecognized output format {}.",output_format);
            let run_error = RunError::new(USAGE, error_msg);
            return Err(run_error);
        }

        Ok(output)

        // let mut graphviz_visitor = GraphVizVisitor::new(semantic_parser.get_arcanum(), comments);
        // graphviz_visitor.run(&system_node);
        // println!("{}", graphviz_visitor.code);
    }

}

