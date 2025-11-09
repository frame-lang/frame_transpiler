use framec::frame_c::v3::native_region_scanner::c::NativeRegionScannerCV3;
use framec::frame_c::v3::native_region_scanner::cpp::NativeRegionScannerCppV3;
use framec::frame_c::v3::native_region_scanner::java::NativeRegionScannerJavaV3;
use framec::frame_c::v3::native_region_scanner::rust::NativeRegionScannerRustV3;
use framec::frame_c::v3::native_region_scanner::{NativeRegionScannerV3, RegionV3};
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::expander::{CExpanderV3, CppExpanderV3, JavaExpanderV3, RustExpanderV3, FrameStatementExpanderV3};

fn expand(bytes: &[u8], open: usize, scanner: &mut dyn NativeRegionScannerV3, expander: &dyn FrameStatementExpanderV3) -> String {
    let scan = scanner.scan(bytes, open).unwrap();
    let assembler = MirAssemblerV3;
    let mir = assembler.assemble(bytes, &scan.regions).unwrap();
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
fn expander_c_comment_markers() {
    let src = b"{\n// native\n-> $Next(42)\n}\n";
    let out = expand(src, 0, &mut NativeRegionScannerCV3, &CExpanderV3);
    assert!(out.contains("// frame:transition Next(42)"));
}

#[test]
fn expander_cpp_raw_and_transition() {
    let src = b"{\nauto s = R\"TAG( } )TAG\";\n-> $Go()\n}\n";
    let out = expand(src, 0, &mut NativeRegionScannerCppV3, &CppExpanderV3);
    assert!(out.contains("// frame:transition Go()"));
}

#[test]
fn expander_java_forward() {
    let src = b"{\n// x\n=> $^\n}\n";
    let out = expand(src, 0, &mut NativeRegionScannerJavaV3, &JavaExpanderV3);
    assert!(out.contains("// frame:forward"));
}

#[test]
fn expander_rust_stack_ops() {
    let src = b"{\n// y\n$$[+]\n$$[-]\n}\n";
    let out = expand(src, 0, &mut NativeRegionScannerRustV3, &RustExpanderV3);
    assert!(out.contains("// frame:stack_push"));
    assert!(out.contains("// frame:stack_pop"));
}

