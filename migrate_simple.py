#!/usr/bin/env python3
"""
Simple Frame v0.31 Migration: Convert basic action calls to explicit self syntax
"""

import re
import sys
from pathlib import Path
import argparse

def migrate_frame_file(filepath, dry_run=False, verbose=False):
    """Migrate a single Frame file to explicit self syntax"""
    
    content = filepath.read_text()
    lines = content.split('\n')
    new_lines = []
    changes = []
    
    # Built-in functions that should NOT be prefixed with self
    builtin_functions = {'print', 'str', 'len', 'int', 'float', 'bool', 'range', 'list', 'dict'}
    
    # Simple patterns for common cases that need migration
    patterns = [
        # Domain variable in print() function call
        (r'(\s+print\()(\w+)(\))', r'\1self.\2\3', 'domain variable in print'),
        
        # Domain variable in str() function call  
        (r'(\s+.*str\()(\w+)(\))', r'\1self.\2\3', 'domain variable in str'),
        
        # Domain variable assignments
        (r'^(\s+)(\w+)(\s*=\s*)', r'\1self.\2\3', 'domain variable assignment'),
        
        # Simple action calls (but not builtin functions)
        (r'^(\s+)(\w+)(\(\))$', lambda m: f'{m.group(1)}self.{m.group(2)}{m.group(3)}' if m.group(2) not in builtin_functions else m.group(0), 'action call'),
    ]
    
    in_machine_block = False
    in_actions_block = False
    in_string_context = False
    
    for line_num, line in enumerate(lines, 1):
        original_line = line
        
        # Track context
        if 'machine:' in line:
            in_machine_block = True
            if verbose:
                print(f"Line {line_num}: Entering machine block")
        elif 'actions:' in line:
            in_actions_block = True
            if verbose:
                print(f"Line {line_num}: Entering actions block")
        elif line.strip().startswith('}') and (in_machine_block or in_actions_block):
            if in_machine_block and verbose:
                print(f"Line {line_num}: Exiting machine block")
            if in_actions_block and verbose:
                print(f"Line {line_num}: Exiting actions block")
            in_machine_block = False
            in_actions_block = False
        
        # Skip lines that shouldn't be modified
        if (
            '//' in line or  # Comments
            '"' in line or   # String literals (simplified)
            "'" in line or   # String literals
            'self.' in line or  # Already has self
            'system.' in line or  # Already has system
            line.strip().startswith('#') or  # Frame comments
            not (in_machine_block or in_actions_block)  # Not in relevant blocks
        ):
            new_lines.append(line)
            continue
        
        # Only modify lines in machine or actions blocks
        if in_machine_block or in_actions_block:
            # Skip method declarations (lines that define methods)
            if re.match(r'\s+\w+\s*\([^)]*\)\s*(\{|:.*\{)', line):
                new_lines.append(line)
                continue
            
            # Apply simple transformations
            modified = False
            for pattern, replacement, desc in patterns:
                if replacement is None:  # Skip pattern
                    continue
                
                # Handle lambda replacements
                if callable(replacement):
                    match = re.search(pattern, line)
                    if match:
                        new_line = replacement(match)
                        if new_line != line:
                            changes.append(f"Line {line_num}: {desc} - {line.strip()} -> {new_line.strip()}")
                            if verbose:
                                print(f"Line {line_num}: Applied {desc}")
                            line = new_line
                            modified = True
                            break
                else:
                    # Handle string replacements
                    if re.search(pattern, line):
                        new_line = re.sub(pattern, replacement, line)
                        if new_line != line:
                            changes.append(f"Line {line_num}: {desc} - {line.strip()} -> {new_line.strip()}")
                            if verbose:
                                print(f"Line {line_num}: Applied {desc}")
                            line = new_line
                            modified = True
                            break
        
        new_lines.append(line)
    
    # Report results
    if changes:
        print(f"  Found {len(changes)} potential changes:")
        for change in changes[:5]:  # Show first 5
            print(f"    {change}")
        if len(changes) > 5:
            print(f"    ... and {len(changes) - 5} more")
        
        if not dry_run:
            # Write the modified file
            filepath.write_text('\n'.join(new_lines))
            print(f"  Applied changes to {filepath}")
        else:
            print(f"  Dry run - no changes written")
    else:
        print(f"  No changes needed")
    
    return len(changes)

def main():
    parser = argparse.ArgumentParser(description='Simple Frame v0.31 migration script')
    parser.add_argument('pattern', nargs='?', default='*.frm', help='File pattern to process')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying them')
    parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    # Find Frame files
    src_dir = Path('framec_tests/python/src')
    if not src_dir.exists():
        print(f"Error: Directory {src_dir} not found")
        sys.exit(1)
    
    frame_files = list(src_dir.glob(args.pattern))
    if not frame_files:
        print(f"No .frm files found matching pattern: {args.pattern}")
        return
    
    print(f"Found {len(frame_files)} Frame files to process")
    
    total_changes = 0
    for filepath in sorted(frame_files)[:5]:  # Limit to first 5 for testing
        print(f"\nProcessing {filepath.name}...")
        try:
            changes = migrate_frame_file(filepath, args.dry_run, args.verbose)
            total_changes += changes
        except Exception as e:
            print(f"  Error: {e}")
    
    action = "Would make" if args.dry_run else "Made"
    print(f"\n{action} {total_changes} total changes")

if __name__ == '__main__':
    main()