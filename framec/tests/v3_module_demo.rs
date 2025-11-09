use framec::frame_c::v3::compile_module_demo;
use framec::frame_c::compiler::TargetLanguage;

#[test]
fn module_demo_preserves_text_and_transforms_bodies_python() {
    let src = "@target python_3\nheader\n{\n-> $Next\n}\nfooter\n";
    let out = compile_module_demo(src, TargetLanguage::Python3).unwrap();
    assert!(out.contains("header"));
    assert!(out.contains("# frame:transition Next"));
    assert!(out.contains("footer"));
}

#[test]
fn module_demo_handles_two_bodies_typescript() {
    let src = "@target typescript\npre\n{\n=> $^\n}\nmid\n{\n-> $Go(1)\n}\npost\n";
    let out = compile_module_demo(src, TargetLanguage::TypeScript).unwrap();
    assert!(out.contains("// frame:forward"));
    assert!(out.contains("// frame:transition Go(1)"));
}
