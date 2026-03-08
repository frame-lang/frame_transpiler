#!/bin/bash
# Frame Transpiler Version Sync Script
# Ensures version.toml mirrors the workspace package version declared in Cargo.toml
# Usage: ./scripts/sync-versions.sh [--dry-run]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

DRY_RUN=false
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=true
    echo -e "${YELLOW}🔍 DRY RUN MODE - No files will be modified${NC}"
fi

echo -e "${BLUE}🔧 Frame Transpiler Version Sync${NC}"

if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${RED}❌ Error: cargo is required but not installed${NC}"
    exit 1
fi

# Determine workspace version from cargo metadata
VERSION=$(python3 - <<'PY'
import json
import subprocess
import sys

try:
    result = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        capture_output=True,
        text=True,
        check=True,
    )
except subprocess.CalledProcessError as exc:
    sys.exit(1)

json_lines = [
    line.strip()
    for line in result.stdout.splitlines()
    if line.strip().startswith("{") or line.strip().startswith("[")
]

if not json_lines:
    sys.exit(1)

metadata = json.loads("".join(json_lines))
packages = metadata.get("packages", [])
target_pkg = None

for pkg in packages:
    if pkg.get("name") == "framec":
        target_pkg = pkg
        break

if target_pkg is None and packages:
    target_pkg = packages[0]

if target_pkg is None:
    sys.exit(1)

print(target_pkg["version"])
PY
) || VERSION=""

if [ -z "$VERSION" ]; then
    echo -e "${RED}❌ Error: Could not determine version from cargo metadata${NC}"
    exit 1
fi

echo -e "${YELLOW}📋 Workspace version: $VERSION${NC}"

if [ ! -f "version.toml" ]; then
    echo -e "${RED}❌ Error: version.toml not found. Run from project root.${NC}"
    exit 1
fi

# Split semantic version components
IFS='.' read -r VERSION_MAJOR VERSION_MINOR VERSION_PATCH <<< "$VERSION"

if [ -z "$VERSION_MAJOR" ] || [ -z "$VERSION_MINOR" ] || [ -z "$VERSION_PATCH" ]; then
    echo -e "${RED}❌ Error: Version '$VERSION' is not in semantic x.y.z format${NC}"
    exit 1
fi

echo -e "${BLUE}🔄 Updating version.toml to mirror workspace version...${NC}"

apply_replacement() {
    local pattern="$1"
    local replacement="$2"

    if ! grep -q "$pattern" version.toml; then
        echo -e "   ${RED}⚠ Pattern not found:${NC} $pattern"
        return
    fi

    if [ "$DRY_RUN" = false ]; then
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s|$pattern|$replacement|" version.toml
        else
            sed -i "s|$pattern|$replacement|" version.toml
        fi
    fi
    echo -e "   ${GREEN}✔${NC} $replacement"
}

apply_replacement '^major = .*' "major = $VERSION_MAJOR"
apply_replacement '^minor = .*' "minor = $VERSION_MINOR"
apply_replacement '^patch = .*' "patch = $VERSION_PATCH"
apply_replacement '^full = ".*"' "full = \"$VERSION\""

echo
if [ "$DRY_RUN" = false ]; then
    echo -e "${GREEN}✅ Sync complete. version.toml now reflects Cargo workspace version.${NC}"
else
    echo -e "${YELLOW}🔍 Dry run complete - no changes made.${NC}"
fi
