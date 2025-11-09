use framec::frame_c::v3::import_scanner::c::ImportScannerCV3;
use framec::frame_c::v3::import_scanner::ImportScannerV3;
use framec::frame_c::v3::prolog_scanner::PrologScannerV3;

#[test]
fn c_includes_detected_with_continuations() {
    let src = b"@target c\n#include <stdio.h>\\\n\n#include \"x.h\"\nint main() { return 0; }\n";
    let prolog = PrologScannerV3.scan(src).unwrap();
    let spans = ImportScannerCV3.scan(src, prolog.end);
    assert!(spans.len() >= 2);
    assert!(src[spans[0].start..spans[0].end].starts_with(b"#include"));
}

