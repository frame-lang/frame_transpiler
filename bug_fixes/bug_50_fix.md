# Bug #50 Fix: Language-Specific Tests Filtering

## Problem
Language-specific tests were being included when running category-specific tests (e.g., "core"), causing Python-specific tests to appear in all test runs regardless of the target language or category selected.

## Root Cause
The `discover_tests()` method in `frame_test_runner.py` was always discovering language-specific tests for all configured languages, regardless of the category filter.

## Solution
Modified the test discovery logic to:
1. Only include language-specific tests when `--categories all` is used
2. Allow explicit selection with `--categories language_specific_python`
3. Exclude language-specific tests from common category runs

## Code Changes
File: `framec_tests/runner/frame_test_runner.py`

Changed the language-specific test discovery from always running to conditional:
- When `"all"` in categories: Include language-specific tests
- Otherwise: Only include if explicitly requested in categories

## Testing & Verification

### Before Fix
```bash
$ python3 framec_tests/runner/frame_test_runner.py --languages python --categories core
Discovered 60 tests in 2 categories  # Wrong: included 29 Python-specific tests
```

### After Fix
```bash
$ python3 framec_tests/runner/frame_test_runner.py --languages python --categories core
Discovered 31 tests in 1 categories  # Correct: only core tests

$ python3 framec_tests/runner/frame_test_runner.py --categories language_specific_python
Discovered 29 tests in 1 categories  # Still works when explicitly requested

$ python3 framec_tests/runner/frame_test_runner.py --categories all
Discovered 446 tests in 9 categories  # Includes everything with "all"
```

## Impact
- Test runs are now more accurate and predictable
- TypeScript no longer attempts to run Python-specific tests
- Category filtering works as expected
- No breaking changes - all existing functionality preserved

## Status
✅ Fixed and verified on 2025-10-13