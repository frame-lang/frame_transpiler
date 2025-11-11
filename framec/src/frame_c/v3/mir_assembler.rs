use crate::frame_c::v3::frame_statement_parser::{FrameStatementParserV3, ParseErrorV3};
use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::validator::ValidationIssueV3;
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

    pub fn assemble_collect(&self, bytes: &[u8], regions: &[RegionV3]) -> (Vec<MirItemV3>, Vec<ValidationIssueV3>) {
        let parser = FrameStatementParserV3;
        let mut out_mir = Vec::new();
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        for r in regions {
            if let RegionV3::FrameSegment{..} = r {
                match parser.parse_segment(bytes, r) {
                    Ok(m) => out_mir.push(m),
                    Err(e) => issues.push(ValidationIssueV3 { message: format!("Parse error: {:?}", e) }),
                }
            }
        }
        (out_mir, issues)
    }
}
