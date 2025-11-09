use framec::frame_c::v3::module_partitioner::ModulePartitionerV3;
use framec::frame_c::v3::validator::BodyKindV3;
use framec::frame_c::compiler::TargetLanguage;

#[test]
fn detects_prolog_and_body_kinds() {
    let src = b"@target python_3\n\nhandler on_event {\n-> $A\n}\n\naction do_stuff {\n-> $B\n}\n";
    let parts = ModulePartitionerV3::partition(src, TargetLanguage::Python3).unwrap();
    assert!(parts.prolog.is_some());
    assert_eq!(parts.bodies.len(), 2);
    // First body should be Handler, second Action (heuristic)
    assert!(matches!(parts.bodies[0].kind, BodyKindV3::Handler | BodyKindV3::Unknown));
    assert!(matches!(parts.bodies[1].kind, BodyKindV3::Action | BodyKindV3::Unknown));
    // Owner ids present (best-effort token after keyword)
    assert!(parts.bodies[0].owner_id.is_some());
    assert!(parts.bodies[1].owner_id.is_some());
}
