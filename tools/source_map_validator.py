#!/usr/bin/env python3
"""
Source Map Validation Tool for Frame Transpiler
Provides standardized analysis that both VS Code extension and transpiler team can use
"""
import json
import subprocess
import sys
from collections import defaultdict

def analyze_source_map(frm_file_path):
    """Generate comprehensive source map analysis"""
    
    # Generate debug output
    result = subprocess.run(
        ['./target/release/framec', '-l', 'python_3', '--debug-output', frm_file_path],
        capture_output=True, text=True, cwd='/Users/marktruluck/projects/frame_transpiler'
    )
    
    if result.returncode != 0:
        print(f"ERROR: Transpilation failed: {result.stderr}")
        return None
        
    try:
        data = json.loads(result.stdout)
    except json.JSONDecodeError:
        print(f"ERROR: Invalid JSON output from transpiler")
        return None
    
    # Extract data
    python_lines = data['python'].split('\n')
    mappings = data['sourceMap']['mappings']
    
    # Read the frame source file directly since it's not in debug output
    with open(frm_file_path, 'r') as f:
        frame_content = f.read().split('\n')
    
    # Build analysis
    analysis = {
        'total_python_lines': len([line for line in python_lines if line.strip()]),
        'total_frame_lines': len([line for line in frame_content if line.strip()]),
        'total_mappings': len(mappings),
        'mapped_python_lines': set(m['pythonLine'] for m in mappings),
        'mapped_frame_lines': set(m['frameLine'] for m in mappings),
        'python_coverage': 0,
        'frame_coverage': 0,
        'gaps': [],
        'duplicates': [],
        'main_function_analysis': None
    }
    
    # Calculate coverage
    analysis['python_coverage'] = len(analysis['mapped_python_lines']) / analysis['total_python_lines'] * 100
    analysis['frame_coverage'] = len(analysis['mapped_frame_lines']) / analysis['total_frame_lines'] * 100
    
    # Find gaps in Python coverage
    python_lines_with_content = []
    for i, line in enumerate(python_lines, 1):
        if line.strip() and not line.strip().startswith('#'):
            python_lines_with_content.append(i)
    
    for py_line in python_lines_with_content:
        if py_line not in analysis['mapped_python_lines']:
            analysis['gaps'].append({
                'python_line': py_line, 
                'content': python_lines[py_line-1].strip()[:60]
            })
    
    # Find duplicate mappings
    frame_to_python = defaultdict(list)
    for mapping in mappings:
        frame_to_python[mapping['frameLine']].append(mapping['pythonLine'])
    
    for frame_line, python_lines_list in frame_to_python.items():
        if len(python_lines_list) > 1:
            analysis['duplicates'].append({
                'frame_line': frame_line,
                'python_lines': python_lines_list,
                'count': len(python_lines_list)
            })
    
    # Analyze main function specifically (Frame lines 46-79 in test_debug_entry.frm)
    main_mappings = [m for m in mappings if 46 <= m['frameLine'] <= 79]
    if main_mappings:
        main_frame_lines = set(m['frameLine'] for m in main_mappings)
        unmapped_main_lines = []
        for frame_line in range(46, 80):
            if frame_line not in main_frame_lines:
                frame_content_line = frame_content[frame_line-1] if frame_line <= len(frame_content) else ""
                if frame_content_line.strip():
                    unmapped_main_lines.append({
                        'frame_line': frame_line,
                        'content': frame_content_line.strip()[:60]
                    })
        
        analysis['main_function_analysis'] = {
            'total_lines': 34,  # Lines 46-79
            'mapped_lines': len(main_frame_lines),
            'unmapped_lines': unmapped_main_lines,
            'coverage': len(main_frame_lines) / 34 * 100
        }
    
    return analysis

def print_analysis(analysis, verbose=False, frm_file_path=None):
    """Print formatted analysis results"""
    print("=== SOURCE MAP ANALYSIS REPORT ===")
    print(f"Transpiler Version: {get_transpiler_version()}")
    print(f"Total Mappings: {analysis['total_mappings']}")
    print(f"Python Coverage: {analysis['python_coverage']:.1f}%")
    print(f"Frame Coverage: {analysis['frame_coverage']:.1f}%")
    print()
    
    # Main function analysis
    if analysis['main_function_analysis']:
        main = analysis['main_function_analysis']
        print("=== MAIN FUNCTION ANALYSIS (Lines 46-79) ===")
        print(f"Coverage: {main['coverage']:.1f}% ({main['mapped_lines']}/{main['total_lines']})")
        
        if main['unmapped_lines']:
            print("Unmapped Frame lines:")
            for item in main['unmapped_lines'][:10]:  # Show first 10
                print(f"  Line {item['frame_line']}: {item['content']}")
            if len(main['unmapped_lines']) > 10:
                print(f"  ... and {len(main['unmapped_lines']) - 10} more")
        print()
    
    # Show gaps
    if analysis['gaps'] and verbose:
        print("=== UNMAPPED PYTHON LINES ===")
        for gap in analysis['gaps'][:10]:
            print(f"Python {gap['python_line']}: {gap['content']}")
        if len(analysis['gaps']) > 10:
            print(f"... and {len(analysis['gaps']) - 10} more gaps")
        print()
    
    # Show duplicates with Bug #27 specific analysis
    if analysis['duplicates']:
        print("=== DUPLICATE MAPPINGS ===")
        bug27_patterns = 0
        
        # Read frame content once for Bug #27 analysis
        frame_lines = []
        if frm_file_path:
            try:
                with open(frm_file_path, 'r') as f:
                    frame_lines = f.readlines()
            except:
                pass
        
        for dup in analysis['duplicates']:
            print(f"Frame {dup['frame_line']} → {dup['count']} Python lines: {dup['python_lines']}")
            
            # Check for Bug #27 specific patterns
            if frame_lines and dup['frame_line'] <= len(frame_lines):
                content = frame_lines[dup['frame_line']-1].strip()
                if ('() {' in content or '-> $' in content):
                    bug27_patterns += 1
                    print(f"  ⚠️ Bug #27 pattern: {content}")
        
        if bug27_patterns > 0:
            print(f"  📋 Bug #27 Impact: {bug27_patterns} event handler/transition duplicates detected")
        print()
    
    # Status assessment
    print("=== ASSESSMENT ===")
    if analysis['main_function_analysis']:
        main_coverage = analysis['main_function_analysis']['coverage']
        unmapped_executable = [line for line in analysis['main_function_analysis']['unmapped_lines'] 
                              if not line['content'].startswith('#') and line['content'] not in ['}', '{']]
        executable_coverage = (analysis['main_function_analysis']['mapped_lines'] / 
                             (analysis['main_function_analysis']['mapped_lines'] + len(unmapped_executable))) * 100
        
        print(f"Main function executable coverage: {executable_coverage:.1f}%")
        if executable_coverage >= 100:
            print("✅ PERFECT: 100% executable statement coverage")
        elif executable_coverage >= 95:
            print("✅ EXCELLENT: >95% executable statement coverage")
        elif executable_coverage >= 90:
            print("✅ GOOD: >90% executable statement coverage")
        elif executable_coverage >= 80:
            print("⚠️  FAIR: >80% executable statement coverage") 
        else:
            print("❌ POOR: <80% executable statement coverage")
            
        if unmapped_executable:
            print("Unmapped executable statements:")
            for line in unmapped_executable[:5]:
                print(f"  Line {line['frame_line']}: {line['content']}")
    
    if analysis['duplicates']:
        if len(analysis['duplicates']) <= 2:
            print(f"⚠️  MINOR: {len(analysis['duplicates'])} duplicate mappings (acceptable)")
        elif len(analysis['duplicates']) <= 5:
            print(f"⚠️  WARNING: {len(analysis['duplicates'])} duplicate mappings detected")
        else:
            print(f"❌ CRITICAL: {len(analysis['duplicates'])} duplicate mappings detected")
    else:
        print("✅ No duplicate mappings")

def get_transpiler_version():
    """Get current transpiler version"""
    try:
        result = subprocess.run(['./target/release/framec', '--version'], 
                              capture_output=True, text=True, 
                              cwd='/Users/marktruluck/projects/frame_transpiler')
        return result.stdout.strip()
    except:
        return "unknown"

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 source_map_validator.py <frame_file.frm> [--verbose]")
        sys.exit(1)
    
    frm_file = sys.argv[1]
    verbose = '--verbose' in sys.argv
    
    analysis = analyze_source_map(frm_file)
    if analysis:
        print_analysis(analysis, verbose, frm_file)
    else:
        sys.exit(1)

if __name__ == "__main__":
    main()