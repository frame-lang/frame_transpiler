use std::collections::HashSet;

use crate::frame_c::v3::ast::ModuleAst;
use crate::frame_c::visitors::TargetLanguage;

/// Parser utilities for `interface:` sections.
///
/// This is a Frame‑only mini‑parser that walks the `interface:` span for each
/// system and extracts method names from signature‑only lines:
///
///   - `ident(params...)`
///
/// Bodies are not allowed under `interface:`; those semantics are enforced
/// elsewhere in the validator.
pub struct InterfaceParserV3;

impl InterfaceParserV3 {
    /// Collect the union of all interface method names declared across all
    /// systems in a module. This is used to validate `system.method(...)`
    /// calls in native regions (E406).
    pub fn collect_all_interface_method_names(
        &self,
        bytes: &[u8],
        module: &ModuleAst,
        _lang: TargetLanguage,
    ) -> HashSet<String> {
        let mut names = HashSet::new();
        for sys in &module.systems {
            if let Some(span) = sys.sections.interface {
                self.collect_in_system(bytes, span.start, span.end, &mut names);
            }
        }
        names
    }

    fn collect_in_system(
        &self,
        bytes: &[u8],
        start: usize,
        end: usize,
        out: &mut HashSet<String>,
    ) {
        let n = bytes.len();
        let mut i = start.min(n);
        let end = end.min(n);
        while i < end {
            // Move to SOL or next non‑whitespace.
            while i < end
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n')
            {
                i += 1;
            }
            if i >= end {
                break;
            }
            let line_start = i;

            // Skip comment‑only lines quickly.
            if bytes[i] == b'#' {
                while i < end && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            if i + 1 < end && bytes[i] == b'/' {
                let c2 = bytes[i + 1];
                if c2 == b'/' {
                    while i < end && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                } else if c2 == b'*' {
                    i += 2;
                    while i + 1 < end {
                        if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                            i += 2;
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }
            }

            // Skip the `interface:` header line itself if we happen to see it
            // at the beginning of the span.
            let mut j = i;
            while j < end && (bytes[j] == b' ' || bytes[j] == b'\t') {
                j += 1;
            }
            let kw_start = j;
            while j < end && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if kw_start < j && j < end && bytes[j] == b':' {
                // This is the `interface:` line; skip it.
                while i < end && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }

            // Otherwise, treat the first identifier on the line as a potential
            // interface method name, provided it is followed by `(` on the
            // same physical line.
            let name_start = kw_start;
            let name_end = j;
            if name_start < name_end {
                let mut k = j;
                while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') {
                    k += 1;
                }
                if k < end && bytes[k] == b'(' {
                    let name =
                        String::from_utf8_lossy(&bytes[name_start..name_end]).to_string();
                    if !name.is_empty() {
                        out.insert(name);
                    }
                }
            }

            // Move to next line.
            i = line_start;
            while i < end && bytes[i] != b'\n' {
                i += 1;
            }
            if i < end {
                i += 1;
            }
        }
    }
}

