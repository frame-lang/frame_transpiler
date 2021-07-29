use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use crate::compiler::Exe;
use crate::frame_c::scanner::TokenType::*;

enum MatchType {
    BoolTok,
    StringTok,
    NumberTok,
//    None,
}

pub(crate) struct Scanner {
    source:String,
    tokens:Vec<Token>,
    start:usize,
    current:usize,
    token_str:String,
    pub has_errors:bool,
    pub errors:String,
    // The test_t_stack stack is to parse nested tests.  It is necessary
    // because the tokenizer should change how is scans the matches based
    // on the test type. Therefore we have to remember that
    // what the current test type was in order to change the scanner
    // and pop it off when done with the test.
    test_t_stack:Vec<MatchType>,
    line:usize,
    keywords:HashMap<String,TokenType>,
//    match_type:MatchType,
}

impl Scanner {


    pub(crate) fn new(source:String) -> Scanner {
        let keywords: HashMap<String, TokenType> = [
            ("null".to_string(), TokenType::NullTok),
            ("nil".to_string(), TokenType::NilTok),
            ("true".to_string(), TokenType::TrueTok),
            ("false".to_string(), TokenType::FalseTok),
            ("var".to_string(), TokenType::VarTok),
            ("const".to_string(), TokenType::ConstTok),
            ("-interface-".to_string(), TokenType::InterfaceBlockTok),
            ("-machine-".to_string(), TokenType::MachineBlockTok),
            ("-actions-".to_string(), TokenType::ActionsBlockTok),
            ("-domain-".to_string(), TokenType::DomainBlockTok),
        ].iter().cloned().collect();

        Scanner {
            source,
            tokens: Vec::new(),
            start:0,
            current:0,
            token_str:String::new(),
            has_errors:false,
            errors:String::new(),
            test_t_stack:Vec::new(),
            line:1,
            keywords,
       //     match_type:MatchType::None,
        }
    }


    // NOTE! The self param is NOT &self. That is how
    // the member variable token can move ownership to the
    // caller.
    pub fn scan_tokens(mut self) -> (bool,String,Vec<Token>) {

        // Scan header
        while self.is_whitespace() {
            self.advance();
        }
        if self.peek() == '`' {
            self.sync_start();
            if !self.match_first_header_token() {
                return (self.has_errors,self.errors.clone(),self.tokens);
            }
            self.sync_start();
            while !self.is_at_end() {
                if self.peek() == '`' {
                    self.add_string_token_literal(SuperStringTok, TokenLiteral::None);
                    self.sync_start();
                    if self.match_last_header_token() {
                        break;
                    }
                }
                self.advance();
            }
        }

        while !self.is_at_end() {
            self.sync_start();
            self.scan_token();
        }

        // todo: the literal needs to be an optional type of generic object
        let len = self.current - self.start;
        self.tokens.push(
            Token::new(EofTok
                       , "".to_string()
                       , TokenLiteral::None
                       , self.line
                       , self.start
                       , len));
        return (self.has_errors,self.errors.clone(),self.tokens);
    }

    fn is_whitespace(&self) -> bool {
        if self.peek() == ' '
            || self.peek() == '\n'
            || self.peek() == '\r'
            || self.peek() == '\t'  {
            return true;
        }
        return false;
    }

    fn match_first_header_token(&mut self,) -> bool {
        for _i in 0..3 {
            if !self.match_char('`') {
                self.error(self.line, "Malformed header token.");
                return false
            }
        }
        self.add_string_token_literal(ThreeTicksTok, TokenLiteral::None);

        true
    }

    fn match_last_header_token(&mut self,) -> bool {
        for _i in 0..3 {
            if !self.match_char('`') {
                return false
            }
        }
        self.add_string_token_literal(ThreeTicksTok, TokenLiteral::None);

        true
    }

    fn sync_start(&mut self) {
        self.start = self.current;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c:char = self.advance();
        match c {
            '(' => self.add_token(LParenTok),
            ')' => self.add_token(RParenTok),
            '[' => self.add_token(LBracketTok),
            ']' => self.add_token(RBracketTok),
            '|' => {
                if self.match_char('|') {
                    if self.match_char('*') {
                        self.add_token(AnyMessageTok);
                    } else if self.match_char('.') {
                        self.add_token(PipePipeDotTok);
                    } else if self.match_char('[') {
                        self.add_token(PipePipeLBracketTok);
                    } else {
                        self.add_token(PipePipeTok);
                    }
                } else {
                    self.add_token(PipeTok)
                }
            },
            '*' => self.add_token(StarTok),
            '+' => self.add_token(PlusTok),
            '!' => {
                if self.match_char('=') {
                    self.add_token(BangEqualTok);
                } else {
                    self.add_token(BangTok);
                }

            }
            '$' => {
                enum StackType {Push,Pop}

                if self.match_char('$') {
                    let st;
                    if self.match_char('[') {
                        if self.match_char('+') {
                            st = StackType::Push;
                        } else if self.match_char('-') {
                            st = StackType::Pop;
                        } else {
                            self.error(self.line, "Unexpected character.");
                            return;
                        }
                        if !self.match_char(']') {
                            self.error(self.line, "Unexpected character.");
                            return;
                        }
                        match st {
                            StackType::Push => { self.add_token(StateStackOperationPushTok); return;},
                            StackType::Pop => { self.add_token(StateStackOperationPopTok); return} ,
                        }
                    }
                }

                self.add_token(StateTok)
            },
            '^' => self.add_token(CaretTok),
            '>' => {
                if self.match_char('>') {
                    if self.match_char('>') {
                        self.add_token(GTx3Tok);
                    } else {
                        self.add_token(GTx2Tok);
                    }
                } else if self.match_char('=') {
                    self.add_token(GreaterEqualTok);
                } else {
                    self.add_token(GTTok);
                }

            },
            '<' => {
                if self.match_char('<') {
                    if self.match_char('<') {
                        self.add_token(LTx3Tok);
                    } else {
                        self.add_token(LTx2Tok);
                    }
                } else if self.match_char('=') {
                    self.add_token(LessEqualTok);
                } else {
                    self.add_token(LTTok);
                }

            },
            '&' => {
                if self.match_char('&') {
                    self.add_token(LogicalAndTok)
                } else if self.match_char('|') {
                    self.add_token(LogicalXorTok)
                } else {
                    self.add_token(AndTok)
                }
            },
            '?' => {
                if self.match_char('!') {
                    self.add_token(BoolTestFalseTok);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::BoolTok);
                } else if self.match_char('~') {
                    self.add_token(StringTestTok);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::StringTok);
                } else if self.match_char('#') {
                    self.add_token(NumberTestTok);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::NumberTok);
                } else {
                    self.add_token(BoolTestTrueTok);
                    // Store the context for the parse
                    self.test_t_stack.push(MatchType::BoolTok);
                }
            },
            '@' => self.add_token(AtTok),
            ' ' => return,
            '\r' => return,
            '\t' => return,
            '\n' => {
            //    self.line += 1;
                return;
            },
            '-' => {
                if !self.block_keyword() {
                    if self.match_char('>') {
                        // -> or ->>
                        if self.match_char('>') {
                            // ->>
                            self.add_token(ChangeStateTok);
                        } else {
                            // ->
                            self.add_token(TransitionTok);
                        }
                    } else if self.match_char('-') {
                        // --- comment text
                        if self.match_char('-') {
                            self.single_line_comment();
                        } else {
                            self.add_token(DashDashTok);
                        }
                    } else if self.is_digit(self.peek()) {
                        self.number();
                    } else {
                        self.add_token(DashTok);
                    }
                }
            },
            '{' => {
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.multi_line_comment();
                    } else {
                        panic!("Unexpected character.");
                    }
                } else {
                    self.add_token(OpenBraceTok);
                }
            },
            '}' => {
                self.add_token(CloseBraceTok);
            }
            ':' => {
                if self.match_char(':') {
                    self.add_token(TestTerminatorTok);
                    self.test_t_stack.pop();
                } else if self.match_char('>') {
                    self.add_token(ElseContinueTok);
                } else {
                    self.add_token(ColonTok);
                }
            },
            ';' => self.add_token(SemicolonTok),
            '"' => self.string(),
            '`' => self.super_string(),
            '#' => {
                if self.match_char('#') {
                    self.add_token(SystemEndTok);
                } else if self.match_char('[') {
                    self.add_token(OuterAttributeTok)   // #[
                } else if self.match_char('!') {
                    if self.match_char('[') {           // #![
                        self.add_token(InnerAttributeTok);
                    } else {
                        self.add_token(ErrorTok);       // #!
                    }
                } else {
                    self.add_token(SystemTok);
                }
            },
            '=' => {
                if self.match_char('>') {
                    self.add_token(DispatchTok);
                } else if self.match_char('=') {
                    self.add_token(EqualEqualTok);
                } else {
                    self.add_token(EqualsTok);
                }
            },
            '/' => {
                if self.match_char('/') {
                    if self.match_char('!') {
                        self.add_token(MatchNullStringTok);
                    } else {
                        self.add_token(MatchEmptyStringTok);
                    }
                } else {
                    self.add_token_sync_start(ForwardSlashTok);
                    self.scan_match();
                }
            },
            '.' => {
                self.add_token(DotTok);
            },
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error(self.line,&format!("Found unexpected character '{}'.",c));
                    self.add_token(ErrorTok);
                }
            }
        }
    }

    fn match_char(&mut self,expected:char) -> bool {
        if self.is_at_end() { return false; }
        let c = self.source.as_bytes()[self.current] as char;
        if c != expected {
            return false;
        }
        self.current += 1;
        self.token_str = String::from(&self.source[self.start..self.current]);

        true
    }

    // TODO: beware - mixing UTF-8 strings and chars here
    fn advance(&mut self) -> char {
        self.current += 1;
        self.token_str = String::from(&self.source[self.start..self.current]);
        let c:char = self.source.as_bytes()[self.current - 1] as char;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let c:char = self.source.as_bytes()[self.current] as char;
        c
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn is_digit(&self, c:char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        }
        while self.is_digit(self.peek()) {
            self.advance();
        }

        let number: f32 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_literal(NumberTok, TokenLiteral::Float(number));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // See if the identifier is a reserved word.
        let text = &self.source[self.start..self.current].to_owned();

        let kw = &self.keywords.get(text);
        if let Some(keyword) = kw {
            let tok_type = (*keyword).clone();
            self.add_token(tok_type);
        } else {
            self.add_token(IdentifierTok);
        }
    }

    // TODO: handle EOF w/ error
    fn single_line_comment(&mut self) {
        if !self.is_at_end() {
            while self.peek() != '\n' {
                self.advance();
            }
            self.add_token(SingleLineCommentTok);
            return;
        }
    }

    // TODO: handle EOF w/ error
    fn multi_line_comment(&mut self) {
        while !self.is_at_end() {
            while self.peek() != '-' {
                self.advance();
            }
            self.advance();
            if self.peek() != '-' {
                continue;
            }
            self.advance();
            if self.peek() != '}' {
                continue;
            }
            self.advance();

            self.add_token(MultiLineCommentTok);
            return;
        }
    }

    fn scan_match(&mut self) {
        match self.test_t_stack.last() {
            Some(MatchType::StringTok) => self.scan_string_match(),
            Some(MatchType::NumberTok) => self.scan_number_match(),
            Some(_) => {},
            None => {},
        }
    }

    // Scan the string looking for the end of the match test ('/')
    // or the end of the current match string ('|').
    // match_string_test -> '/' match_string_pattern ('|' match_string_pattern)* '/'

    fn scan_string_match(&mut self) {
        while self.peek() != '/' {
            if self.peek() == '|' {
                self.add_token_sync_start(MatchStringTok);
                self.advance();
                self.add_token_sync_start(PipeTok);
            }
            self.advance();
        }
        self.add_token_sync_start(MatchStringTok);
        self.advance();
        self.add_token_sync_start(ForwardSlashTok);
    }

    // match_number_test -> '/' match_number_pattern ('|' match_number_pattern)* '/'

    fn scan_number_match(&mut self) {
        while self.peek() != '/' {
            if self.peek() == '|' {
                self.number();
                self.advance();
                self.add_token_sync_start(PipeTok);
            }
            self.advance();
        }
        self.number();

        self.sync_start();
        if !self.match_char('/') {
            // TODO
            panic!("todo");
        }
        self.add_token_sync_start(ForwardSlashTok);
    }

    fn block_keyword(&mut self) -> bool {

        // TODO: handle this:
        // #M1
        //     -in-
        // ##

        let start_pos = self.current;
        // let mut block_name:&str;

        let block_sections= [
            ("interface-", InterfaceBlockTok),
            ("machine-", MachineBlockTok),
            ("actions-", ActionsBlockTok),
            ("domain-", DomainBlockTok),
        ];

        // TODO: this is **horribly** ineffcient.

        for (block_name,token_type) in block_sections.iter() {
            for (i,c) in block_name.chars().enumerate() {
                if !self.match_char(c) {
                    break;
                }
                if i == block_name.len() - 1 {
                    self.add_token(*token_type);
                    return true;
                }
            }

            self.current = start_pos;
        }

        self.current = start_pos;
        false
    }

    fn is_alpha(&self, c:char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn is_alpha_numeric(&self, c:char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn add_token_sync_start(&mut self, tok_type:TokenType) {
        self.add_token_literal(tok_type, TokenLiteral::None);
        self.sync_start();
    }

    fn add_token(&mut self, tok_type:TokenType) {
        Exe::debug_print(&format!("{:?}", tok_type));
        self.add_token_literal(tok_type, TokenLiteral::None);
    }

    fn add_token_literal(&mut self, tok_type:TokenType, literal:TokenLiteral) {
        let lex = self.source[self.start..self.current].to_owned();
        let len = self.current - self.start;
        self.tokens.push(Token::new(tok_type, lex, literal, self.line, self.start,len));
    }

    fn add_string_token_literal(&mut self, tok_type:TokenType, literal:TokenLiteral) {
        let lex = self.source[self.start+1..self.current-1].to_owned();
        let len = self.current - self.start;
        self.tokens.push(Token::new(tok_type, lex, literal, self.line,self.start,len));
    }

    fn error(&mut self, line:usize,error_msg:&str) {
        let error = &format!("Line {} : Error: {}\n", line, error_msg);
        self.has_errors = true;
        self.errors.push_str(error);
    }

    fn string(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // self.line += 1;
            } else if c == '"' {
                break;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            self.error(self.line,"Unterminated string.");
        }

        self.advance();
        self. add_string_token_literal(StringTok, TokenLiteral::None);
    }

    fn super_string(&mut self) {
        let start_line = self.line;
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // self.line += 1;
            } else if c == '`' {
                break;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            self.error(start_line,"Unterminated super string.");
            return;
        }

        self.advance();
        self.add_string_token_literal(SuperStringTok, TokenLiteral::None);
    }
}


#[derive(Copy, Clone)]
#[derive(Debug,PartialOrd, PartialEq)]
pub enum TokenType {
    EofTok,
    IdentifierTok,
    StateTok,
    GTTok,                  // >
    GTx2Tok,                // >>
    GTx3Tok,                // >>
    PlusTok,                // +
    DashTok,                // -
    DashDashTok,            // --
    StarTok,                // *
    EqualEqualTok,          // ==
    BangTok,                // !
    BangEqualTok,           // !=
    GreaterEqualTok,        // >=
    LessEqualTok,           // <=
    LTTok,                  // <
    LTx2Tok,                // <<
    LTx3Tok,                // <<<
    AndTok,                 // &
    PipeTok,                // |
    CaretTok,               // ^
    LogicalAndTok,          // &&
    LogicalXorTok,          // &|
    SystemTok,              // #
    SystemEndTok,           // ##
    OuterAttributeTok,      // #[
    InnerAttributeTok,      // #![
    InterfaceBlockTok,      // -interface-
    MachineBlockTok,        // -machine-
    ActionsBlockTok,        // -actions-
    DomainBlockTok,         // -domain-
    LParenTok,
    RParenTok,
    LBracketTok,
    RBracketTok,
    TransitionTok,
    ChangeStateTok,
    StringTok,
    ThreeTicksTok,                  // ```
    SuperStringTok,                 // `stuff + "stuff"`
    NumberTok,                      // 1, 1.01
    VarTok,                         // let
    ConstTok,                       // const
    SingleLineCommentTok,           // --- comment
    MultiLineCommentTok,            // {-- comments --}
    OpenBraceTok,                   // {
    CloseBraceTok,                  // }
    TrueTok,                        // true
    FalseTok,                       // false
    NullTok,                        // null
    NilTok,                         // nil
    ColonTok,                       // :
    SemicolonTok,                   // ;
    DispatchTok,                    // =>
    EqualsTok,                      // =
    BoolTestTrueTok,                // ?
    BoolTestFalseTok,               // ?!
    StringTestTok,                  // ?~
    NumberTestTok,                  // ?#
    ElseContinueTok,                // :>
    TestTerminatorTok,              // ::
    ForwardSlashTok,                // /
    MatchStringTok,                 // /<string>/ - contains <string>
    MatchNullStringTok,             // //!
    MatchEmptyStringTok,            // //
    StateStackOperationPushTok,     // $$[+]
    StateStackOperationPopTok,      // $$[-]
    DotTok,                         // .
    AtTok,                          // @
    PipePipeTok,                    // ||
    PipePipeDotTok,                 // ||.
    PipePipeLBracketTok,            // ||[
    AnyMessageTok,                  // ||*
    ErrorTok,

}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self)
    }
}

#[derive(Debug,Clone)]
pub enum TokenLiteral {
    //Integer(i32),
    Float(f32),
    // Double(f64),
    None,
}

impl Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self)
    }
}

#[derive(Debug,Clone)]
pub struct Token {
    pub token_type:TokenType,
    pub lexeme:String,
    literal:TokenLiteral,
    pub line:usize,
    pub start:usize,
    pub length:usize,
}

impl Token {

    pub fn new(token_type:TokenType,lexeme:String,literal:TokenLiteral,line:usize,start:usize,length:usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            start,
            length,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} {} {}",self.token_type,self.lexeme,self.literal)
    }
}

