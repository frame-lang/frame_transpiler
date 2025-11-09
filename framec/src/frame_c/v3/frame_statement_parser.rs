use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::{RegionSpan, RegionV3, FrameSegmentKindV3};

#[derive(Debug)]
pub enum ParseErrorV3Kind { InvalidHead, UnbalancedArgs }

#[derive(Debug)]
pub struct ParseErrorV3 { pub kind: ParseErrorV3Kind, pub message: String }

impl ParseErrorV3 {
    fn err(kind: ParseErrorV3Kind, msg: &str) -> Self { Self{ kind, message: msg.to_string() } }
}

pub struct FrameStatementParserV3;

impl FrameStatementParserV3 {
    pub fn parse_segment(&self, bytes: &[u8], seg: &RegionV3) -> Result<MirItemV3, ParseErrorV3> {
        let (span, kind) = match seg { RegionV3::FrameSegment{ span, kind, .. } => (*span, *kind), _ => return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "not a frame segment")) };
        let text = &bytes[span.start..span.end];
        match kind {
            FrameSegmentKindV3::Transition => self.parse_transition(text, span),
            FrameSegmentKindV3::Forward => Ok(MirItemV3::Forward{ span }),
            FrameSegmentKindV3::StackPush => Ok(MirItemV3::StackPush{ span }),
            FrameSegmentKindV3::StackPop => Ok(MirItemV3::StackPop{ span }),
        }
    }

    fn parse_transition(&self, line: &[u8], span: RegionSpan) -> Result<MirItemV3, ParseErrorV3> {
        // Expect: -> $State(args?)
        // Find "$" then identifier
        let mut i=0usize;
        while i<line.len() && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+2<=line.len() && line[i]==b'-' && line[i+1]==b'>') { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing ->")); }
        i+=2; while i<line.len() && line[i].is_ascii_whitespace() { i+=1; }
        if i>=line.len() || line[i]!=b'$' { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing $")); }
        i+=1; let name_start=i; while i<line.len() && is_ident(line[i]) { i+=1; }
        if i==name_start { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing state name")); }
        let target = String::from_utf8_lossy(&line[name_start..i]).to_string();
        while i<line.len() && line[i].is_ascii_whitespace() { i+=1; }
        let mut args: Vec<String> = Vec::new();
        if i<line.len() && line[i]==b'(' {
            let (arg_text, next) = self.balanced_paren_block(line, i)?; // returns content inside parens
            args = self.split_top_level_commas(arg_text);
            i = next;
        }
        Ok(MirItemV3::Transition{ target, args, span })
    }

    fn balanced_paren_block<'a>(&self, line: &'a [u8], open_idx: usize) -> Result<(&'a [u8], usize), ParseErrorV3> {
        let mut i=open_idx; let mut depth=0i32; let n=line.len();
        let mut in_s: Option<u8> = None; // quote char
        if line[i]!=b'(' { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "expected (")); }
        while i<n { let c=line[i];
            if let Some(q)=in_s { if c==b'\\' { i+=2; continue; } if c==q { in_s=None; i+=1; continue; } i+=1; continue; }
            match c {
                b'\''|b'"' => { in_s=Some(c); i+=1; }
                b'(' => { depth+=1; i+=1; }
                b')' => { depth-=1; i+=1; if depth==0 { let start=open_idx+1; let end=i-1; return Ok((&line[start..end], i)); } }
                b'['|b'{' => { depth+=1; i+=1; }
                b']'|b'}' => { depth-=1; i+=1; }
                _ => { i+=1; }
            }
        }
        Err(ParseErrorV3::err(ParseErrorV3Kind::UnbalancedArgs, "unterminated args"))
    }

    fn split_top_level_commas(&self, text: &[u8]) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut i=0usize; let n=text.len(); let mut depth=0i32; let mut in_s: Option<u8>=None; let mut start=0usize;
        while i<n {
            let c=text[i];
            if let Some(q)=in_s { if c==b'\\' { i+=2; continue; } if c==q { in_s=None; i+=1; continue; } i+=1; continue; }
            match c { b'\''|b'"' => { in_s=Some(c); i+=1; }
                b'('|b'['|b'{' => { depth+=1; i+=1; }
                b')'|b']'|b'}' => { depth-=1; i+=1; }
                b',' if depth==0 => {
                    let s = String::from_utf8_lossy(&text[start..i]).trim().to_string();
                    if !s.is_empty() { out.push(s); }
                    i+=1; start=i;
                }
                _ => { i+=1; }
            }
        }
        let s = String::from_utf8_lossy(&text[start..n]).trim().to_string();
        if !s.is_empty() { out.push(s); }
        out
    }
}

fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b==b'_' }
