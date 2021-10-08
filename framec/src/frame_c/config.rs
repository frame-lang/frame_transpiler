use crate::frame_c::ast::{AttributeNode, SystemNode};
use crate::frame_c::utils::{frame_exitcode, RunError};
use figment::providers::{Format, Yaml};
use figment::value::{Dict, Map, Value};
use figment::{Error, Figment, Metadata, Profile, Provider};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Write out the default configuration to `config.yaml` in the current working
/// directory.
pub fn write_default_yaml_file() -> Result<(), RunError> {
    let default_config = FrameConfig::default();
    match serde_yaml::to_string(&default_config) {
        Ok(serialized) => match fs::write("config.yaml", serialized) {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!("Error writing default config.yaml: {}", err);
                Err(RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*msg))
            }
        },
        Err(err) => {
            let msg = format!("Error serializing default configuration: {}", err);
            Err(RunError::new(frame_exitcode::DEFAULT_CONFIG_ERR, &*msg))
        }
    }
}

/// Generate a configuration by merging the default configuration with an
/// optional local configuration file and any configuration attributes from
/// the Frame spec.
pub fn generate_config(
    local_config: &Option<PathBuf>,
    system_node: &SystemNode,
) -> Result<FrameConfig, Error> {
    let mut figment = FrameConfig::default().figment();
    if let Some(path) = local_config {
        figment = figment.merge(Yaml::file(path));
    }
    figment.merge(Figment::from(system_node)).extract()
}

/// The root struct of a frame configuration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameConfig {
    pub codegen: CodeGenConfig,
}

/// Configuration options related to code generation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeGenConfig {
    pub common: CommonConfig,
    pub rust: RustConfig,
}

/// Code generation options shared among all backends.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonConfig {
    pub features: CommonFeatures,
    pub code: CommonCode,
}

/// Code generation features shared among all backends.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonFeatures {}

/// Naming options for generated code shared among all backends.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonCode {}

/// Code generation options specific to the Rust backend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustConfig {
    pub features: RustFeatures,
    pub code: RustCode,
}

/// Code generation features specific to the Rust backend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustFeatures {
    /// When enabled, generated code will attempt to conform to standard Rust
    /// naming conventions. However, options in `RustCode` are in general not
    /// overridden by this feature.
    pub follow_rust_naming: bool,

    /// When enabled, generate an empty implementation of the `Action` trait.
    /// This is nice so that the unextended output of Frame compiles. Actions
    /// may still be overridden by implementing them directly in an `impl`
    /// block for the generated state machine type.
    pub generate_action_impl: bool,

    /// When enabled, generates "hook" methods that will be invoked on every
    /// transition or change-state. These hook methods are added to the
    /// `Action` trait and must be implemented.
    pub generate_hook_methods: bool,

    /// When enabled, generates code that links into the Frame runtime system.
    /// See the `frame_runtime` crate. This crate provides reflection and
    /// monitoring capabilities to running state machines.
    pub runtime_support: bool,
}

/// Naming options for generated code specific to the Rust backend. These
/// options can be used to tweak the names of types, methods, fields, and
/// variables in generated code.
///
/// These options are rather "use at your own risk" for now since we are not
/// testing Frame with anything other than the defaults. Unless you have some
/// strong reason to do otherwise, it's probably best to leave them be. :-)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCode {
    pub action_prefix: String,
    pub action_suffix: String,
    pub actions_prefix: String,
    pub actions_suffix: String,

    pub enter_token: String,
    pub exit_token: String,
    pub enter_msg: String,
    pub exit_msg: String,
    pub event_args_suffix: String,
    pub event_args_method_suffix: String,
    pub enter_args_member_name: String,
    pub exit_args_member_name: String,

    pub frame_event_type_name: String,
    pub frame_event_variable_name: String,
    pub frame_event_args_attribute_name: String,
    pub frame_event_args_type_name: String,
    pub frame_event_message_attribute_name: String,
    pub frame_event_message_type_name: String,
    pub frame_event_return_attribute_name: String,
    pub frame_event_return_type_name: String,

    pub initialize_method_name: String,
    pub handle_event_method_name: String,
    pub change_state_method_name: String,
    pub transition_method_name: String,

    pub state_handler_name_prefix: String,
    pub state_handler_name_suffix: String,

    pub state_var_name: String,
    pub state_args_suffix: String,
    pub state_args_var_name: String,
    pub state_vars_suffix: String,
    pub state_vars_var_name: String,

    pub state_context_type_name: String,
    pub state_context_var_name: String,
    pub state_context_suffix: String,
    pub state_context_method_suffix: String,
    pub this_state_context_var_name: String,

    pub state_enum_suffix: String,
    pub state_enum_traits: String,

    pub change_state_hook_method_name: String,
    pub transition_hook_method_name: String,

    pub state_stack_var_name: String,
    pub state_stack_push_method_name: String,
    pub state_stack_pop_method_name: String,

    pub callback_manager_var_name: String,
    pub state_cell_var_name: String,
}

impl FrameConfig {
    /// Generate a configuration from any `Provider`.
    pub fn from<T: Provider>(provider: T) -> Result<FrameConfig, Error> {
        Figment::from(provider).extract()
    }
    /// Access this configuration as a `Figment`, which is useful for merging
    /// with configurations from other sources.
    pub fn figment(&self) -> Figment {
        Figment::from(self)
    }
}

/// Make `FrameConfig` a `Provider` for composability.
impl Provider for FrameConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("FrameConfig struct")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::from(self, Profile::Default).data()
    }
}

/// Make `AttributeNode` a `Provider`.
impl Provider for AttributeNode {
    fn metadata(&self) -> Metadata {
        Metadata::named("AttributeNode")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let mut map = Map::new();
        let attr_name = &self.name;
        let config_path;
        let config_value;
        if let Some(path) = attr_name.strip_prefix("feature:") {
            // this attribute is a boolean feature option
            config_path = path;
            match self.value.parse::<bool>() {
                Ok(value) => {
                    config_value = Value::from(value);
                }
                Err(err) => {
                    return Err(Error::from(format!(
                        "Error parsing boolean feature option: {:?}",
                        err
                    )));
                }
            }
        } else if let Some(path) = attr_name.strip_prefix("code:") {
            // this attribute is a string (codegen related) config option
            config_path = path;
            config_value = Value::from(self.value.clone());
        } else {
            return Ok(map);
        }
        // recursively generate the chain of dictionaries for this option
        let mut dict = Dict::new();
        let mut iter = config_path.rsplit('.');
        if let Some(key) = iter.next() {
            dict.insert(String::from(key), config_value);
            for key in iter {
                let mut next = Dict::new();
                next.insert(String::from(key), Value::from(dict));
                dict = next;
            }
        }
        map.insert(Profile::Default, dict);
        Ok(map)
    }
}

/// Make `SystemNode` a `Provider` by extracting configuration attribute
/// settings from the Frame spec.
impl Provider for SystemNode {
    fn metadata(&self) -> Metadata {
        Metadata::named("SystemNode attributes")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let mut figment = Figment::new();
        if let Some(attributes) = &self.attributes_opt {
            for attr in attributes.values() {
                figment = figment.merge(Figment::from(attr));
            }
        }
        figment.data()
    }
}

// Defaults

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
