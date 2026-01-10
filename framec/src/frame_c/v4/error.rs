// Frame v4 Error Reporting - Enhanced diagnostics for Frame errors
//
// Provides context-aware error messages for Frame structural issues
// while delegating native code errors to native compilers

use super::ast::SourceLocation;

/// Simple error accumulator for v4
#[derive(Debug, Clone)]
pub struct ErrorsAcc {
    errors: Vec<String>,
}

impl ErrorsAcc {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn push_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    pub fn add_error(&mut self, message: String, _location: SourceLocation) {
        // For now, just add the message
        self.errors.push(message);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn from_error(error: String) -> Self {
        let mut acc = Self::new();
        acc.push_error(error);
        acc
    }
    
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

/// Enhanced error with source context
#[derive(Debug, Clone)]
pub struct FrameError {
    pub message: String,
    pub location: SourceLocation,
    pub error_code: ErrorCode,
    pub help: Option<String>,
    pub source_snippet: Option<SourceSnippet>,
}

#[derive(Debug, Clone)]
pub enum ErrorCode {
    // Structure errors
    E001_InvalidBlockOrder,
    E002_DuplicateBlock,
    E003_MissingRequiredBlock,
    
    // Parameter errors
    E010_StartParamMismatch,
    E011_EnterParamMismatch,
    E012_DomainParamMissing,
    E013_ParamOrderViolation,
    
    // State errors
    E020_UndefinedState,
    E021_DuplicateState,
    E022_NoInitialState,
    
    // Interface errors
    E030_UnimplementedInterface,
    E031_InterfaceSignatureMismatch,
    
    // Annotation errors
    E040_InvalidAnnotation,
    E041_MissingTarget,
    
    // System tracking errors
    E050_InvalidSystemCall,
    E051_PrivateMethodAccess,
}

#[derive(Debug, Clone)]
pub struct SourceSnippet {
    pub lines: Vec<String>,
    pub highlight_line: usize,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

impl FrameError {
    pub fn new(
        message: String,
        location: SourceLocation,
        error_code: ErrorCode,
    ) -> Self {
        Self {
            message,
            location,
            error_code,
            help: None,
            source_snippet: None,
        }
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    pub fn with_snippet(mut self, snippet: SourceSnippet) -> Self {
        self.source_snippet = Some(snippet);
        self
    }

    /// Format error for display
    pub fn format(&self) -> String {
        let mut output = String::new();
        
        // Error header
        output.push_str(&format!(
            "[{:?}] {}\n",
            self.error_code,
            self.message
        ));
        
        // Location
        output.push_str(&format!(
            "  --> {}:{}:{}\n",
            self.location.file,
            self.location.line,
            self.location.column
        ));
        
        // Source snippet if available
        if let Some(snippet) = &self.source_snippet {
            output.push_str("   |\n");
            for (i, line) in snippet.lines.iter().enumerate() {
                let line_num = self.location.line + i;
                output.push_str(&format!("{:3} | {}\n", line_num, line));
                
                // Highlight line
                if i == snippet.highlight_line {
                    output.push_str("   | ");
                    for _ in 0..snippet.highlight_start {
                        output.push(' ');
                    }
                    for _ in snippet.highlight_start..snippet.highlight_end {
                        output.push('^');
                    }
                    output.push_str(" ");
                    output.push_str(&self.message);
                    output.push('\n');
                }
            }
            output.push_str("   |\n");
        }
        
        // Help text
        if let Some(help) = &self.help {
            output.push_str(&format!("   = help: {}\n", help));
        }
        
        output
    }
}

/// Create a parameter mismatch error with helpful context
pub fn param_mismatch_error(
    system_params: &str,
    state_params: &str,
    location: SourceLocation,
) -> FrameError {
    FrameError::new(
        format!("Parameter mismatch: system has '{}', state expects '{}'", 
                system_params, state_params),
        location,
        ErrorCode::E010_StartParamMismatch,
    ).with_help(
        "Ensure start state parameters match exactly with system declaration".to_string()
    )
}

/// Create an undefined state error
pub fn undefined_state_error(
    state_name: &str,
    location: SourceLocation,
) -> FrameError {
    FrameError::new(
        format!("State '{}' is not defined", state_name),
        location,
        ErrorCode::E020_UndefinedState,
    ).with_help(
        format!("Define state '{}' in the machine block or check for typos", state_name)
    )
}