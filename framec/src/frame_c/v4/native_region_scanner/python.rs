use super::*;
use crate::frame_c::v4::body_closer::python::BodyCloserPyV3;
use crate::frame_c::v4::body_closer::BodyCloserV3;

pub struct NativeRegionScannerPyV3;

impl NativeRegionScannerV3 for NativeRegionScannerPyV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserPyV3;
        let close = closer.close_byte(bytes, open_brace_index).map_err(|e| ScanErrorV3{ kind: ScanErrorV3Kind::UnterminatedProtected, message: format!("{:?}", e) })?;
        let mut regions: Vec<RegionV3> = Vec::new();
        let mut i = open_brace_index + 1;
        let end = close;
        let mut seg_start = i;
        let mut at_sol = true; let mut indent = 0usize;
        while i < end {
            let b = bytes[i];
            if at_sol {
                if b == b' ' || b == b'\t' { indent += 1; i+=1; continue; }
                // Transition-Forward: -> => $State (transition then forward event)
                if b == b'-' && i+1<end && bytes[i+1]==b'>' {
                    let mut k = i + 2;
                    while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                    // Check for => (forward after transition)
                    if k+1 < end && bytes[k] == b'=' && bytes[k+1] == b'>' {
                        k += 2;
                        while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                        if k < end && bytes[k] == b'$' {
                            let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                            if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                            let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                            regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::TransitionForward, indent });
                            i=j; seg_start=i; at_sol=true; indent=0; continue;
                        }
                    }
                    // Check for -> pop$ (pop transition without backtick)
                    if k+3 < end && bytes[k] == b'p' && bytes[k+1] == b'o' && bytes[k+2] == b'p' && bytes[k+3] == b'$' {
                        let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                        if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                        let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::StackPop, indent });
                        i=j; seg_start=i; at_sol=true; indent=0; continue;
                    }
                    // Regular transition: -> $ or -> (enter_args) $
                    // Check for optional enter args: -> (args) $State
                    if k < end && bytes[k] == b'(' {
                        if let Some(k2) = balanced_paren_end_py(bytes, k, end) {
                            k = k2;
                            while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                        }
                    }
                    // Must have $ after -> or -> (args)
                    if k < end && bytes[k] == b'$' {
                        // When at SOL, exclude the indentation from the NativeText region
                        let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                        if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                        // consume to end-of-statement on this physical line (before ';' or '#')
                        let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                        i=j; seg_start=i; at_sol=true; indent=0; continue;
                    }
                }
                // Forward: => $^ (accept partial head to surface parser error)
                if b == b'=' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    // When at SOL, exclude the indentation from the NativeText region
                    let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                    if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                    let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Transition with leading exit args: ( ... ) -> ( ... ) $State
                if b == b'(' {
                    if let Some(mut k) = balanced_paren_end_py(bytes, i, end) {
                        while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                        if k+1 < end && bytes[k] == b'-' && bytes[k+1] == b'>' {
                            k += 2; while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                            // Optional enter args
                            if k < end && bytes[k] == b'(' {
                                if let Some(k2) = balanced_paren_end_py(bytes, k, end) { k = k2; }
                            }
                            while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                            if k < end && bytes[k] == b'$' {
                                // When at SOL, exclude the indentation from the NativeText region
                                let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                                if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                                let mut j = i; j = find_frame_line_end_py(bytes, j, end);
                                regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                                i = j; seg_start = i; at_sol = true; indent = 0; continue;
                            }
                        }
                    }
                }
                // Stack: $$[+/-] (V3 syntax)
                if b == b'$' && i+2<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' {
                    // When at SOL, exclude the indentation from the NativeText region
                    let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                    if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                    let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                    let kind = if i+3<end && bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V4: `push$ (stack push with backtick)
                if b == b'`' && i+5<end && bytes[i+1]==b'p' && bytes[i+2]==b'u' && bytes[i+3]==b's' && bytes[i+4]==b'h' && bytes[i+5]==b'$' {
                    let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                    if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                    let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::StackPush, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V4: `-> pop$ (pop transition with backtick)
                if b == b'`' && i+7<end && bytes[i+1]==b'-' && bytes[i+2]==b'>' && bytes[i+3]==b' ' && bytes[i+4]==b'p' && bytes[i+5]==b'o' && bytes[i+6]==b'p' && bytes[i+7]==b'$' {
                    let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                    if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                    let mut j=i; j = find_frame_line_end_py(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::StackPop, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Return value sugar: ^ <expr> at start of line
                if b == b'^' {
                    let native_end = if indent > 0 { i.saturating_sub(indent) } else { i };
                    if seg_start < native_end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: native_end } }); }
                    let start = i;
                    i += 1; // Skip '^'
                    while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
                    i = find_frame_line_end_py(bytes, i, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturn, indent });
                    seg_start = i; at_sol = true; indent = 0; continue;
                }
                at_sol = false; indent=0;
            }
            match b {
                b'\n' => { at_sol = true; indent=0; i+=1; }
                b'#' => { while i<end && bytes[i]!=b'\n' { i+=1; } }
                b'\'' | b'"' => {
                    let q = b; let mut triple=false; if i+2<end && bytes[i+1]==q && bytes[i+2]==q { triple=true; }
                    i+=1; if triple { i+=2; }
                    loop { if i>=end { break; } if triple { if bytes[i]==q && i+2<end && bytes[i+1]==q && bytes[i+2]==q { i+=3; break; } i+=1; } else { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==q { i+=1; break; } i+=1; } }
                }
                // State variable reference: $.varName
                b'$' if i+1 < end && bytes[i+1] == b'.' => {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    // Find end of identifier: $.varName
                    let var_start = i;
                    i += 2; // Skip "$."
                    while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: var_start, end: i }, kind: FrameSegmentKindV3::StateVar, indent: 0 });
                    seg_start = i;
                }
                // System return: system.return = <expr> or system.return
                b's' if i+12 < end && &bytes[i..i+13] == b"system.return" => {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let start = i;
                    i += 13; // Skip "system.return"
                    // Skip whitespace
                    while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
                    if i < end && bytes[i] == b'=' && (i+1 >= end || bytes[i+1] != b'=') {
                        // system.return = <expr> - find end of expression
                        i += 1; // Skip '='
                        i = find_frame_line_end_py(bytes, i, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturn, indent: 0 });
                    } else {
                        // bare system.return - just the expression read
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturnExpr, indent: 0 });
                    }
                    seg_start = i;
                }
                _ => { i+=1; }
            }
        }
        if seg_start < end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

fn find_frame_line_end_py(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_s: Option<u8> = None;
    while j < end {
        let b = bytes[j];
        if b == b'\n' { break; }
        if let Some(q) = in_s {
            if b == b'\\' { j += 2; continue; }
            if b == q { in_s = None; j += 1; continue; }
            j += 1; continue;
        }
        match b {
            b'\'' | b'"' => { in_s = Some(b); j += 1; }
            b'#' => { break; }
            b';' => { break; }
            _ => { j += 1; }
        }
    }
    j
}

fn balanced_paren_end_py(bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
    if i >= end || bytes[i] != b'(' { return None; }
    let mut depth: i32 = 0;
    let mut in_s: Option<u8> = None;
    while i < end {
        let b = bytes[i];
        if let Some(q) = in_s {
            if b == b'\\' { i += 2; continue; }
            if b == q { in_s = None; i += 1; continue; }
            i += 1; continue;
        }
        match b {
            b'\'' | b'"' => { in_s = Some(b); i += 1; }
            b'(' => { depth += 1; i += 1; }
            b')' => { depth -= 1; i += 1; if depth == 0 { return Some(i); } }
            _ => { i += 1; }
        }
    }
    None
}
