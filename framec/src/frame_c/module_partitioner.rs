/// ModulePartitioner: partitions a ModuleUnit into Blocks and Regions.
/// Blocks are brace-delimited scopes; Regions are arbitrary contiguous spans.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    System,
    Body, // operation/action/handler bodies
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    Prolog,
    Import,
    SectionOperations,
    SectionInterface,
    SectionMachine,
    SectionActions,
    SectionDomain,
    Native, // NativeRegion inside a Body block
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub kind: BlockKind,
    pub header_line: usize,
    pub body_start_line: usize,
    pub body_end_line: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RegionInfo {
    pub kind: RegionKind,
    pub start_line: usize,
    pub end_line: usize,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ModulePartitions {
    pub blocks: Vec<BlockInfo>,
    pub regions: Vec<RegionInfo>,
}

#[allow(dead_code)]
pub struct ModulePartitioner<'a> {
    src: &'a str,
}

impl<'a> ModulePartitioner<'a> {
    #[allow(dead_code)]
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Minimal Phase I: detect a single system block and section headers.
    #[allow(dead_code)]
    pub fn partition(&mut self) -> ModulePartitions {
        let mut parts = ModulePartitions::default();
        let mut depth: i32 = 0;
        let mut in_system = false;
        let mut body_start: Option<usize> = None;

        for (idx, raw_line) in self.src.lines().enumerate() {
            let line_no = idx + 1;
            let line = raw_line.trim();

            // Detect the start of a system block before mutating depth so we don't
            // miss a header that includes an opening brace on the same line.
            if depth == 0 && line.starts_with("system ") {
                in_system = true;
                parts.blocks.push(BlockInfo {
                    kind: BlockKind::System,
                    header_line: line_no,
                    body_start_line: 0,
                    body_end_line: 0,
                });
            }

            // Track brace depth and the beginning of the system body.
            if line.contains('{') {
                let open_count = line.matches('{').count() as i32;
                // If we are at depth 0 and see an opening brace for a system header,
                // the body starts on the next line.
                if in_system && body_start.is_none() && depth == 0 && open_count > 0 {
                    body_start = Some(line_no + 1);
                }
                depth += open_count;
            }

            if in_system && depth == 1 {
                let region_kind = if line.starts_with("operations:") {
                    Some(RegionKind::SectionOperations)
                } else if line.starts_with("interface:") {
                    Some(RegionKind::SectionInterface)
                } else if line.starts_with("machine:") {
                    Some(RegionKind::SectionMachine)
                } else if line.starts_with("actions:") {
                    Some(RegionKind::SectionActions)
                } else if line.starts_with("domain:") {
                    Some(RegionKind::SectionDomain)
                } else {
                    None
                };
                if let Some(kind) = region_kind {
                    parts.regions.push(RegionInfo {
                        kind,
                        start_line: line_no,
                        end_line: line_no,
                    });
                }
            }

            if line.contains('}') {
                depth -= line.matches('}').count() as i32;
                if in_system && depth == 0 {
                    let end_line = line_no - 1;
                    if let Some(last) = parts
                        .blocks
                        .iter_mut()
                        .rev()
                        .find(|b| b.kind == BlockKind::System)
                    {
                        last.body_start_line = body_start.unwrap_or(line_no);
                        last.body_end_line = end_line;
                    }
                    in_system = false;
                    body_start = None;
                }
            }
        }

        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partitions_system_and_sections() {
        let src = r#"@target typescript
system Empty {
    operations:
    interface:
    machine:
    actions:
    domain:
}
"#;
        let mut p = ModulePartitioner::new(src);
        let parts = p.partition();
        assert_eq!(parts.blocks.len(), 1);
        assert_eq!(parts.regions.len(), 5);
    }
}
