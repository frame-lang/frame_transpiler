#!/usr/bin/env python3
"""
Comprehensive script to fix all types of indentation errors in generated Python files.
"""

import os
import re
from pathlib import Path

script_dir = Path(__file__).parent
src_dir = script_dir.parent / "src"

def fix_all_indentation_issues(content):
    """Fix all types of indentation errors comprehensively"""
    
    lines = content.split('\n')
    fixed_lines = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Pattern 1: Discriminant on separate line after if/def/etc.
        if (line.strip() == 'Discriminant(4)' and 
            fixed_lines and 
            (fixed_lines[-1].strip().endswith(':') or 
             '# DEBUG_EXPR_TYPE:' in fixed_lines[-1])):
            
            # This should be combined with previous line or replaced with pass
            if '# DEBUG_EXPR_TYPE:' in fixed_lines[-1]:
                # Combine with previous comment line
                fixed_lines[-1] = fixed_lines[-1].rstrip() + ' ' + line.strip()
            else:
                # Add a pass statement instead
                indent = len(line) - len(line.lstrip())
                fixed_lines.append(' ' * indent + 'pass')
            i += 1
            continue
        
        # Pattern 2: Unexpected indent on function calls or statements
        if (line.strip() and 
            not line.strip().startswith('#') and 
            line.startswith(' ') and
            fixed_lines and
            not fixed_lines[-1].strip().endswith(':') and
            not fixed_lines[-1].strip().startswith('def ') and
            not fixed_lines[-1].strip().startswith('if ') and
            not fixed_lines[-1].strip().startswith('elif ') and
            not fixed_lines[-1].strip().startswith('else:') and
            not fixed_lines[-1].strip().startswith('while ') and
            not fixed_lines[-1].strip().startswith('for ') and
            not fixed_lines[-1].strip().startswith('try:') and
            not fixed_lines[-1].strip().startswith('except') and
            not fixed_lines[-1].strip().startswith('finally:') and
            not fixed_lines[-1].strip().startswith('with ') and
            not fixed_lines[-1].strip().startswith('class ') and
            'return' not in fixed_lines[-1]):
            
            # Check if this line should be at the same level as previous
            prev_indent = len(fixed_lines[-1]) - len(fixed_lines[-1].lstrip())
            curr_indent = len(line) - len(line.lstrip())
            
            # If current line is indented more than previous but previous doesn't end with :
            if curr_indent > prev_indent:
                # De-indent to match previous line
                line = ' ' * prev_indent + line.lstrip()
        
        # Pattern 3: Fix broken debug comments across lines
        if ('# DEBUG_EXPR_TYPE:' in line and 
            i + 1 < len(lines) and 
            lines[i + 1].strip().startswith('Discriminant')):
            
            # Combine the two lines
            line = line.rstrip() + ' ' + lines[i + 1].strip()
            i += 1  # Skip the next line since we combined it
        
        # Pattern 4: Empty function/if/elif blocks need pass statements
        if (line.strip().endswith(':') and 
            (line.strip().startswith('def ') or 
             line.strip().startswith('if ') or 
             line.strip().startswith('elif ') or 
             line.strip().startswith('else:'))):
            
            fixed_lines.append(line)
            
            # Look ahead to see if next non-empty line is properly indented
            j = i + 1
            found_content = False
            expected_indent = (len(line) - len(line.lstrip())) + 4
            
            while j < len(lines):
                next_line = lines[j]
                if next_line.strip():
                    if next_line.strip().startswith('Discriminant') or next_line.strip().startswith('#'):
                        # Skip debug comments
                        j += 1
                        continue
                    
                    next_indent = len(next_line) - len(next_line.lstrip())
                    if next_indent >= expected_indent:
                        found_content = True
                    break
                j += 1
            
            # If no properly indented content found, add pass
            if not found_content:
                fixed_lines.append(' ' * expected_indent + 'pass')
            
            i += 1
            continue
        
        fixed_lines.append(line)
        i += 1
    
    return '\n'.join(fixed_lines)

def fix_python_file(py_file):
    """Apply comprehensive indentation fixes to a Python file"""
    try:
        with open(py_file, 'r') as f:
            content = f.read()
        
        original_content = content
        content = fix_all_indentation_issues(content)
        
        if content != original_content:
            with open(py_file, 'w') as f:
                f.write(content)
            return True, "Fixed indentation issues"
        else:
            return True, "No changes needed"
            
    except Exception as e:
        return False, f"Error: {e}"

def main():
    print("🔧 Comprehensive Indentation Error Fix")
    print("=" * 50)
    
    # Find files that had indentation errors from the test
    problem_files = [
        'SimpleSystemParamsTest.py',
        'SystemParametersTest.py',
        'SystemsTest.py', 
        'TransitionsTest.py',
        'test_all_blocks_comprehensive.py',
        'test_blocks_simple.py',
        'test_call_chain_debug.py',
        'test_call_chain_scope.py'
    ]
    
    fixed_count = 0
    error_count = 0
    
    for filename in problem_files:
        py_file = src_dir / filename
        if py_file.exists():
            print(f"Processing: {filename}")
            success, message = fix_python_file(py_file)
            
            if success:
                if "Fixed" in message:
                    print(f"  ✅ {message}")
                    fixed_count += 1
                else:
                    print(f"  ⚪ {message}")
            else:
                print(f"  ❌ {message}")
                error_count += 1
        else:
            print(f"  ⚠️ File not found: {filename}")
    
    print("\n" + "=" * 50)
    print("📊 COMPREHENSIVE INDENTATION FIX SUMMARY:")
    print(f"✅ Files Fixed: {fixed_count}")
    print(f"❌ Files with Errors: {error_count}")
    
    if fixed_count > 0:
        print(f"\n🎉 Fixed indentation in {fixed_count} files!")

if __name__ == "__main__":
    main()