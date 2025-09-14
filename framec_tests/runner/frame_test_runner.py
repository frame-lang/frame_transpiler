#!/usr/bin/env python3
"""
Frame Test Runner - Comprehensive testing tool for Frame language transpiler

This is the standard test runner for the Frame transpiler project. It provides:
- Transpilation testing
- Execution testing  
- Test matrix generation
- Configurable test sets
- Detailed reporting

Author: Frame Development Team
Version: 1.0.0
"""

import os
import sys
import subprocess
import json
import argparse
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple, Optional

# Configuration
DEFAULT_FRAMEC_PATH = "/Users/marktruluck/projects/frame_transpiler/target/debug/framec"
DEFAULT_TEST_DIR = "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src"
DEFAULT_OUTPUT_DIR = "/Users/marktruluck/projects/frame_transpiler/framec_tests/reports"
DEFAULT_TIMEOUT = 5  # seconds

class FrameTestRunner:
    """Main test runner class for Frame transpiler testing"""
    
    def __init__(self, framec_path=None, test_dir=None, output_dir=None, timeout=None):
        self.framec_path = framec_path or DEFAULT_FRAMEC_PATH
        self.test_dir = test_dir or DEFAULT_TEST_DIR
        self.output_dir = output_dir or DEFAULT_OUTPUT_DIR
        self.timeout = timeout or DEFAULT_TIMEOUT
        self.results = []
        self.verbose = False
        
    def is_multifile_test(self, frm_file: str) -> bool:
        """
        Check if a Frame file requires multifile compilation
        by looking for Frame import statements (import ... from "*.frm")
        
        Args:
            frm_file: Path to the .frm file
            
        Returns:
            True if the file contains Frame imports
        """
        try:
            with open(frm_file, 'r') as f:
                for line in f:
                    # Check for Frame imports (files ending with .frm)
                    if 'import' in line and '.frm' in line:
                        return True
            return False
        except:
            return False
    
    def run_transpiler(self, frm_file: str) -> Tuple[bool, str, str]:
        """
        Run the Frame transpiler on a .frm file
        
        Args:
            frm_file: Path to the .frm file
            
        Returns:
            Tuple of (success, stdout, stderr)
        """
        try:
            # Check if this is a multifile test
            if self.is_multifile_test(frm_file):
                # Use multifile flag for tests with Frame imports
                cmd = [self.framec_path, "-m", frm_file, "-l", "python_3"]
            else:
                # Standard single-file compilation
                cmd = [self.framec_path, "-l", "python_3", frm_file]
                
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.timeout
            )
            return result.returncode == 0, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return False, "", "Timeout during transpilation"
        except Exception as e:
            return False, "", str(e)

    def run_python(self, py_file: str) -> Tuple[bool, str, str]:
        """
        Run a Python file and check if it executes successfully
        
        Args:
            py_file: Path to the .py file
            
        Returns:
            Tuple of (success, stdout, stderr)
        """
        try:
            result = subprocess.run(
                ["python3", py_file],
                capture_output=True,
                text=True,
                timeout=self.timeout
            )
            return result.returncode == 0, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return False, "", "Timeout during execution"
        except Exception as e:
            return False, "", str(e)

    def test_file(self, frm_file: str) -> Dict:
        """
        Test a single Frame file
        
        Args:
            frm_file: Path to the .frm file
            
        Returns:
            Dictionary with test results
        """
        py_file = frm_file.replace('.frm', '.py')
        
        # Transpile
        transpile_success, transpile_out, transpile_err = self.run_transpiler(frm_file)
        
        if not transpile_success:
            return {
                'file': os.path.basename(frm_file),
                'transpile': False,
                'execute': False,
                'error': transpile_err[:200] if transpile_err else "Transpilation failed"
            }
        
        # Save transpiled output
        with open(py_file, 'w') as f:
            f.write(transpile_out)
        
        # Execute
        execute_success, execute_out, execute_err = self.run_python(py_file)
        
        return {
            'file': os.path.basename(frm_file),
            'transpile': True,
            'execute': execute_success,
            'output': execute_out[:200] if execute_out else None,
            'error': execute_err[:200] if execute_err and not execute_success else None
        }

    def run_tests(self, test_pattern: str = "test_*.frm", test_list: List[str] = None) -> List[Dict]:
        """
        Run tests based on pattern or explicit list
        
        Args:
            test_pattern: Glob pattern for test files
            test_list: Explicit list of test files
            
        Returns:
            List of test results
        """
        if test_list:
            test_files = test_list
        else:
            # Find all test files matching pattern
            test_files = sorted([
                os.path.join(self.test_dir, f) 
                for f in os.listdir(self.test_dir) 
                if f.startswith(test_pattern.replace('*.frm', '').replace('*', '')) 
                and f.endswith('.frm')
            ])
        
        print(f"Found {len(test_files)} test files")
        
        self.results = []
        for i, frm_file in enumerate(test_files, 1):
            if self.verbose:
                print(f"Testing {i}/{len(test_files)}: {os.path.basename(frm_file)}...", end=' ')
            
            result = self.test_file(frm_file)
            self.results.append(result)
            
            if self.verbose:
                if result['transpile'] and result['execute']:
                    print("✅ PASS")
                elif result['transpile']:
                    print("⚠️ Transpile OK, Execute FAIL")
                else:
                    print("❌ FAIL")
        
        return self.results

    def generate_matrix(self, version: str = "v0.31") -> str:
        """
        Generate a test matrix markdown report
        
        Args:
            version: Version string for the report
            
        Returns:
            Path to generated report
        """
        if not self.results:
            raise ValueError("No test results available. Run tests first.")
        
        # Calculate statistics
        total = len(self.results)
        transpile_success = sum(1 for r in self.results if r['transpile'])
        execute_success = sum(1 for r in self.results if r['execute'])
        complete_success = sum(1 for r in self.results if r['transpile'] and r['execute'])
        
        # Generate markdown report
        os.makedirs(self.output_dir, exist_ok=True)
        report_path = os.path.join(self.output_dir, f'test_matrix_{version}.md')
        
        with open(report_path, 'w') as f:
            f.write(f"# Frame {version} Test Matrix\n\n")
            f.write(f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M')}  \n")
            f.write(f"**Total Tests**: {total}  \n")
            f.write(f"**Current Branch**: {version}  \n\n")
            
            f.write("## Summary Statistics\n\n")
            f.write("| Metric | Count | Percentage |\n")
            f.write("|--------|-------|------------|\n")
            f.write(f"| **Total Tests** | {total} | 100% |\n")
            f.write(f"| **Transpilation Success** | {transpile_success} | {transpile_success/total*100:.1f}% |\n")
            f.write(f"| **Execution Success** | {execute_success} | {execute_success/total*100:.1f}% |\n")
            f.write(f"| **Complete Success** | {complete_success} | {complete_success/total*100:.1f}% |\n\n")
            
            # Version-specific features
            if version == "v0.31":
                f.write("## v0.31 Features\n\n")
                f.write("✅ **IMPORT STATEMENTS**: Native import support without backticks\n")
                f.write("✅ **SELF EXPRESSION**: Standalone self usage (e.g., `jsonpickle.encode(self)`)\n")
                f.write("✅ **STATIC METHOD VALIDATION**: Parse-time validation for @staticmethod\n")
                f.write("✅ **OPERATIONS DEFAULT**: Operations are instance methods by default\n\n")
            
            # Failed tests
            f.write("## Failed Tests\n\n")
            failed = [r for r in self.results if not r['execute']]
            if failed:
                f.write("| Test File | Transpile | Execute | Error |\n")
                f.write("|-----------|-----------|---------|-------|\n")
                for r in failed:
                    f.write(f"| {r['file']} | {'✅' if r['transpile'] else '❌'} | ❌ | {r.get('error', 'N/A')} |\n")
            else:
                f.write("🎉 **All tests passing!**\n")
            
            # Detailed results
            f.write("\n## Test Details\n\n")
            f.write("| Test File | Transpile | Execute | Status |\n")
            f.write("|-----------|-----------|---------|--------|\n")
            for r in self.results:
                status = "✅ PASS" if r['transpile'] and r['execute'] else "❌ FAIL"
                f.write(f"| {r['file']} | {'✅' if r['transpile'] else '❌'} | {'✅' if r['execute'] else '❌'} | {status} |\n")
        
        return report_path

    def save_json_results(self, version: str = "v0.31") -> str:
        """
        Save test results as JSON
        
        Args:
            version: Version string for the results
            
        Returns:
            Path to JSON file
        """
        if not self.results:
            raise ValueError("No test results available. Run tests first.")
        
        os.makedirs(self.output_dir, exist_ok=True)
        json_path = os.path.join(self.output_dir, f'test_results_{version}.json')
        
        total = len(self.results)
        transpile_success = sum(1 for r in self.results if r['transpile'])
        execute_success = sum(1 for r in self.results if r['execute'])
        complete_success = sum(1 for r in self.results if r['transpile'] and r['execute'])
        
        with open(json_path, 'w') as f:
            json.dump({
                'timestamp': datetime.now().isoformat(),
                'version': version,
                'summary': {
                    'total': total,
                    'transpile_success': transpile_success,
                    'execute_success': execute_success,
                    'complete_success': complete_success,
                    'success_rate': f"{complete_success/total*100:.1f}%"
                },
                'results': self.results
            }, f, indent=2)
        
        return json_path

def main():
    """Main entry point for command-line usage"""
    parser = argparse.ArgumentParser(
        description='Frame Test Runner - Comprehensive testing tool for Frame transpiler',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Run all tests and generate matrix
  %(prog)s --all --matrix
  
  # Run specific test pattern
  %(prog)s --pattern "test_import*.frm"
  
  # Run tests from config file
  %(prog)s --config configs/hsm_tests.json
  
  # Run with custom timeout
  %(prog)s --all --timeout 10
  
  # Verbose output
  %(prog)s --all --verbose
        """
    )
    
    parser.add_argument('--all', action='store_true', 
                       help='Run all test_*.frm files')
    parser.add_argument('--pattern', type=str,
                       help='Run tests matching pattern (e.g., "test_import*.frm")')
    parser.add_argument('--config', type=str,
                       help='Run tests from JSON config file')
    parser.add_argument('--matrix', action='store_true',
                       help='Generate test matrix report')
    parser.add_argument('--json', action='store_true',
                       help='Save results as JSON')
    parser.add_argument('--version', type=str, default='v0.31',
                       help='Version string for reports (default: v0.31)')
    parser.add_argument('--timeout', type=int, default=5,
                       help='Timeout in seconds for each test (default: 5)')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Verbose output')
    parser.add_argument('--framec', type=str,
                       help='Path to framec executable')
    parser.add_argument('--test-dir', type=str,
                       help='Directory containing test files')
    parser.add_argument('--output-dir', type=str,
                       help='Directory for output files')
    
    args = parser.parse_args()
    
    # Create runner
    runner = FrameTestRunner(
        framec_path=args.framec,
        test_dir=args.test_dir,
        output_dir=args.output_dir,
        timeout=args.timeout
    )
    runner.verbose = args.verbose
    
    # Determine what tests to run
    test_list = None
    test_pattern = "test_*.frm"
    
    if args.config:
        # Load test list from config
        with open(args.config) as f:
            config = json.load(f)
            test_list = [os.path.join(runner.test_dir, f) for f in config.get('tests', [])]
    elif args.pattern:
        test_pattern = args.pattern
    elif not args.all:
        print("Error: Specify --all, --pattern, or --config")
        return 1
    
    # Run tests
    results = runner.run_tests(test_pattern=test_pattern, test_list=test_list)
    
    # Generate reports
    if args.matrix:
        report_path = runner.generate_matrix(version=args.version)
        print(f"\nTest matrix saved to: {report_path}")
    
    if args.json:
        json_path = runner.save_json_results(version=args.version)
        print(f"JSON results saved to: {json_path}")
    
    # Print summary
    total = len(results)
    complete_success = sum(1 for r in results if r['transpile'] and r['execute'])
    
    print(f"\n=== SUMMARY ===")
    print(f"Total Tests: {total}")
    print(f"Passed: {complete_success}")
    print(f"Failed: {total - complete_success}")
    print(f"Success Rate: {complete_success/total*100:.1f}%")
    
    return 0 if complete_success == total else 1

if __name__ == "__main__":
    sys.exit(main())