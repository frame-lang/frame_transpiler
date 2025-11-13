#![allow(dead_code)]

use std::collections::HashMap;
use crate::frame_c::visitors::TargetLanguage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FidSymbolKind {
    Function,
    Method,
    Type,
    Const,
}

#[derive(Debug, Clone)]
pub struct FidParam {
    pub name: String,
    pub ty: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FidSymbol {
    pub name: String,
    pub kind: FidSymbolKind,
    pub params: Vec<FidParam>,
    pub result: Option<String>,
}

#[derive(Debug)]
pub struct FidIndex {
    pub language: TargetLanguage,
    pub symbols: HashMap<String, FidSymbol>,
}

impl FidIndex {
    pub fn new(language: TargetLanguage) -> Self {
        FidIndex { language, symbols: HashMap::new() }
    }

    pub fn insert(&mut self, sym: FidSymbol) {
        self.symbols.insert(sym.name.clone(), sym);
    }

    pub fn get(&self, name: &str) -> Option<&FidSymbol> {
        self.symbols.get(name)
    }
}

/// Phase A stub: build a minimal FID index from an in-memory list.
pub fn build_fid_index(language: TargetLanguage, entries: Vec<FidSymbol>) -> FidIndex {
    let mut idx = FidIndex::new(language);
    for e in entries { idx.insert(e); }
    idx
}
