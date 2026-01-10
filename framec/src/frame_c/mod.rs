pub mod cli;
pub mod compiler;
pub mod config; // Project configuration system (v0.58)
pub mod utils;
mod visitors; // Retains TargetLanguage enum only (V3 rebuild)
pub mod v3; // V3 pipeline (rebuild from first principles)
pub mod v4; // V4 pure preprocessor implementation
