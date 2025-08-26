#!/usr/bin/env python3
"""
Comprehensive test validation script for Frame v0.30
Tests all .frm files by transpiling and executing them
"""

import os
import subprocess
import sys
from pathlib import Path
import json

def run_command(cmd, cwd=None):
    """Run a command and return success status and output"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=cwd, timeout=5)
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, "", "Timeout"
    except Exception as e:
        return False, "", str(e)

def main():
    # Paths
    project_root = Path("/Users/marktruluck/projects/frame_transpiler")
    src_dir = project_root / "framec_tests/python/src"
    framec = project_root / "target/debug/framec"
    
    # Check framec exists
    if not framec.exists():
        print(f"Error: framec not found at {framec}")
        return 1
    
    # Find all .frm files
    frm_files = sorted(src_dir.glob("*.frm"))
    print(f"Found {len(frm_files)} Frame test files\n")
    
    # Test categories
    results = {
        "transpile_success": [],
        "transpile_fail": [],
        "execute_success": [],
        "execute_fail": [],
        "syntax_error": [],
        "runtime_error": []
    }
    
    # Test each file
    for frm_file in frm_files:
        test_name = frm_file.stem
        py_file = frm_file.with_suffix('.py')
        
        print(f"Testing {test_name}...", end=" ")
        
        # Transpile
        transpile_cmd = f"{framec} -l python_3 {frm_file.name}"
        success, stdout, stderr = run_command(transpile_cmd, cwd=src_dir)
        
        if not success:
            results["transpile_fail"].append(test_name)
            print("❌ Transpile failed")
            continue
        
        # Save transpiled output
        with open(py_file, 'w') as f:
            f.write(stdout)
        
        results["transpile_success"].append(test_name)
        
        # Execute Python
        exec_cmd = f"python3 {py_file.name}"
        success, stdout, stderr = run_command(exec_cmd, cwd=src_dir)
        
        if success:
            results["execute_success"].append(test_name)
            print("✅ Success")
        else:
            if "SyntaxError" in stderr:
                results["syntax_error"].append(test_name)
                print("❌ Syntax error")
            elif stderr:
                results["runtime_error"].append(test_name)
                print("❌ Runtime error")
            else:
                results["execute_fail"].append(test_name)
                print("❌ Execute failed")
    
    # Print summary
    print("\n" + "="*60)
    print("VALIDATION SUMMARY")
    print("="*60)
    
    total = len(frm_files)
    transpile_success = len(results["transpile_success"])
    execute_success = len(results["execute_success"])
    
    print(f"Total test files: {total}")
    print(f"Transpile success: {transpile_success}/{total} ({transpile_success*100//total}%)")
    print(f"Execute success: {execute_success}/{total} ({execute_success*100//total}%)")
    
    print(f"\nBreakdown:")
    print(f"  ✅ Fully passing: {execute_success}")
    print(f"  🔧 Transpiles but fails execution: {transpile_success - execute_success}")
    print(f"  ❌ Transpile failures: {len(results['transpile_fail'])}")
    
    if results["syntax_error"]:
        print(f"\nSyntax errors: {len(results['syntax_error'])}")
        for test in results["syntax_error"][:5]:
            print(f"  - {test}")
        if len(results["syntax_error"]) > 5:
            print(f"  ... and {len(results['syntax_error']) - 5} more")
    
    if results["runtime_error"]:
        print(f"\nRuntime errors: {len(results['runtime_error'])}")
        for test in results["runtime_error"][:5]:
            print(f"  - {test}")
        if len(results["runtime_error"]) > 5:
            print(f"  ... and {len(results['runtime_error']) - 5} more")
    
    # Save detailed results
    results_file = src_dir.parent / "scripts" / "validation_results.json"
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nDetailed results saved to: {results_file}")
    
    return 0 if execute_success == total else 1

if __name__ == "__main__":
    sys.exit(main())