use super::native_region_scanner::RegionSpan;

pub trait ImportScannerV3 {
    fn scan(&self, bytes: &[u8], start: usize) -> Vec<RegionSpan>;
}

pub mod python;
pub mod typescript;
pub mod csharp;
pub mod c;
pub mod cpp;
pub mod java;
pub mod rust;
// Future: pub mod typescript; pub mod csharp; pub mod c; pub mod cpp; pub mod java; pub mod rust;
