use framec::frame_c::v3::body_closer::python::BodyCloserPyV3;
use framec::frame_c::v3::body_closer::typescript::BodyCloserTsV3;
use framec::frame_c::v3::body_closer::csharp::BodyCloserCsV3;
use framec::frame_c::v3::body_closer::c::BodyCloserCV3;
use framec::frame_c::v3::body_closer::cpp::BodyCloserCppV3;
use framec::frame_c::v3::body_closer::rust::BodyCloserRustV3;
use framec::frame_c::v3::body_closer::{BodyCloserV3, CloseErrorV3Kind};
use framec::frame_c::v3::native_region_scanner::csharp::NativeRegionScannerCsV3;
use framec::frame_c::v3::native_region_scanner::c::NativeRegionScannerCV3;
use framec::frame_c::v3::native_region_scanner::rust::NativeRegionScannerRustV3;
use framec::frame_c::v3::native_region_scanner::{NativeRegionScannerV3, RegionV3};

#[test]
fn py_unterminated_triple_quote_errors() {
    let src = b"{\n\"\"\"unterminated\n}"; // missing closing triple
    let mut c = BodyCloserPyV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedString));
}

#[test]
fn ts_unterminated_block_comment_errors() {
    let src = b"{\n/* unterminated\n}";
    let mut c = BodyCloserTsV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedComment));
}

#[test]
fn cs_interpolated_verbatim_ignores_inner_braces() {
    // use a raw string for the Rust test file to avoid escaping hell in Rust itself
    let src = br#"{
var s = $@"path {root} ""quoted""";
-> $Next
}
"#;
    let mut s = NativeRegionScannerCsV3;
    let res = s.scan(src, 0).unwrap();
    // Ensure exactly one FrameSegment for transition
    assert_eq!(res.regions.iter().filter(|r| matches!(r, RegionV3::FrameSegment{..})).count(), 1);
}

#[test]
fn cs_raw_with_dollars_closer() {
    let src = b"{\nvar s = $$\"\"\" value {{x}} \"\"\";\n}\n";
    let mut c = BodyCloserCsV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len()-2);
}

#[test]
fn cs_raw_multiple_dollar_arities() {
    let src1 = br#"{
var s1 = $""" value {x} """;
}
"#;
    let src2 = br#"{
var s2 = $$""" value {{x}} """;
}
"#;
    let src3 = br#"{
var s3 = $$$""" value {{{x}}} """;
}
"#;
    let mut c = BodyCloserCsV3; assert!(c.close_byte(src1,0).is_ok());
    let mut c2 = BodyCloserCsV3; assert!(c2.close_byte(src2,0).is_ok());
    let mut c3 = BodyCloserCsV3; assert!(c3.close_byte(src3,0).is_ok());
}

#[test]
fn c_line_comment_directive_like_tokens_ignored() {
    let src = b"{\n// -> $Fake\n}\n";
    let mut s = NativeRegionScannerCV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(res.regions.iter().filter(|r| matches!(r, RegionV3::FrameSegment{..})).count(), 0);
}

#[test]
fn cpp_raw_delimiter_brace_safe() {
    let src = b"{\nauto s = R\"END( } )END\";\n}\n";
    let mut c = BodyCloserCppV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len()-2);
}

#[test]
fn rust_nested_block_comments_and_scanner_after() {
    let src = b"{\n/* outer /* inner */ still */\n-> $Go\n}\n";
    let mut s = NativeRegionScannerRustV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(res.regions.iter().filter(|r| matches!(r, RegionV3::FrameSegment{..})).count(), 1);
}

// Note: Rust lifetimes like &'a are not yet disambiguated in the closer; covered in future work.
#[test]
fn ts_nested_templates_with_directive_like_tokens() {
    let src = b"{\nconst s = `a ${ `b ${1+2}` } c`;\n=> $^\n}\n";
    // ensure scanner still finds the forward at SOL after nested templates
    let mut s = framec::frame_c::v3::native_region_scanner::typescript::NativeRegionScannerTsV3;
    let res = s.scan(src, 0).unwrap();
    assert_eq!(res.regions.iter().filter(|r| matches!(r, framec::frame_c::v3::native_region_scanner::RegionV3::FrameSegment{..})).count(), 1);
}
