#!/usr/bin/env python3
"""
Script to fix specific runtime errors in generated Python test files.

ERROR PATTERNS IDENTIFIED:
1. NAME_ERROR: Incomplete __kernel calls (missing parentheses/args)
2. TYPE_ERROR: main() functions with parameters called without arguments  
3. TYPE_ERROR: System constructors not accepting required parameters
4. ATTRIBUTE_ERROR: Incomplete method calls or missing implementations

FIX STRATEGIES:
- Pattern 1: Fix truncated __kernel calls
- Pattern 2: Add default parameter values to main() functions
- Pattern 3: Update system constructors to accept parameters
- Pattern 4: Fix incomplete attribute accesses
"""

import os
import re
import sys
from pathlib import Path

script_dir = Path(__file__).parent
src_dir = script_dir.parent / "src"

def fix_incomplete_kernel_calls(content):
    """Fix incomplete __kernel calls that are missing parentheses or arguments"""
    # Pattern: self.__kernel or self.__ker (incomplete)
    fixed = re.sub(r'self\.__ker$', 'self.__kernel(__e)', content, flags=re.MULTILINE)
    fixed = re.sub(r'self\.__kernel$', 'self.__kernel(__e)', fixed, flags=re.MULTILINE)
    
    # Pattern: self.__kernel( without closing )
    lines = fixed.split('\n')
    fixed_lines = []
    
    for line in lines:
        if 'self.__kernel(' in line and not line.strip().endswith(')'):
            # Check if it's an incomplete call
            if line.count('(') > line.count(')'):
                line = line.rstrip() + ')'
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_main_function_parameters(content):
    """Fix main() functions that have parameters but are called without arguments"""
    # Pattern: def main(param1, param2): called as main()
    
    # Find main function definition with parameters
    main_match = re.search(r'def main\(([^)]+)\):', content)
    if main_match and '__main__' in content:
        params = main_match.group(1).strip()
        if params and not params.startswith('sys_arg'):
            # Skip if not system argument parameters
            return content
            
        # Replace the call to provide default arguments
        if 'main()' in content:
            # Count parameters to provide default values
            param_list = [p.strip() for p in params.split(',') if p.strip()]
            default_args = ', '.join(['"default"'] * len(param_list))
            
            content = content.replace('main()', f'main({default_args})')
    
    return content

def fix_system_constructor_parameters(content):
    """Fix system constructors that don't accept required parameters"""
    lines = content.split('\n')
    fixed_lines = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Look for system instantiation with parameters
        match = re.search(r'(\w+)\s*=\s*(\w+System\w*)\((.*)\)', line)
        if match:
            var_name, class_name, args = match.groups()
            if args.strip() and ',' in args:  # Multiple arguments
                # Find the class definition
                for j in range(i + 1, len(lines)):
                    if f'class {class_name}:' in lines[j]:
                        # Check next few lines for __init__
                        for k in range(j + 1, min(j + 10, len(lines))):
                            if 'def __init__(self):' in lines[k]:
                                # Add parameters to constructor
                                param_count = len([a.strip() for a in args.split(',') if a.strip()])
                                param_list = ', '.join([f'arg{n}' for n in range(param_count)])
                                lines[k] = lines[k].replace('def __init__(self):', f'def __init__(self, {param_list}):')
                                break
                        break
        
        fixed_lines.append(line)
        i += 1
    
    return '\n'.join(fixed_lines)

def fix_attribute_errors(content):
    """Fix incomplete attribute accesses and method calls"""
    # Pattern: self.__kernel without call
    fixed = re.sub(r'(\s+)self\.__kernel\s*$', r'\1self.__kernel(__e)', content, flags=re.MULTILINE)
    
    # Pattern: Missing return statements in methods
    lines = fixed.split('\n')
    fixed_lines = []
    
    for i, line in enumerate(lines):
        fixed_lines.append(line)
        
        # If we see an incomplete method call at end of method, fix it
        if (line.strip().startswith('self.__kernel') and 
            not line.strip().endswith(')') and 
            i < len(lines) - 1 and 
            (lines[i + 1].strip() == '' or lines[i + 1].strip().startswith('return'))):
            
            if '__e' not in line:
                fixed_lines[-1] = line.rstrip() + '(__e)'
    
    return '\n'.join(fixed_lines)

def fix_debug_comment_issues(content):
    """Fix malformed debug comments that break Python syntax"""
    # Pattern: })# DEBUG_EXPR_TYPE: -> proper comment
    fixed = re.sub(r'\}+\)?(# DEBUG_EXPR_TYPE:.*)', r'\1', content)
    
    # Pattern: Missing proper line endings
    fixed = re.sub(r'(DEBUG_EXPR_TYPE:.*?)(\w)', r'\1\n\2', fixed)
    
    return fixed

def fix_python_file(py_file):
    """Apply all fixes to a Python file"""
    try:
        with open(py_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_incomplete_kernel_calls(content)
        content = fix_main_function_parameters(content)  
        content = fix_system_constructor_parameters(content)
        content = fix_attribute_errors(content)
        content = fix_debug_comment_issues(content)
        
        # Only write if content changed
        if content != original_content:
            with open(py_file, 'w') as f:
                f.write(content)
            return True, "Fixed"
        else:
            return True, "No changes needed"
            
    except Exception as e:
        return False, f"Error: {e}"

def main():
    print("🔧 Fixing Python Runtime Errors")
    print("=" * 50)
    
    # Find all Python test files
    py_files = list(src_dir.glob("*.py"))
    py_files = [f for f in py_files if 'Test.py' in f.name or f.name.startswith('test_')]
    py_files.sort()
    
    fixed_count = 0
    error_count = 0
    
    for py_file in py_files:
        print(f"Processing: {py_file.name}")
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
    
    print("\n" + "=" * 50)
    print("📊 FIX SUMMARY:")
    print(f"✅ Files Fixed: {fixed_count}")
    print(f"⚪ Files Unchanged: {len(py_files) - fixed_count - error_count}")
    print(f"❌ Files with Errors: {error_count}")
    
    if fixed_count > 0:
        print(f"\n🎉 Fixed {fixed_count} files! Re-run tests to validate.")

if __name__ == "__main__":
    main()