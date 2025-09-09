#!/usr/bin/env python3
"""
Migrate Frame test files from old comment syntax to v0.40 Python-style comments.
- Converts // comments to # comments
- Removes /* */ C-style comments
- Preserves {-- --} Frame documentation comments
"""

import os
import re
import sys
from pathlib import Path

def migrate_file_comments(content):
    """Migrate a single file's comment syntax."""
    lines = content.split('\n')
    migrated = []
    in_c_comment = False
    
    for line in lines:
        # Skip if we're inside a C-style comment
        if in_c_comment:
            if '*/' in line:
                in_c_comment = False
                # Skip the line entirely (remove C-style comments)
                continue
            else:
                # Skip lines that are part of C-style comment
                continue
        
        # Check for start of C-style comment
        if '/*' in line:
            # Handle single-line C-style comments
            if '*/' in line:
                # Remove the comment but keep the rest of the line
                line = re.sub(r'/\*.*?\*/', '', line)
            else:
                # Multi-line C-style comment starts
                in_c_comment = True
                # Keep the part before the comment
                line = line[:line.index('/*')].rstrip()
                if not line:
                    continue
        
        # Convert // comments to # comments
        # Be careful not to convert // inside strings
        if '//' in line:
            # Simple approach: look for // and convert to #
            # Check if it's actually a comment (not in a string)
            parts = line.split('//')
            if len(parts) >= 2:
                # Crude check - if the // appears after quotes, might be in string
                before_comment = parts[0]
                
                # Count quotes before the //
                single_quotes = before_comment.count("'")
                double_quotes = before_comment.count('"')
                
                # If even number of quotes, we're likely outside a string
                if single_quotes % 2 == 0 and double_quotes % 2 == 0:
                    # It's a real comment, convert it
                    comment_part = '//'.join(parts[1:])
                    line = before_comment + '#' + comment_part
        
        migrated.append(line)
    
    return '\n'.join(migrated)

def process_directory(directory_path):
    """Process all .frm files in a directory."""
    path = Path(directory_path)
    frm_files = list(path.glob('**/*.frm'))
    
    print(f"Found {len(frm_files)} .frm files to migrate")
    
    success_count = 0
    error_count = 0
    errors = []
    
    for frm_file in frm_files:
        try:
            # Read the file
            with open(frm_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Skip if no comments to migrate
            if '//' not in content and '/*' not in content:
                continue
                
            # Migrate the content
            migrated = migrate_file_comments(content)
            
            # Write back
            with open(frm_file, 'w', encoding='utf-8') as f:
                f.write(migrated)
            
            success_count += 1
            print(f"✓ Migrated: {frm_file.name}")
            
        except Exception as e:
            error_count += 1
            errors.append((frm_file, str(e)))
            print(f"✗ Error processing {frm_file}: {e}")
    
    print(f"\n=== Migration Complete ===")
    print(f"Successfully migrated: {success_count} files")
    print(f"Errors: {error_count} files")
    
    if errors:
        print("\nFiles with errors:")
        for file, error in errors:
            print(f"  {file}: {error}")
    
    return success_count, error_count

def main():
    # Target the test directory
    test_dir = "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src"
    
    if not os.path.exists(test_dir):
        print(f"Error: Test directory not found: {test_dir}")
        sys.exit(1)
    
    print(f"Migrating Frame test files in: {test_dir}")
    print("Converting // comments to # comments")
    print("Removing /* */ C-style comments")
    print("Preserving {-- --} Frame documentation comments\n")
    
    success, errors = process_directory(test_dir)
    
    if errors > 0:
        sys.exit(1)

if __name__ == '__main__':
    main()