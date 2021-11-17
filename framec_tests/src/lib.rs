// Tests with runtime_support disabled.
mod branch;
mod config;
mod empty;
mod event_handler;
mod handler_calls;
mod hierarchical_guard;
mod r#match;
mod rust_naming_off;
mod rust_naming_on;
mod simple_handler_calls;
mod state_context;
mod var_scope;

// Tests with runtime_support enabled.
mod basic;
// mod hierarchical;
// mod state_context_runtime;
// mod state_context_stack;
// mod state_params;
// mod state_stack;
mod state_vars;
// mod transition;
// mod transition_params;
