use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Default)]
pub struct InterfaceMethodMeta {
    /// Optional declared return type (target-language specific, e.g. TypeScript type expr).
    pub return_type: Option<String>,
    /// Optional initializer expression for system.return (raw text in header scope).
    pub return_init: Option<String>,
}

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
                self.collect_names_in_system(bytes, span.start, span.end, &mut names);
            }
        }
        names
    }

    /// Collect per-system interface method metadata (return type and initializer)
    /// keyed by system name and method name.
    ///
    /// The returned map is:
    ///   system_name -> { method_name -> InterfaceMethodMeta }
    pub fn collect_method_metadata(
        &self,
        bytes: &[u8],
        module: &ModuleAst,
        _lang: TargetLanguage,
    ) -> HashMap<String, HashMap<String, InterfaceMethodMeta>> {
        let mut out: HashMap<String, HashMap<String, InterfaceMethodMeta>> = HashMap::new();
        for sys in &module.systems {
            if let Some(span) = sys.sections.interface {
                let mut per_system: HashMap<String, InterfaceMethodMeta> = HashMap::new();
                self.collect_meta_in_system(bytes, span.start, span.end, &mut per_system);
                if !per_system.is_empty() {
                    out.insert(sys.name.clone(), per_system);
                }
            }
        }
        out
    }

    fn collect_names_in_system(
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

    fn collect_meta_in_system(
        &self,
        bytes: &[u8],
        start: usize,
        end: usize,
        out: &mut HashMap<String, InterfaceMethodMeta>,
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

            // Skip comment-only lines quickly.
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
                while i < end && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }

            // Extract full line slice for header parsing.
            let mut line_end = j;
            // Advance to end of line.
            while line_end < end && bytes[line_end] != b'\n' {
                line_end += 1;
            }
            let line_bytes = &bytes[line_start..line_end];
            if let Ok(line_str) = std::str::from_utf8(line_bytes) {
                if let Some((name, meta)) = parse_interface_header_meta(line_str) {
                    out.insert(name, meta);
                }
            }

            // Move to next line.
            i = line_end;
            if i < end {
                i += 1;
            }
        }
    }
}

/// Best-effort parser for a single interface header line.
/// Recognizes: `name(params)[: TypeExpr][ = Expr]`
pub(crate) fn parse_interface_header_meta(line: &str) -> Option<(String, InterfaceMethodMeta)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    // First identifier
    let mut chars = trimmed.char_indices();
    let (name_start, ch0) = chars.next()?;
    if !(ch0.is_ascii_alphabetic() || ch0 == '_' || ch0 == '$') {
        return None;
    }
    let mut name_end = name_start + ch0.len_utf8();
    for (idx, ch) in chars.by_ref() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            name_end = idx + ch.len_utf8();
        } else {
            break;
        }
    }
    let name = trimmed[name_start..name_end].to_string();
    // Find parameter list parentheses.
    let rest = &trimmed[name_end..];
    let lp = rest.find('(')?; // must exist
    // naive paren match: find first ')'
    let rp_rel = rest[lp + 1..].find(')')?;
    let after_params = lp + 1 + rp_rel + 1;
    let mut return_type: Option<String> = None;
    let mut return_init: Option<String> = None;
    let mut tail = rest[after_params..].trim_start();

    // Optional : TypeExpr
    if tail.starts_with(':') {
        tail = tail[1..].trim_start();
        let mut end = tail.len();
        // TypeExpr runs until '=' or comment start
        for (idx, ch) in tail.char_indices() {
            if ch == '=' || ch == '#' || (ch == '/' && tail[idx + ch.len_utf8()..].starts_with('/')) {
                end = idx;
                break;
            }
        }
        let ty = tail[..end].trim();
        if !ty.is_empty() {
            return_type = Some(ty.to_string());
        }
        tail = tail[end..].trim_start();
    }

    // Optional = Expr
    if tail.starts_with('=') {
        tail = tail[1..].trim_start();
        let mut end = tail.len();
        for (idx, ch) in tail.char_indices() {
            if ch == '#' || (ch == '/' && tail[idx + ch.len_utf8()..].starts_with('/')) {
                end = idx;
                break;
            }
        }
        let expr = tail[..end].trim();
        if !expr.is_empty() {
            return_init = Some(expr.to_string());
        }
    }

    Some((name, InterfaceMethodMeta { return_type, return_init }))
}
