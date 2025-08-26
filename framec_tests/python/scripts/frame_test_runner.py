#!/usr/bin/env python3
"""
Reusable Frame Test Runner
Can transpile and test any set of Frame files based on patterns or config files.
"""

import os
import sys
import json
import subprocess
import argparse
from pathlib import Path
from typing import List, Dict, Tuple

class FrameTestRunner:
    def __init__(self, framec_path: str = None):
        self.script_dir = Path(__file__).parent
        self.project_root = self.script_dir.parent.parent.parent
        self.src_dir = self.script_dir.parent / "src"
        self.generated_dir = self.script_dir.parent / "generated"
        self.framec = framec_path or str(self.project_root / "target" / "debug" / "framec")
        
    def transpile_file(self, frm_file: Path, force: bool = False) -> Tuple[bool, str]:
        """Transpile a single .frm file to Python."""
        py_file = frm_file.parent / f"{frm_file.stem}.py"
        
        # Skip if already exists and not forcing
        if py_file.exists() and not force:
            return True, f"Already exists: {py_file.name}"
        
        try:
            result = subprocess.run(
                [self.framec, "-l", "python_3", str(frm_file)],
                capture_output=True,
                text=True,
                timeout=10
            )
            
            if result.returncode == 0:
                # Save the output
                py_file.write_text(result.stdout)
                return True, f"Generated: {py_file.name}"
            else:
                # Check if it's stderr that we should ignore (debug output)
                if result.stdout:
                    # Got output despite error code, save it
                    py_file.write_text(result.stdout)
                    return True, f"Generated (with warnings): {py_file.name}"
                return False, result.stderr or "Unknown transpilation error"
                
        except subprocess.TimeoutExpired:
            return False, "Transpilation timeout"
        except Exception as e:
            return False, str(e)
    
    def run_python_file(self, py_file: Path) -> Tuple[bool, str, str]:
        """Run a generated Python file and capture output."""
        try:
            result = subprocess.run(
                [sys.executable, str(py_file)],
                capture_output=True,
                text=True,
                timeout=5
            )
            
            return result.returncode == 0, result.stdout, result.stderr
            
        except subprocess.TimeoutExpired:
            return False, "", "Execution timeout"
        except Exception as e:
            return False, "", str(e)
    
    def find_test_files(self, patterns: List[str]) -> List[Path]:
        """Find test files matching given patterns."""
        test_files = []
        for pattern in patterns:
            test_files.extend(self.src_dir.glob(pattern))
        return sorted(set(test_files))
    
    def load_config(self, config_file: Path) -> Dict:
        """Load test configuration from JSON file."""
        with open(config_file) as f:
            return json.load(f)
    
    def run_tests(self, patterns: List[str] = None, config_file: Path = None, 
                  transpile_only: bool = False, force_transpile: bool = False,
                  verbose: bool = False) -> int:
        """Main test runner."""
        
        # Get list of files to test
        if config_file and config_file.exists():
            config = self.load_config(config_file)
            patterns = config.get("patterns", patterns)
            transpile_only = config.get("transpile_only", transpile_only)
            
        if not patterns:
            patterns = ["test_*.frm"]
        
        test_files = self.find_test_files(patterns)
        
        if not test_files:
            print(f"No test files found matching patterns: {patterns}")
            return 1
        
        print(f"Found {len(test_files)} test files")
        print("=" * 60)
        
        transpile_passed = 0
        transpile_failed = 0
        run_passed = 0
        run_failed = 0
        
        for frm_file in test_files:
            test_name = frm_file.stem
            print(f"\n{test_name}")
            print("-" * 40)
            
            # Transpile
            success, message = self.transpile_file(frm_file, force_transpile)
            
            if verbose or not success:
                print(f"  Transpile: {message}")
            
            if success:
                transpile_passed += 1
                
                if not transpile_only:
                    # Run the generated Python
                    py_file = frm_file.parent / f"{test_name}.py"
                    if py_file.exists():
                        passed, stdout, stderr = self.run_python_file(py_file)
                        
                        if passed:
                            print(f"  ✅ PASSED")
                            run_passed += 1
                            if verbose and stdout:
                                print(f"  Output: {stdout[:200]}")
                        else:
                            print(f"  ❌ FAILED")
                            run_failed += 1
                            if stderr:
                                print(f"  Error: {stderr[:200]}")
            else:
                transpile_failed += 1
                print(f"  ❌ Transpilation failed: {message[:200]}")
        
        # Summary
        print("\n" + "=" * 60)
        print("SUMMARY")
        print("=" * 60)
        print(f"Transpilation: {transpile_passed} passed, {transpile_failed} failed")
        
        if not transpile_only:
            print(f"Execution: {run_passed} passed, {run_failed} failed")
            success_rate = run_passed / len(test_files) * 100 if test_files else 0
            print(f"Success rate: {success_rate:.1f}%")
        
        return 1 if (transpile_failed > 0 or run_failed > 0) else 0


def main():
    parser = argparse.ArgumentParser(description="Frame Test Runner")
    parser.add_argument("patterns", nargs="*", default=["test_*.frm"],
                       help="File patterns to test (e.g., 'test_multi*.frm')")
    parser.add_argument("-c", "--config", type=Path,
                       help="JSON config file with test settings")
    parser.add_argument("-t", "--transpile-only", action="store_true",
                       help="Only transpile, don't run tests")
    parser.add_argument("-f", "--force", action="store_true",
                       help="Force re-transpilation even if .py exists")
    parser.add_argument("-v", "--verbose", action="store_true",
                       help="Verbose output")
    parser.add_argument("--framec", help="Path to framec binary")
    
    args = parser.parse_args()
    
    runner = FrameTestRunner(args.framec)
    return runner.run_tests(
        patterns=args.patterns,
        config_file=args.config,
        transpile_only=args.transpile_only,
        force_transpile=args.force,
        verbose=args.verbose
    )


if __name__ == "__main__":
    sys.exit(main())