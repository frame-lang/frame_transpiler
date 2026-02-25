//! PragmaScanner - Identifies Frame pragma lines while respecting language-specific strings/comments
//!
//! This scanner separates Frame pragmas (`@@target`, `@@system`, etc.) from native code regions.
//! It uses the SyntaxSkipper trait to properly handle language-specific string and comment syntax,
//! ensuring that `@@` appearing inside strings or comments is NOT recognized as a pragma.
//!
//! # Architecture
//!
//! The scanner operates byte-by-byte, tracking:
//! - Start-of-line (SOL) position for pragma detection
//! - Protected regions (strings/comments) via SyntaxSkipper
//!
//! # Output
//!
//! Returns `Vec<PragmaRegion>` where each region is either:
//! - `NativeText` - Native code that passes through verbatim
//! - `PragmaLine` - A Frame pragma line with its kind
//!
//! # Example
//!
//! ```text
//! import os                    # NativeText
//! @@target python_3            # PragmaLine(Target)
//! def foo():                   # NativeText
//!     print("@@target")        # NativeText (@@target inside string)
//! @@system MySystem {          # PragmaLine(System)
//! ```

use super::native_region_scanner::RegionSpan;
use super::native_region_scanner::unified::SyntaxSkipper;

/// The kind of Frame pragma
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PragmaKind {
    /// @@target <language>
    Target,
    /// @@codegen { ... }
    Codegen,
    /// @@system <Name> { ... }
    System,
    /// @@persist
    Persist,
    /// @@async
    Async,
    /// @@run-expect <pattern>
    RunExpect,
    /// Unknown or future pragma
    Other,
}

/// A region in the source file - either native code or a pragma line
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PragmaRegion {
    /// Native code - passes through the transpiler unchanged
    NativeText { span: RegionSpan },
    /// Frame pragma line
    PragmaLine { span: RegionSpan, kind: PragmaKind },
}

/// Result of pragma scanning
#[derive(Debug)]
pub struct PragmaScanResult {
    pub regions: Vec<PragmaRegion>,
}

/// Error during pragma scanning
#[derive(Debug)]
pub struct PragmaScanError {
    pub message: String,
}

/// PragmaScanner - identifies Frame pragmas while respecting strings/comments
pub struct PragmaScanner;

impl PragmaScanner {
    /// Scan source bytes and return pragma vs native regions
    ///
    /// # Arguments
    /// * `skipper` - Language-specific syntax skipper for strings/comments
    /// * `bytes` - Source file bytes
    ///
    /// # Returns
    /// * `PragmaScanResult` with regions identifying pragmas and native code
    pub fn scan<S: SyntaxSkipper>(
        &self,
        skipper: &S,
        bytes: &[u8],
    ) -> Result<PragmaScanResult, PragmaScanError> {
        let mut regions: Vec<PragmaRegion> = Vec::new();
        let n = bytes.len();
        let mut i = 0usize;
        let mut seg_start = 0usize;
        let mut at_sol = true;

        while i < n {
            let b = bytes[i];

            // At start of line, check for @@ pragma
            if at_sol {
                // Skip leading whitespace
                let line_start = i;
                while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') {
                    i += 1;
                }

                // Check for @@ pragma
                if i + 1 < n && bytes[i] == b'@' && bytes[i + 1] == b'@' {
                    // Found pragma - emit any preceding native text
                    if seg_start < line_start {
                        regions.push(PragmaRegion::NativeText {
                            span: RegionSpan { start: seg_start, end: line_start },
                        });
                    }

                    // Find end of pragma line
                    let pragma_start = i;
                    let kind = Self::identify_pragma_kind(bytes, i);

                    // For @@codegen and @@system, find the closing brace
                    let pragma_end = if matches!(kind, PragmaKind::Codegen | PragmaKind::System) {
                        Self::find_pragma_block_end(skipper, bytes, i)
                    } else {
                        // Simple pragma - find end of line
                        Self::find_line_end(bytes, i)
                    };

                    regions.push(PragmaRegion::PragmaLine {
                        span: RegionSpan { start: pragma_start, end: pragma_end },
                        kind,
                    });

                    i = pragma_end;
                    seg_start = i;
                    at_sol = true;
                    continue;
                }

                // Not a pragma - continue scanning as native code
                at_sol = false;
                i = line_start; // Reset to line start for normal scanning
            }

            // Try to skip protected regions (strings/comments)
            if let Some(new_i) = skipper.skip_comment(bytes, i, n) {
                i = new_i;
                continue;
            }
            if let Some(new_i) = skipper.skip_string(bytes, i, n) {
                i = new_i;
                continue;
            }

            // Track newlines for SOL detection
            if b == b'\n' {
                at_sol = true;
                i += 1;
                continue;
            }

            i += 1;
        }

        // Emit any remaining native text
        if seg_start < n {
            regions.push(PragmaRegion::NativeText {
                span: RegionSpan { start: seg_start, end: n },
            });
        }

        Ok(PragmaScanResult { regions })
    }

    /// Identify the kind of pragma from @@<keyword>
    fn identify_pragma_kind(bytes: &[u8], start: usize) -> PragmaKind {
        let n = bytes.len();
        let mut i = start + 2; // Skip @@

        // Extract keyword
        let kw_start = i;
        while i < n && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'-') {
            i += 1;
        }

        let keyword = &bytes[kw_start..i];

        match keyword {
            b"target" => PragmaKind::Target,
            b"codegen" => PragmaKind::Codegen,
            b"system" => PragmaKind::System,
            b"persist" => PragmaKind::Persist,
            b"async" => PragmaKind::Async,
            b"run-expect" => PragmaKind::RunExpect,
            _ => PragmaKind::Other,
        }
    }

    /// Find end of a simple pragma line (no block)
    fn find_line_end(bytes: &[u8], start: usize) -> usize {
        let n = bytes.len();
        let mut i = start;
        while i < n && bytes[i] != b'\n' {
            i += 1;
        }
        if i < n && bytes[i] == b'\n' {
            i += 1; // Include the newline
        }
        i
    }

    /// Find end of a pragma block (@@codegen { ... } or @@system Name { ... })
    fn find_pragma_block_end<S: SyntaxSkipper>(
        skipper: &S,
        bytes: &[u8],
        start: usize,
    ) -> usize {
        let n = bytes.len();
        let mut i = start;

        // Find opening brace
        while i < n && bytes[i] != b'{' {
            i += 1;
        }

        if i >= n {
            // No opening brace found - treat as simple pragma
            return Self::find_line_end(bytes, start);
        }

        // Use body_closer to find matching brace
        let mut closer = skipper.body_closer();
        match closer.close_byte(bytes, i) {
            Ok(close_pos) => {
                let mut end = close_pos + 1;
                // Include trailing newline if present
                if end < n && bytes[end] == b'\n' {
                    end += 1;
                }
                end
            }
            Err(_) => {
                // Fallback: scan to end of file
                n
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test skipper that handles // and "..."
    struct TestSkipper;

    impl SyntaxSkipper for TestSkipper {
        fn body_closer(&self) -> Box<dyn super::super::body_closer::BodyCloser> {
            Box::new(super::super::body_closer::python::BodyCloserPy)
        }

        fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
            // Skip // to end of line
            if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                let mut j = i + 2;
                while j < end && bytes[j] != b'\n' {
                    j += 1;
                }
                return Some(j);
            }
            // Skip # to end of line (Python)
            if bytes[i] == b'#' {
                let mut j = i + 1;
                while j < end && bytes[j] != b'\n' {
                    j += 1;
                }
                return Some(j);
            }
            None
        }

        fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
            // Skip "..." with backslash escapes
            if bytes[i] == b'"' {
                let mut j = i + 1;
                let mut escaped = false;
                while j < end {
                    if escaped {
                        escaped = false;
                        j += 1;
                        continue;
                    }
                    if bytes[j] == b'\\' {
                        escaped = true;
                        j += 1;
                        continue;
                    }
                    if bytes[j] == b'"' {
                        return Some(j + 1);
                    }
                    j += 1;
                }
            }
            None
        }

        fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
            let mut i = start;
            while i < end && bytes[i] != b'\n' {
                i += 1;
            }
            i
        }

        fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
            let mut depth = 0;
            let mut j = i;
            while j < end {
                match bytes[j] {
                    b'(' => depth += 1,
                    b')' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(j + 1);
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            None
        }
    }

    #[test]
    fn test_simple_pragma() {
        let source = b"@@target python_3\n";
        let scanner = PragmaScanner;
        let result = scanner.scan(&TestSkipper, source).unwrap();

        assert_eq!(result.regions.len(), 1);
        match &result.regions[0] {
            PragmaRegion::PragmaLine { kind, .. } => {
                assert_eq!(*kind, PragmaKind::Target);
            }
            _ => panic!("Expected PragmaLine"),
        }
    }

    #[test]
    fn test_pragma_in_string_ignored() {
        let source = b"print(\"@@target python_3\")\n";
        let scanner = PragmaScanner;
        let result = scanner.scan(&TestSkipper, source).unwrap();

        assert_eq!(result.regions.len(), 1);
        match &result.regions[0] {
            PragmaRegion::NativeText { .. } => {}
            _ => panic!("Expected NativeText - @@target inside string should be ignored"),
        }
    }

    #[test]
    fn test_pragma_in_comment_ignored() {
        let source = b"# @@target python_3\n";
        let scanner = PragmaScanner;
        let result = scanner.scan(&TestSkipper, source).unwrap();

        assert_eq!(result.regions.len(), 1);
        match &result.regions[0] {
            PragmaRegion::NativeText { .. } => {}
            _ => panic!("Expected NativeText - @@target inside comment should be ignored"),
        }
    }

    #[test]
    fn test_mixed_content() {
        let source = b"import os\n@@target python_3\ndef foo(): pass\n";
        let scanner = PragmaScanner;
        let result = scanner.scan(&TestSkipper, source).unwrap();

        assert_eq!(result.regions.len(), 3);
        assert!(matches!(&result.regions[0], PragmaRegion::NativeText { .. }));
        assert!(matches!(&result.regions[1], PragmaRegion::PragmaLine { kind: PragmaKind::Target, .. }));
        assert!(matches!(&result.regions[2], PragmaRegion::NativeText { .. }));
    }
}
