// Frame v4 MIR - Mixed Intermediate Representation
// Tracks Frame constructs within native code blocks while maintaining
// the system-focused parsing approach

use super::ast::SourceLocation;

/// Represents a Frame construct found within native code
#[derive(Debug, Clone, PartialEq)]
pub enum MirItem {
    /// Transition with argument buckets
    /// Syntax: (exit_args)? -> (enter_args)? $State(state_params?)
    Transition {
        target: String,
        exit_args: Vec<String>,
        enter_args: Vec<String>,
        state_args: Vec<String>,
        span: RegionSpan,
    },
    
    /// Forward to parent/interface
    Forward {
        span: RegionSpan,
    },
    
    /// Stack push
    StackPush {
        span: RegionSpan,
    },
    
    /// Stack pop
    StackPop {
        span: RegionSpan,
    },
    
    /// System return assignment
    SystemReturn {
        expression: String,
        span: RegionSpan,
    },
}

/// Span within the native code block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionSpan {
    pub start: usize,
    pub end: usize,
}

/// Region types found in native code
#[derive(Debug, Clone, PartialEq)]
pub enum Region {
    /// Native code text
    NativeText {
        content: String,
        span: RegionSpan,
    },
    
    /// Frame construct segment
    FrameSegment {
        item: MirItem,
        span: RegionSpan,
        indent: usize,
    },
}

/// Result of scanning a native code block for Frame constructs
#[derive(Debug, Clone)]
pub struct MirBlock {
    pub regions: Vec<Region>,
    pub symbols: SymbolTable,
}

/// Symbol tracking across native and Frame contexts
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    /// Parameters available in current scope
    pub parameters: Vec<String>,
    
    /// Local variables defined in native code
    pub locals: Vec<String>,
    
    /// Domain variables accessible
    pub domain_vars: Vec<String>,
    
    /// System instances for tracking
    pub system_instances: Vec<SystemInstance>,
}

#[derive(Debug, Clone)]
pub struct SystemInstance {
    pub variable_name: String,
    pub system_type: String,
}

impl MirBlock {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            symbols: SymbolTable::default(),
        }
    }
    
    /// Extract just the Frame statements for validation
    pub fn frame_statements(&self) -> Vec<MirItem> {
        self.regions.iter()
            .filter_map(|r| match r {
                Region::FrameSegment { item, .. } => Some(item.clone()),
                _ => None,
            })
            .collect()
    }
    
    /// Reconstruct the native code with Frame statements expanded
    pub fn generate_code<F>(&self, expand_frame: F) -> String
    where
        F: Fn(&MirItem) -> String,
    {
        let mut output = String::new();
        
        for region in &self.regions {
            match region {
                Region::NativeText { content, .. } => {
                    output.push_str(content);
                }
                Region::FrameSegment { item, indent, .. } => {
                    // Add indentation
                    for _ in 0..*indent {
                        output.push(' ');
                    }
                    // Expand Frame statement to native code
                    output.push_str(&expand_frame(item));
                }
            }
        }
        
        output
    }
}