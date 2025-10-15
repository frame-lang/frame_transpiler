# Bug #050: Language-Specific Tests Running for All Languages

## Metadata
```yaml
bug_number: 050
title: "Language-Specific Tests Running for All Languages"
status: Open
priority: Low
category: Tooling
discovered_version: v0.82.2
fixed_version: 
reporter: Test Suite Analysis
assignee: 
created_date: 2025-10-13
resolved_date: 
```

## Description
The test runner attempts to run language-specific tests (e.g., Python async tests) for all target languages, causing false failures when testing TypeScript or other targets.

## Reproduction Steps
1. Run test suite with TypeScript target
2. Observe Python-specific tests like `test_async_*.frm` being run
3. Tests fail because TypeScript doesn't support async/await Frame syntax

## Test Case
```bash
python3 framec_tests/runner/frame_test_runner.py --lang typescript
# Attempts to transpile test_async_basic.frm which is Python-specific
```

## Expected Behavior
Language-specific tests should only run for their intended target language.

## Actual Behavior
All tests run for all languages, causing false failures.

## Impact
- **Severity**: Low - Doesn't affect functionality, just test reporting
- **Scope**: Test runner accuracy
- **Workaround**: Manually filter test results

## Technical Analysis
The test runner needs to:
1. Identify language-specific test directories
2. Skip tests not applicable to current target language
3. Report skipped tests separately from failures

### Root Cause
Test runner doesn't implement language filtering logic.

### Affected Files
- `framec_tests/runner/frame_test_runner.py`
- Test organization structure

## Proposed Solution

### Option 1: Directory-Based Filtering
Use `language_specific/` directory structure:
- `language_specific/python/` - Python-only tests
- `language_specific/typescript/` - TypeScript-only tests
- `common/` - Cross-language tests

### Option 2: Metadata in Test Files
Add metadata comments to test files:
```frame
# @targets: python, rust
# @skip: typescript, cpp
```

### Option 3: Configuration File
Create test configuration:
```json
{
  "tests": {
    "test_async_*.frm": ["python"],
    "test_import_*.frm": ["python"],
    "test_*.frm": ["all"]
  }
}
```

## Test Coverage
- [ ] Implement language filtering
- [ ] Test with each target language
- [ ] Verify correct test counts
- [ ] Update documentation

## Related Issues
None

## Work Log
- 2025-10-13: Bug documented

---
*Bug tracking policy version: 1.0*