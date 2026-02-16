use super::*;
use crate::frame_c::v4::body_closer::rust::BodyCloserRustV3;
use crate::frame_c::v4::body_closer::BodyCloserV3;

pub struct NativeRegionScannerRustV3;

impl NativeRegionScannerV3 for NativeRegionScannerRustV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserRustV3;
        let close = closer.close_byte(bytes, open_brace_index).map_err(|e| ScanErrorV3{ kind: ScanErrorV3Kind::UnterminatedProtected, message: format!("{:?}", e) })?;
        let mut regions: Vec<RegionV3> = Vec::new();
        let mut i = open_brace_index + 1; let end=close; let mut seg_start=i; let mut at_sol=true; let mut indent=0usize; let mut block_nest:i32=0;
        while i<end {
            if block_nest>0 { if i+1<end && bytes[i]==b'/' && bytes[i+1]==b'*' { block_nest+=1; i+=2; continue; } if i+1<end && bytes[i]==b'*' && bytes[i+1]==b'/' { block_nest-=1; i+=2; continue; } i+=1; continue; }
            let b=bytes[i]; if at_sol { if b==b' '||b==b'\t' { indent+=1; i+=1; continue; }
                // V4: -> $State (transition) or -> pop$ (pop transition)
                if b == b'-' && i+1<end && bytes[i+1]==b'>' {
                    // Check for -> pop$ (pop transition)
                    if i+7<end && bytes[i+2]==b' ' && bytes[i+3]==b'p' && bytes[i+4]==b'o' && bytes[i+5]==b'p' && bytes[i+6]==b'$' {
                        if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                        let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::StackPop, indent });
                        i=j; seg_start=i; at_sol=true; indent=0; continue;
                    }
                    // Regular transition -> $State
                    if i+3<end && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                        if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                        let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Transition, indent });
                        i=j; seg_start=i; at_sol=true; indent=0; continue;
                    }
                }
                // V4: => $^ (forward)
                if b == b'=' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V4: `push$ (stack push with backtick)
                if b == b'`' && i+5<end && bytes[i+1]==b'p' && bytes[i+2]==b'u' && bytes[i+3]==b's' && bytes[i+4]==b'h' && bytes[i+5]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::StackPush, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V4: `-> pop$ (pop transition with backtick)
                if b == b'`' && i+7<end && bytes[i+1]==b'-' && bytes[i+2]==b'>' && bytes[i+3]==b' ' && bytes[i+4]==b'p' && bytes[i+5]==b'o' && bytes[i+6]==b'p' && bytes[i+7]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::StackPop, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V4: push$ (stack push without backtick - legacy)
                if b == b'p' && i+4<end && bytes[i+1]==b'u' && bytes[i+2]==b's' && bytes[i+3]==b'h' && bytes[i+4]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::StackPush, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // V3 compat: $$[+] or $$[-]
                if b == b'$' && i+2<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_rust(bytes, j, end);
                    let kind = if i+3<end && bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Return value sugar: ^ <expr> at start of line
                if b == b'^' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let start = i;
                    i += 1; // Skip '^'
                    while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
                    i = find_frame_line_end_rust(bytes, i, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturn, indent });
                    seg_start = i; at_sol = true; indent = 0; continue;
                }
                at_sol=false; indent=0; }
            match b { b'\n'=>{at_sol=true;indent=0;i+=1;}
                b'/' if i+1<end && bytes[i+1]==b'/' => { i+=2; while i<end && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<end && bytes[i+1]==b'*' => { block_nest=1; i+=2; }
                b'\'' | b'"' => { let q=b; i+=1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==q { i+=1; break; } i+=1; } }
                b'r' => { // skip simple raw r#"..."#
                    let mut j=i+1; let mut hashes=0usize; if j<end && bytes[j]==b'#' { while j<end && bytes[j]==b'#' { hashes+=1; j+=1; } }
                    if j<end && bytes[j]==b'"' { j+=1; loop { if j>=end { break; } if bytes[j]==b'"' { let mut k=j+1; let mut m=0usize; while m<hashes && k<end && bytes[k]==b'#' { m+=1; k+=1; } if m==hashes { i=k; break; } } j+=1; } } else { i+=1; }
                }
                // State variable reference: $.varName
                b'$' if i+1 < end && bytes[i+1] == b'.' => {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
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
                        i = find_frame_line_end_rust(bytes, i, end);
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
        if seg_start<end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

fn find_frame_line_end_rust(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_s: Option<u8> = None;
    let mut raw_hashes: usize = 0;
    while j < end {
        let b = bytes[j];
        if b == b'\n' { break; }
        if raw_hashes > 0 {
            if b == b'"' {
                let mut k = j + 1; let mut m = 0usize; while k < end && m < raw_hashes && bytes[k] == b'#' { k += 1; m += 1; }
                if m == raw_hashes { j = k; raw_hashes = 0; continue; }
            }
            j += 1; continue;
        }
        if let Some(q) = in_s {
            if b == b'\\' { j += 2; continue; }
            if b == q { in_s = None; j += 1; continue; }
            j += 1; continue;
        }
        if b == b';' { break; }
        if b == b'/' && j+1 < end && bytes[j+1] == b'/' { break; }
        if b == b'/' && j+1 < end && bytes[j+1] == b'*' { break; }
        if b == b'\'' || b == b'"' { in_s = Some(b); j += 1; continue; }
        if b == b'r' { // possible raw string r#####" ... "#####
            let mut k = j + 1; let mut hashes = 0usize;
            if k < end && bytes[k] == b'#' { while k < end && bytes[k] == b'#' { hashes += 1; k += 1; } }
            if k < end && bytes[k] == b'"' { raw_hashes = hashes; j = k + 1; continue; }
        }
        j += 1;
    }
    j
}
