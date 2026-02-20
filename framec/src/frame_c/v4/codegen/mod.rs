//! Frame V4 Code Generation Infrastructure
//!
//! This module provides a proper AST-based code generation architecture:
//! - `ast.rs`: CodegenNode - Language-agnostic intermediate representation
//! - `backend.rs`: LanguageBackend trait for language-specific emission
//! - `system_codegen.rs`: System code generation from Frame AST
//! - `backends/`: Language-specific backend implementations
//!
//! This replaces the string-template code generation with proper AST traversal.

pub mod ast;
pub mod backend;
pub mod system_codegen;
pub mod backends;

pub use ast::CodegenNode;
pub use backend::{LanguageBackend, EmitContext, ClassSyntax, get_backend};
pub use system_codegen::{generate_system, generate_rust_compartment_types, generate_compartment_class, generate_frame_event_class};
