# Stage 4 Cleanup Manifest

## What's Being Removed
- `framec_tests/` directory (295MB, 1595 test files)
- Python test runner and related scripts
- All embedded test infrastructure

## What's Preserved
- Tests have been migrated to: `framepiler_test_env/framepiler/`
- 558 PRT priority tests are in the shared environment
- Docker-based test execution is operational

## Backup Created
- Date: $(date)
- Location: framec_tests_backup_$(date +%Y%m%d).tar.gz
EOF < /dev/null