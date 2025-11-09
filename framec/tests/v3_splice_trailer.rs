use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::expander::{PyExpanderV3, FrameStatementExpanderV3};
use framec::frame_c::v3::splice::SplicerV3;

#[test]
fn build_trailer_json_contains_entries() {
    let src = b"{\n-> $Go(1)\n}\n";
    let scan = NativeRegionScannerPyV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let exps: Vec<String> = mir.iter().map(|m| PyExpanderV3.expand(m, 0)).collect();
    let sp = SplicerV3.splice(src, &scan.regions, &exps);
    let trailer = sp.build_trailer_json();
    assert!(trailer.contains("\"map\""));
    assert!(trailer.contains("\"origin\":\"frame\""));
}

