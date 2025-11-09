use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::typescript::NativeRegionScannerTsV3;
use framec::frame_c::v3::native_region_scanner::csharp::NativeRegionScannerCsV3;
use framec::frame_c::v3::native_region_scanner::c::NativeRegionScannerCV3;
use framec::frame_c::v3::native_region_scanner::cpp::NativeRegionScannerCppV3;
use framec::frame_c::v3::native_region_scanner::java::NativeRegionScannerJavaV3;
use framec::frame_c::v3::native_region_scanner::rust::NativeRegionScannerRustV3;
use framec::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;
use framec::frame_c::v3::mir::MirItemV3;

fn assemble(bytes: &[u8], open: usize, scanner: &mut dyn NativeRegionScannerV3) -> Vec<MirItemV3> {
    let scan = scanner.scan(bytes, open).unwrap();
    let asm = MirAssemblerV3;
    asm.assemble(bytes, &scan.regions).unwrap()
}

#[test]
fn mir_python_transition_forward() { let v=assemble(b"{\n-> $S(1)\n=> $^\n}\n",0,&mut NativeRegionScannerPyV3); assert!(matches!(v[0], MirItemV3::Transition{..})); assert!(matches!(v[1], MirItemV3::Forward{..})); }

#[test]
fn mir_typescript_stack_ops() { let v=assemble(b"{\n$$[+]\n$$[-]\n}\n",0,&mut NativeRegionScannerTsV3); assert!(matches!(v[0], MirItemV3::StackPush{..})); assert!(matches!(v[1], MirItemV3::StackPop{..})); }

#[test]
fn mir_csharp_transition() { let v=assemble(b"{\n-> $Next\n}\n",0,&mut NativeRegionScannerCsV3); assert!(matches!(v[0], MirItemV3::Transition{..})); }

#[test]
fn mir_c_transition() { let v=assemble(b"{\n-> $Go(aa, 3)\n}\n",0,&mut NativeRegionScannerCV3); if let MirItemV3::Transition{ ref args, .. } = v[0] { assert_eq!(args.len(), 2); } }

#[test]
fn mir_cpp_forward() { let v=assemble(b"{\n=> $^\n}\n",0,&mut NativeRegionScannerCppV3); assert!(matches!(v[0], MirItemV3::Forward{..})); }

#[test]
fn mir_java_transition() { let v=assemble(b"{\n-> $S\n}\n",0,&mut NativeRegionScannerJavaV3); assert!(matches!(v[0], MirItemV3::Transition{..})); }

#[test]
fn mir_rust_stack_push() { let v=assemble(b"{\n$$[+]\n}\n",0,&mut NativeRegionScannerRustV3); assert!(matches!(v[0], MirItemV3::StackPush{..})); }

