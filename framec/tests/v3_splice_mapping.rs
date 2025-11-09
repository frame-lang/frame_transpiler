use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::typescript::NativeRegionScannerTsV3;
use framec::frame_c::v3::native_region_scanner::{NativeRegionScannerV3, RegionV3};
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::expander::{PyExpanderV3, TsExpanderV3, FrameStatementExpanderV3};
use framec::frame_c::v3::splice::{SplicerV3, OriginV3};

#[test]
fn splice_builds_map_py() {
    let src = b"{\nprint(1)\n-> $Next(7)\n}\n";
    let mut scanner = NativeRegionScannerPyV3;
    let scan = scanner.scan(src, 0).unwrap();
    let asm = MirAssemblerV3;
    let mir = asm.assemble(src, &scan.regions).unwrap();
    let exp = PyExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let splicer = SplicerV3;
    let sp = splicer.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("# frame:transition Next(7)"));
    assert!(sp.splice_map.iter().any(|(_, o)| matches!(o, OriginV3::Frame{..})));
    assert!(sp.splice_map.iter().any(|(_, o)| matches!(o, OriginV3::Native{..})));
    // spans monotonically non-overlapping within text bounds
    let mut last_end = 0usize;
    for (span, _) in &sp.splice_map {
        assert!(span.start <= span.end && span.end <= sp.text.len());
        assert!(span.start >= last_end); // no overlap, ordered
        last_end = span.end;
    }
}

#[test]
fn splice_builds_map_ts() {
    let src = b"{\nconsole.log(1);\n=> $^\n}\n";
    let mut scanner = NativeRegionScannerTsV3;
    let scan = scanner.scan(src, 0).unwrap();
    let asm = MirAssemblerV3;
    let mir = asm.assemble(src, &scan.regions).unwrap();
    let exp = TsExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let splicer = SplicerV3;
    let sp = splicer.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:forward"));
    assert!(sp.splice_map.iter().any(|(_, o)| matches!(o, OriginV3::Frame{..})));
    let mut last_end = 0usize;
    for (span, _) in &sp.splice_map {
        assert!(span.start <= span.end && span.end <= sp.text.len());
        assert!(span.start >= last_end);
        last_end = span.end;
    }
}
