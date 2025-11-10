use super::*;
use crate::frame_c::v3::body_closer::cpp::BodyCloserCppV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;

pub struct NativeRegionScannerCppV3;

impl NativeRegionScannerV3 for NativeRegionScannerCppV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserCppV3;
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
                b'R' if i+1<end && bytes[i+1]==b'"' => { // raw string, skip to closing )delim"
                    let mut j=i+2; let mut delim=Vec::new(); while j<end && bytes[j]!=b'(' { delim.push(bytes[j]); j+=1; if delim.len()>32 { break; } }
                    if j<end && bytes[j]==b'(' { j+=1; loop { if j>=end { break; } if bytes[j]==b')' { let mut k=j+1; let mut m=0usize; while m<delim.len() && k<end && bytes[k]==delim[m] { k+=1; m+=1; } if m==delim.len() && k<end && bytes[k]==b'"' { i=k+1; break; } } j+=1; } }
                    else { i+=1; }
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
