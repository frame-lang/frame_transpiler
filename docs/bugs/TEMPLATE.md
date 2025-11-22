# Bug #NNN: [Title]

## Metadata
```yaml
bug_number: NNN
title: "Clear, descriptive title"
status: Open  # Allowed: Open|Fixed|Closed|Reopen|Won't Fix|Duplicate
priority: High|Medium|Low|Critical
category: Parser|Semantic|CodeGen|Runtime|Tooling|Documentation
discovered_version: vX.Y.Z
fixed_version: 
reporter: Name or System
assignee: 
created_date: YYYY-MM-DD
resolved_date: 
```

## Description
[Clear, concise description of the bug. What is broken? What should happen instead?]

## Reproduction Steps
1. [First step]
2. [Second step]
3. [Continue as needed]
4. [Observe the error]

## Build/Release Artifacts
- framec binary: `./target/release/framec` (or full path used for this bug)
- Generated artifacts: `[path/to/generated.ts or .py, adapter harness scripts, etc.]`

## Test Case
```frame
// Minimal Frame code that reproduces the issue
system TestCase {
    // ...
}
```

## Expected Behavior
[What should happen when the above code is run/compiled]

## Actual Behavior
[What actually happens, including error messages]

```
// Paste actual error output here
```

## Impact
- **Severity**: [How badly does this affect users?]
- **Scope**: [How many features/tests are affected?]
- **Workaround**: [Is there a way around this issue?]

## Technical Analysis
[Deep dive into why this bug occurs - may be filled in during investigation]

### Root Cause
[Once identified, document the root cause]

### Affected Files
- `path/to/file.rs` - [Brief description]
- `path/to/other.rs` - [Brief description]

## Proposed Solution
[How should this be fixed? Multiple options can be listed]

### Option 1: [Name]
[Description of approach]
- Pros: [Benefits]
- Cons: [Drawbacks]

### Option 2: [Name]
[Description of approach]
- Pros: [Benefits]
- Cons: [Drawbacks]

## Test Coverage
- [ ] Unit test added
- [ ] Integration test added
- [ ] Regression test added
- [ ] Manual testing completed

## Related Issues
- Bug #XXX - [Related bug if any]
- Bug #YYY - [Another related bug]

## Work Log
- YYYY-MM-DD: [Initial report] - Reporter
- YYYY-MM-DD: [Investigation started] - Developer
- YYYY-MM-DD: [Root cause identified] - Developer
- YYYY-MM-DD: [Marked Fixed in vX.Y.Z] - Developer
- YYYY-MM-DD: [Marked Closed after verification] - Reporter/Owner

## Resolution
[Once Fixed or Closed, document what was done to fix it and how verification was performed]

### Fix Summary
[Brief description of the fix]

### Verification
[How the fix was verified — tests, commands, artifacts]

### Lessons Learned
[Any insights gained from this bug]

---
*Bug tracking policy version: 1.0*
