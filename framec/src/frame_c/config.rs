use crate::frame_c::ast::{AttributeNode, SystemNode};
use crate::frame_c::utils::{frame_exitcode, RunError};
use figment::providers::{Format, Yaml};
use figment::value::{Dict, Map, Value};
use figment::{Error, Figment, Metadata, Profile, Provider};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

/// The root struct of a frame configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameConfig {
    pub codegen: CodeGenConfig,
}

impl FrameConfig {
    /// Write out the default configuration to `config.yaml` in the current working directory.
    pub fn write_default_yaml_file() -> Result<(), RunError> {
        let default_config = FrameConfig::default();
        match serde_yaml::to_string(&default_config) {
            Ok(serialized) => match fs::write("config.yaml", serialized) {
                Ok(_) => Ok(()),
                Err(err) => {
                    let msg = format!("Error writing default config.yaml: {}", err);
                    Err(RunError::new(frame_exitcode::CONFIG_ERR, &*msg))
                }
            },
            Err(err) => {
                let msg = format!("Error serializing default configuration: {}", err);
                Err(RunError::new(frame_exitcode::CONFIG_ERR, &*msg))
            }
        }
    }

    /// Load a configuration by merging the default configuration with an optional local
    /// configuration file, then overriding any configuration attributes defined in the Frame spec.
    pub fn load(
        local_config: &Option<PathBuf>,
        system_node: &SystemNode,
    ) -> Result<FrameConfig, Error> {
        let mut figment = FrameConfig::default().figment();
        if let Some(path) = local_config {
            figment = figment.merge(Yaml::file(path));
        }
        figment.merge(Figment::from(system_node)).extract()
    }
}

/// Configuration options related to code generation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeGenConfig {
    pub common: CommonConfig,
    pub rust: RustConfig,
    pub golang: GolangConfig,
    pub smcat: SmcatConfig,
}

/// Code generation options shared among all backends.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonConfig {
    pub features: CommonFeatures,
    pub code: CommonCode,
}

/// Code generation options specific to the Rust backend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GolangConfig {
//    pub features: GolangFeatures,
    pub code: GolangCode,
}

/// Naming options for generated code specific to the Rust backend. These options can be used to
/// tweak the names of types, methods, fields, and variables in generated code.
///
/// These options are "use at your own risk" for now since we are not testing Frame with anything
/// other than the defaults. Unless you have some strong reason to do otherwise, it's probably best
/// to leave them be. :-)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GolangCode {
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

    pub assignment_temp_var_name: String,
    pub state_handler_name_prefix: String,
    pub state_handler_name_suffix: String,

    pub state_var_name: String,
    pub state_args_suffix: String,
    pub state_args_var_name: String,
    pub state_vars_suffix: String,
    pub state_vars_var_name: String,
    pub state_name_use_sysname_prefix: bool, // auto prefix the state name w/ system name

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

    pub runtime_info_module_name: String,
    pub runtime_module_use_as_name: String,
    pub machine_info_function_name: String,
    pub pop_state_info_name: String,
    pub event_monitor_var_name: String,
    pub transition_info_arg_name: String,
}

impl Default for GolangCode {
    fn default() -> Self {
        GolangCode {
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

            assignment_temp_var_name: String::from("assign_temp"),
            state_handler_name_prefix: String::from(""),
            state_handler_name_suffix: String::from("_handler"),

            state_var_name: String::from("state"),
            state_args_suffix: String::from("StateArgs"),
            state_args_var_name: String::from("state_args"),
            state_vars_suffix: String::from("StateVars"),
            state_vars_var_name: String::from("state_vars"),
            state_name_use_sysname_prefix:true,

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

            runtime_info_module_name: String::from("runtime_info"),
            runtime_module_use_as_name: String::from("runtime"),
            machine_info_function_name: String::from("machine_info"),
            pop_state_info_name: String::from("$$[-]"),
            event_monitor_var_name: String::from("event_monitor"),
            transition_info_arg_name: String::from("transition_info"),
        }
    }
}
/// Code generation features shared among all backends.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonFeatures {}

/// Naming options for generated code shared among all backends.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonCode {}

/// Code generation options specific to the Golang backend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustConfig {
    pub features: RustFeatures,
    pub code: RustCode,
    pub runtime: RustRuntime,
}

/// Code generation features specific to the Rust backend.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustFeatures {
    /// When enabled, generated code will attempt to conform to standard Rust naming conventions.
    /// However, options in `RustCode` are in general not overridden by this feature.
    ///
    /// Default is `true`.
    pub follow_rust_naming: bool,

    /// When enabled, generate an empty implementation of the `Action` trait. This is nice so that
    /// the unextended output of Frame compiles. Actions may still be overridden by implementing
    /// them directly in an `impl` block for the generated state machine type.
    ///
    /// Default is `true`.
    pub generate_action_impl: bool,

    /// When enabled, generates "hook" methods that will be invoked on every transition or
    /// change-state. These hook methods are added to the `Action` trait and must be implemented.
    ///
    /// Default is `false`.
    pub generate_hook_methods: bool,

    /// When enabled, generates code that links into the Frame runtime system. See the
    /// `frame_runtime` crate. This crate provides reflection and monitoring capabilities to
    /// running state machines.
    ///
    /// To use the runtime interface, include the `frame_runtime` crate and import one the
    /// following modules:
    ///
    ///  * `frame_runtime::unsync` if the `thread_safe` feature is disabled (default)
    ///  * `frame_runtime::sync` if the `thread_safe` feature is enabled
    ///
    /// By default, the `runtime_support` feature is `false`.
    pub runtime_support: bool,

    /// When enabled, generates a state machine that implements the `Send` trait, and so can be
    /// safely passed acrosss thread boundries.
    ///
    /// Default is `false`.
    pub thread_safe: bool,
}

/// Naming options for generated code specific to the Rust backend. These options can be used to
/// tweak the names of types, methods, fields, and variables in generated code.
///
/// These options are "use at your own risk" for now since we are not testing Frame with anything
/// other than the defaults. Unless you have some strong reason to do otherwise, it's probably best
/// to leave them be. :-)
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

    pub assignment_temp_var_name: String,
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

    pub runtime_info_module_name: String,
    pub runtime_module_use_as_name: String,
    pub machine_info_function_name: String,
    pub pop_state_info_name: String,
    pub event_monitor_var_name: String,
    pub transition_info_arg_name: String,
}

/// Initial settings for the Rust runtime system. These options are only relevant if
/// [RustFeatures.runtime_support] is enabled. These options can be changed at runtime later.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustRuntime {
    /// The number of handled events to save in the event history. A value of `0` disables the
    /// event history feature, while a negative value allows the history to grow to unbounded size
    /// (in which case it should be occasionally manually cleared).
    ///
    /// Default is `0`, disabling event history tracking.
    pub event_history_capacity: i32,

    /// The number of transitions to save in the transition history. A value of `0` disables the
    /// event history feature, while a negative value allows the history to grow to unbounded size
    /// (in which case it should be occasionally manually cleared).
    ///
    /// Default is `1`, storing the most recent transition only.
    pub transition_history_capacity: i32,
}

impl RustRuntime {
    /// Get the event history capacity as a value suitable for the event monitor.
    pub fn event_history_capacity(&self) -> Option<usize> {
        self.event_history_capacity.try_into().ok()
    }

    /// Get the transition history capacity as a value suitable for the event monitor.
    pub fn transition_history_capacity(&self) -> Option<usize> {
        self.transition_history_capacity.try_into().ok()
    }
}

/// Code generation options specific to the Smcat backend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmcatConfig {
    pub features: SmcatFeatures,
    pub code: SmcatCode,
}

/// Code generation features specific to the Smcat backend.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmcatFeatures {}

/// Style options for generated code specific to the Smcat backend.
///
/// See the sections "colors and line width", "classes", and "overriding the type of a state" in
/// the smcat README: <https://github.com/sverweij/state-machine-cat/blob/develop/README.md>
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmcatCode {
    /// Style settings for nodes that do not have any children.
    pub simple_state_node_style: String,
    /// Style settings for nodes that have sub-states as children.
    pub parent_state_node_style: String,
    /// Style settings for "change-state" transitions.
    pub change_state_edge_style: String,
    /// Style settings for standard transitions.
    pub transition_edge_style: String,
}

impl FrameConfig {
    /// Generate a configuration from any `Provider`.
    pub fn from<T: Provider>(provider: T) -> Result<FrameConfig, Error> {
        Figment::from(provider).extract()
    }
    /// Access this configuration as a `Figment`, which is useful for merging with configurations
    /// from other sources.
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

/// Make `AttributeNode` a `Provider`. An attribute may contain zero or one configuration settings.
///
/// The format of an attribute name is `[full.path.to.attribute]:[type]`, where the path is the
/// sequence of field names starting from the `FrameConfig` struct to access the attribute value,
/// and the type is one of currently supported attribute types:
///
///  * `bool`, corresponding to Rust's `bool` type
///  * `int`, corresponding to Rust's `i32` type
///  * `str`, corresponding to Rust's `String` type
///
/// For example, to enable the Rust backend's boolean `runtime_support` feature, the following
/// attribute statement would be used.
///
/// ```text
/// #[codegen.rust.features.runtime_support:bool="true"]
/// ```
impl Provider for AttributeNode {
    fn metadata(&self) -> Metadata {
        Metadata::named("AttributeNode")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let mut map = Map::new();
        let attr_name = &self.name;
        let config_path;
        let config_value;
        if let Some(path) = attr_name.strip_suffix(":bool") {
            // this attribute is a boolean option
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
        } else if let Some(path) = attr_name.strip_suffix(":int") {
            // this attribute is an integer option
            config_path = path;
            match self.value.parse::<i32>() {
                Ok(value) => {
                    config_value = Value::from(value);
                }
                Err(err) => {
                    return Err(Error::from(format!(
                        "Error parsing integer feature option: {:?}",
                        err
                    )));
                }
            }
        } else if let Some(path) = attr_name.strip_suffix(":str") {
            // this attribute is a string config option
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

/// Make `SystemNode` a `Provider` by extracting and merging all configuration attribute settings
/// from the Frame spec.
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

impl Default for RustFeatures {
    fn default() -> Self {
        RustFeatures {
            follow_rust_naming: true,
            generate_action_impl: true,
            generate_hook_methods: false,
            runtime_support: false,
            thread_safe: false,
        }
    }
}

impl Default for RustCode {
    fn default() -> Self {
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

            assignment_temp_var_name: String::from("assign_temp"),
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

            runtime_info_module_name: String::from("runtime_info"),
            runtime_module_use_as_name: String::from("runtime"),
            machine_info_function_name: String::from("machine_info"),
            pop_state_info_name: String::from("$$[-]"),
            event_monitor_var_name: String::from("event_monitor"),
            transition_info_arg_name: String::from("transition_info"),
        }
    }
}

impl Default for RustRuntime {
    fn default() -> Self {
        RustRuntime {
            event_history_capacity: 0,
            transition_history_capacity: 1,
        }
    }
}

impl Default for SmcatCode {
    fn default() -> Self {
        SmcatCode {
            simple_state_node_style: String::from("class=\"simple\""),
            parent_state_node_style: String::from("class=\"parent\""),
            change_state_edge_style: String::from("class=\"change-state\""),
            transition_edge_style: String::from("class=\"standard\""),
        }
    }
}
