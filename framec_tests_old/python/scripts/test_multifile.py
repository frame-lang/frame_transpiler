#!/usr/bin/env python3
"""
Automated test suite for Frame v0.57 multi-file module system.
Tests import resolution, dependency management, and incremental compilation.
"""

import os
import sys
import json
import subprocess
import time
import hashlib
from pathlib import Path

FRAMEC_PATH = "/Users/marktruluck/projects/frame_transpiler/target/release/framec"
TEST_DIR = "/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src"
TEMP_DIR = "/tmp/frame_multifile_tests"

class MultiFileTestSuite:
    def __init__(self):
        self.results = []
        self.passed = 0
        self.failed = 0
        
    def run_test(self, test_name, main_file, expected_output=None, should_fail=False):
        """Run a single multi-file test."""
        print(f"\n[TEST] {test_name}")
        print("-" * 60)
        
        try:
            # Compile with multi-file flag
            main_path = os.path.join(TEST_DIR, main_file)
            output_path = os.path.join(TEMP_DIR, f"{test_name}.py")
            
            cmd = [FRAMEC_PATH, "-m", main_path, "-l", "python_3"]
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if should_fail:
                if result.returncode != 0:
                    print(f"✅ {test_name}: Expected failure occurred")
                    print(f"   Error: {result.stderr.strip()}")
                    self.passed += 1
                    self.results.append((test_name, "PASS", "Expected compilation failure"))
                    return True
                else:
                    print(f"❌ {test_name}: Expected failure but compilation succeeded")
                    self.failed += 1
                    self.results.append((test_name, "FAIL", "Expected failure but succeeded"))
                    return False
                    
            if result.returncode != 0:
                print(f"❌ {test_name}: Compilation failed")
                print(f"   Error: {result.stderr}")
                self.failed += 1
                self.results.append((test_name, "FAIL", f"Compilation error: {result.stderr}"))
                return False
                
            # Write output to file
            with open(output_path, 'w') as f:
                f.write(result.stdout)
                
            # Execute if expected output provided
            if expected_output:
                exec_result = subprocess.run(
                    ["python3", output_path],
                    capture_output=True,
                    text=True,
                    timeout=5
                )
                
                actual_output = exec_result.stdout.strip()
                if actual_output == expected_output.strip():
                    print(f"✅ {test_name}: Output matches expected")
                    self.passed += 1
                    self.results.append((test_name, "PASS", "Output correct"))
                    return True
                else:
                    print(f"❌ {test_name}: Output mismatch")
                    print(f"   Expected: {expected_output}")
                    print(f"   Actual: {actual_output}")
                    self.failed += 1
                    self.results.append((test_name, "FAIL", "Output mismatch"))
                    return False
            else:
                print(f"✅ {test_name}: Compilation successful")
                self.passed += 1
                self.results.append((test_name, "PASS", "Compilation only"))
                return True
                
        except subprocess.TimeoutExpired:
            print(f"❌ {test_name}: Execution timeout")
            self.failed += 1
            self.results.append((test_name, "FAIL", "Timeout"))
            return False
        except Exception as e:
            print(f"❌ {test_name}: Unexpected error: {e}")
            self.failed += 1
            self.results.append((test_name, "FAIL", str(e)))
            return False
            
    def test_caching_performance(self):
        """Test incremental compilation caching."""
        print(f"\n[TEST] Caching Performance")
        print("-" * 60)
        
        main_file = "test_multifile_performance.frm"
        main_path = os.path.join(TEST_DIR, main_file)
        
        # First compilation (cold cache)
        start = time.time()
        cmd = [FRAMEC_PATH, "-m", main_path, "-l", "python_3"]
        subprocess.run(cmd, capture_output=True, text=True)
        cold_time = time.time() - start
        
        # Second compilation (warm cache)
        start = time.time()
        subprocess.run(cmd, capture_output=True, text=True)
        warm_time = time.time() - start
        
        improvement = ((cold_time - warm_time) / cold_time) * 100
        
        print(f"Cold cache: {cold_time:.3f}s")
        print(f"Warm cache: {warm_time:.3f}s")
        print(f"Improvement: {improvement:.1f}%")
        
        if warm_time < cold_time:
            print(f"✅ Caching Performance: {improvement:.1f}% faster")
            self.passed += 1
            self.results.append(("Caching", "PASS", f"{improvement:.1f}% improvement"))
        else:
            print(f"❌ Caching Performance: No improvement")
            self.failed += 1
            self.results.append(("Caching", "FAIL", "No cache improvement"))
            
    def run_all_tests(self):
        """Run all multi-file tests."""
        # Create temp directory
        os.makedirs(TEMP_DIR, exist_ok=True)
        
        print("=" * 70)
        print("Frame v0.57 Multi-File Module System Test Suite")
        print("=" * 70)
        
        # Basic import test
        self.run_test(
            "Basic Import",
            "test_multifile_main.frm",
            expected_output="Running tests...\nUtils.helper() returned: 42\nCalculator.add(5, 3) = 8\nCalculator.multiply(4, 5) = 20\nmultiply_and_add result: 140\nAll tests passed!"
        )
        
        # Complex compilation
        self.run_test(
            "Complex Multi-File",
            "test_multifile_complex.frm",
            expected_output="Complex multi-file test\nAdder result: 8\nMultiplier result: 20\nBoth results: 28"
        )
        
        # Large compilation
        self.run_test(
            "Large Compilation",
            "test_multifile_large.frm",
            expected_output="Starting large compilation benchmark...\nProcess all result: 457900\nComplex calculation count: 20\nBenchmark complete!"
        )
        
        # Invalid import (should fail)
        self.run_test(
            "Invalid Import Detection",
            "test_symbols_invalid.frm",
            should_fail=True
        )
        
        # Circular dependency (should fail)
        self.run_test(
            "Circular Dependency Detection",
            "test_circular_main.frm",
            should_fail=True
        )
        
        # Performance benchmark
        self.run_test(
            "Performance Benchmark",
            "test_multifile_performance.frm",
            expected_output="Starting performance test...\nResult: 247545\nPerformance test complete!"
        )
        
        # Caching performance
        self.test_caching_performance()
        
        # Print summary
        print("\n" + "=" * 70)
        print("TEST SUMMARY")
        print("=" * 70)
        print(f"Total tests: {self.passed + self.failed}")
        print(f"Passed: {self.passed}")
        print(f"Failed: {self.failed}")
        print(f"Success rate: {(self.passed / (self.passed + self.failed) * 100):.1f}%")
        
        # Print results table
        print("\nDetailed Results:")
        print("-" * 70)
        for name, status, note in self.results:
            status_symbol = "✅" if status == "PASS" else "❌"
            print(f"{status_symbol} {name:30} {note}")
            
        return self.failed == 0

if __name__ == "__main__":
    suite = MultiFileTestSuite()
    success = suite.run_all_tests()
    sys.exit(0 if success else 1)