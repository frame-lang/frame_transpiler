//! Scanner traits and factory functions
//!
//! This module eliminates the 6x scanner duplication by providing a unified
//! trait and factory function for language-specific scanners.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::native_region_scanner::{
    NativeRegionScannerV3, ScanResultV3, ScanErrorV3,
    python::NativeRegionScannerPyV3,
    typescript::NativeRegionScannerTsV3,
    rust::NativeRegionScannerRustV3,
    csharp::NativeRegionScannerCsV3,
    c::NativeRegionScannerCV3,
    cpp::NativeRegionScannerCppV3,
    java::NativeRegionScannerJavaV3,
};

/// Trait for region scanning (wrapper around NativeRegionScannerV3)
pub trait RegionScannerTrait: Send + Sync {
    /// Scan native code for Frame regions
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3>;
}

// Implement trait for all scanner types
impl RegionScannerTrait for NativeRegionScannerPyV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerTsV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerRustV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerCsV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerCV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerCppV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

impl RegionScannerTrait for NativeRegionScannerJavaV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        NativeRegionScannerV3::scan(self, bytes, open_brace_index)
    }
}

/// Get the appropriate region scanner for a target language
///
/// This factory function replaces the duplicated match statements that were
/// scattered throughout mod.rs.
pub fn get_region_scanner(target: TargetLanguage) -> Box<dyn RegionScannerTrait> {
    match target {
        TargetLanguage::Python3 => Box::new(NativeRegionScannerPyV3),
        TargetLanguage::TypeScript => Box::new(NativeRegionScannerTsV3),
        TargetLanguage::Rust => Box::new(NativeRegionScannerRustV3),
        TargetLanguage::CSharp => Box::new(NativeRegionScannerCsV3),
        TargetLanguage::C => Box::new(NativeRegionScannerCV3),
        TargetLanguage::Cpp => Box::new(NativeRegionScannerCppV3),
        TargetLanguage::Java => Box::new(NativeRegionScannerJavaV3),
        // Fallback for unsupported languages
        _ => Box::new(NativeRegionScannerPyV3), // Python as default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scanner_python() {
        let scanner = get_region_scanner(TargetLanguage::Python3);
        // Just verify it returns a scanner
        assert!(true);
        let _ = scanner;
    }

    #[test]
    fn test_get_scanner_typescript() {
        let scanner = get_region_scanner(TargetLanguage::TypeScript);
        let _ = scanner;
    }

    #[test]
    fn test_get_scanner_all_languages() {
        let languages = vec![
            TargetLanguage::Python3,
            TargetLanguage::TypeScript,
            TargetLanguage::Rust,
            TargetLanguage::CSharp,
            TargetLanguage::C,
            TargetLanguage::Cpp,
            TargetLanguage::Java,
        ];

        for lang in languages {
            let scanner = get_region_scanner(lang);
            let _ = scanner;
        }
    }
}
