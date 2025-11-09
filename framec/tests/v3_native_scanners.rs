use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::typescript::NativeRegionScannerTsV3;
use framec::frame_c::v3::native_region_scanner::csharp::NativeRegionScannerCsV3;
use framec::frame_c::v3::native_region_scanner::{NativeRegionScannerV3, RegionV3, FrameSegmentKindV3};
use framec::frame_c::v3::native_region_scanner::c::NativeRegionScannerCV3;
use framec::frame_c::v3::native_region_scanner::cpp::NativeRegionScannerCppV3;
use framec::frame_c::v3::native_region_scanner::java::NativeRegionScannerJavaV3;
use framec::frame_c::v3::native_region_scanner::rust::NativeRegionScannerRustV3;

fn count_segments(regions: &[RegionV3], kind: FrameSegmentKindV3) -> usize {
    regions.iter().filter(|r| matches!(r, RegionV3::FrameSegment{ kind: k, .. } if *k==kind)).count()
}

#[test]
fn py_detects_transition_at_sol() {
    let src = b"{\n-> $Go()\n}\n";
    let mut s = NativeRegionScannerPyV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Transition), 1);
}

#[test]
fn ts_ignores_lambda_but_detects_forward() {
    let src = b"{\nconst f = () => 1;\n=> $^\n}\n";
    let mut s = NativeRegionScannerTsV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Forward), 1);
}

#[test]
fn cs_preprocessor_ignored_and_transition_detected() {
    let src = b"{\n#if DEBUG\n#endif\n-> $Next\n}\n";
    let mut s = NativeRegionScannerCsV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Transition), 1);
}

#[test]
fn cs_raw_string_ignored_then_forward_detected() {
    let src = b"{\nvar s = $\"\"\" inside -> $Fake \"\"\";\n=> $^\n}\n";
    let mut s = NativeRegionScannerCsV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Forward), 1);
}

#[test]
fn py_string_at_sol_does_not_match_directive() {
    let src = b"{\n\"-> $Not\"\n}\n";
    let mut s = NativeRegionScannerPyV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Transition), 0);
}

#[test]
fn ts_template_does_not_match_directive_inside() {
    let src = b"{\nconst s = `=> $^`;\n}\n";
    let mut s = NativeRegionScannerTsV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Forward), 0);
}

#[test]
fn c_comment_does_not_match_directive() {
    let src = b"{\n// $$[+]\n}\n";
    let mut s = NativeRegionScannerCV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::StackPush), 0);
}

#[test]
fn c_comment_ignored_then_stack_detected() {
    let src = b"{\n/* -> $Fake */\n$$[+]\n}\n";
    let mut s = NativeRegionScannerCV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::StackPush), 1);
}

#[test]
fn cpp_raw_string_ignored_then_transition_detected() {
    let src = b"{\nauto s = R\"( -> $Fake )\";\n-> $Go\n}\n";
    let mut s = NativeRegionScannerCppV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Transition), 1);
}

#[test]
fn java_block_comment_ignored_then_forward_detected() {
    let src = b"{\n/* => $^ in comment */\n=> $^\n}\n";
    let mut s = NativeRegionScannerJavaV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::Forward), 1);
}

#[test]
fn rust_raw_string_ignored_then_stack_pop_detected() {
    let src = b"{\nlet s = r#\" $$[-] in string \"#;\n$$[-]\n}\n";
    let mut s = NativeRegionScannerRustV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(count_segments(&res.regions, FrameSegmentKindV3::StackPop), 1);
}
