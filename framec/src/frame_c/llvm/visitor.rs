use crate::frame_c::ast::*;
use std::collections::HashMap;

use super::builder::LLVMModuleBuilder;
use super::context::{SystemEmitContext, SystemSummary};

pub struct LLVMVisitor;

impl LLVMVisitor {
    pub fn new() -> Self {
        LLVMVisitor
    }

    pub fn run(&self, frame_module: &FrameModule) -> String {
        let mut builder = LLVMModuleBuilder::new();

        let mut summaries = Vec::new();

        for system in &frame_module.systems {
            if let Some(ctx) = SystemEmitContext::new(system) {
                let struct_fields = ctx.struct_fields();
                builder.ensure_struct(&ctx.struct_name, &struct_fields);
                builder.emit_init_function(&ctx);
                builder.emit_deinit_function(&ctx);

                if let Some(interface_block) = &system.interface_block_node_opt {
                    for method_rc in &interface_block.interface_methods {
                        let method_ref = method_rc.borrow();
                        let method_names = ctx.method_names(&method_ref);
                        builder.emit_interface_method(&ctx, &method_names, &method_ref);
                    }
                }

                if ctx.has_actions() {
                    for action in ctx.actions_iter() {
                        builder.emit_action_function(&ctx, action);
                    }
                }

                summaries.push(ctx.summary());
            }
        }

        let summary_lookup: HashMap<String, SystemSummary> = summaries
            .into_iter()
            .map(|summary| (summary.raw_name.clone(), summary))
            .collect();

        for function_rc in &frame_module.functions {
            let function = function_rc.borrow();
            if function.name == "main"
                && function
                    .params
                    .as_ref()
                    .map_or(true, |params| params.is_empty())
            {
                builder.emit_main_function(&function, &summary_lookup);
            }
        }

        builder.finish()
    }
}
