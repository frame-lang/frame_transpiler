use super::*;
use crate::frame_c::v4::body_closer::java::BodyCloserJavaV3;
use crate::frame_c::v4::body_closer::BodyCloserV3;

pub struct NativeRegionScannerJavaV3;

impl NativeRegionScannerV3 for NativeRegionScannerJavaV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserJavaV3;
        let close = closer.close_byte(bytes, open_brace_index).map_err(|e| ScanErrorV3{ kind: ScanErrorV3Kind::UnterminatedProtected, message: format!("{:?}", e) })?;
        let mut regions: Vec<RegionV3> = Vec::new();
        let mut i = open_brace_index + 1; let end=close; let mut seg_start=i; let mut at_sol=true; let mut indent=0usize;
        while i<end {
            let b=bytes[i]; if at_sol { if b==b' '||b==b'\t' { indent+=1; i+=1; continue; }
                if b == b'-' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_c_like(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Transition, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'=' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_c_like(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'$' && i+2<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; j = find_frame_line_end_c_like(bytes, j, end);
                    let kind = if i+3<end && bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                at_sol=false; indent=0; }
            match b { b'\n'=>{at_sol=true;indent=0;i+=1;}
                b'/' if i+1<end && bytes[i+1]==b'/' => { i+=2; while i<end && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<end && bytes[i+1]==b'*' => { i+=2; while i+1<end { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; } }
                b'\'' | b'"' => { let q=b; i+=1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==q { i+=1; break; } i+=1; } }
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
                    while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
                    if i < end && bytes[i] == b'=' && (i+1 >= end || bytes[i+1] != b'=') {
                        i += 1;
                        i = find_frame_line_end_c_like(bytes, i, end);
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturn, indent: 0 });
                    } else {
                        regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturnExpr, indent: 0 });
                    }
                    seg_start = i;
                }
                // Return value sugar: ^ (caret) - returns from handler
                b'^' => {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let start = i;
                    i += 1;
                    while i < end && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
                    i = find_frame_line_end_c_like(bytes, i, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start, end: i }, kind: FrameSegmentKindV3::SystemReturn, indent: 0 });
                    seg_start = i;
                }
                _ => { i+=1; }
            }
        }
        if seg_start<end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

fn find_frame_line_end_c_like(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_s: Option<u8> = None;
    while j < end {
        let b = bytes[j];
        if b == b'\n' { break; }
        if let Some(q) = in_s {
            if b == b'\\' { j += 2; continue; }
            if b == q { in_s = None; j += 1; continue; }
            j += 1; continue;
        }
        if b == b';' { break; }
        if b == b'/' && j+1 < end && bytes[j+1] == b'/' { break; }
        if b == b'/' && j+1 < end && bytes[j+1] == b'*' { break; }
        if b == b'\'' || b == b'"' { in_s = Some(b); j += 1; continue; }
        j += 1;
    }
    j
}
