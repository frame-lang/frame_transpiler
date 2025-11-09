use framec::frame_c::v3::import_scanner::cpp::ImportScannerCppV3;
use framec::frame_c::v3::import_scanner::ImportScannerV3;
use framec::frame_c::v3::prolog_scanner::PrologScannerV3;

#[test]
fn cpp_includes_and_using_detected() {
    let src = b"@target cpp\n#include <vector>\nusing namespace std;\nauto s = R\"( import not real )\";\n";
    let prolog = PrologScannerV3.scan(src).unwrap();
    let spans = ImportScannerCppV3.scan(src, prolog.end);
    assert!(spans.len() >= 2);
    assert!(src[spans[0].start..spans[0].end].starts_with(b"#include"));
    assert!(src[spans[1].start..spans[1].end].starts_with(b"using"));
}

