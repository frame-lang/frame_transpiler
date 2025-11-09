use super::*;
use crate::frame_c::v3::body_closer::rust::BodyCloserRustV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;

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
                if b == b'-' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Transition, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'=' && i+4<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' && bytes[i+4]==b'^' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'$' && i+4<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' && (bytes[i+3]==b'+' || bytes[i+3]==b'-') && bytes[i+4]==b']' {
                    if seg_start<i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end:i } }); }
                    let mut j=i; while j<end && bytes[j]!=b'\n' { j+=1; }
                    let kind = if bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start:i, end:j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
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
                _ => { i+=1; }
            }
        }
        if seg_start<end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

