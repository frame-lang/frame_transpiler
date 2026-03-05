use std::collections::HashMap;

use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::ast::{Span, StateDecl};
use crate::frame_c::visitors::TargetLanguage;
// Choose the first declared state for a system via Arcanum spans.
pub(crate) fn first_state_for_system<'a>(arc: &'a Arcanum, sys_name: &str) -> Option<&'a StateDecl> {
    let sys = arc.systems.get(sys_name)?;
    let mut best: Option<&StateDecl> = None;
    let mut best_start: Option<usize> = None;
    for mach in sys.machines.values() {
        for st in mach.states.values() {
            match best_start {
                None => {
                    best_start = Some(st.span.start);
                    best = Some(st);
                }
                Some(cur) => {
                    if st.span.start < cur {
                        best_start = Some(st.span.start);
                        best = Some(st);
                    }
                }
            }
        }
    }
    best
}

// Extract bare parameter names from a header string.
pub(crate) fn header_param_names(hdr: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(lp) = hdr.find('(') {
        if let Some(rp_rel) = hdr[lp + 1..].find(')') {
            let rp = lp + 1 + rp_rel;
            let inside = &hdr[lp + 1..rp];
            for raw in inside.split(',') {
                let t = raw.trim();
                if t.is_empty() {
                    continue;
                }
                let base = t.split(|c| c == '=' || c == ':').next().unwrap_or("").trim();
                if !base.is_empty() {
                    names.push(base.to_string());
                }
            }
        }
    }
    names
}

/// Extract parameter names from a `$State(...)` header using the state's span.
/// This reads the header line starting at `span.start` up to the end of line
/// and then delegates to `header_param_names`.
#[allow(dead_code)]
pub(crate) fn state_header_param_names(bytes: &[u8], span: &Span) -> Vec<String> {
    let n = bytes.len();
    if span.start >= n {
        return Vec::new();
    }
    let mut line_end = span.start;
    while line_end < n && bytes[line_end] != b'\n' {
        line_end += 1;
    }
    let hdr = String::from_utf8_lossy(&bytes[span.start..line_end]).to_string();
    header_param_names(&hdr)
}

// Collect domain variable names per system by scanning each `domain:` block.
pub(crate) fn collect_domain_vars_per_system(
    bytes: &[u8],
    start: usize,
    lang: TargetLanguage,
) -> HashMap<String, Vec<String>> {
    use crate::frame_c::v4::body_closer as closer;
    use std::collections::HashMap as Map;

    fn close_system(bytes: &[u8], open: usize, lang: TargetLanguage) -> Option<usize> {
        closer::close_body(bytes, open, lang).ok()
    }

    let mut result: Map<String, Vec<String>> = Map::new();
    let n = bytes.len();
    let mut i = start;

    while i < n {
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') {
            i += 1;
        }
        if i >= n {
            break;
        }
        // Skip comment-only lines.
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
        // Read potential `@@system` keyword, optionally preceded by `@@persist`.
        let mut j = i;
        while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') {
            j += 1;
        }
        // Optional system-level attribute: `@@persist`.
        if j + 9 <= n && &bytes[j..j+9] == b"@@persist" {
            j += 9;
            while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') {
                j += 1;
            }
        }
        // Require `@@system` keyword.
        if j + 8 > n || &bytes[j..j+8] != b"@@system" {
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        j += 8;
        // Read system name.
        let mut k = j;
        while k < n && (bytes[k] == b' ' || bytes[k] == b'\t') {
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
            while q < close && (bytes[q] == b' ' || bytes[q] == b'\t' || bytes[q] == b'\r' || bytes[q] == b'\n') {
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
                let kw = String::from_utf8_lossy(&bytes[sec_start..s]).to_ascii_lowercase();
                if kw.as_str() == "domain"
                    || kw.as_str() == "operations"
                    || kw.as_str() == "interface"
                    || kw.as_str() == "machine"
                    || kw.as_str() == "actions"
                {
                    marks.push((line, kw));
                }
            }
            while q < close && bytes[q] != b'\n' {
                q += 1;
            }
        }

        // Find domain block range, if present.
        if let Some((idx, (_, _))) =
            marks.iter().enumerate().find(|(_, (_, kw))| kw.as_str() == "domain")
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
            let mut names: Vec<String> = Vec::new();
            while p < dom_end {
                // Start of line inside domain block.
                let mut s = p;
                // Move to first non-space/tabs.
                while s < dom_end && (bytes[s] == b' ' || bytes[s] == b'\t') {
                    s += 1;
                }
                if s >= dom_end {
                    break;
                }
                // Skip blank or comment-only lines.
                if bytes[s] == b'#' {
                    while p < dom_end && bytes[p] != b'\n' {
                        p += 1;
                    }
                    if p < dom_end {
                        p += 1;
                    }
                    continue;
                }
                // Optional 'var' keyword.
                let mut j = s;
                while j < dom_end && bytes[j].is_ascii_alphabetic() {
                    j += 1;
                }
                let mut ident_start = s;
                if j > s {
                    let word = String::from_utf8_lossy(&bytes[s..j]).to_ascii_lowercase();
                    if word == "var" {
                        // Skip 'var' and following whitespace.
                        let mut k2 = j;
                        while k2 < dom_end && (bytes[k2] == b' ' || bytes[k2] == b'\t') {
                            k2 += 1;
                        }
                        ident_start = k2;
                    }
                }
                // Read identifier.
                let mut ident_end = ident_start;
                if ident_start < dom_end
                    && ((bytes[ident_start] as char).is_ascii_alphabetic()
                        || bytes[ident_start] == b'_')
                {
                    ident_end += 1;
                    while ident_end < dom_end
                        && ((bytes[ident_end] as char).is_ascii_alphanumeric()
                            || bytes[ident_end] == b'_')
                    {
                        ident_end += 1;
                    }
                    let name = String::from_utf8_lossy(&bytes[ident_start..ident_end]).to_string();
                    names.push(name);
                }
                // Move to next line.
                while p < dom_end && bytes[p] != b'\n' {
                    p += 1;
                }
                if p < dom_end {
                    p += 1;
                }
            }
            if !names.is_empty() {
                result.insert(sys_name.clone(), names);
            }
        }

        // Advance past this system.
        i = close + 1;
        continue;
    }

    result
}
