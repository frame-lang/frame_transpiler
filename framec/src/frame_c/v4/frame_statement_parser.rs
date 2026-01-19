use crate::frame_c::v4::mir::MirItemV3;
use crate::frame_c::v4::native_region_scanner::{RegionSpan, RegionV3, FrameSegmentKindV3};

#[derive(Debug)]
pub enum ParseErrorV3Kind { InvalidHead, UnbalancedArgs, MissingState, TrailingTokens }

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
            FrameSegmentKindV3::Forward => self.parse_forward(text, span),
            FrameSegmentKindV3::StackPush => self.parse_stack(text, span, true),
            FrameSegmentKindV3::StackPop => self.parse_stack(text, span, false),
        }
    }

    fn parse_transition(&self, line: &[u8], span: RegionSpan) -> Result<MirItemV3, ParseErrorV3> {
        // Expect: (exit_args)? -> (enter_args)? [$label?] $State(state_params?)
        let n = line.len();
        let mut i=0usize;
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Optional (exit_args)
        let mut exit_args: Vec<String> = Vec::new();
        if i<n && line[i]==b'(' {
            let (arg_text, next) = self.balanced_paren_block(line, i)?;
            exit_args = self.split_top_level_commas(arg_text);
            i = next;
            while i<n && line[i].is_ascii_whitespace() { i+=1; }
        }
        // Required '->'
        if !(i+2<=n && line[i]==b'-' && line[i+1]==b'>') { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing ->")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Optional (enter_args)
        let mut enter_args: Vec<String> = Vec::new();
        if i<n && line[i]==b'(' {
            let (arg_text, next) = self.balanced_paren_block(line, i)?;
            enter_args = self.split_top_level_commas(arg_text);
            i = next;
            while i<n && line[i].is_ascii_whitespace() { i+=1; }
        }
        // Optional label: read an identifier if present, but ignore it for now
        if i<n && (line[i].is_ascii_alphabetic() || line[i]==b'_') {
            let mut j=i+1; while j<n && (line[j].is_ascii_alphanumeric() || line[j]==b'_') { j+=1; }
            // Only treat as label if next non-space isn't '$'
            let mut k=j; while k<n && line[k].is_ascii_whitespace() { k+=1; }
            if k<n && line[k]!=b'$' {
                i = k; // skip label
            }
        }
        // '$' State
        if i>=n || line[i]!=b'$' { return Err(ParseErrorV3::err(ParseErrorV3Kind::MissingState, "expected $State after '->'")); }
        i+=1; let name_start=i;
        if i>=n || !is_ident_start(line[i]) { return Err(ParseErrorV3::err(ParseErrorV3Kind::MissingState, "invalid state name start")); }
        i+=1; while i<n && is_ident(line[i]) { i+=1; }
        let target = String::from_utf8_lossy(&line[name_start..i]).to_string();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Optional (state_params)
        let mut state_args: Vec<String> = Vec::new();
        if i<n && line[i]==b'(' {
            let (arg_text, next) = self.balanced_paren_block(line, i)?;
            state_args = self.split_top_level_commas(arg_text);
            i = next;
        }
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i<n { return Err(ParseErrorV3::err(ParseErrorV3Kind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(MirItemV3::Transition{ target, exit_args, enter_args, state_args, span })
    }

    fn parse_forward(&self, line: &[u8], span: RegionSpan) -> Result<MirItemV3, ParseErrorV3> {
        // Expect: => $^
        let mut i = 0usize; let n = line.len();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+2<=n && line[i]==b'=' && line[i+1]==b'>') { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing =>")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+2<=n && line[i]==b'$' && line[i+1]==b'^') { return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "missing $^")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i < n { return Err(ParseErrorV3::err(ParseErrorV3Kind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(MirItemV3::Forward{ span })
    }

    fn parse_stack(&self, line: &[u8], span: RegionSpan, is_push: bool) -> Result<MirItemV3, ParseErrorV3> {
        // Expect: $$[+] or $$[-]
        let mut i = 0usize; let n = line.len();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+5<=n && line[i]==b'$' && line[i+1]==b'$' && line[i+2]==b'[' && (line[i+3]==b'+' || line[i+3]==b'-') && line[i+4]==b']') {
            return Err(ParseErrorV3::err(ParseErrorV3Kind::InvalidHead, "malformed stack op"));
        }
        i+=5; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i < n { return Err(ParseErrorV3::err(ParseErrorV3Kind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(if is_push { MirItemV3::StackPush{ span } } else { MirItemV3::StackPop{ span } })
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

fn is_ident_start(b: u8) -> bool { b.is_ascii_alphabetic() || b==b'_' }
fn is_ident(b: u8) -> bool { b.is_ascii_alphanumeric() || b==b'_' }
