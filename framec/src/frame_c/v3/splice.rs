use crate::frame_c::v3::native_region_scanner::{RegionSpan, RegionV3};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginV3 {
    Native { source: RegionSpan },
    Frame { source: RegionSpan },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplicedBodyV3 {
    pub text: String,
    pub splice_map: Vec<(RegionSpan, OriginV3)>,
}

pub struct SplicerV3;

impl SplicerV3 {
    pub fn splice(&self, bytes: &[u8], regions: &[RegionV3], expansions: &[String]) -> SplicedBodyV3 {
        let mut out = String::new();
        let mut map: Vec<(RegionSpan, OriginV3)> = Vec::new();
        let mut mi = 0usize; // index into expansions
        for r in regions {
            match r {
                RegionV3::NativeText { span } => {
                    let start_pos = out.len();
                    out.push_str(std::str::from_utf8(&bytes[span.start..span.end]).unwrap_or(""));
                    let end_pos = out.len();
                    map.push((RegionSpan { start: start_pos, end: end_pos }, OriginV3::Native { source: *span }));
                }
                RegionV3::FrameSegment { span, .. } => {
                    let start_pos = out.len();
                    let exp = &expansions[mi];
                    mi += 1;
                    out.push_str(exp);
                    let end_pos = out.len();
                    map.push((RegionSpan { start: start_pos, end: end_pos }, OriginV3::Frame { source: *span }));
                }
            }
        }
        SplicedBodyV3 { text: out, splice_map: map }
    }
}

impl SplicedBodyV3 {
    pub fn build_trailer_json(&self) -> String {
        // Minimal JSON trailer with span origins
        let mut s = String::from("{\"map\":[");
        let mut first = true;
        for (span, origin) in &self.splice_map {
            if !first { s.push(','); } else { first = false; }
            s.push_str(&format!("{{\"targetStart\":{},\"targetEnd\":{},", span.start, span.end));
            match origin {
                OriginV3::Frame{ source } => {
                    s.push_str(&format!("\"origin\":\"frame\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end));
                }
                OriginV3::Native{ source } => {
                    s.push_str(&format!("\"origin\":\"native\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end));
                }
            }
        }
        s.push_str("],\"version\":1}");
        s
    }

    pub fn map_spliced_range_to_origin(&self, start: usize, end: usize) -> Option<(OriginV3, RegionSpan)> {
        for (tgt, origin) in &self.splice_map {
            if start >= tgt.start && start < tgt.end {
                // compute offset within target segment
                let mut off_start = start - tgt.start;
                let mut off_end = if end <= tgt.end { end - tgt.start } else { tgt.end - tgt.start };
                match origin {
                    OriginV3::Frame { source } => {
                        let src_start = source.start + off_start.min(source.end.saturating_sub(source.start));
                        let src_end = source.start + off_end.min(source.end.saturating_sub(source.start));
                        return Some((OriginV3::Frame { source: *source }, RegionSpan { start: src_start, end: src_end }));
                    }
                    OriginV3::Native { source } => {
                        let src_start = source.start + off_start.min(source.end.saturating_sub(source.start));
                        let src_end = source.start + off_end.min(source.end.saturating_sub(source.start));
                        return Some((OriginV3::Native { source: *source }, RegionSpan { start: src_start, end: src_end }));
                    }
                }
            }
        }
        None
    }
}
