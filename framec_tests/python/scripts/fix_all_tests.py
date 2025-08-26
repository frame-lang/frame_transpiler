#!/usr/bin/env python3
"""
Script to systematically fix and regenerate all problematic Frame test files.
This addresses IndentationError issues by regenerating outdated Python files.
"""

import os
import subprocess
import sys
from pathlib import Path

# Get script directory
script_dir = Path(__file__).parent
project_root = script_dir.parent.parent.parent
src_dir = script_dir.parent / "src"
framec_binary = project_root / "target" / "debug" / "framec"

def run_framec(frm_file, output_file):
    """Run framec transpiler on a .frm file"""
    try:
        cmd = [str(framec_binary), "-l", "python_3", str(frm_file)]
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=str(project_root))
        
        if result.returncode == 0:
            # Write output to file
            with open(output_file, 'w') as f:
                f.write(result.stdout)
            print(f"✅ REGENERATED: {output_file.name}")
            return True
        else:
            print(f"❌ FAILED: {frm_file.name} - {result.stderr.strip()}")
            return False
    except Exception as e:
        print(f"❌ ERROR: {frm_file.name} - {e}")
        return False

def test_python_file(py_file):
    """Test if a Python file compiles correctly"""
    try:
        # Try to compile the Python file
        with open(py_file, 'r') as f:
            content = f.read()
        
        compile(content, py_file, 'exec')
        return True
    except SyntaxError as e:
        print(f"❌ SYNTAX ERROR: {py_file.name} - Line {e.lineno}: {e.msg}")
        return False
    except Exception as e:
        print(f"❌ COMPILE ERROR: {py_file.name} - {e}")
        return False

def main():
    print("🔧 Frame Test File Regeneration and Validation")
    print("=" * 60)
    
    # Find all .frm files
    frm_files = list(src_dir.glob("*.frm"))
    frm_files.sort()
    
    print(f"Found {len(frm_files)} Frame test files")
    
    regenerated = 0
    failed_transpile = 0
    syntax_ok = 0
    syntax_error = 0
    
    for frm_file in frm_files:
        py_file = frm_file.with_suffix('.py')
        
        print(f"\n📁 Processing: {frm_file.name}")
        
        # Regenerate Python file
        if run_framec(frm_file, py_file):
            regenerated += 1
            
            # Test Python syntax
            if test_python_file(py_file):
                syntax_ok += 1
                print(f"✅ SYNTAX: {py_file.name} compiles correctly")
            else:
                syntax_error += 1
        else:
            failed_transpile += 1
    
    print("\n" + "=" * 60)
    print("🏁 SUMMARY:")
    print(f"📊 Total files: {len(frm_files)}")
    print(f"✅ Transpiled successfully: {regenerated}")
    print(f"❌ Transpilation failed: {failed_transpile}")
    print(f"✅ Python syntax OK: {syntax_ok}")
    print(f"❌ Python syntax errors: {syntax_error}")
    print(f"📈 Success rate: {(syntax_ok / len(frm_files)) * 100:.1f}%")

if __name__ == "__main__":
    main()