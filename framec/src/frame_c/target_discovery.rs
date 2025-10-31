use crate::frame_c::scanner::TargetRegion;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct TargetDiscoveryPass {
    target_regions: Arc<Vec<TargetRegion>>,
}

impl TargetDiscoveryPass {
    pub(crate) fn new(target_regions: Arc<Vec<TargetRegion>>) -> Self {
        TargetDiscoveryPass { target_regions }
    }

    pub(crate) fn target_regions_in_range(
        &self,
        start_line: usize,
        end_line: usize,
    ) -> Vec<(usize, &TargetRegion)> {
        self.target_regions
            .iter()
            .enumerate()
            .filter_map(|(idx, region)| {
                let region_start = region.source_map.frame_start_line;
                let region_end = frame_end_line(region);
                let overlaps = region_start <= end_line && region_end >= start_line;
                if overlaps {
                    Some((idx, region))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn region_by_index(&self, index: usize) -> Option<&TargetRegion> {
        self.target_regions.get(index)
    }
}

fn frame_end_line(region: &TargetRegion) -> usize {
    let start = region.source_map.frame_start_line;
    let line_count = region.raw_content.lines().count();
    if line_count == 0 {
        start
    } else {
        start + line_count - 1
    }
}
