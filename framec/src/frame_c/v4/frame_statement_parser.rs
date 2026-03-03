use crate::frame_c::v4::mir::MirItem;
use crate::frame_c::v4::native_region_scanner::{RegionSpan, Region, FrameSegmentKind};

#[derive(Debug)]
pub enum ParseErrorKind { InvalidHead, UnbalancedArgs, MissingState, TrailingTokens }

#[derive(Debug)]
pub struct ParseError { pub kind: ParseErrorKind, pub message: String }

impl ParseError {
    fn err(kind: ParseErrorKind, msg: &str) -> Self { Self{ kind, message: msg.to_string() } }
}

pub struct FrameStatementParser;

impl FrameStatementParser {
    pub fn parse_segment(&self, bytes: &[u8], seg: &Region) -> Result<MirItem, ParseError> {
        let (span, kind) = match seg { Region::FrameSegment{ span, kind, .. } => (*span, *kind), _ => return Err(ParseError::err(ParseErrorKind::InvalidHead, "not a frame segment")) };
        let text = &bytes[span.start..span.end];
        match kind {
            FrameSegmentKind::Transition => self.parse_transition(text, span),
            FrameSegmentKind::TransitionForward => self.parse_transition_forward(text, span),
            FrameSegmentKind::Forward => self.parse_forward(text, span),
            FrameSegmentKind::StackPush => self.parse_stack(text, span, true),
            FrameSegmentKind::StackPop => self.parse_stack(text, span, false),
            FrameSegmentKind::StateVar | FrameSegmentKind::StateVarAssign => {
                // State variables are handled inline by the splicer expansion
                // No MIR item needed - just pass through
                Err(ParseError::err(ParseErrorKind::InvalidHead, "state var handled by splicer"))
            }
            FrameSegmentKind::ReturnSugar => self.parse_return_sugar(text, span),
            // Context syntax - handled inline by the splicer expansion
            FrameSegmentKind::ContextParamShorthand |
            FrameSegmentKind::ContextReturn |
            FrameSegmentKind::ContextEvent |
            FrameSegmentKind::ContextData |
            FrameSegmentKind::ContextDataAssign |
            FrameSegmentKind::ContextParams |
            FrameSegmentKind::TaggedInstantiation => {
                Err(ParseError::err(ParseErrorKind::InvalidHead, "context/tagged syntax handled by splicer"))
            }
        }
    }

    fn parse_transition(&self, line: &[u8], span: RegionSpan) -> Result<MirItem, ParseError> {
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
        if !(i+2<=n && line[i]==b'-' && line[i+1]==b'>') { return Err(ParseError::err(ParseErrorKind::InvalidHead, "missing ->")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Optional (enter_args)
        let mut enter_args: Vec<String> = Vec::new();
        if i<n && line[i]==b'(' {
            let (arg_text, next) = self.balanced_paren_block(line, i)?;
            enter_args = self.split_top_level_commas(arg_text);
            i = next;
            while i<n && line[i].is_ascii_whitespace() { i+=1; }
        }
        // Check for pop-transition: -> $$[-]
        if i+5<=n && &line[i..i+5] == b"pop$" {
            // Pop-transition - target comes from stack at runtime
            return Ok(MirItem::Transition{
                target: "pop$".to_string(),
                exit_args: exit_args,
                enter_args: enter_args,
                state_args: vec![],
                span
            });
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
        if i>=n || line[i]!=b'$' { return Err(ParseError::err(ParseErrorKind::MissingState, "expected $State after '->'")); }
        i+=1; let name_start=i;
        if i>=n || !is_ident_start(line[i]) { return Err(ParseError::err(ParseErrorKind::MissingState, "invalid state name start")); }
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
        if i<n { return Err(ParseError::err(ParseErrorKind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(MirItem::Transition{ target, exit_args, enter_args, state_args, span })
    }

    fn parse_transition_forward(&self, line: &[u8], span: RegionSpan) -> Result<MirItem, ParseError> {
        // Expect: -> => $State
        let n = line.len();
        let mut i=0usize;
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Required '->'
        if !(i+2<=n && line[i]==b'-' && line[i+1]==b'>') { return Err(ParseError::err(ParseErrorKind::InvalidHead, "missing ->")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Required '=>'
        if !(i+2<=n && line[i]==b'=' && line[i+1]==b'>') { return Err(ParseError::err(ParseErrorKind::InvalidHead, "missing =>")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // '$' State
        if i>=n || line[i]!=b'$' { return Err(ParseError::err(ParseErrorKind::MissingState, "expected $State after '-> =>'")); }
        i+=1; let name_start=i;
        if i>=n || !is_ident_start(line[i]) { return Err(ParseError::err(ParseErrorKind::MissingState, "invalid state name start")); }
        i+=1; while i<n && is_ident(line[i]) { i+=1; }
        let target = String::from_utf8_lossy(&line[name_start..i]).to_string();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i<n { return Err(ParseError::err(ParseErrorKind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(MirItem::TransitionForward{ target, span })
    }

    fn parse_forward(&self, line: &[u8], span: RegionSpan) -> Result<MirItem, ParseError> {
        // Expect: => $^
        let mut i = 0usize; let n = line.len();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+2<=n && line[i]==b'=' && line[i+1]==b'>') { return Err(ParseError::err(ParseErrorKind::InvalidHead, "missing =>")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+2<=n && line[i]==b'$' && line[i+1]==b'^') { return Err(ParseError::err(ParseErrorKind::InvalidHead, "missing $^")); }
        i+=2; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i < n { return Err(ParseError::err(ParseErrorKind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(MirItem::Forward{ span })
    }

    fn parse_stack(&self, line: &[u8], span: RegionSpan, is_push: bool) -> Result<MirItem, ParseError> {
        // Expect: $$[+] or $$[-]
        let mut i = 0usize; let n = line.len();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if !(i+5<=n && line[i]==b'$' && line[i+1]==b'$' && line[i+2]==b'[' && (line[i+3]==b'+' || line[i+3]==b'-') && line[i+4]==b']') {
            return Err(ParseError::err(ParseErrorKind::InvalidHead, "malformed stack op"));
        }
        i+=5; while i<n && line[i].is_ascii_whitespace() { i+=1; }
        if i < n { return Err(ParseError::err(ParseErrorKind::TrailingTokens, "unexpected trailing tokens after Frame statement")); }
        Ok(if is_push { MirItem::StackPush{ span } } else { MirItem::StackPop{ span } })
    }

    fn parse_return_sugar(&self, line: &[u8], span: RegionSpan) -> Result<MirItem, ParseError> {
        // Expect: return <expr>
        let mut i = 0usize; let n = line.len();
        while i<n && line[i].is_ascii_whitespace() { i+=1; }
        // Check for return <expr>
        if i + 6 <= n && &line[i..i+6] == b"return" {
            i += 6;
            while i<n && line[i].is_ascii_whitespace() { i+=1; }
            let expr = String::from_utf8_lossy(&line[i..]).trim().to_string();
            return Ok(MirItem::ReturnSugar{ expr, span });
        }
        Err(ParseError::err(ParseErrorKind::InvalidHead, "malformed return"))
    }

    fn balanced_paren_block<'a>(&self, line: &'a [u8], open_idx: usize) -> Result<(&'a [u8], usize), ParseError> {
        let mut i=open_idx; let mut depth=0i32; let n=line.len();
        let mut in_s: Option<u8> = None; // quote char
        if line[i]!=b'(' { return Err(ParseError::err(ParseErrorKind::InvalidHead, "expected (")); }
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
        Err(ParseError::err(ParseErrorKind::UnbalancedArgs, "unterminated args"))
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
