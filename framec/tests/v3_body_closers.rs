use framec::frame_c::v3::body_closer::c::BodyCloserCV3;
use framec::frame_c::v3::body_closer::cpp::BodyCloserCppV3;
use framec::frame_c::v3::body_closer::csharp::BodyCloserCsV3;
use framec::frame_c::v3::body_closer::java::BodyCloserJavaV3;
use framec::frame_c::v3::body_closer::python::BodyCloserPyV3;
use framec::frame_c::v3::body_closer::rust::BodyCloserRustV3;
use framec::frame_c::v3::body_closer::typescript::BodyCloserTsV3;
use framec::frame_c::v3::body_closer::{BodyCloserV3, CloseErrorV3Kind};

#[test]
fn py_triple_quote_and_close() {
    let src = b"{\nx = \"\"\" a } b \"\"\"\n}\n";
    let mut c = BodyCloserPyV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn ts_template_nested_close() {
    let src = b"{\nconst s = `a ${ { y: '}' } } b`;\n}\n";
    let mut c = BodyCloserTsV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn ts_unterminated_template_errors() {
    let src = b"{\nconst s = `unterminated ${ 1 + 2`;\n";
    let mut c = BodyCloserTsV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedString));
}

#[test]
fn cs_verbatim_and_raw() {
    let src = b"{\nvar a = @\"}\"; var b = $\"\"\" x } y \"\"\";\n}\n";
    let mut c = BodyCloserCsV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn c_block_comment_and_char() {
    let src = b"{\n/* } */ char c = '}';\n}\n";
    let mut c = BodyCloserCV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn cpp_raw_string() {
    let src = b"{\nauto s = R\"TAG( } )TAG\";\n}\n";
    let mut c = BodyCloserCppV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn java_string() {
    let src = b"{\nString s=\"}\";\n}\n";
    let mut c = BodyCloserJavaV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn rust_nested_block_comment_and_raw_string() {
    let src = b"{\n/* /* nested */ */ let s = r#\" } \"#;\n}\n";
    let mut c = BodyCloserRustV3;
    let idx = c.close_byte(src, 0).unwrap();
    assert_eq!(idx, src.len() - 2);
}

#[test]
fn unmatched_braces_error() {
    let src = b"{ let x = 1;";
    let mut cp = BodyCloserPyV3;
    let err = cp.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnmatchedBraces));
}

#[test]
fn py_unterminated_triple_quote_errors() {
    let src = b"{\ns = \"\"\"unterminated\n";
    let mut c = BodyCloserPyV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedString));
}

#[test]
fn cs_unterminated_normal_string_errors() {
    let src = b"{\nvar s = \"oops;\n"; // closes neither string nor body
    let mut c = BodyCloserCsV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedString));
}

#[test]
fn cpp_unterminated_block_comment_errors() {
    let src = b"{\n/* never closes\n";
    let mut c = BodyCloserCppV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedComment));
}

#[test]
fn rust_unterminated_raw_string_errors() {
    let src = b"{\nlet s = r###\" never closes\n";
    let mut c = BodyCloserRustV3;
    let err = c.close_byte(src, 0).unwrap_err();
    // Implementation may classify as raw or generic unterminated string depending on path
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedRawString | CloseErrorV3Kind::UnterminatedString));
}

#[test]
fn java_unterminated_char_literal_errors() {
    let src = b"{\nchar c = '\\';\n";
    let mut c = BodyCloserJavaV3;
    let err = c.close_byte(src, 0).unwrap_err();
    assert!(matches!(err.kind, CloseErrorV3Kind::UnterminatedString));
}
