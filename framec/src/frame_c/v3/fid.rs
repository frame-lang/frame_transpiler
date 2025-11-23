use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::frame_c::visitors::TargetLanguage;

/// Minimal representation of a symbol entry in a FID file.
#[derive(Debug, Clone, Deserialize)]
pub struct FidSymbol {
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FidOrigin {
    pub path: String,
    #[serde(default)]
    pub hash: Option<String>,
    #[serde(default)]
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FidFile {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub target: String,
    #[serde(rename = "sourceKind")]
    #[serde(default)]
    pub source_kind: Option<String>,
    #[serde(default)]
    pub origin: Option<FidOrigin>,
    #[serde(default)]
    pub symbols: Vec<FidSymbol>,
}

#[derive(Debug, Clone)]
pub struct FidSnapshot {
    pub file_path: PathBuf,
    pub file: FidFile,
}

#[derive(Debug, Clone)]
pub struct FidIndex {
    pub target: String,
    pub snapshots: Vec<FidSnapshot>,
    pub by_name: HashMap<String, Vec<FidSymbol>>,
}

impl FidIndex {
    pub fn empty_for(target: &str) -> Self {
        FidIndex {
            target: target.to_string(),
            snapshots: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    fn target_dir_for(lang: TargetLanguage) -> Option<&'static str> {
        match lang {
            TargetLanguage::Python3 => Some("python"),
            TargetLanguage::TypeScript => Some("typescript"),
            TargetLanguage::Rust => Some("rust"),
            _ => None,
        }
    }

    /// Load all `.fid.json` files for a given target language from the provided
    /// cache root (e.g., `.frame/cache/fid`), returning an index of symbols.
    ///
    /// This is Phase A: advisory and PRT-only. Callers are expected to gate
    /// usage via environment or CLI flags; missing directories/files are not
    /// treated as hard errors at the compiler level.
    pub fn load_for_target_from_root(
        root: &Path,
        lang: TargetLanguage,
    ) -> Result<Self, String> {
        let tdir = match Self::target_dir_for(lang) {
            Some(d) => d,
            None => return Err("FID not enabled for this target language".into()),
        };
        let target_root = root.join(tdir);
        if !target_root.is_dir() {
            return Ok(FidIndex::empty_for(tdir));
        }
        let mut snapshots = Vec::new();
        let mut by_name: HashMap<String, Vec<FidSymbol>> = HashMap::new();
        for entry in fs::read_dir(&target_root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext != "json" {
                    continue;
                }
            } else {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let file: FidFile = match serde_json::from_str(&text) {
                Ok(f) => f,
                Err(_) => {
                    // Ignore malformed FID files in Phase A; callers can surface
                    // more detailed diagnostics later if needed.
                    continue;
                }
            };
            if file.symbols.is_empty() {
                continue;
            }
            for sym in &file.symbols {
                by_name.entry(sym.name.clone()).or_default().push(sym.clone());
            }
            snapshots.push(FidSnapshot { file_path: path.clone(), file });
        }
        Ok(FidIndex {
            target: tdir.to_string(),
            snapshots,
            by_name,
        })
    }
}

/// Convenience helper: load a FID index for a given language if the environment
/// requests it. This is intentionally conservative and advisory:
///
/// - Requires `FRAME_USE_FID=1` to be set.
/// - Uses `FRAME_FID_ROOT` when present, otherwise `.frame/cache/fid`.
/// - Returns `None` if FID is disabled, unsupported for the language, or if
///   the cache directory does not exist.
pub fn load_from_env(lang: TargetLanguage) -> Option<FidIndex> {
    match std::env::var("FRAME_USE_FID").ok().as_deref() {
        Some("1") => {}
        _ => return None,
    }
    let root = std::env::var("FRAME_FID_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".frame/cache/fid"));
    let idx = FidIndex::load_for_target_from_root(&root, lang).ok()?;
    Some(idx)
}

