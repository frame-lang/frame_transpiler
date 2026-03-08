use crate::frame_c::v4::native_region_scanner::{RegionSpan, Region};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    Native { source: RegionSpan },
    Frame { source: RegionSpan },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplicedBody {
    pub text: String,
    pub splice_map: Vec<(RegionSpan, Origin)>,
}

pub struct Splicer;

impl Splicer {
    pub fn splice(&self, bytes: &[u8], regions: &[Region], expansions: &[String]) -> SplicedBody {
        let mut out = String::new();
        let mut map: Vec<(RegionSpan, Origin)> = Vec::new();
        let mut mi = 0usize; // index into expansions
        for r in regions {
            match r {
                Region::NativeText { span } => {
                    let start_pos = out.len();
                    out.push_str(std::str::from_utf8(&bytes[span.start..span.end]).unwrap_or(""));
                    let end_pos = out.len();
                    map.push((RegionSpan { start: start_pos, end: end_pos }, Origin::Native { source: *span }));
                }
                Region::FrameSegment { span, .. } => {
                    let start_pos = out.len();
                    let exp_str: &str = if mi < expansions.len() { &expansions[mi] } else { "" };
                    mi += 1;
                    out.push_str(exp_str);
                    let end_pos = out.len();
                    map.push((RegionSpan { start: start_pos, end: end_pos }, Origin::Frame { source: *span }));
                }
            }
        }
        SplicedBody { text: out, splice_map: map }
    }
}

impl SplicedBody {
    pub fn build_trailer_json(&self) -> String {
        // Minimal JSON trailer with span origins and schema version
        let mut s = String::from("{\"map\":[");
        let mut first = true;
        for (span, origin) in &self.splice_map {
            if !first { s.push(','); } else { first = false; }
            s.push_str(&format!("{{\"targetStart\":{},\"targetEnd\":{},", span.start, span.end));
            match origin {
                Origin::Frame{ source } => {
                    s.push_str(&format!("\"origin\":\"frame\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end));
                }
                Origin::Native{ source } => {
                    s.push_str(&format!("\"origin\":\"native\",\"sourceStart\":{},\"sourceEnd\":{} }}", source.start, source.end));
                }
            }
        }
        s.push_str("]");
        // Include both version and schemaVersion for compatibility
        s.push_str(",\"version\":1,\"schemaVersion\":1}");
        s
    }

    pub fn map_spliced_range_to_origin(&self, start: usize, end: usize) -> Option<(Origin, RegionSpan)> {
        for (tgt, origin) in &self.splice_map {
            if start >= tgt.start && start < tgt.end {
                // compute offset within target segment
                let off_start = start - tgt.start;
                let off_end = if end <= tgt.end { end - tgt.start } else { tgt.end - tgt.start };
                match origin {
                    Origin::Frame { source } => {
                        let src_start = source.start + off_start.min(source.end.saturating_sub(source.start));
                        let src_end = source.start + off_end.min(source.end.saturating_sub(source.start));
                        return Some((Origin::Frame { source: *source }, RegionSpan { start: src_start, end: src_end }));
                    }
                    Origin::Native { source } => {
                        let src_start = source.start + off_start.min(source.end.saturating_sub(source.start));
                        let src_end = source.start + off_end.min(source.end.saturating_sub(source.start));
                        return Some((Origin::Native { source: *source }, RegionSpan { start: src_start, end: src_end }));
                    }
                }
            }
        }
        None
    }
}

impl SplicedBody {
    pub fn build_line_map_json(&self, source_bytes: &[u8]) -> String {
        fn offset_to_line(s: &str, off: usize) -> usize {
            let bytes = s.as_bytes();
            let mut i = 0usize; let mut line = 1usize;
            while i < bytes.len() && i < off { if bytes[i] == b'\n' { line += 1; } i += 1; }
            line
        }
        let source_str = std::str::from_utf8(source_bytes).unwrap_or("");
        let target_str = &self.text;
        let mut out = String::from("{\"mappings\":[");
        let mut first = true;
        for (tgt, origin) in &self.splice_map {
            let tline = offset_to_line(target_str, tgt.start);
            let (origin_str, src_start) = match origin {
                Origin::Frame{ source } => ("frame", source.start),
                Origin::Native{ source } => ("native", source.start),
            };
            let sline = offset_to_line(source_str, src_start);
            if !first { out.push(','); } else { first = false; }
            out.push_str(&format!("{{\"targetLine\":{},\"sourceLine\":{},\"origin\":\"{}\"}}", tline, sline, origin_str));
        }
        out.push_str("] ,\"schemaVersion\":1}");
        out
    }
}
