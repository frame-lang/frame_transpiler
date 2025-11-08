use std::collections::{HashMap, VecDeque};

use super::visitors::TargetLanguage;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegionKind {
    TsTemplateLiteral,
    PyFString,
    PyTripleQuotedString,
    BlockComment,
    LineComment,
    StringLiteral,
}

#[derive(Clone, Debug)]
pub struct RegionEnvelope {
    pub id: u64,
    pub kind: RegionKind,
    pub language: TargetLanguage,
    pub start_offset: usize,
    pub end_offset: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum RegionFailure {
    UnterminatedTemplateLiteral { start_line: usize },
    UnterminatedBlockComment { start_line: usize },
    UnterminatedTripleQuote { delimiter: &'static str, start_line: usize },
    UnterminatedFStringExpr { start_line: usize },
}

#[derive(Clone, Debug)]
pub enum ScanResult {
    Ok(RegionEnvelope),
    Failure(RegionFailure),
}

pub trait RegionScanner {
    /// Return true if a region of this type begins at `idx` in `chars`.
    fn try_open(&self, chars: &[char], idx: usize) -> bool;
    /// Scan a region starting at `start_idx` and `start_line` in `source`.
    fn scan(&self, source: &str, start_idx: usize, start_line: usize) -> ScanResult;
    fn kind(&self) -> RegionKind;
    fn language(&self) -> TargetLanguage;
}

#[derive(Default)]
pub struct RegionQueue {
    jobs: VecDeque<RegionEnvelope>,
}

impl RegionQueue {
    pub fn new() -> Self { Self { jobs: VecDeque::new() } }
    pub fn enqueue(&mut self, env: RegionEnvelope) { self.jobs.push_back(env); }
    pub fn dequeue(&mut self) -> Option<RegionEnvelope> { self.jobs.pop_front() }
    pub fn is_empty(&self) -> bool { self.jobs.is_empty() }
}

// ------------------ TypeScript Template Literal Scanner (DPDA) ------------------

pub struct TsTemplateScanner;

impl TsTemplateScanner {
    pub fn new() -> Self { Self }
}

impl RegionScanner for TsTemplateScanner {
    fn try_open(&self, chars: &[char], idx: usize) -> bool {
        chars.get(idx).copied() == Some('`')
    }

    fn scan(&self, source: &str, start_idx: usize, start_line: usize) -> ScanResult {
        // Byte-based DPDA: track nested ${...}
        let bytes = source.as_bytes();
        let mut i = start_idx + 1; // skip opening backtick (byte index)
        let mut line = start_line;
        let mut depth: i32 = 0;
        while i < bytes.len() {
            let b = bytes[i];
            match b {
                b'\n' => { line += 1; i += 1; }
                b'\\' => { i = i.saturating_add(2); }
                b'`' => {
                    if depth == 0 {
                        return ScanResult::Ok(RegionEnvelope { id: 0, kind: RegionKind::TsTemplateLiteral, language: TargetLanguage::TypeScript, start_offset: start_idx, end_offset: i, start_line, end_line: line, metadata: HashMap::new() });
                    }
                    i += 1;
                }
                b'$' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'{' { depth += 1; i += 2; } else { i += 1; }
                }
                b'}' => { if depth > 0 { depth -= 1; } i += 1; }
                _ => { i += 1; }
            }
        }
        ScanResult::Failure(RegionFailure::UnterminatedTemplateLiteral { start_line })
    }

    fn kind(&self) -> RegionKind { RegionKind::TsTemplateLiteral }
    fn language(&self) -> TargetLanguage { TargetLanguage::TypeScript }
}

// ------------------ Python Triple Quote Scanner (DFA) ------------------

pub struct PyTripleQuoteScanner { delim: &'static str } // """ or '''

impl PyTripleQuoteScanner {
    pub fn new_double() -> Self { Self { delim: "\"\"\"" } }
    pub fn new_single() -> Self { Self { delim: "'''" } }
}

impl RegionScanner for PyTripleQuoteScanner {
    fn try_open(&self, chars: &[char], idx: usize) -> bool {
        let d = self.delim.chars().collect::<Vec<_>>();
        if idx + 2 >= chars.len() { return false; }
        chars[idx] == d[0] && chars[idx + 1] == d[1] && chars[idx + 2] == d[2]
    }

    fn scan(&self, source: &str, start_idx: usize, start_line: usize) -> ScanResult {
        let chars: Vec<char> = source.chars().collect();
        let d = self.delim.chars().collect::<Vec<_>>();
        let mut i = start_idx + 3; // skip opening
        let mut line = start_line;
        while i + 2 < chars.len() {
            let ch = chars[i];
            if ch == '\n' { line += 1; i += 1; continue; }
            if chars[i] == d[0] && chars[i + 1] == d[1] && chars[i + 2] == d[2] {
                return ScanResult::Ok(RegionEnvelope { id: 0, kind: RegionKind::PyTripleQuotedString, language: TargetLanguage::Python3, start_offset: start_idx, end_offset: i + 2, start_line, end_line: line, metadata: HashMap::new() });
            }
            i += 1;
        }
        ScanResult::Failure(RegionFailure::UnterminatedTripleQuote { delimiter: self.delim, start_line })
    }

    fn kind(&self) -> RegionKind { RegionKind::PyTripleQuotedString }
    fn language(&self) -> TargetLanguage { TargetLanguage::Python3 }
}

// ------------------ Python FString Scanner (DPDA-lite) ------------------

pub struct PyFStringScanner { quote: char, raw: bool }

impl PyFStringScanner {
    pub fn new(quote: char, raw: bool) -> Self { Self { quote, raw } }
}

impl RegionScanner for PyFStringScanner {
    fn try_open(&self, chars: &[char], idx: usize) -> bool {
        // opener: f" or f' or rf"/fr"
        if idx + 1 >= chars.len() { return false; }
        let c0 = chars[idx];
        let c1 = chars[idx + 1];
        if (c0 == 'f' || c0 == 'F') && (c1 == '"' || c1 == '\'') { return true; }
        if (c0 == 'r' || c0 == 'R') && (c1 == '"' || c1 == '\'') { return true; }
        if (c0 == 'r' || c0 == 'R' || c0 == 'f' || c0 == 'F') && (idx + 2 < chars.len()) {
            let c2 = chars[idx + 1];
            let c3 = chars[idx + 2];
            if (c2 == 'f' || c2 == 'F' || c2 == 'r' || c2 == 'R') && (c3 == '"' || c3 == '\'') {
                return true;
            }
        }
        false
    }

    fn scan(&self, source: &str, start_idx: usize, start_line: usize) -> ScanResult {
        // Minimal f-string scanner: handle optional rf/fr prefixes, escapes, doubled braces, and {expr} with simple depth.
        let bytes = source.as_bytes();
        let mut i = start_idx;
        // Consume possible one or two-letter prefix
        if i < bytes.len() && (bytes[i] == b'r' || bytes[i] == b'R' || bytes[i] == b'f' || bytes[i] == b'F') {
            i += 1;
            if i < bytes.len() && (bytes[i] == b'r' || bytes[i] == b'R' || bytes[i] == b'f' || bytes[i] == b'F') {
                i += 1;
            }
        }
        if i >= bytes.len() { return ScanResult::Failure(RegionFailure::UnterminatedFStringExpr { start_line }); }
        let quote = bytes[i] as char;
        i += 1;
        let mut line = start_line;
        let mut depth: i32 = 0;
        while i < bytes.len() {
            let b = bytes[i];
            match b {
                b'\n' => { line += 1; i += 1; }
                b'\\' => { i += 2; } // skip escaped char
                b'{' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'{' { i += 2; continue; }
                    depth += 1; i += 1;
                }
                b'}' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'}' { i += 2; continue; }
                    if depth > 0 { depth -= 1; i += 1; continue; }
                    i += 1; // stray }
                }
                _ => {
                    if (b as char) == quote && depth == 0 { return ScanResult::Ok(RegionEnvelope { id: 0, kind: RegionKind::PyFString, language: TargetLanguage::Python3, start_offset: start_idx, end_offset: i, start_line, end_line: line, metadata: HashMap::new() }); }
                    i += 1;
                }
            }
        }
        ScanResult::Failure(RegionFailure::UnterminatedFStringExpr { start_line })
    }

    fn kind(&self) -> RegionKind { RegionKind::PyFString }
    fn language(&self) -> TargetLanguage { TargetLanguage::Python3 }
}
