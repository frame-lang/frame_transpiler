use crate::frame_c::v3::mir::MirItemV3;
use crate::frame_c::v3::native_region_scanner::RegionV3;

#[derive(Debug, Clone)]
pub struct ValidationIssueV3 { pub message: String }

#[derive(Debug, Clone)]
pub struct ValidationResultV3 { pub ok: bool, pub issues: Vec<ValidationIssueV3> }

pub struct ValidatorV3;

impl ValidatorV3 {
    // Minimal structural rule: once a terminal MIR is seen, no further MIR items may follow.
    pub fn validate_regions_mir(&self, regions: &[RegionV3], mir: &[MirItemV3]) -> ValidationResultV3 {
        let mut issues: Vec<ValidationIssueV3> = Vec::new();
        // terminal kinds: Transition, Forward, StackPush, StackPop
        let is_terminal = |m: &MirItemV3| match m { MirItemV3::Transition{..} | MirItemV3::Forward{..} | MirItemV3::StackPush{..} | MirItemV3::StackPop{..} => true };
        // ensure no MIR after terminal
        if let Some((idx, _)) = mir.iter().enumerate().find(|(_, m)| is_terminal(m)) {
            if idx + 1 < mir.len() {
                issues.push(ValidationIssueV3 { message: "Terminal Frame statement must be last MIR item".to_string() });
            }
        }
        ValidationResultV3 { ok: issues.is_empty(), issues }
    }

    // Expanded API with body-kind policy (not yet wired with full module context).
    pub fn validate_regions_mir_with_policy(&self, regions: &[RegionV3], mir: &[MirItemV3], policy: ValidatorPolicyV3) -> ValidationResultV3 {
        let mut res = self.validate_regions_mir(regions, mir);
        if let Some(kind) = policy.body_kind {
            match kind {
                BodyKindV3::Action | BodyKindV3::Operation => {
                    if !mir.is_empty() {
                        // Frame statements are disallowed in actions/ops. Only advisory for now.
                        res.issues.push(ValidationIssueV3 { message: "Frame statements are not allowed in actions/operations".to_string() });
                    }
                }
                BodyKindV3::Handler | BodyKindV3::Unknown => {}
            }
        }
        res.ok = res.issues.is_empty();
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKindV3 { Handler, Action, Operation, Unknown }

#[derive(Debug, Clone, Default)]
pub struct ValidatorPolicyV3 { pub body_kind: Option<BodyKindV3> }
