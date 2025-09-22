#!/usr/bin/env python3
"""
Source Map Validation Tool for Frame Transpiler
Validates that Frame source lines map correctly to Python output lines

Usage:
    python3 validate_source_maps.py                    # Run default test files
    python3 validate_source_maps.py file1.frm file2.frm  # Test specific files
"""

import json
import sys
import subprocess
import os
from pathlib import Path

def run_transpiler(frame_file, debug=True):
    """Run the Frame transpiler and get output with source map"""
    # Try to find framec in common locations
    framec_paths = [
        "/Users/marktruluck/projects/frame_transpiler/target/release/framec",
        "./target/release/framec",
        "../target/release/framec",
        "framec"
    ]
    
    framec = None
    for path in framec_paths:
        if os.path.exists(path) or os.system(f"which {path} >/dev/null 2>&1") == 0:
            framec = path
            break
    
    if not framec:
        print("Error: Could not find framec executable")
        return None
    
    cmd = [framec, "-l", "python_3", frame_file]
    
    if debug:
        cmd.append("--debug-output")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        if debug:
            return json.loads(result.stdout)
        else:
            return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running transpiler: {e}")
        print(f"stderr: {e.stderr}")
        return None
    except json.JSONDecodeError as e:
        print(f"Error parsing JSON output: {e}")
        return None

def read_frame_file(filepath):
    """Read Frame source file and return lines"""
    try:
        with open(filepath, 'r') as f:
            return f.readlines()
    except FileNotFoundError:
        print(f"Error: File not found: {filepath}")
        return []

def parse_python_output(json_output):
    """Extract Python code lines from debug output"""
    if isinstance(json_output, dict) and 'python' in json_output:
        return json_output['python'].split('\n')
    return []

def validate_mapping(frame_file, verbose=False):
    """Validate source mappings for a Frame file"""
    print(f"\n{'='*60}")
    print(f"Validating: {frame_file}")
    print('='*60)
    
    # Get debug output with source map
    debug_output = run_transpiler(frame_file, debug=True)
    if not debug_output:
        print("Failed to get debug output")
        return False
    
    # Get plain Python output
    python_code = run_transpiler(frame_file, debug=False)
    if not python_code:
        print("Failed to get Python output")
        return False
    
    # Parse outputs
    frame_lines = read_frame_file(frame_file)
    if not frame_lines:
        return False
        
    python_lines = python_code.split('\n')
    source_map = debug_output.get('sourceMap', {}).get('mappings', [])
    
    print(f"\nFrame file has {len(frame_lines)} lines")
    print(f"Python output has {len(python_lines)} lines")
    print(f"Source map has {len(source_map)} mappings")
    
    # Validate each mapping
    issues = []
    for mapping in source_map:
        frame_line = mapping['frameLine']
        python_line = mapping['pythonLine']
        
        # Get actual content
        frame_content = frame_lines[frame_line - 1].strip() if frame_line <= len(frame_lines) else "OUT OF RANGE"
        python_content = python_lines[python_line - 1].strip() if python_line <= len(python_lines) else "OUT OF RANGE"
        
        # Check for specific patterns
        if '$>() {' in frame_content or '<$() {' in frame_content:
            # Event handler declarations should map to function definitions
            if not python_content.startswith('def __handle_') and not python_content.startswith('async def __handle_'):
                issues.append({
                    'frame_line': frame_line,
                    'python_line': python_line,
                    'frame_content': frame_content,
                    'python_content': python_content,
                    'issue': 'Event handler declaration should map to function definition'
                })
        
        elif 'print(' in frame_content:
            # Print statements should map to print statements
            if 'print(' not in python_content:
                issues.append({
                    'frame_line': frame_line,
                    'python_line': python_line,
                    'frame_content': frame_content,
                    'python_content': python_content,
                    'issue': 'Print statement mapping mismatch'
                })
        
        elif frame_content == 'return':
            # Return statements should map to return statements
            if python_content != 'return':
                issues.append({
                    'frame_line': frame_line,
                    'python_line': python_line,
                    'frame_content': frame_content,
                    'python_content': python_content,
                    'issue': 'Return statement mapping mismatch'
                })
        
        # Verbose output for all mappings
        if verbose:
            print(f"  Frame {frame_line:3}: {frame_content[:40]:40} -> Python {python_line:3}: {python_content[:40]}")
    
    # Report results
    if issues:
        print(f"\n❌ Found {len(issues)} mapping issues:\n")
        for issue in issues:
            print(f"Frame line {issue['frame_line']}: {issue['frame_content']}")
            print(f"  -> Maps to Python line {issue['python_line']}: {issue['python_content']}")
            print(f"  Issue: {issue['issue']}\n")
        return False
    else:
        print("\n✅ All mappings validated successfully!")
        
        # Show some example mappings
        print("\nExample mappings:")
        for i, mapping in enumerate(source_map[:5]):
            frame_line = mapping['frameLine']
            python_line = mapping['pythonLine']
            frame_content = frame_lines[frame_line - 1].strip() if frame_line <= len(frame_lines) else "N/A"
            python_content = python_lines[python_line - 1].strip() if python_line <= len(python_lines) else "N/A"
            
            print(f"  Frame {frame_line:3}: {frame_content[:40]:40} -> Python {python_line:3}: {python_content[:40]}")
        
        if len(source_map) > 5:
            print(f"  ... and {len(source_map) - 5} more mappings")
        
        return True

def main():
    """Main validation routine"""
    # Default test files if none provided
    if len(sys.argv) == 1:
        # Try to find the files relative to project root
        project_root = Path(__file__).parent.parent
        test_files = [
            project_root / "framec_tests/python/src/positive_tests/test_multi_systems_with_main.frm",
            project_root / "framec_tests/python/src/positive_tests/test_hierarchy.frm",
            project_root / "framec_tests/python/src/positive_tests/test_state_parameters_simple.frm"
        ]
        # Convert to strings and filter existing files
        test_files = [str(f) for f in test_files if f.exists()]
        
        if not test_files:
            print("Error: No test files found. Please specify Frame files to validate.")
            print("Usage: python3 validate_source_maps.py file1.frm file2.frm")
            return 1
    else:
        test_files = sys.argv[1:]
    
    print("Frame Transpiler Source Map Validator")
    print("Version: 1.0")
    print(f"Transpiler: v0.71")
    
    all_valid = True
    for test_file in test_files:
        if os.path.exists(test_file):
            if not validate_mapping(test_file, verbose='--verbose' in sys.argv):
                all_valid = False
        else:
            print(f"\nSkipping {test_file} - file not found")
            all_valid = False
    
    print("\n" + "="*60)
    if all_valid:
        print("✅ ALL SOURCE MAPS VALID")
    else:
        print("❌ SOME SOURCE MAPS HAVE ISSUES")
    print("="*60)
    
    return 0 if all_valid else 1

if __name__ == "__main__":
    sys.exit(main())