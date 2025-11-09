use framec::frame_c::v3::native_region_scanner::csharp::NativeRegionScannerCsV3;
use framec::frame_c::v3::native_region_scanner::c::NativeRegionScannerCV3;
use framec::frame_c::v3::native_region_scanner::cpp::NativeRegionScannerCppV3;
use framec::frame_c::v3::native_region_scanner::java::NativeRegionScannerJavaV3;
use framec::frame_c::v3::native_region_scanner::rust::NativeRegionScannerRustV3;
use framec::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::expander::{CExpanderV3, CppExpanderV3, JavaExpanderV3, RustExpanderV3, FrameStatementExpanderV3};
use framec::frame_c::v3::splice::{SplicerV3, OriginV3};

#[test]
fn splice_cs_maps_frame_and_native() {
    let src = br#"{
// native
-> $Go(1)
}
"#;
    let bytes = src;
    let scan = NativeRegionScannerCsV3.scan(bytes, 0).unwrap();
    let mir = MirAssemblerV3.assemble(bytes, &scan.regions).unwrap();
    let exp = CExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let sp = SplicerV3.splice(bytes, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:transition Go(1)"));
    assert!(sp.splice_map.iter().any(|(_, o)| matches!(o, OriginV3::Frame{..})));
    assert!(sp.splice_map.iter().any(|(_, o)| matches!(o, OriginV3::Native{..})));
}

#[test]
fn splice_c_maps_frame_and_native() {
    let src = b"{\n// n\n=> $^\n}\n";
    let scan = NativeRegionScannerCV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let exp = CExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let sp = SplicerV3.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:forward"));
}

#[test]
fn splice_cpp_maps_frame_and_native() {
    let src = b"{\nauto s = R\"TAG( } )TAG\";\n$$[+]\n}\n";
    let scan = NativeRegionScannerCppV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let exp = CppExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let sp = SplicerV3.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:stack_push"));
}

#[test]
fn splice_java_maps_frame_and_native() {
    let src = b"{\n-> $S\n}\n";
    let scan = NativeRegionScannerJavaV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let exp = JavaExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let sp = SplicerV3.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:transition S()") || sp.text.contains("// frame:transition S"));
}

#[test]
fn splice_rust_maps_frame_and_native() {
    let src = b"{\n// rust\n=> $^\n}\n";
    let scan = NativeRegionScannerRustV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let exp = RustExpanderV3;
    let exps: Vec<String> = mir.iter().map(|m| exp.expand(m, 0)).collect();
    let sp = SplicerV3.splice(src, &scan.regions, &exps);
    assert!(sp.text.contains("// frame:forward"));
}
