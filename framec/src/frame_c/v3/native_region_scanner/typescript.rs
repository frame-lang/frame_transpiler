use super::*;
use crate::frame_c::v3::body_closer::typescript::BodyCloserTsV3;
use crate::frame_c::v3::body_closer::BodyCloserV3;

pub struct NativeRegionScannerTsV3;

impl NativeRegionScannerV3 for NativeRegionScannerTsV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        let mut closer = BodyCloserTsV3;
        let close = closer.close_byte(bytes, open_brace_index).map_err(|e| ScanErrorV3{ kind: ScanErrorV3Kind::UnterminatedProtected, message: format!("{:?}", e) })?;
        let mut regions: Vec<RegionV3> = Vec::new();
        let mut i = open_brace_index + 1;
        let end = close;
        let mut seg_start = i;
        let mut at_sol = true; let mut indent = 0usize; let mut tmpl_brace: i32 = 0;
        while i < end {
            let b = bytes[i];
            if at_sol {
                if b == b' ' || b == b'\t' { indent += 1; i+=1; continue; }
                if b == b'-' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; j = find_frame_line_end_ts(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                // Transition with leading exit args: ( ... ) -> ( ... ) $State
                if b == b'(' {
                    if let Some(mut k) = balanced_paren_end_ts(bytes, i, end) {
                        while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                        if k+1 < end && bytes[k] == b'-' && bytes[k+1] == b'>' {
                            k += 2; while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                            if k < end && bytes[k] == b'(' {
                                if let Some(k2) = balanced_paren_end_ts(bytes, k, end) { k = k2; }
                            }
                            while k < end && (bytes[k] == b' ' || bytes[k] == b'\t') { k += 1; }
                            if k < end && bytes[k] == b'$' {
                                if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                                let mut j = i; j = find_frame_line_end_ts(bytes, j, end);
                                regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Transition, indent });
                                i = j; seg_start = i; at_sol = true; indent = 0; continue;
                            }
                        }
                    }
                }
                if b == b'=' && i+3<end && bytes[i+1]==b'>' && bytes[i+2]==b' ' && bytes[i+3]==b'$' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; j = find_frame_line_end_ts(bytes, j, end);
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind: FrameSegmentKindV3::Forward, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                if b == b'$' && i+2<end && bytes[i+1]==b'$' && bytes[i+2]==b'[' {
                    if seg_start < i { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end: i } }); }
                    let mut j=i; j = find_frame_line_end_ts(bytes, j, end);
                    let kind = if i+3<end && bytes[i+3]==b'+' { FrameSegmentKindV3::StackPush } else { FrameSegmentKindV3::StackPop };
                    regions.push(RegionV3::FrameSegment{ span: RegionSpan{ start: i, end: j }, kind, indent });
                    i=j; seg_start=i; at_sol=true; indent=0; continue;
                }
                at_sol = false; indent=0;
            }
            match b {
                b'\n' => { at_sol = true; indent=0; i+=1; }
                b'/' if i+1<end && bytes[i+1]==b'/' => { i+=2; while i<end && bytes[i]!=b'\n' { i+=1; } }
                b'/' if i+1<end && bytes[i+1]==b'*' => { i+=2; while i+1<end { if bytes[i]==b'*' && bytes[i+1]==b'/' { i+=2; break; } i+=1; } }
                b'\'' | b'"' => { let q=b; i+=1; while i<end { if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==q { i+=1; break; } i+=1; } }
                b'`' => {
                    i+=1; tmpl_brace=0; while i<end { if bytes[i]==b'`' && tmpl_brace==0 { i+=1; break; } if bytes[i]==b'\\' { i+=2; continue; } if bytes[i]==b'$' && i+1<end && bytes[i+1]==b'{' { tmpl_brace+=1; i+=2; continue; } if bytes[i]==b'}' && tmpl_brace>0 { tmpl_brace-=1; i+=1; continue; } i+=1; }
                }
                _ => { i+=1; }
            }
        }
        if seg_start < end { regions.push(RegionV3::NativeText{ span: RegionSpan{ start: seg_start, end } }); }
        Ok(ScanResultV3{ close_byte: close, regions })
    }
}

fn find_frame_line_end_ts(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_s: Option<u8> = None; // ' or "
    let mut in_tpl = false; // backtick
    while j < end {
        let b = bytes[j];
        if b == b'\n' { break; }
        if in_tpl {
            if b == b'`' { in_tpl = false; j += 1; continue; }
            if b == b'\\' { j += 2; continue; }
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
        match b { b'\'' | b'"' => { in_s = Some(b); j += 1; } , b'`' => { in_tpl = true; j += 1; }, _ => { j += 1; } }
    }
    j
}

fn balanced_paren_end_ts(bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
    if i >= end || bytes[i] != b'(' { return None; }
    let mut depth: i32 = 0;
    let mut in_s: Option<u8> = None; // ' or "
    let mut in_tpl = false;
    while i < end {
        let b = bytes[i];
        if in_tpl {
            if b == b'`' { in_tpl = false; i += 1; continue; }
            if b == b'\\' { i += 2; continue; }
            i += 1; continue;
        }
        if let Some(q) = in_s {
            if b == b'\\' { i += 2; continue; }
            if b == q { in_s = None; i += 1; continue; }
            i += 1; continue;
        }
        match b {
            b'\'' | b'\"' => { in_s = Some(b); i += 1; }
            b'`' => { in_tpl = true; i += 1; }
            b'(' => { depth += 1; i += 1; }
            b')' => { depth -= 1; i += 1; if depth == 0 { return Some(i); } }
            _ => { i += 1; }
        }
    }
    None
}
