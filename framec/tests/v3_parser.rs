use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::mir::MirItemV3;

#[test]
fn parse_transition_and_forward() {
    let src = b"{\n-> $Running(a, b[1], { 'x': 1 })\n=> $^\n}\n";
    let mut scanner = NativeRegionScannerPyV3;
    let scan = scanner.scan(src, 0).unwrap();
    let assembler = MirAssemblerV3;
    let mir = assembler.assemble(src, &scan.regions).unwrap();
    assert!(matches!(mir[0], MirItemV3::Transition{ ref target, .. } if target == "Running"));
    if let MirItemV3::Transition{ args, .. } = &mir[0] {
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "a");
    }
    assert!(matches!(mir[1], MirItemV3::Forward{..}));
}

#[test]
fn parse_stack_ops() {
    let src = b"{\n$$[+]\n$$[-]\n}\n";
    let mut scanner = NativeRegionScannerPyV3;
    let scan = scanner.scan(src, 0).unwrap();
    let assembler = MirAssemblerV3;
    let mir = assembler.assemble(src, &scan.regions).unwrap();
    assert!(matches!(mir[0], MirItemV3::StackPush{..}));
    assert!(matches!(mir[1], MirItemV3::StackPop{..}));
}

