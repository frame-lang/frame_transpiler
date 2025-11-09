use crate::frame_c::v3::frame_statement_parser::{FrameStatementParserV3, ParseErrorV3};
use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::RegionV3;

pub struct MirAssemblerV3;

impl MirAssemblerV3 {
    pub fn assemble(&self, bytes: &[u8], regions: &[RegionV3]) -> Result<Vec<MirItemV3>, ParseErrorV3> {
        let parser = FrameStatementParserV3;
        let mut out = Vec::new();
        for r in regions {
            if let RegionV3::FrameSegment{..} = r {
                out.push(parser.parse_segment(bytes, r)?);
            }
        }
        Ok(out)
    }
}

