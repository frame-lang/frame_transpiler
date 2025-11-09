use framec::frame_c::v3::CompilerV3;
use framec::frame_c::compiler::TargetLanguage;

#[test]
fn demo_python_body() {
    let src = "{\nprint(1)\n-> $Next(2)\n}\n";
    let out = CompilerV3::compile_single_file(None, src, Some(TargetLanguage::Python3), false).unwrap();
    assert!(out.contains("# frame:transition Next(2)"));
}

#[test]
fn demo_typescript_body() {
    let src = "{\nconsole.log(1);\n=> $^\n}\n";
    let out = CompilerV3::compile_single_file(None, src, Some(TargetLanguage::TypeScript), false).unwrap();
    assert!(out.contains("// frame:forward"));
}

#[test]
fn demo_c_body() {
    let src = "{\n// c\n-> $Go(1)\n}\n";
    let out = CompilerV3::compile_single_file(None, src, Some(TargetLanguage::C), false).unwrap();
    assert!(out.contains("// frame:transition Go(1)"));
}
