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
import re
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
        
    def is_negative_test(self, frm_file: str) -> bool:
        """
        Check if a test is expected to fail (negative test)
        
        Args:
            frm_file: Path to the .frm file
            
        Returns:
            True if the test is in negative_tests directory
        """
        return 'negative_tests' in frm_file
    
    def parse_test_expectations(self, frm_file: str) -> Dict:
        """
        Parse test expectations from structured comments in Frame file
        
        Args:
            frm_file: Path to the .frm file
            
        Returns:
            Dictionary with test expectations
        """
        expectations = {
            'expect': None,  # 'error' or 'warning'
            'error_message': None,
            'error_pattern': None,  # regex pattern for flexible matching
            'error_type': None,
            'error_line': None,
            'warning_message': None
        }
        
        try:
            with open(frm_file, 'r') as f:
                # Only check first 20 lines for expectations
                for i, line in enumerate(f):
                    if i > 20:  # Stop after 20 lines
                        break
                    
                    line = line.strip()
                    if not line.startswith('#'):
                        continue
                    
                    # Remove # and any leading space
                    line = line[1:].strip()
                    
                    # Parse @test-expect directive
                    if line.startswith('@test-expect:'):
                        expectations['expect'] = line.split(':', 1)[1].strip()
                    elif line.startswith('@error-message:'):
                        expectations['error_message'] = line.split(':', 1)[1].strip().strip('"')
                    elif line.startswith('@error-pattern:'):
                        expectations['error_pattern'] = line.split(':', 1)[1].strip().strip('"')
                    elif line.startswith('@error-patterns:'):
                        # Support multiple patterns in one line
                        patterns = line.split(':', 1)[1].strip().strip('"')
                        expectations['error_pattern'] = patterns
                    elif line.startswith('@error-type:'):
                        expectations['error_type'] = line.split(':', 1)[1].strip()
                    elif line.startswith('@error-line:'):
                        expectations['error_line'] = line.split(':', 1)[1].strip()
                    elif line.startswith('@warning-message:'):
                        expectations['warning_message'] = line.split(':', 1)[1].strip().strip('"')
        except:
            pass
        
        return expectations
    
    def validate_error_expectation(self, expected: Dict, actual_error: str) -> Tuple[bool, str]:
        """
        Validate if actual error matches expected error
        
        Args:
            expected: Dictionary with expected error details
            actual_error: Actual error message from transpiler
            
        Returns:
            Tuple of (matches, reason)
        """
        if not expected.get('expect'):
            # No expectations defined
            return True, "No expectations defined"
        
        if expected['expect'] == 'error':
            if not actual_error:
                return False, "Expected error but transpilation succeeded"
            
            # Check error message exact match
            if expected.get('error_message'):
                if expected['error_message'] not in actual_error:
                    return False, f"Error message mismatch. Expected: '{expected['error_message']}'"
            
            # Check error pattern regex match
            if expected.get('error_pattern'):
                # Support multiple patterns separated by |
                patterns = expected['error_pattern'].split('|') if '|' in expected['error_pattern'] else [expected['error_pattern']]
                
                matched = False
                for pattern_str in patterns:
                    pattern = re.compile(pattern_str.strip(), re.IGNORECASE | re.MULTILINE)
                    if pattern.search(actual_error):
                        matched = True
                        break
                
                if not matched:
                    return False, f"Error doesn't match pattern: '{expected['error_pattern']}'"
            
            # Check error type
            if expected.get('error_type'):
                if expected['error_type'] not in actual_error:
                    return False, f"Error type mismatch. Expected: '{expected['error_type']}'"
            
            # Check error line if specified
            if expected.get('error_line'):
                # Look for line number in error message
                line_pattern = re.compile(r'line (\d+)', re.IGNORECASE)
                match = line_pattern.search(actual_error)
                if match:
                    actual_line = int(match.group(1))
                    expected_line = expected['error_line']
                    
                    # Handle range like "5-6"
                    if '-' in str(expected_line):
                        start, end = map(int, expected_line.split('-'))
                        if not (start <= actual_line <= end):
                            return False, f"Error line mismatch. Expected: {expected_line}, Got: {actual_line}"
                    else:
                        if actual_line != int(expected_line):
                            return False, f"Error line mismatch. Expected: {expected_line}, Got: {actual_line}"
            
            return True, "Error matches expectations"
        
        elif expected['expect'] == 'warning':
            # For now, we'll just check if there's a warning in output
            if expected.get('warning_message'):
                if expected['warning_message'] not in actual_error:
                    return False, f"Warning message not found: '{expected['warning_message']}'"
            return True, "Warning matches expectations"
        
        return True, "Unknown expectation type"
    
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
        is_negative = self.is_negative_test(frm_file)
        
        # Parse test expectations from file
        expectations = self.parse_test_expectations(frm_file) if is_negative else {}
        
        # Transpile
        transpile_success, transpile_out, transpile_err = self.run_transpiler(frm_file)
        
        # For negative tests, validate against expectations
        if is_negative:
            if expectations.get('expect'):
                # Validate against structured expectations
                matches, reason = self.validate_error_expectation(expectations, transpile_err)
                
                return {
                    'file': os.path.basename(frm_file),
                    'transpile': False,  # Expected to fail
                    'execute': False,    # N/A for negative tests
                    'negative_test': True,
                    'expected_failure': True,
                    'expectation_match': matches,
                    'expectation_reason': reason,
                    'expectations': expectations,
                    'error': transpile_err[:500] if transpile_err else "No error output"
                }
            else:
                # No structured expectations, use old behavior
                if not transpile_success:
                    return {
                        'file': os.path.basename(frm_file),
                        'transpile': False,  # Expected to fail
                        'execute': False,    # N/A for negative tests
                        'negative_test': True,
                        'expected_failure': True,
                        'error': transpile_err[:200] if transpile_err else "Transpilation failed (expected)"
                    }
                else:
                    # Negative test passed transpilation unexpectedly
                    return {
                        'file': os.path.basename(frm_file),
                        'transpile': True,
                        'execute': False,
                        'negative_test': True,
                        'expected_failure': False,
                        'error': "Negative test unexpectedly passed transpilation"
                    }
        
        # Positive test - normal flow
        if not transpile_success:
            return {
                'file': os.path.basename(frm_file),
                'transpile': False,
                'execute': False,
                'negative_test': False,
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
            'negative_test': False,
            'output': execute_out[:500] if execute_out else None,
            'error': execute_err[:500] if execute_err and not execute_success else None,
            'runtime_error': execute_err if not execute_success else None  # Full error for debugging
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
            # Find all test files in new directory structure
            test_files = []
            
            # Use a set to avoid duplicates
            test_files_set = set()
            
            # Search in positive_tests (including multifile subdirectories)
            positive_dir = os.path.join(self.test_dir, 'positive_tests')
            if os.path.exists(positive_dir):
                for root, dirs, files in os.walk(positive_dir):
                    for f in files:
                        if f.endswith('.frm') and f.startswith('test_'):
                            full_path = os.path.join(root, f)
                            # Normalize path to avoid duplicates
                            test_files_set.add(os.path.normpath(full_path))
            
            # Search in negative_tests (including multifile subdirectories)
            negative_dir = os.path.join(self.test_dir, 'negative_tests')
            if os.path.exists(negative_dir):
                for root, dirs, files in os.walk(negative_dir):
                    for f in files:
                        if f.endswith('.frm') and f.startswith('test_'):
                            full_path = os.path.join(root, f)
                            # Normalize path to avoid duplicates
                            test_files_set.add(os.path.normpath(full_path))
            
            # If new structure doesn't exist, fall back to old structure
            if not test_files_set:
                for f in os.listdir(self.test_dir):
                    if f.startswith(test_pattern.replace('*.frm', '').replace('*', '')) and f.endswith('.frm'):
                        full_path = os.path.join(self.test_dir, f)
                        test_files_set.add(os.path.normpath(full_path))
            
            # Convert set to sorted list
            test_files = sorted(list(test_files_set))
        
        print(f"Found {len(test_files)} test files")
        
        self.results = []
        for i, frm_file in enumerate(test_files, 1):
            if self.verbose:
                print(f"Testing {i}/{len(test_files)}: {os.path.basename(frm_file)}...", end=' ')
            
            result = self.test_file(frm_file)
            self.results.append(result)
            
            if self.verbose:
                if result.get('negative_test', False):
                    # Negative test
                    if result.get('expectation_match') is not None:
                        # Has structured expectations
                        if result['expectation_match']:
                            print("✅ PASS (Expected Error Matched)")
                        else:
                            print(f"❌ FAIL (Negative Test: {result.get('expectation_reason', 'Expectation mismatch')})")
                    elif result.get('expected_failure', False):
                        print("✅ PASS (Expected Failure)")
                    else:
                        print("❌ FAIL (Negative Test: Should have failed)")
                elif result['transpile'] and result['execute']:
                    print("✅ PASS")
                elif result['transpile']:
                    # Show runtime error details
                    error_msg = result.get('error', 'Unknown error')
                    if error_msg and len(error_msg) > 100:
                        error_msg = error_msg[:100] + "..."
                    print(f"❌ FAIL (Runtime: {error_msg})")
                else:
                    print("❌ FAIL (Transpilation)")
        
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
        # Count positive tests that pass normally and negative tests that correctly match expectations
        complete_success = sum(1 for r in self.results if 
                              (r['transpile'] and r['execute']) or  # Normal passing tests
                              (r.get('negative_test', False) and r.get('expectation_match', False) == True)  # Negative tests with matched expectations
                              )
        
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
        # Count positive tests that pass normally and negative tests that correctly match expectations
        complete_success = sum(1 for r in self.results if 
                              (r['transpile'] and r['execute']) or  # Normal passing tests
                              (r.get('negative_test', False) and r.get('expectation_match', False) == True)  # Negative tests with matched expectations
                              )
        
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
    
    # Print detailed summary
    total = len(results)
    
    # Categorize results
    positive_pass = sum(1 for r in results if not r.get('negative_test', False) and r['transpile'] and r['execute'])
    positive_transpile_fail = sum(1 for r in results if not r.get('negative_test', False) and not r['transpile'])
    positive_runtime_fail = sum(1 for r in results if not r.get('negative_test', False) and r['transpile'] and not r['execute'])
    
    negative_pass = sum(1 for r in results if r.get('negative_test', False) and 
                       (r.get('expectation_match', False) == True or r.get('expected_failure', False)))
    negative_fail = sum(1 for r in results if r.get('negative_test', False) and 
                       not (r.get('expectation_match', False) == True or r.get('expected_failure', False)))
    
    complete_success = positive_pass + negative_pass
    
    print(f"\n=== SUMMARY ===")
    print(f"Total Tests: {total}")
    print(f"Passed: {complete_success} ({complete_success/total*100:.1f}%)")
    print(f"Failed: {total - complete_success} ({(total - complete_success)/total*100:.1f}%)")
    
    print(f"\n=== BREAKDOWN ===")
    print(f"Positive Tests:")
    print(f"  ✅ Passed: {positive_pass}")
    print(f"  ❌ Transpilation Failed: {positive_transpile_fail}")
    print(f"  ❌ Runtime Failed: {positive_runtime_fail}")
    
    print(f"\nNegative Tests:")
    print(f"  ✅ Passed (Expected Error): {negative_pass}")
    print(f"  ❌ Failed (Unexpected Result): {negative_fail}")
    
    # List runtime failures if any
    if positive_runtime_fail > 0:
        print(f"\n=== RUNTIME FAILURES ===")
        for r in results:
            if not r.get('negative_test', False) and r['transpile'] and not r['execute']:
                error = r.get('error', 'Unknown error')
                if len(error) > 150:
                    error = error[:150] + "..."
                print(f"  • {r['file']}: {error}")
    
    return 0 if complete_success == total else 1

if __name__ == "__main__":
    sys.exit(main())