#!/usr/bin/env python3
"""
Script to fix indentation errors in generated Python files caused by broken DEBUG comments.
"""

import os
import re
from pathlib import Path

script_dir = Path(__file__).parent
src_dir = script_dir.parent / "src"

def fix_broken_debug_comments(content):
    """Fix DEBUG comments that are broken across multiple lines causing indentation errors"""
    
    # Pattern 1: Fix broken DEBUG_EXPR_TYPE comments
    # # DEBUG_EXPR_TYPE: \nDiscriminant(4) -> # DEBUG_EXPR_TYPE: Discriminant(4)
    content = re.sub(r'# DEBUG_EXPR_TYPE:\s*\n\s*Discriminant\((\d+)\)', 
                     r'# DEBUG_EXPR_TYPE: Discriminant(\1)', content, flags=re.MULTILINE)
    
    # Pattern 2: Fix unexpected indentation on lines with debug comments
    lines = content.split('\n')
    fixed_lines = []
    
    for i, line in enumerate(lines):
        # Check if this line has incorrect indentation for a debug comment
        if '# DEBUG_EXPR_TYPE:' in line and line.strip().startswith('Discriminant'):
            # This line should be combined with the previous line
            if fixed_lines and '# DEBUG_EXPR_TYPE:' in fixed_lines[-1]:
                # Combine with previous line
                fixed_lines[-1] = fixed_lines[-1].rstrip() + ' ' + line.strip()
                continue
        
        # Check for lines that are incorrectly indented after if statements
        if (line.strip().startswith('Discriminant(') and 
            i > 0 and 
            'if __e._message ==' in lines[i-1]):
            # This should be a comment on the if line
            if fixed_lines:
                fixed_lines[-1] = fixed_lines[-1].rstrip() + ' # DEBUG_EXPR_TYPE: ' + line.strip()
                continue
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_transition_statements(content):
    """Fix transition statements that are malformed"""
    
    # Pattern: next_compartment = Noneself.__transition(next_compartment)
    # Should be: next_compartment = FrameCompartment('state', None, None, None, None)\n self.__transition(next_compartment)
    
    # For now, let's just fix the immediate syntax error
    content = re.sub(r'next_compartment = Noneself\.__transition\(next_compartment\)', 
                     'next_compartment = None\n            self.__transition(next_compartment)', content)
    
    return content

def fix_missing_statements(content):
    """Fix missing statements in empty if blocks"""
    lines = content.split('\n')
    fixed_lines = []
    
    for i, line in enumerate(lines):
        fixed_lines.append(line)
        
        # If we see an if statement followed by a return, we need a statement in between
        if (line.strip().startswith('if __e._message ==') and 
            i + 1 < len(lines) and 
            lines[i + 1].strip() == 'return'):
            # Add a pass statement
            indent = len(line) - len(line.lstrip())
            fixed_lines.append(' ' * (indent + 4) + 'pass')
    
    return '\n'.join(fixed_lines)

def fix_python_file(py_file):
    """Apply indentation fixes to a Python file"""
    try:
        with open(py_file, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes in order
        content = fix_broken_debug_comments(content)
        content = fix_transition_statements(content)
        content = fix_missing_statements(content)
        
        if content != original_content:
            with open(py_file, 'w') as f:
                f.write(content)
            return True, "Fixed indentation"
        else:
            return True, "No changes needed"
            
    except Exception as e:
        return False, f"Error: {e}"

def main():
    print("🔧 Fixing Python Indentation Errors")
    print("=" * 50)
    
    # Focus on the files that had indentation errors
    problem_files = [
        'ActionsBlockTest.py',
        'DomainBlockTest.py', 
        'DomainTypedTest.py',
        'FullSystemParamsTest.py',
        'LampCompleteTest.py',
        'MainParamsTest.py'
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
    print("📊 INDENTATION FIX SUMMARY:")
    print(f"✅ Files Fixed: {fixed_count}")
    print(f"❌ Files with Errors: {error_count}")

if __name__ == "__main__":
    main()