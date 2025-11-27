use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::ast::Span;
use crate::frame_c::v3::system_param_semantics::header_param_names;

/// Parser utilities for `machine:` sections. For now this is focused on entry
/// handlers `$>()` so that system-parameter semantics (E417) can be driven
/// from a dedicated parser instead of ad-hoc scans.
pub struct MachineParserV3;

impl MachineParserV3 {
    /// Find the parameter names for a state's `$>()` handler, if any, by
    /// scanning within the `machine:` section span for the named state and
    /// then looking inside its body for a SOL `$>()` header.
    pub fn find_entry_params_in_machine(
        &self,
        bytes: &[u8],
        machine_span: &Span,
        state_name: &str,
        _lang: TargetLanguage,
    ) -> Option<Vec<String>> {
        let n = bytes.len();
        if machine_span.start >= n || machine_span.end > n {
            return None;
        }

        // First locate the state header `$StateName` inside the machine span.
        let name_bytes = state_name.as_bytes();
        let mut i = machine_span.start;
        let state_body_start = loop {
            if i >= machine_span.end {
                return None;
            }
            // Move to SOL inside machine span.
            while i < machine_span.end
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n')
            {
                i += 1;
            }
            if i >= machine_span.end {
                return None;
            }
            let line_start = i;
            // Skip comment-only lines.
            if bytes[i] == b'#' {
                while i < machine_span.end && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            if i + 1 < machine_span.end && bytes[i] == b'/' && (bytes[i + 1] == b'/' || bytes[i + 1] == b'*') {
                while i < machine_span.end && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            // Look for `$StateName` header.
            if bytes[i] == b'$' {
                let mut k = i + 1;
                while k < machine_span.end
                    && ((bytes[k] as char).is_ascii_alphanumeric() || bytes[k] == b'_')
                {
                    k += 1;
                }
                if &bytes[i + 1..k] == name_bytes {
                    // Found the desired state header; skip to next line to begin scanning body.
                    let mut p = line_start;
                    while p < machine_span.end && bytes[p] != b'\n' {
                        p += 1;
                    }
                    if p < machine_span.end {
                        p += 1;
                    }
                    break p;
                }
            }
            // Move to next line.
            while i < machine_span.end && bytes[i] != b'\n' {
                i += 1;
            }
            if i < machine_span.end {
                i += 1;
            }
        };

        // Now scan from just after the state header downwards until we hit the next state
        // header `$OtherState` or leave the machine span.
        let mut j = state_body_start;
        while j < machine_span.end && j < n {
            let line_start = j;
            let mut line_end = line_start;
            while line_end < machine_span.end && line_end < n && bytes[line_end] != b'\n' {
                line_end += 1;
            }
            // First non-space.
            let mut s = line_start;
            while s < line_end && (bytes[s] == b' ' || bytes[s] == b'\t' || bytes[s] == b'\r') {
                s += 1;
            }
            if s >= line_end {
                j = if line_end < machine_span.end { line_end + 1 } else { machine_span.end };
                continue;
            }
            // Next state header? Stop scanning this state.
            if bytes[s] == b'$' {
                let mut k = s + 1;
                while k < line_end
                    && ((bytes[k] as char).is_ascii_alphanumeric() || bytes[k] == b'_')
                {
                    k += 1;
                }
                if k > s + 1 && &bytes[s + 1..k] != name_bytes {
                    break;
                }
            }
            // Comment-only lines.
            if bytes[s] == b'#' {
                j = if line_end < machine_span.end { line_end + 1 } else { machine_span.end };
                continue;
            }
            if s + 1 < line_end && bytes[s] == b'/' && (bytes[s + 1] == b'/' || bytes[s + 1] == b'*') {
                j = if line_end < machine_span.end { line_end + 1 } else { machine_span.end };
                continue;
            }
            // Look for `$>()` header.
            if bytes[s] == b'$' && s + 1 < line_end && bytes[s + 1] == b'>' {
                let mut hdr_end = s;
                while hdr_end < line_end && bytes[hdr_end] != b'{' {
                    hdr_end += 1;
                }
                let hdr_str = String::from_utf8_lossy(&bytes[s..hdr_end]).to_string();
                let params = header_param_names(&hdr_str);
                return Some(params);
            }

            j = if line_end < machine_span.end { line_end + 1 } else { machine_span.end };
        }

        None
    }
}
