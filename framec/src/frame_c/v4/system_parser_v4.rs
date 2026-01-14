use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v3::ast::{ModuleAst, SystemAst, SystemParamsAst, SystemSectionsAst, Span, SystemSectionKind};
// For now, use v3's body closer until we adapt it for v4
use crate::frame_c::v3::body_closer::{self as closer, BodyCloserV3};

/// Parser for outer V4 system headers and section layout.
///
/// This is a Frame-only parser adapted from V3: it understands `@@system` headers, 
/// optional system parameters `($(start), $>(enter), domain)`, and the locations of
/// `operations:`, `interface:`, `machine:`, `actions:`, and `domain:` blocks.
/// Modified to handle v4 syntax including @@persist and @@system annotations.
pub struct SystemParserV4;

impl SystemParserV4 {
    pub fn parse_module(bytes: &[u8], lang: TargetLanguage) -> ModuleAst {
        let n = bytes.len();
        let mut i = 0usize;
        let mut systems = Vec::new();

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

            let system_line_start = i;

            // Optional per-system attribute at SOL, e.g. `@persist system Foo {`.
            // For now we only recognize `@persist` without parameters and treat it
            // as an opt-in for persistence helpers during codegen.
            let mut persist_attr: Option<crate::frame_c::v3::ast::PersistAttrAst> = None;

            // Look for optional `@@persist` followed by `@@system` keyword at SOL (v4 syntax).
            let mut j = i;
            while j < n && is_space(bytes[j]) {
                j += 1;
            }
            
            // Check for @@ prefix (v4 annotations)
            if j + 1 < n && bytes[j] == b'@' && bytes[j + 1] == b'@' {
                j += 2; // Skip @@
                let attr_start = j;
                while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                if attr_start == j {
                    // Malformed attribute; skip this line.
                    while i < n && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
                let attr =
                    String::from_utf8_lossy(&bytes[attr_start..j]).to_ascii_lowercase();
                
                if attr.as_str() == "persist" {
                    // Found @@persist annotation
                    persist_attr = Some(crate::frame_c::v3::ast::PersistAttrAst {
                        save_name: None,
                        restore_name: None,
                    });
                    // Skip any whitespace between the attribute and `@@system`.
                    while j < n && is_space(bytes[j]) {
                        j += 1;
                    }
                    // Now look for @@system
                    if j + 1 < n && bytes[j] == b'@' && bytes[j + 1] == b'@' {
                        j += 2;
                        let kw_start = j;
                        while j < n && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                            j += 1;
                        }
                        let kw = String::from_utf8_lossy(&bytes[kw_start..j]).to_ascii_lowercase();
                        if kw.as_str() != "system" {
                            // Not @@system, skip line
                            while i < n && bytes[i] != b'\n' {
                                i += 1;
                            }
                            continue;
                        }
                    } else {
                        // Expected @@system after @@persist
                        while i < n && bytes[i] != b'\n' {
                            i += 1;
                        }
                        continue;
                    }
                } else if attr.as_str() == "system" {
                    // Found @@system directly (no @@persist)
                    // This is valid, continue to parse system
                } else {
                    // Unknown @@annotation at SOL; skip this line for now.
                    while i < n && bytes[i] != b'\n' {
                        i += 1;
                    }
                    continue;
                }
            } else {
                // No @@ prefix found, skip line (v4 requires @@system)
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
            let name = String::from_utf8_lossy(&bytes[name_start..k]).to_string();

            // Parse optional system parameter list: `( $(...), $>(...), domain... )`.
            while k < n && is_space(bytes[k]) {
                k += 1;
            }
            let mut params = SystemParamsAst {
                start_params: Vec::new(),
                enter_params: Vec::new(),
                domain_params: Vec::new(),
            };
            if k < n && bytes[k] == b'(' {
                k += 1; // after '('
                while k < n {
                    while k < n && (bytes[k] == b' ' || bytes[k] == b'\t') {
                        k += 1;
                    }
                    if k >= n || bytes[k] == b')' {
                        break;
                    }
                    // $(param, ...)
                    if bytes[k] == b'$' && k + 1 < n && bytes[k + 1] == b'(' {
                        k += 2;
                        while k < n && bytes[k] != b')' {
                            if (bytes[k] as char).is_ascii_alphabetic() || bytes[k] == b'_' {
                                let ident_start = k;
                                k += 1;
                                while k < n
                                    && ((bytes[k] as char).is_ascii_alphanumeric()
                                        || bytes[k] == b'_')
                                {
                                    k += 1;
                                }
                                let ident = String::from_utf8_lossy(&bytes[ident_start..k]).to_string();
                                params.start_params.push(ident);
                            } else {
                                k += 1;
                            }
                        }
                        if k < n && bytes[k] == b')' {
                            k += 1;
                        }
                    }
                    // $>(param, ...)
                    else if bytes[k] == b'$' && k + 1 < n && bytes[k + 1] == b'>' {
                        k += 2;
                        while k < n && (bytes[k] == b' ' || bytes[k] == b'\t') {
                            k += 1;
                        }
                        if k < n && bytes[k] == b'(' {
                            k += 1;
                            while k < n && bytes[k] != b')' {
                                if (bytes[k] as char).is_ascii_alphabetic() || bytes[k] == b'_' {
                                    let ident_start = k;
                                    k += 1;
                                    while k < n
                                        && ((bytes[k] as char).is_ascii_alphanumeric()
                                            || bytes[k] == b'_')
                                    {
                                        k += 1;
                                    }
                                    let ident = String::from_utf8_lossy(&bytes[ident_start..k]).to_string();
                                    params.enter_params.push(ident);
                                } else {
                                    k += 1;
                                }
                            }
                            if k < n && bytes[k] == b')' {
                                k += 1;
                            }
                        }
                    }
                    // Domain parameter: IDENT at top level.
                    else if (bytes[k] as char).is_ascii_alphabetic() || bytes[k] == b'_' {
                        let ident_start = k;
                        k += 1;
                        while k < n
                            && ((bytes[k] as char).is_ascii_alphanumeric() || bytes[k] == b'_')
                        {
                            k += 1;
                        }
                        let ident = String::from_utf8_lossy(&bytes[ident_start..k]).to_string();
                        params.domain_params.push(ident);
                    } else {
                        k += 1;
                    }

                    // Skip to next ',' or ')'.
                    while k < n && (bytes[k] == b' ' || bytes[k] == b'\t') {
                        k += 1;
                    }
                    if k < n && bytes[k] == b',' {
                        k += 1;
                        continue;
                    }
                    if k < n && bytes[k] == b')' {
                        k += 1;
                        break;
                    }
                }
            }

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
            let mut sections = SystemSectionsAst::default();
            let mut marks: Vec<(usize, String)> = Vec::new();
            let mut section_order: Vec<SystemSectionKind> = Vec::new();
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
                    if kw_sec.as_str() == "operations"
                        || kw_sec.as_str() == "interface"
                        || kw_sec.as_str() == "machine"
                        || kw_sec.as_str() == "actions"
                        || kw_sec.as_str() == "domain"
                    {
                        let kind = match kw_sec.as_str() {
                            "operations" => SystemSectionKind::Operations,
                            "interface" => SystemSectionKind::Interface,
                            "machine" => SystemSectionKind::Machine,
                            "actions" => SystemSectionKind::Actions,
                            "domain" => SystemSectionKind::Domain,
                            _ => continue,
                        };
                        marks.push((line, kw_sec));
                        section_order.push(kind);
                    }
                }
                while q < close && bytes[q] != b'\n' {
                    q += 1;
                }
            }

            // Turn section markers into spans.
            for idx in 0..marks.len() {
                let (line, kw) = &marks[idx];
                let start_sec = *line;
                let end_sec = if idx + 1 < marks.len() {
                    marks[idx + 1].0
                } else {
                    close
                };
                let span = Span {
                    start: start_sec,
                    end: end_sec,
                };
                match kw.as_str() {
                    "operations" => sections.operations = Some(span),
                    "interface" => sections.interface = Some(span),
                    "machine" => sections.machine = Some(span),
                    "actions" => sections.actions = Some(span),
                    "domain" => sections.domain = Some(span),
                    _ => {}
                }
            }

            let sys_ast = SystemAst {
                name,
                params,
                span: Span {
                    start: system_line_start,
                    end: close,
                },
                sections,
                section_order,
                persist_attr,
            };
            if std::env::var("FRAME_DEBUG_SYSPARAMS").ok().as_deref() == Some("1") {
                eprintln!(
                    "[sysparams] parsed system name={} machine_span={:?}",
                    sys_ast.name,
                    sys_ast.sections.machine
                );
            }
            systems.push(sys_ast);

            // Advance past this system.
            i = close + 1;
        }

        ModuleAst { systems }
    }
}
