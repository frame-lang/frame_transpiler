use super::*;
use crate::frame_c::v3::body_closer::csharp::BodyCloserCsV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;

pub struct NativeRegionScannerCsV3;

impl NativeRegionScannerV3 for NativeRegionScannerCsV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserCsV3;
        let close = closer.close_byte(bytes, open_brace_index).map_err(|e| ScanErrorV3{ kind: ScanErrorV3Kind::UnterminatedProtected, message: format!("{:?}", e) })?;
        let mut regions: Vec<RegionV3> = Vec::new();
        let mut i = open_brace_index + 1; let end = close; let mut seg_start=i; let mut at_sol=true; let mut indent=0usize;
        while i<end {
            let b=bytes[i];
            if at_sol {
                if b==b' ' || b==b'\t' { indent+=1; i+=1; continue; }
                if b==b'#' { while i<end && bytes[i]!=b'\n' { i+=1; } at_sol=true; indent=0; continue; }
                if b == b'-' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'=' && i+4<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' && bytes[i+4]==b'^' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'$' && i+4<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' && (bytes[i+3]==b'+' || bytes[i+3]==b'-') && bytes[i+4]==b']' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    let kind = if bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                at_sol=false; indent=0;
            }
            match b {
                b'\n' => { at_sol=true; indent=0; i+=1; }
                b'/' if i+1<end && bytes[i+1]==b'/' => { i+=2; while i<end && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<end && bytes[i+1]==b'*' => { i+=2; while i+1<end { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; } }
                b'\'' => { i+=1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'\'' { i+=1; break; } i+=1; } }
                b'"' => { // normal string
                    i+=1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; }
                }
                b'@' if i+1<end && bytes[i+1]==b'"' => { // verbatim or @$""
                    i+=2; loop { if i>=end { break; } if i+1<end && bytes[i]==b'"' && bytes[i+1]==b'"' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; }
                }
                b'$' => {
                    // $"..." or raw $"""...""" (with N quotes) or $@"..." handled via @ branch after this if `$@`.
                    let mut j=i; let mut dollars=0; while j<end && bytes[j]==b'$' { dollars+=1; j+=1; }
                    // if next is '@', fall through so '@' handler consumes verbatim string
                    if j<end && bytes[j]==b'@' { i+=1; continue; }
                    // count quotes for potential raw string
                    let mut k=j; let mut quotes=0; while k<end && bytes[k]==b'"' { quotes+=1; k+=1; }
                    if quotes>=3 { // raw string with possible interpolation
                        i = k; // inside raw; consume until we see quotes>=N
                        loop {
                            if i>=end { break; }
                            if bytes[i]==b'"' {
                                let mut q=0; let mut p=i; while p<end && bytes[p]==b'"' { q+=1; p+=1; }
                                if q>=quotes { i=p; break; }
                                i=p; continue;
                            }
                            i+=1;
                        }
                    } else if j<end && bytes[j]==b'"' { // interpolated normal string $"..."
                        i = j+1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'"' { i+=1; break; } i+=1; }
                    } else {
                        i+=1;
                    }
                }
                _ => { i+=1; }
            }
        }
        if seg_start<end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}
