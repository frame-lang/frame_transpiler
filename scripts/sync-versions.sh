#!/bin/bash
# Frame Transpiler Version Sync Script
# Manually sync versions from version.toml to all relevant files
# Usage: ./scripts/sync-versions.sh [--dry-run]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

DRY_RUN=false
if [ "$1" = "--dry-run" ]; then
    DRY_RUN=true
    echo -e "${YELLOW}🔍 DRY RUN MODE - No files will be modified${NC}"
fi

echo -e "${BLUE}🔧 Frame Transpiler Version Sync${NC}"

# Check if we're in the right directory
if [ ! -f "version.toml" ]; then
    echo -e "${RED}❌ Error: version.toml not found. Run from project root.${NC}"
    exit 1
fi

# Extract version from version.toml
VERSION=$(grep '^full = ' version.toml | sed 's/.*"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
    echo -e "${RED}❌ Error: Could not extract version from version.toml${NC}"
    exit 1
fi

echo -e "${YELLOW}📋 Target version: $VERSION${NC}"

# Function to update file version
update_file_version() {
    local file="$1"
    local pattern="$2"
    local replacement="$3"
    local description="$4"
    
    if [ ! -f "$file" ]; then
        echo -e "${RED}   ❌ Warning: $file not found${NC}"
        return
    fi
    
    local current_line=$(grep "$pattern" "$file" || echo "")
    if [ -z "$current_line" ]; then
        echo -e "${RED}   ❌ Warning: Pattern not found in $file${NC}"
        return
    fi
    
    echo -e "${BLUE}   📄 $description${NC}"
    echo -e "      File: $file"
    echo -e "      Current: $current_line"
    echo -e "      New:     $replacement"
    
    if [ "$DRY_RUN" = false ]; then
        # Create backup
        cp "$file" "$file.sync-backup"
        
        # Apply the change
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS
            sed -i '' "s|$pattern|$replacement|g" "$file"
        else
            # Linux
            sed -i "s|$pattern|$replacement|g" "$file"
        fi
        
        # Verify the change
        if grep -q "$replacement" "$file"; then
            echo -e "${GREEN}      ✅ Updated successfully${NC}"
            rm "$file.sync-backup"
        else
            echo -e "${RED}      ❌ Update failed, restoring backup${NC}"
            mv "$file.sync-backup" "$file"
        fi
    else
        echo -e "${YELLOW}      🔍 Would update (dry run)${NC}"
    fi
    echo
}

echo -e "${BLUE}🔄 Syncing all version references...${NC}"
echo

# Update Cargo.toml files
update_file_version "framec/Cargo.toml" \
    '^version = .*' \
    "version = \"$VERSION\"" \
    "framec package version"

update_file_version "frame_build/Cargo.toml" \
    '^version = .*' \
    "version = \"$VERSION\"" \
    "frame_build package version"

# Update compiler version string
update_file_version "framec/src/frame_c/compiler.rs" \
    'framec_v[0-9.]*' \
    "framec_v$VERSION" \
    "compiler version string"

# Show summary
echo -e "${BLUE}📊 Summary:${NC}"
if [ -f "framec/Cargo.toml" ]; then
    FRAMEC_VER=$(grep '^version = ' framec/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    echo -e "   framec/Cargo.toml: $FRAMEC_VER"
fi

if [ -f "frame_build/Cargo.toml" ]; then
    FRAME_BUILD_VER=$(grep '^version = ' frame_build/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    echo -e "   frame_build/Cargo.toml: $FRAME_BUILD_VER"
fi

if [ -f "framec/src/frame_c/compiler.rs" ]; then
    COMPILER_VER=$(grep 'framec_v' framec/src/frame_c/compiler.rs | sed 's/.*framec_v\([0-9.]*\).*/\1/' | head -1)
    echo -e "   compiler version string: framec_v$COMPILER_VER"
fi

echo
if [ "$DRY_RUN" = false ]; then
    echo -e "${GREEN}✅ Version sync complete!${NC}"
    echo -e "${YELLOW}💡 Next steps:${NC}"
    echo -e "   1. Run 'cargo build --release' to update Cargo.lock"
    echo -e "   2. Test with 'framec --version'"
    echo -e "   3. Commit changes with git"
else
    echo -e "${YELLOW}🔍 Dry run complete - no changes made${NC}"
    echo -e "${BLUE}💡 Run without --dry-run to apply changes${NC}"
fi