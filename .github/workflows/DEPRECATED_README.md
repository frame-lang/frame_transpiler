# Deprecated CI/CD Workflows

## Status: DEPRECATED as of 2024-12-12

The following workflows are deprecated and will be removed after the shared environment is fully validated:

- `v3_all.yml` - Replaced by `v3_shared_env.yml`
- `v3_exec_smoke.yml` - Tests now run in shared environment
- `v3_curated_exec.yml` - Tests now run in shared environment  
- `test_frame.yml` - Legacy Python runner, no longer used
- `test.yml` - Legacy test workflow

## Migration

All test execution has been moved to the shared test environment at:
https://github.com/frame-lang/framepiler_test_env

The new workflow `v3_shared_env.yml` handles all V3 testing by:
1. Checking out both transpiler and test environment repos
2. Building framec from the transpiler
3. Running tests from the shared environment with Docker isolation

## Removal Timeline

These deprecated workflows will be removed after:
- 2 weeks of successful runs with the new workflow
- All team members have confirmed the migration
- CI/CD documentation is updated