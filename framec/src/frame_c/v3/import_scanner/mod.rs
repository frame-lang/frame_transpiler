use super::native_region_scanner::RegionSpan;
use super::validator::ValidationIssueV3;

#[derive(Debug, Default, Clone)]
pub struct ImportScanResultV3 {
    pub spans: Vec<RegionSpan>,
    pub issues: Vec<ValidationIssueV3>,
}

pub trait ImportScannerV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResultV3;
}

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;
// Future: pub mod typescript; pub mod csharp; pub mod c; pub mod cpp; pub mod java; pub mod rust;
