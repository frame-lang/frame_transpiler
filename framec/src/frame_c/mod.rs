mod ast;
pub mod cli;
pub mod compiler;
pub mod config;
pub mod modules;  // Multi-file module system (v0.57)
mod parser;
mod scanner;
mod symbol_table;
pub mod utils;
mod visitors;
