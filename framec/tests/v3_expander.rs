use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::typescript::NativeRegionScannerTsV3;
use framec::frame_c::v3::native_region_scanner::{NativeRegionScannerV3, RegionV3};
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::expander::{PyExpanderV3, TsExpanderV3, FrameStatementExpanderV3};

fn expand_body_py(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut scanner = NativeRegionScannerPyV3;
    let scan = scanner.scan(bytes, 0).unwrap();
    let assembler = MirAssemblerV3;
    let mir = assembler.assemble(bytes, &scan.regions).unwrap();
    let expander = PyExpanderV3;
    let mut out = String::new();
    let mut mi = 0usize;
    for r in &scan.regions {
        match r {
            RegionV3::NativeText{ span } => out.push_str(std::str::from_utf8(&bytes[span.start..span.end]).unwrap()),
            RegionV3::FrameSegment{ indent, .. } => { out.push_str(&expander.expand(&mir[mi], *indent)); mi+=1; }
        }
    }
    out
}

fn expand_body_ts(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut scanner = NativeRegionScannerTsV3;
    let scan = scanner.scan(bytes, 0).unwrap();
    let assembler = MirAssemblerV3;
    let mir = assembler.assemble(bytes, &scan.regions).unwrap();
    let expander = TsExpanderV3;
    let mut out = String::new();
    let mut mi = 0usize;
    for r in &scan.regions {
        match r {
            RegionV3::NativeText{ span } => out.push_str(std::str::from_utf8(&bytes[span.start..span.end]).unwrap()),
            RegionV3::FrameSegment{ indent, .. } => { out.push_str(&expander.expand(&mir[mi], *indent)); mi+=1; }
        }
    }
    out
}

#[test]
fn expander_py_inserts_comments_with_indent() {
    let src = "{\n    if cond:\n        -> $Next(1, 2)\n}\n";
    let out = expand_body_py(src);
    assert!(out.contains("# frame:transition Next(1, 2)"));
    assert!(out.contains("    # frame:transition"));
}

#[test]
fn expander_ts_inserts_comments_with_indent() {
    let src = "{\nif (cond) {\n  => $^\n}\n}\n";
    let out = expand_body_ts(src);
    assert!(out.contains("// frame:forward"));
}
