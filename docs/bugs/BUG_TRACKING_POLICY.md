# Frame Transpiler Bug Tracking Policy

## Overview
This document defines the bug tracking process for the Frame Transpiler project. All bugs are tracked as individual markdown files in the `/docs/bugs/` directory structure.

## Directory Structure
```
docs/bugs/
├── BUG_TRACKING_POLICY.md    # This file
├── TEMPLATE.md                # Bug report template
├── INDEX.md                   # Master index of all bugs
├── open/                      # Active bugs
│   └── bug_NNN_short_title.md
└── closed/                    # Resolved bugs
    └── bug_NNN_short_title.md
```

## Bug Numbering Scheme
- Bugs are numbered sequentially starting from #001
- The next available bug number is tracked in INDEX.md
- Never reuse bug numbers
- Format: `bug_NNN_short_title.md` where:
  - NNN is the zero-padded bug number (e.g., 001, 048, 123)
  - short_title is a kebab-case summary (5-7 words max)

## File Naming Examples
- `bug_048_unreachable_return_after_transition.md`
- `bug_049_typescript_transpilation_rate_low.md`
- `bug_050_test_runner_language_filtering.md`

## Bug Report Structure
Every bug report must follow the template in TEMPLATE.md and include:
1. **Metadata Header** (YAML front matter)
2. **Description** (clear problem statement)
3. **Reproduction Steps**
4. **Expected vs Actual Behavior**
5. **Impact Assessment**
6. **Technical Analysis** (if available)
7. **Proposed Solution**
8. **Test Cases**

## Workflow

### 1. Submitting a New Bug
1. Check INDEX.md for the next available bug number
2. Copy TEMPLATE.md to `open/bug_NNN_short_title.md`
3. Fill out all sections of the template
4. Update INDEX.md with the new bug entry
5. Commit with message: `bug: Add Bug #NNN - [Short Description]`

### 2. Working on a Bug
1. Update bug status to "In Progress" in the metadata
2. Add your name as assignee
3. Document investigation findings in the Technical Analysis section
4. Update work log at bottom of file

### 3. Resolving a Bug
1. Move file from `open/` to `closed/`
2. Update status to "Resolved" or "Won't Fix"
3. Add resolution details and version fixed
4. Update INDEX.md to reflect closed status
5. Commit with message: `fix(vX.Y.Z): Resolve Bug #NNN - [Short Description]`

### 4. Reopening a Bug
1. Move file from `closed/` back to `open/`
2. Update status to "Reopened"
3. Add reopening reason in work log
4. Update INDEX.md accordingly

## Bug Priorities
- **Critical**: System crash, data loss, security issue
- **High**: Major functionality broken, blocks development
- **Medium**: Feature partially broken, has workaround
- **Low**: Minor issue, cosmetic, enhancement

## Bug Categories
- **Parser**: Syntax analysis, AST generation
- **Semantic**: Type checking, symbol resolution
- **CodeGen**: Target language generation
- **Runtime**: Frame runtime behavior
- **Tooling**: CLI, test runner, build system
- **Documentation**: Docs, examples, tutorials

## Status Values
- **Open**: Bug confirmed, awaiting fix
- **In Progress**: Actively being worked on
- **Resolved**: Fixed and verified
- **Won't Fix**: Intentional behavior or out of scope
- **Duplicate**: Duplicate of another bug
- **Reopened**: Previously fixed but recurred

## Version Tracking
- Always note the version where bug was discovered
- Always note the version where bug was fixed
- Use semantic versioning: vMAJOR.MINOR.PATCH

## Git Integration
- Reference bug numbers in commits: `Bug #048`
- Use conventional commits:
  - `bug: Add Bug #048 - Unreachable return statements`
  - `fix(v0.82.3): Resolve Bug #048 - Unreachable return statements`
  - `test: Add tests for Bug #048`

## Search and Discovery
- Use grep to search bug content: `grep -r "pattern" docs/bugs/`
- Check INDEX.md for quick bug lookup
- Use git history to track bug lifecycle

## Migration from Legacy Systems
When migrating bugs from old tracking systems:
1. Preserve original bug numbers if possible
2. Add migration note in bug file
3. Mark source system in metadata

## Review Process
1. All bug reports should be reviewed for completeness
2. Verify reproduction steps work
3. Ensure proper categorization and priority
4. Check for duplicates before creating new bug

## Maintenance
- Quarterly review of old open bugs
- Archive bugs older than 1 year to `docs/bugs/archive/`
- Update INDEX.md regularly
- Keep statistics in INDEX.md header

## Contact
For questions about bug tracking:
- Check this policy document first
- Consult INDEX.md for examples
- Ask in project discussions

---
*Policy Version: 1.0*  
*Last Updated: 2025-10-13*  
*Next Review: 2025-01-13*