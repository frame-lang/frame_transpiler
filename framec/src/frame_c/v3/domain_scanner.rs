use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::body_closer::{self as closer, BodyCloserV3};

/// Issue produced when scanning a `domain:` block.
/// The message is a human-readable description; the caller wraps it into
/// whatever error/diagnostic type it needs.
#[derive(Debug, Clone)]
pub struct DomainDeclIssue {
    pub system_name: String,
    pub message: String,
}

/// Scanner for `domain:` blocks inside a system.
///
/// Semantics:
/// - `domain:` is a native block; we do not parse expressions.
/// - At SOL inside `domain:`, only declaration-shaped lines are allowed:
///     - `var ident = <expr>`
///     - `ident = <expr>`
/// - Any other non-blank, non-comment line is reported as E419.
pub struct DomainBlockScannerV3;

impl DomainBlockScannerV3 {
    pub fn validate_decls_only(
        &self,
        bytes: &[u8],
        start: usize,
        lang: TargetLanguage,
    ) -> Vec<DomainDeclIssue> {
        let mut issues = Vec::new();

        fn is_space(b: u8) -> bool {
            b == b' ' || b == b'\t'
        }

        fn close_system(bytes: &[u8], open: usize, lang: TargetLanguage) -> Option<usize> {
            match lang {
                TargetLanguage::Python3 => closer::python::BodyCloserPyV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::TypeScript => closer::typescript::BodyCloserTsV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::CSharp => closer::csharp::BodyCloserCsV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::C => closer::c::BodyCloserCV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::Cpp => closer::cpp::BodyCloserCppV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::Java => closer::java::BodyCloserJavaV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                TargetLanguage::Rust => closer::rust::BodyCloserRustV3
                    .close_byte(&bytes[open..], 0)
                    .ok()
                    .map(|c| open + c),
                _ => None,
            }
        }

        let n = bytes.len();
        let mut i = start;

        while i < n {
            // Skip whitespace and blank lines.
            while i < n
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n')
            {
                i += 1;
            }
            if i >= n {
                break;
            }
            // Skip comment-only lines quickly.
            if bytes[i] == b'#' {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            if i + 1 < n && bytes[i] == b'/' {
                let c2 = bytes[i + 1];
                if c2 == b'/' {
                    while i < n && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                } else if c2 == b'*' {
                    i += 2;
                    while i + 1 < n {
                        if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                            i += 2;
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }
            }

            // Look for `system` keyword at SOL.
            let mut j = i;
            while j < n && is_space(bytes[j]) {
                j += 1;
            }
            let kw_start = j;
            while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if kw_start == j {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
            if kw.as_str() != "system" {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }

            // Read system name.
            let mut k = j;
            while k < n && is_space(bytes[k]) {
                k += 1;
            }
            let name_start = k;
            while k < n && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'_') {
                k += 1;
            }
            if name_start == k {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let sys_name = String::from_utf8_lossy(&bytes[name_start..k]).to_string();

            // Find opening '{' for this system.
            while k < n && bytes[k] != b'{' && bytes[k] != b'\n' {
                k += 1;
            }
            if k >= n || bytes[k] != b'{' {
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            let open = k;
            let close = match close_system(bytes, open, lang) {
                Some(c) => c,
                None => {
                    while i < n && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
            };

            // Collect section markers inside this system.
            let mut marks: Vec<(usize, String)> = Vec::new();
            let mut q = open + 1;
            while q < close {
                while q < close
                    && (bytes[q] == b' ' || bytes[q] == b'\t' || bytes[q] == b'\r' || bytes[q] == b'\n')
                {
                    q += 1;
                }
                if q >= close {
                    break;
                }
                let line = q;
                // Skip comments.
                if bytes[q] == b'#' {
                    while q < close && bytes[q] != b'\n' {
                        q += 1;
                    }
                    continue;
                }
                if q + 1 < close && bytes[q] == b'/' {
                    let c2 = bytes[q + 1];
                    if c2 == b'/' {
                        while q < close && bytes[q] != b'\n' {
                            q += 1;
                        }
                        continue;
                    } else if c2 == b'*' {
                        q += 2;
                        while q + 1 < close {
                            if bytes[q] == b'*' && bytes[q + 1] == b'/' {
                                q += 2;
                                break;
                            }
                            q += 1;
                        }
                        continue;
                    }
                }
                let mut s = q;
                while s < close && (bytes[s] == b' ' || bytes[s] == b'\t') {
                    s += 1;
                }
                let sec_start = s;
                while s < close && (bytes[s].is_ascii_alphanumeric() || bytes[s] == b'_') {
                    s += 1;
                }
                if sec_start < s && s < close && bytes[s] == b':' {
                    let kw_sec =
                        String::from_utf8_lossy(&bytes[sec_start..s]).to_ascii_lowercase();
                    if kw_sec.as_str() == "domain"
                        || kw_sec.as_str() == "operations"
                        || kw_sec.as_str() == "interface"
                        || kw_sec.as_str() == "machine"
                        || kw_sec.as_str() == "actions"
                    {
                        marks.push((line, kw_sec));
                    }
                }
                while q < close && bytes[q] != b'\n' {
                    q += 1;
                }
            }

            // Find domain block range, if present.
            if let Some((idx, _)) = marks
                .iter()
                .enumerate()
                .find(|(_, (_, kw_sec))| kw_sec.as_str() == "domain")
            {
                let dom_start = marks[idx].0;
                let dom_end = if idx + 1 < marks.len() {
                    marks[idx + 1].0
                } else {
                    close
                };

                // Skip the `domain:` line itself.
                let mut p = dom_start;
                while p < dom_end && bytes[p] != b'\n' {
                    p += 1;
                }
                if p < dom_end {
                    p += 1;
                }

                while p < dom_end {
                    let line_start = p;
                    // Move to end of line first to compute bounds.
                    let mut line_end = line_start;
                    while line_end < dom_end && bytes[line_end] != b'\n' {
                        line_end += 1;
                    }
                    // Find first non-space/tabs.
                    let mut s = line_start;
                    while s < line_end && (bytes[s] == b' ' || bytes[s] == b'\t') {
                        s += 1;
                    }
                    if s >= line_end {
                        // blank line
                        p = if line_end < dom_end { line_end + 1 } else { dom_end };
                        continue;
                    }
                    // Skip comment-only lines.
                    if bytes[s] == b'#' {
                        p = if line_end < dom_end { line_end + 1 } else { dom_end };
                        continue;
                    }
                    if s + 1 < line_end && bytes[s] == b'/' && bytes[s + 1] == b'/' {
                        p = if line_end < dom_end { line_end + 1 } else { dom_end };
                        continue;
                    }
                    // Optional 'var' keyword.
                    let mut ident_start = s;
                    if bytes[s].is_ascii_alphabetic() {
                        let mut w = s;
                        while w < line_end && bytes[w].is_ascii_alphabetic() {
                            w += 1;
                        }
                        let word = String::from_utf8_lossy(&bytes[s..w]).to_ascii_lowercase();
                        if word == "var" {
                            // Move to identifier after 'var'.
                            let mut k2 = w;
                            while k2 < line_end
                                && (bytes[k2] == b' ' || bytes[k2] == b'\t')
                            {
                                k2 += 1;
                            }
                            ident_start = k2;
                        }
                    }
                    // Read identifier.
                    let mut ident_end = ident_start;
                    while ident_end < line_end
                        && ((bytes[ident_end] as char).is_ascii_alphanumeric()
                            || bytes[ident_end] == b'_')
                    {
                        ident_end += 1;
                    }
                    let has_ident = ident_end > ident_start;
                    // Require '=' somewhere after the identifier on the same line.
                    let mut has_equal = false;
                    let mut t = ident_end;
                    while t < line_end {
                        if bytes[t] == b'=' {
                            has_equal = true;
                            break;
                        }
                        t += 1;
                    }
                    if !has_ident || !has_equal {
                        issues.push(DomainDeclIssue {
                            system_name: sys_name.clone(),
                            message: format!(
                                "E419: domain block for system '{}' may only contain declarations",
                                sys_name
                            ),
                        });
                    }

                    p = if line_end < dom_end { line_end + 1 } else { dom_end };
                }
            }

            // Advance past this system.
            i = close + 1;
        }

        issues
    }
}
