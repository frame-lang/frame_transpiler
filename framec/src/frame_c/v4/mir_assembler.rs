use crate::frame_c::v4::frame_statement_parser::{FrameStatementParserV3, ParseErrorV3};
use crate::frame_c::v4::mir::MirItemV3;
use crate::frame_c::v4::validator::ValidationIssueV3;
use crate::frame_c::v4::native_region_scanner::RegionV3;

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
                    Err(e) => {
                        use crate::frame_c::v4::frame_statement_parser::ParseErrorV3Kind as K;
                        let msg = match e.kind {
                            K::MissingState => format!("E300: {}", e.message),
                            K::UnbalancedArgs => format!("E301: {}", e.message),
                            K::TrailingTokens => format!("E302: {}", e.message),
                            K::InvalidHead => "E200: invalid Frame statement at start of line".to_string(),
                        };
                        issues.push(ValidationIssueV3 { message: msg });
                    },
                }
            }
        }
        (out_mir, issues)
    }
}
