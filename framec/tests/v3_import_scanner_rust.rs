use framec::frame_c::v3::import_scanner::rust::ImportScannerRustV3;
use framec::frame_c::v3::import_scanner::ImportScannerV3;
use framec::frame_c::v3::prolog_scanner::PrologScannerV3;

#[test]
fn rust_use_and_extern_detected_with_raw_strings_ignored() {
    let src = b"@target rust\nuse std::{fmt, io};\nextern crate foo;\nlet s = r#\"use not real\"#;\n";
    let prolog = PrologScannerV3.scan(src).unwrap();
    let spans = ImportScannerRustV3.scan(src, prolog.end);
    assert!(spans.len() >= 2);
    assert!(src[spans[0].start..spans[0].end].starts_with(b"use"));
    assert!(src[spans[1].start..spans[1].end].starts_with(b"extern"));
}

