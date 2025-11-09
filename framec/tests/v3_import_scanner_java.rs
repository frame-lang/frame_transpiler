use framec::frame_c::v3::import_scanner::java::ImportScannerJavaV3;
use framec::frame_c::v3::import_scanner::ImportScannerV3;
use framec::frame_c::v3::prolog_scanner::PrologScannerV3;

#[test]
fn java_package_and_import_detected() {
    let src = b"@target java\npackage a.b;\nimport java.util.*;\nclass C {}\n";
    let prolog = PrologScannerV3.scan(src).unwrap();
    let spans = ImportScannerJavaV3.scan(src, prolog.end);
    assert!(spans.len() >= 2);
    assert!(src[spans[0].start..spans[0].end].starts_with(b"package"));
    assert!(src[spans[1].start..spans[1].end].starts_with(b"import"));
}

