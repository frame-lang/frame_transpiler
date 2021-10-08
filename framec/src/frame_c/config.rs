use crate::frame_c::utils::{frame_exitcode, RunError};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn write_default_yaml_file() -> Result<(), RunError> {
    let default_config = FrameConfig::default();
    match serde_yaml::to_string(&default_config) {
        Ok(serialized) => {
            match fs::write("config.yaml", serialized) {
                Ok(_) => Ok(()),
                Err(err) => {
                    let msg = format!("Error writing default config.yaml: {}", err);
                    Err(RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*msg))
                }
            }
        }
        Err(err) => {
            let msg = format!("Error serializing default configuration: {}", err);
            Err(RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*msg))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameConfig {
    codegen: CodeGenConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeGenConfig {
    common: CommonConfig,
    rust: RustConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonConfig {
    features: CommonFeatures,
    code: CommonCode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonFeatures {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonCode {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustConfig {
    features: RustFeatures,
    code: RustCode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustFeatures {
    follow_rust_naming: bool,
    generate_action_impl: bool,
    generate_hook_methods: bool,
    runtime_support: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCode {
    action_prefix: String,
    action_suffix: String,
    actions_prefix: String,
    actions_suffix: String,

    enter_token: String,
    exit_token: String,
    enter_msg: String,
    exit_msg: String,
    event_args_suffix: String,
    event_args_method_suffix: String,
    enter_args_member_name: String,
    exit_args_member_name: String,

    frame_event_type_name: String,
    frame_event_variable_name: String,
    frame_event_args_attribute_name: String,
    frame_event_args_type_name: String,
    frame_event_message_attribute_name: String,
    frame_event_message_type_name: String,
    frame_event_return_attribute_name: String,
    frame_event_return_type_name: String,

    initialize_method_name: String,
    handle_event_method_name: String,
    change_state_method_name: String,
    transition_method_name: String,

    state_handler_name_prefix: String,
    state_handler_name_suffix: String,

    state_var_name: String,
    state_args_suffix: String,
    state_args_var_name: String,
    state_vars_suffix: String,
    state_vars_var_name: String,

    state_context_type_name: String,
    state_context_var_name: String,
    state_context_suffix: String,
    state_context_method_suffix: String,
    this_state_context_var_name: String,

    state_enum_suffix: String,
    state_enum_traits: String,

    change_state_hook_method_name: String,
    transition_hook_method_name: String,

    state_stack_var_name: String,
    state_stack_push_method_name: String,
    state_stack_pop_method_name: String,

    callback_manager_var_name: String,
    state_cell_var_name: String,
}

impl Default for FrameConfig {
    fn default() -> FrameConfig {
        FrameConfig {
            codegen: CodeGenConfig::default(),
        }
    }
}

impl Default for CodeGenConfig {
    fn default() -> CodeGenConfig {
        CodeGenConfig {
            common: CommonConfig::default(),
            rust: RustConfig::default(),
        }
    }
}

impl Default for CommonConfig {
    fn default() -> CommonConfig {
        CommonConfig {
            features: CommonFeatures::default(),
            code: CommonCode::default(),
        }
    }
}

impl Default for CommonFeatures {
    fn default() -> CommonFeatures {
        CommonFeatures {}
    }
}

impl Default for CommonCode {
    fn default() -> CommonCode {
        CommonCode {}
    }
}

impl Default for RustConfig {
    fn default() -> RustConfig {
        RustConfig {
            features: RustFeatures::default(),
            code: RustCode::default(),
        }
    }
}

impl Default for RustFeatures {
    fn default() -> RustFeatures {
        RustFeatures {
            follow_rust_naming: true,
            generate_action_impl: true,
            generate_hook_methods: true,
            runtime_support: true,
        }
    }
}

impl Default for RustCode {
    fn default() -> RustCode {
        RustCode {
            action_prefix: String::from(""),
            action_suffix: String::from(""),
            actions_prefix: String::from(""),
            actions_suffix: String::from("Actions"),

            enter_token: String::from(">"),
            exit_token: String::from("<"),
            enter_msg: String::from("Enter"),
            exit_msg: String::from("Exit"),
            event_args_suffix: String::from("Args"),
            event_args_method_suffix: String::from("_args"),
            enter_args_member_name: String::from("enter_args"),
            exit_args_member_name: String::from("exit_args"),

            frame_event_type_name: String::from("FrameEvent"),
            frame_event_variable_name: String::from("frame_event"),
            frame_event_args_attribute_name: String::from("arguments"),
            frame_event_args_type_name: String::from("FrameEventArgs"),
            frame_event_message_attribute_name: String::from("message"),
            frame_event_message_type_name: String::from("FrameMessage"),
            frame_event_return_attribute_name: String::from("ret"),
            frame_event_return_type_name: String::from("FrameEventReturn"),

            initialize_method_name: String::from("initialize"),
            handle_event_method_name: String::from("handle_event"),
            change_state_method_name: String::from("change_state"),
            transition_method_name: String::from("transition"),

            state_handler_name_prefix: String::from(""),
            state_handler_name_suffix: String::from("_handler"),

            state_var_name: String::from("state"),
            state_args_suffix: String::from("StateArgs"),
            state_args_var_name: String::from("state_args"),
            state_vars_suffix: String::from("StateVars"),
            state_vars_var_name: String::from("state_vars"),

            state_context_type_name: String::from("StateContext"),
            state_context_var_name: String::from("state_context"),
            state_context_suffix: String::from("StateContext"),
            state_context_method_suffix: String::from("_context"),
            this_state_context_var_name: String::from("this_state_context"),

            state_enum_suffix: String::from("State"),
            state_enum_traits: String::from(
                "Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord",
            ),

            change_state_hook_method_name: String::from("change_state_hook"),
            transition_hook_method_name: String::from("transition_hook"),

            state_stack_var_name: String::from("state_stack"),
            state_stack_push_method_name: String::from("state_stack_push"),
            state_stack_pop_method_name: String::from("state_stack_pop"),

            callback_manager_var_name: String::from("callback_manager"),
            state_cell_var_name: String::from("state_cell"),
        }
    }
}
