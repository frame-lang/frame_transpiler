#!/usr/bin/env python3
"""
Script to analyze specific Python runtime failures and categorize them.
"""

import os
import subprocess
import sys
from pathlib import Path

# Get script directory
script_dir = Path(__file__).parent
src_dir = script_dir.parent / "src"

def run_python_file(py_file):
    """Run a Python file and capture the error"""
    try:
        result = subprocess.run([sys.executable, str(py_file)], 
                              capture_output=True, text=True, 
                              cwd=str(src_dir), timeout=10)
        
        if result.returncode == 0:
            return True, result.stdout
        else:
            return False, result.stderr
    except subprocess.TimeoutExpired:
        return False, "TIMEOUT: Execution exceeded 10 seconds"
    except Exception as e:
        return False, f"ERROR: {e}"

def categorize_error(error_msg):
    """Categorize the type of error"""
    if "NameError" in error_msg:
        return "NAME_ERROR"
    elif "AttributeError" in error_msg:
        return "ATTRIBUTE_ERROR"
    elif "TypeError" in error_msg:
        return "TYPE_ERROR"
    elif "IndentationError" in error_msg:
        return "INDENTATION_ERROR"
    elif "SyntaxError" in error_msg:
        return "SYNTAX_ERROR"
    elif "TIMEOUT" in error_msg:
        return "TIMEOUT"
    elif "infinite" in error_msg.lower():
        return "INFINITE_LOOP"
    elif "RecursionError" in error_msg:
        return "RECURSION_ERROR"
    else:
        return "OTHER"

def main():
    print("🔍 Python Runtime Error Analysis")
    print("=" * 50)
    
    # Find all Python test files
    py_files = list(src_dir.glob("test_*.py"))
    py_files.extend(src_dir.glob("*Test.py"))
    py_files.sort()
    
    success_count = 0
    failure_count = 0
    error_categories = {}
    detailed_errors = []
    
    for py_file in py_files:  # Test ALL files to get complete picture
        print(f"Testing: {py_file.name}")
        success, output = run_python_file(py_file)
        
        if success:
            print(f"  ✅ SUCCESS")
            success_count += 1
        else:
            print(f"  ❌ FAILED")
            failure_count += 1
            
            # Categorize error
            category = categorize_error(output)
            error_categories[category] = error_categories.get(category, 0) + 1
            
            # Store detailed error for analysis
            detailed_errors.append({
                'file': py_file.name,
                'category': category,
                'error': output[:500]  # First 500 chars for more context
            })
            
            print(f"    Category: {category}")
            print(f"    Error: {output[:150]}...")
    
    print("\n" + "=" * 50)
    print("📊 ERROR ANALYSIS:")
    print(f"✅ Success: {success_count}")
    print(f"❌ Failures: {failure_count}")
    print("\nError Categories:")
    for category, count in sorted(error_categories.items()):
        print(f"  {category}: {count}")
    
    print("\n🔧 DETAILED ERRORS (first 5):")
    for i, error in enumerate(detailed_errors[:5]):
        print(f"\n{i+1}. {error['file']} ({error['category']})")
        print(f"   {error['error']}")
        print("-" * 40)

if __name__ == "__main__":
    main()