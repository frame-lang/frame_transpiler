//! Output Assembler (Stage 7 of the V4 Pipeline)
//!
//! Takes the `SourceMap` from the Segmenter (Stage 0) and generated code from
//! the Codegen/Emit stages (Stages 5-6), and produces the final output file.
//!
//! Algorithm:
//! 1. Walk `SourceMap.segments` in order:
//!    - `Segment::Native` → extract text from source bytes at span, append to output
//!    - `Segment::Pragma` → skip (consumed by earlier stages)
//!    - `Segment::System` → look up system name in generated_systems, append generated code
//! 2. Post-process: expand `@@SystemName()` tagged instantiations in native regions
//! 3. Return final assembled output

use std::collections::HashSet;
use crate::frame_c::v4::segmenter::{SourceMap, Segment};
use crate::frame_c::visitors::TargetLanguage;

// ============================================================================
// Assembly Error
// ============================================================================

#[derive(Debug, Clone)]
pub struct AssemblyError {
    pub message: String,
}

impl std::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assembly error: {}", self.message)
    }
}

impl std::error::Error for AssemblyError {}

// ============================================================================
// Public API
// ============================================================================

/// Assemble the final output from source map and generated system code.
///
/// `source_map` — the segmented source from Stage 0
/// `generated_systems` — Vec of (system_name, generated_code) from Stages 5-6
/// `lang` — target language for tagged instantiation expansion
pub fn assemble(
    source_map: &SourceMap,
    generated_systems: &[(String, String)],
    lang: TargetLanguage,
) -> Result<String, AssemblyError> {
    let source = &source_map.source;
    let mut output = String::new();

    // Build lookup for generated systems
    let system_code: std::collections::HashMap<&str, &str> = generated_systems
        .iter()
        .map(|(name, code)| (name.as_str(), code.as_str()))
        .collect();

    let defined_system_names: HashSet<String> = generated_systems
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    // Walk segments in order
    for segment in &source_map.segments {
        match segment {
            Segment::Native { span } => {
                // Extract native text from source bytes
                let text = extract_text(source, span.start, span.end);
                // Expand tagged instantiations (@@SystemName(args)) in native code
                let expanded = expand_tagged_instantiations(
                    &text, &defined_system_names, lang,
                )?;
                output.push_str(&expanded);
            }

            Segment::Pragma { .. } => {
                // Pragmas are consumed by earlier stages — skip them
                // (they don't appear in the output)
            }

            Segment::System { name, .. } => {
                // Look up generated code for this system
                if let Some(code) = system_code.get(name.as_str()) {
                    output.push_str(code);
                } else {
                    return Err(AssemblyError {
                        message: format!(
                            "No generated code for system '{}'. Available: {:?}",
                            name,
                            system_code.keys().collect::<Vec<_>>()
                        ),
                    });
                }
            }
        }
    }

    Ok(output)
}

// ============================================================================
// Internal: Text Extraction
// ============================================================================

/// Extract text from source bytes at the given byte range.
fn extract_text(source: &[u8], start: usize, end: usize) -> String {
    let end = end.min(source.len());
    let start = start.min(end);
    String::from_utf8_lossy(&source[start..end]).into_owned()
}

// ============================================================================
// Internal: Tagged Instantiation Expansion
// ============================================================================

/// Expand `@@SystemName(args)` tagged instantiations in native code.
///
/// In native code regions, users write `@@SystemName(args)` which gets expanded
/// to the appropriate constructor syntax for the target language:
/// - Python: `SystemName(args)`
/// - TypeScript: `new SystemName(args)`
/// - Rust: `SystemName::new(args)`
/// - C: `SystemName_new(args)`
/// - C++/Java/C#: `new SystemName(args)`
fn expand_tagged_instantiations(
    text: &str,
    defined_systems: &HashSet<String>,
    lang: TargetLanguage,
) -> Result<String, AssemblyError> {
    let bytes = text.as_bytes();
    let mut result = String::new();
    let mut i = 0;

    // Determine comment style based on language
    let uses_hash_comments = matches!(lang, TargetLanguage::Python3);
    let uses_c_style_comments = matches!(
        lang,
        TargetLanguage::TypeScript
            | TargetLanguage::Rust
            | TargetLanguage::C
            | TargetLanguage::Cpp
            | TargetLanguage::Java
            | TargetLanguage::CSharp
    );

    while i < bytes.len() {
        // Skip # comments (Python)
        if uses_hash_comments && bytes[i] == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Skip // and /* */ comments (C-style languages)
        if uses_c_style_comments && bytes[i] == b'/' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                while i < bytes.len() && bytes[i] != b'\n' {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                continue;
            }
            if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                result.push(bytes[i] as char);
                result.push(bytes[i + 1] as char);
                i += 2;
                while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                if i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    result.push(bytes[i + 1] as char);
                    i += 2;
                }
                continue;
            }
        }

        // Skip string literals (double-quoted)
        if bytes[i] == b'"' {
            result.push(bytes[i] as char);
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes.len() {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Skip single-quoted strings/chars
        if bytes[i] == b'\'' {
            result.push(bytes[i] as char);
            i += 1;
            while i < bytes.len() && bytes[i] != b'\'' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes.len() {
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        // Look for @@ pattern
        if i + 2 < bytes.len() && bytes[i] == b'@' && bytes[i + 1] == b'@' {
            let start = i;
            i += 2;

            // Check for uppercase letter (system name start)
            if i < bytes.len() && bytes[i].is_ascii_uppercase() {
                let name_start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let name = std::str::from_utf8(&bytes[name_start..i]).unwrap_or("");

                // Check for opening paren
                if i < bytes.len() && bytes[i] == b'(' {
                    let args_start = i + 1;
                    let mut paren_depth = 1;
                    i += 1;
                    while i < bytes.len() && paren_depth > 0 {
                        match bytes[i] {
                            b'(' => paren_depth += 1,
                            b')' => paren_depth -= 1,
                            b'"' => {
                                i += 1;
                                while i < bytes.len() && bytes[i] != b'"' {
                                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                                        i += 1;
                                    }
                                    i += 1;
                                }
                            }
                            b'\'' => {
                                i += 1;
                                while i < bytes.len() && bytes[i] != b'\'' {
                                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                                        i += 1;
                                    }
                                    i += 1;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }

                    if paren_depth == 0 {
                        if defined_systems.contains(name) {
                            let args = std::str::from_utf8(&bytes[args_start..i - 1])
                                .unwrap_or("");
                            let constructor = generate_constructor(name, args, lang);
                            result.push_str(&constructor);
                            continue;
                        } else {
                            return Err(AssemblyError {
                                message: format!(
                                    "Undefined system '{}' in tagged instantiation. Available: {:?}",
                                    name, defined_systems
                                ),
                            });
                        }
                    }
                }
            }

            // Not a valid tagged instantiation — copy original @@ chars
            for b in &bytes[start..i] {
                result.push(*b as char);
            }
            continue;
        }

        // Regular character — copy through
        result.push(bytes[i] as char);
        i += 1;
    }

    Ok(result)
}

/// Generate the language-appropriate constructor call for a system.
fn generate_constructor(name: &str, args: &str, lang: TargetLanguage) -> String {
    match lang {
        TargetLanguage::Python3 => {
            format!("{}({})", name, args)
        }
        TargetLanguage::TypeScript => {
            format!("new {}({})", name, args)
        }
        TargetLanguage::Rust => {
            if args.trim().is_empty() {
                format!("{}::new()", name)
            } else {
                format!("{}::new({})", name, args)
            }
        }
        TargetLanguage::C => {
            if args.trim().is_empty() {
                format!("{}_new()", name)
            } else {
                format!("{}_new({})", name, args)
            }
        }
        TargetLanguage::Cpp | TargetLanguage::Java | TargetLanguage::CSharp => {
            format!("new {}({})", name, args)
        }
        _ => {
            format!("{}({})", name, args)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::segmenter::Span;

    /// Helper: make a SourceMap manually for testing
    fn make_source_map(source: &str, segments: Vec<Segment>) -> SourceMap {
        SourceMap {
            segments,
            source: source.as_bytes().to_vec(),
            target: Some(TargetLanguage::Python3),
        }
    }

    #[test]
    fn test_native_only() {
        let src = "import math\nprint('hello')\n";
        let map = make_source_map(src, vec![
            Segment::Native { span: Span { start: 0, end: src.len() } },
        ]);
        let result = assemble(&map, &[], TargetLanguage::Python3).unwrap();
        assert_eq!(result, src);
    }

    #[test]
    fn test_system_replacement() {
        let src = "prolog\n@@system Foo {\n  machine:\n    $A { }\n}\nepilogue\n";
        let prolog_end = 7;
        let system_start = 7;
        let system_end = 46;
        let epilog_start = 46;
        let map = make_source_map(src, vec![
            Segment::Native { span: Span { start: 0, end: prolog_end } },
            Segment::System {
                outer_span: Span { start: system_start, end: system_end },
                body_span: Span { start: system_start + 16, end: system_end - 1 },
                name: "Foo".to_string(),
            },
            Segment::Native { span: Span { start: epilog_start, end: src.len() } },
        ]);
        let generated = vec![("Foo".to_string(), "class Foo:\n  pass\n".to_string())];
        let result = assemble(&map, &generated, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "prolog\nclass Foo:\n  pass\nepilogue\n");
    }

    #[test]
    fn test_pragma_skipped() {
        let src = "@@target python_3\nimport os\n";
        let map = make_source_map(src, vec![
            Segment::Pragma {
                kind: crate::frame_c::v4::segmenter::PragmaKind::Target,
                span: Span { start: 0, end: 18 },
                value: Some("python_3".to_string()),
            },
            Segment::Native { span: Span { start: 18, end: src.len() } },
        ]);
        let result = assemble(&map, &[], TargetLanguage::Python3).unwrap();
        assert_eq!(result, "import os\n");
    }

    #[test]
    fn test_tagged_instantiation_python() {
        let src = "s = @@Foo()\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "s = Foo()\n");
    }

    #[test]
    fn test_tagged_instantiation_typescript() {
        let src = "let s = @@Foo()\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::TypeScript).unwrap();
        assert_eq!(result, "let s = new Foo()\n");
    }

    #[test]
    fn test_tagged_instantiation_rust() {
        let src = "let s = @@Foo();\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Rust).unwrap();
        assert_eq!(result, "let s = Foo::new();\n");
    }

    #[test]
    fn test_tagged_instantiation_c() {
        let src = "struct Foo* s = @@Foo();\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::C).unwrap();
        assert_eq!(result, "struct Foo* s = Foo_new();\n");
    }

    #[test]
    fn test_tagged_instantiation_with_args() {
        let src = "s = @@Foo(1, \"hello\")\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "s = Foo(1, \"hello\")\n");
    }

    #[test]
    fn test_tagged_instantiation_in_comment_not_expanded() {
        let src = "# s = @@Foo()\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "# s = @@Foo()\n");
    }

    #[test]
    fn test_tagged_instantiation_in_string_not_expanded() {
        let src = "s = \"@@Foo()\"\n";
        let systems: HashSet<String> = vec!["Foo".to_string()].into_iter().collect();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "s = \"@@Foo()\"\n");
    }

    #[test]
    fn test_multiple_systems() {
        // source with prolog, two systems, interstitial native, epilog
        let src = "prolog\n__SYS1__\nnative_between\n__SYS2__\nepilogue\n";
        let s1_start = 7;
        let s1_end = 16;
        let between_start = 16;
        let between_end = 31;
        let s2_start = 31;
        let s2_end = 40;
        let epilog_start = 40;

        let map = make_source_map(src, vec![
            Segment::Native { span: Span { start: 0, end: s1_start } },
            Segment::System {
                outer_span: Span { start: s1_start, end: s1_end },
                body_span: Span { start: s1_start + 2, end: s1_end - 2 },
                name: "Alpha".to_string(),
            },
            Segment::Native { span: Span { start: between_start, end: between_end } },
            Segment::System {
                outer_span: Span { start: s2_start, end: s2_end },
                body_span: Span { start: s2_start + 2, end: s2_end - 2 },
                name: "Beta".to_string(),
            },
            Segment::Native { span: Span { start: epilog_start, end: src.len() } },
        ]);

        let generated = vec![
            ("Alpha".to_string(), "class Alpha: pass\n".to_string()),
            ("Beta".to_string(), "class Beta: pass\n".to_string()),
        ];
        let result = assemble(&map, &generated, TargetLanguage::Python3).unwrap();
        assert!(result.contains("prolog\n"));
        assert!(result.contains("class Alpha: pass\n"));
        assert!(result.contains("\nnative_between\n"));
        assert!(result.contains("class Beta: pass\n"));
        assert!(result.contains("epilogue\n"));
    }

    #[test]
    fn test_missing_system_code_errors() {
        let src = "@@system Foo { }";
        let map = make_source_map(src, vec![
            Segment::System {
                outer_span: Span { start: 0, end: src.len() },
                body_span: Span { start: 14, end: 15 },
                name: "Foo".to_string(),
            },
        ]);
        let result = assemble(&map, &[], TargetLanguage::Python3);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Foo"));
    }

    #[test]
    fn test_undefined_tagged_instantiation_errors() {
        let src = "s = @@Unknown()\n";
        let systems: HashSet<String> = HashSet::new();
        let result = expand_tagged_instantiations(src, &systems, TargetLanguage::Python3);
        assert!(result.is_err());
    }

    #[test]
    fn test_full_assembly_with_tagged_instantiation() {
        // prolog creates an instance using tagged instantiation
        let src_native = "s = @@MySystem()\n";
        let src_system = "@@system MySystem { machine: $A { } }";
        let full_src = format!("{}{}", src_native, src_system);
        let native_end = src_native.len();

        let map = make_source_map(&full_src, vec![
            Segment::Native { span: Span { start: 0, end: native_end } },
            Segment::System {
                outer_span: Span { start: native_end, end: full_src.len() },
                body_span: Span { start: native_end + 20, end: full_src.len() - 1 },
                name: "MySystem".to_string(),
            },
        ]);

        let generated = vec![
            ("MySystem".to_string(), "class MySystem:\n  pass\n".to_string()),
        ];
        let result = assemble(&map, &generated, TargetLanguage::Python3).unwrap();
        assert_eq!(result, "s = MySystem()\nclass MySystem:\n  pass\n");
    }
}
