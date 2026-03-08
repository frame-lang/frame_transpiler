use super::native_region_scanner::RegionSpan;
use super::validator::ValidationIssue;

#[derive(Debug, Default, Clone)]
pub struct ImportScanResult {
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssue>,
}

pub trait ImportScanner {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult;
}

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;
// Future: pub mod typescript; pub mod csharp; pub mod c; pub mod cpp; pub mod java; pub mod rust;
