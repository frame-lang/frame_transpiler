use framec::frame_c::v3::multifile_demo::compile_multiple_bodies_demo;
use framec::frame_c::compiler::TargetLanguage;

#[test]
fn multifile_demo_typescript_two_files() {
    let files = vec![
        ("A.ts", "{\nconsole.log(1);\n=> $^\n}\n"),
        ("B.ts", "{\n-> $Go(2)\n}\n"),
    ];
    let outs = compile_multiple_bodies_demo(files, TargetLanguage::TypeScript).unwrap();
    assert_eq!(outs.len(), 2);
    assert!(outs[0].1.contains("// frame:forward"));
    assert!(outs[1].1.contains("// frame:transition Go(2)"));
}

