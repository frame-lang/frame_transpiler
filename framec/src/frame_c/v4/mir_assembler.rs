use crate::frame_c::v4::frame_statement_parser::{FrameStatementParser, ParseError};
use crate::frame_c::v4::mir::MirItem;
use crate::frame_c::v4::validator::ValidationIssue;
use crate::frame_c::v4::native_region_scanner::Region;

pub struct MirAssembler;

impl MirAssembler {
    pub fn assemble(&self, bytes: &[u8], regions: &[Region]) -> Result<Vec<MirItem>, ParseError> {
        let parser = FrameStatementParser;
        let mut out = Vec::new();
        for r in regions {
            if let Region::FrameSegment{..} = r {
                out.push(parser.parse_segment(bytes, r)?);
            }
        }
        Ok(out)
    }

    pub fn assemble_collect(&self, bytes: &[u8], regions: &[Region]) -> (Vec<MirItem>, Vec<ValidationIssue>) {
        let parser = FrameStatementParser;
        let mut out_mir = Vec::new();
        let mut issues: Vec<ValidationIssue> = Vec::new();
        for r in regions {
            if let Region::FrameSegment{..} = r {
                match parser.parse_segment(bytes, r) {
                    Ok(m) => out_mir.push(m),
                    Err(e) => {
                        use crate::frame_c::v4::frame_statement_parser::ParseErrorKind as K;
                        let msg = match e.kind {
                            K::MissingState => format!("E300: {}", e.message),
                            K::UnbalancedArgs => format!("E301: {}", e.message),
                            K::TrailingTokens => format!("E302: {}", e.message),
                            K::InvalidHead => "E200: invalid Frame statement at start of line".to_string(),
                        };
                        issues.push(ValidationIssue { message: msg });
                    },
                }
            }
        }
        (out_mir, issues)
    }
}
