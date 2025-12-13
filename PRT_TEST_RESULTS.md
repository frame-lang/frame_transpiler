# PRT Language Test Results
## Date: 2025-12-13

## Overall Success Rate: 95% (39/41 tests passing)

### Python: 100% Success ✅
- **14/14 tests passing**
- Categories: data_types (2), imports (6), operators (2), persistence (1), scoping (3)
- No failures

### TypeScript: 100% Success ✅
- **11/11 tests passing**  
- Categories: data_types (3), operators (2), scoping (2), systems (4)
- No failures

### Rust: 87.5% Success ⚠️
- **14/16 tests passing**
- Categories passing: data_types (5/5), operators (5/5), persistence (2/2), systems (2/2)
- **2 Known Issues:**

#### 1. Async Functions Test Failure
- **File:** `async_functions.frm`
- **Issue:** Docker container needs Rust 2021 edition flag (fixed) but also requires tokio runtime
- **Error:** `maybe a missing crate tokio?`
- **Fix needed:** Either:
  - Add tokio to Docker container
  - OR modify Rust transpiler to generate self-contained async code for tests

#### 2. Scoping Test Failure  
- **File:** `function_block_scope.frm`
- **Issue:** Nested helper functions not being generated in Rust output
- **Error:** `cannot find function helper in this scope`
- **Fix needed:** Rust transpiler needs to generate nested function definitions

## Test Infrastructure Status
- ✅ Docker runner working correctly
- ✅ All 607 tests consolidated in `common/test-frames/v3/`
- ✅ Rust 2021 edition flag added to Docker runner
- ⚠️ Rust async tests need tokio dependency in container
- ⚠️ Rust nested functions need transpiler fix

## Recommendations
1. Add tokio as a dev dependency to Rust Docker container
2. Fix Rust transpiler to properly generate nested functions
3. Consider making async tests self-contained (no external deps)

## Running Tests
```bash
export FRAMEPILER_TEST_ENV=/path/to/framepiler_test_env

# Run all PRT tests
for lang in python_3 typescript rust; do
  for category in data_types operators scoping systems async persistence imports; do
    framepiler_test_env/framepiler/docker/target/release/frame-docker-runner \
      $lang v3_$category --framec ./target/release/framec
  done
done
```