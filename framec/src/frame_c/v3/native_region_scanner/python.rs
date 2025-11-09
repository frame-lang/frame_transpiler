use super::*;
use crate::frame_c::v3::body_closer::python::BodyCloserPyV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;

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
                // Transition: -> $
                if b == b'-' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    // consume to end of line
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Forward: => $^
                if b == b'=' && i+4<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' && bytes[i+4]==b'^' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Stack: $$[+/-]
                if b == b'$' && i+4<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' && (bytes[i+3]==b'+' || bytes[i+3]==b'-') && bytes[i+4]==b']' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    let kind = if bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
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
                _ => { i+=1; }
            }
        }
        if seg_start < end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

