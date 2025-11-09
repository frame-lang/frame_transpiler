use framec::frame_c::v3::validate_single_body;
use framec::frame_c::compiler::TargetLanguage;
use framec::frame_c::v3::validator::{ValidatorV3, ValidatorPolicyV3, BodyKindV3};
use framec::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
use framec::frame_c::v3::native_region_scanner::NativeRegionScannerV3;
use framec::frame_c::v3::mir_assembler::MirAssemblerV3;

#[test]
fn validator_terminal_last_fails_when_followed() {
    let src = "{\n-> $A\n=> $^\n}\n";
    let res = validate_single_body(src, Some(TargetLanguage::Python3)).unwrap();
    assert!(!res.ok);
}

#[test]
fn validator_ok_single_terminal() {
    let src = "{\n-> $A\n}\n";
    let res = validate_single_body(src, Some(TargetLanguage::Python3)).unwrap();
    assert!(res.ok);
}

#[test]
fn validator_disallows_frame_in_actions_ops_when_policy_set() {
    // Scan a body and validate with Action policy
    let src = b"{\n-> $Next\n}\n";
    let scan = NativeRegionScannerPyV3.scan(src, 0).unwrap();
    let mir = MirAssemblerV3.assemble(src, &scan.regions).unwrap();
    let res = ValidatorV3.validate_regions_mir_with_policy(&scan.regions, &mir, ValidatorPolicyV3{ body_kind: Some(BodyKindV3::Action) });
    assert!(!res.ok);
}
