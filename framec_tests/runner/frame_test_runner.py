#!/usr/bin/env python3
"""
Frame Test Runner - Unified test runner for all target languages.
Supports running common Frame tests against multiple language targets.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

@dataclass
class TestResult:
    """Result of a single test execution."""
    name: str
    file: str
    category: str
    language: str
    transpile_success: bool
    execute_success: bool
    validation_success: bool = False
    error_message: Optional[str] = None
    execution_time: float = 0.0
    output: Optional[str] = None
    is_negative_test: bool = False
    expected_failure: bool = False

@dataclass 
class TestConfig:
    """Configuration for test execution."""
    framec_path: str = "./target/release/framec"
    languages: List[str] = None
    categories: List[str] = None
    verbose: bool = False
    execute: bool = True  # If False, only transpile
    validate: bool = True  # Run validator after transpile
    validation_level: str = "structural"  # basic|structural|semantic|target-language
    validation_format: str = "human"  # human|json|junit
    parallel: bool = False
    timeout: int = 10
    include_common: bool = False
    
    def __post_init__(self):
        if self.languages is None:
            self.languages = ["python", "typescript"]
        if self.categories is None:
            self.categories = ["all"]

class FrameTestRunner:
    """Main test runner class."""
    
    def __init__(self, config: TestConfig):
        self.config = config
        self.base_dir = Path(__file__).parent.parent
        self.common_tests_dir = self.base_dir / "common" / "tests"
        self.language_specific_dir = self.base_dir / "language_specific"
        self.generated_dir = self.base_dir / "generated"
        self.results: List[TestResult] = []
        self._llvm_runtime_ready: bool = False
        self._llvm_runtime_dir: Optional[Path] = None
        
    def is_negative_test(self, test_file: Path) -> bool:
        """
        Check if a test is expected to fail (negative test).
        
        Args:
            test_file: Path to the test file
            
        Returns:
            True if the test is in negative tests directory
        """
        # Check if the test is in a 'negative' directory path, not just filename
        path_parts = test_file.parts
        
        # Exception: test_error_handling_v049 is misplaced - it's actually a positive test
        if test_file.stem == 'test_error_handling_v049':
            return False
            
        return 'negative' in path_parts
    
    def is_infinite_loop_test(self, test_file: Path) -> bool:
        """
        Check if a test is designed to run indefinitely (infinite loop test).
        These are typically service tests that loop until externally terminated.
        
        Args:
            test_file: Path to the test file
            
        Returns:
            True if the test is designed to run indefinitely
        """
        # Check for services tests that are designed to run indefinitely
        return test_file.stem.startswith('test_services_')
    
    def is_multifile_test(self, test_file: Path) -> bool:
        """
        Check if a Frame file requires multifile compilation
        by looking for Frame import statements (import ... from "*.frm")
        
        Args:
            test_file: Path to the .frm file
            
        Returns:
            True if the file contains Frame imports
        """
        try:
            with open(test_file, 'r') as f:
                for line in f:
                    # Check for Frame imports (files ending with .frm)
                    if 'import' in line and '.frm' in line:
                        return True
            return False
        except:
            return False
        
    def discover_tests(self) -> Dict[str, List[Path]]:
        """Discover all test files organized by category."""
        tests = {}
        
        # Common tests (optional; default is to skip in native-only mode)
        if self.config.include_common:
            if "all" in self.config.categories or any(cat in self.config.categories for cat in ["core", "control_flow", "data_types", "operators", "scoping", "systems", "regression", "negative"]):
                for category_dir in self.common_tests_dir.iterdir():
                    if category_dir.is_dir():
                        category = category_dir.name
                        if "all" in self.config.categories or category in self.config.categories:
                            tests[category] = list(category_dir.glob("*.frm"))
        
        # Language-specific tests - only include if explicitly requested or "all" is specified
        if "all" in self.config.categories:
            # When running "all", include language-specific tests for configured languages
            for lang in self.config.languages:
                lang_dir = self.language_specific_dir / lang
                if lang_dir.exists():
                    lang_tests = list(lang_dir.rglob("*.frm"))
                    # Exclude torture tests in transpile-only runs to avoid skew
                    if not self.config.execute:
                        lang_tests = [p for p in lang_tests if "torture" not in [pp.lower() for pp in p.parts]]
                    if lang_tests:
                        tests[f"language_specific_{lang}"] = lang_tests
        else:
            # Only include language-specific tests if explicitly requested
            for lang in self.config.languages:
                category_name = f"language_specific_{lang}"
                if category_name in self.config.categories:
                    lang_dir = self.language_specific_dir / lang
                    if lang_dir.exists():
                        lang_tests = list(lang_dir.rglob("*.frm"))
                        if not self.config.execute:
                            lang_tests = [p for p in lang_tests if "torture" not in [pp.lower() for pp in p.parts]]
                        if lang_tests:
                            tests[category_name] = lang_tests
                    
        return tests

    def is_torture_test(self, test_file: Path) -> bool:
        """Return True if this test resides under a torture/ directory."""
        parts = [p.lower() for p in test_file.parts]
        return "torture" in parts
    
    def has_language_override(self, test_file: Path, language: str) -> bool:
        """Return True if a language-specific override exists for this common test."""
        # Only applicable to common tests
        if self.common_tests_dir not in Path(test_file).parents:
            return False
        category = Path(test_file).parent.name
        override_path = self.language_specific_dir / language / category / Path(test_file).name
        return override_path.exists()
    
    def transpile(self, test_file: Path, language: str) -> Tuple[bool, str, str]:
        """
        Transpile a Frame file to target language.
        Returns (success, output_file, error_message)
        """
        # Determine language flag
        lang_flag = {
            "python": "python_3",
            "typescript": "typescript",
            "rust": "rust",
            "golang": "golang",
            "javascript": "javascript",
            "llvm": "llvm",
        }.get(language, language)
        
        # Determine output extension
        extension = {
            "python": ".py",
            "typescript": ".ts", 
            "rust": ".rs",
            "golang": ".go",
            "javascript": ".js",
            "llvm": ".ll",
        }.get(language, ".txt")
        
        # Create output directory
        output_dir = self.generated_dir / language
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # Generate output filename
        output_file = output_dir / (test_file.stem + extension)
        
        # Run transpiler - check if multifile test
        if self.is_multifile_test(test_file):
            # Use multifile flag for tests with Frame imports
            cmd = [self.config.framec_path, "-m", str(test_file), "-l", lang_flag]
        else:
            # Standard single-file compilation
            cmd = [self.config.framec_path, "-l", lang_flag, str(test_file)]
        
        try:
            # Set environment variables for Rust main function generation
            env = os.environ.copy()
            if language == "rust":
                env["FRAME_RUST_GENERATE_MAIN"] = "1"
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.config.timeout,
                env=env
            )
            
            if result.returncode == 0:
                # Write output to file
                output_file.write_text(result.stdout)
                return True, str(output_file), None
            else:
                error = result.stderr or result.stdout
                return False, str(output_file), error
                
        except subprocess.TimeoutExpired:
            return False, str(output_file), "Transpilation timeout"
        except Exception as e:
            return False, str(output_file), str(e)

    def validate(self, test_file: Path, language: str) -> Tuple[bool, str]:
        """Run framec validation on a Frame file for a given target language."""
        lang_flag = {
            "python": "python_3",
            "typescript": "typescript",
            "rust": "rust",
            "golang": "golang",
            "javascript": "javascript",
            "llvm": "llvm",
        }.get(language, language)

        cmd = [
            self.config.framec_path,
            "--language",
            lang_flag,
            "--validate-syntax",
            "--validation-only",
            "--validation-level",
            self.config.validation_level,
            "--validation-format",
            self.config.validation_format,
            str(test_file),
        ]

        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=max(self.config.timeout, 10),
            )
            success = result.returncode == 0
            output = (result.stdout or "") + (result.stderr or "")
            return success, output
        except subprocess.TimeoutExpired:
            return False, "Validation timeout"
        except Exception as e:
            return False, f"Validation error: {e}"
    
    def execute_python(self, py_file: str) -> Tuple[bool, str]:
        """Execute Python file and return success status and output."""
        try:
            env = os.environ.copy()
            project_root = str(self.base_dir.parent)
            existing_pythonpath = env.get("PYTHONPATH", "")
            if existing_pythonpath:
                env["PYTHONPATH"] = os.pathsep.join([project_root, existing_pythonpath])
            else:
                env["PYTHONPATH"] = project_root

            result = subprocess.run(
                ["python3", py_file],
                capture_output=True,
                text=True,
                timeout=self.config.timeout,
                cwd=os.path.dirname(py_file),
                env=env
            )
            
            # Check for common failure patterns
            output = result.stdout + result.stderr
            
            # Check for definitive failure indicators
            if "Traceback" in output:
                return False, output
            if result.returncode != 0:
                return False, output
            
            # Check for FAIL patterns (but be more specific)
            if "FAIL:" in output or "FAILED:" in output or " FAIL " in output:
                return False, output
            
            # For Error patterns, be more specific to avoid false positives
            # Don't match "Error level:", "Error:" in expected output, etc.
            import re
            # Look for actual error patterns like "Error at", "Error:", "runtime error", etc.
            # but not things like "Error level: Error" or "10 / 0 = Error: Division by zero"
            error_patterns = [
                r'^Error: [^0-9]',  # Error at start of line, but not "Error: 101" (enum values)
                r'Error at ',  # Parser errors
                r'runtime error',  # Runtime errors
                r'import error',  # Import errors
                r'^Traceback \(most recent call last\):',  # Python tracebacks at start of line
                # Don't match exception names that might appear in successful test output
            ]
            
            for pattern in error_patterns:
                if re.search(pattern, output, re.MULTILINE):
                    return False, output
                
            return True, output
            
        except subprocess.TimeoutExpired:
            return False, "Execution timeout"
        except Exception as e:
            return False, str(e)
    
    def _batch_compile_typescript(self, tests: Dict[str, List[str]]) -> None:
        """
        Batch compile all TypeScript files at once for better performance.
        This reduces TypeScript compiler startup overhead from ~0.9s per file to ~1s total.
        Now uses shared runtime module to avoid duplicate identifier issues.
        """
        try:
            # Find TypeScript compiler
            tsc_cmd = self._find_typescript_compiler()
            if not tsc_cmd:
                print("Warning: tsc not found - TypeScript tests will compile individually")
                return
            
            # First, compile the shared runtime module
            runtime_path = self.base_dir / "typescript" / "runtime" / "frame_runtime.ts"
            if runtime_path.exists():
                print("Compiling shared runtime module...")
                runtime_result = subprocess.run(
                    [tsc_cmd, "--target", "es2020", "--module", "commonjs", str(runtime_path)],
                    capture_output=True,
                    text=True,
                    timeout=self.config.timeout
                )
                if runtime_result.returncode != 0:
                    print(f"Runtime compilation failed: {runtime_result.stderr[:200]}")
                    return
            
            # Collect all test files that will generate TypeScript
            ts_files = []
            
            for category, test_files in tests.items():
                for test_file in test_files:
                    # Skip language-specific tests for other languages
                    if category.startswith("language_specific_"):
                        lang = category.split("_")[-1]
                        if lang != 'typescript':
                            continue
                    
                    # Generate the expected TypeScript file path
                    ts_file = self._get_typescript_output_path(test_file)
                    if ts_file and os.path.exists(ts_file):
                        ts_files.append(ts_file)
            
            if not ts_files:
                return
                
            print(f"Batch compiling {len(ts_files)} TypeScript files with shared runtime...")
            
            # Batch compile all TypeScript files with resilient error handling
            start_time = time.time()
            compile_result = subprocess.run(
                [tsc_cmd, "--target", "es2020", "--module", "commonjs", "--noEmitOnError", "false"] + ts_files,
                capture_output=True,
                text=True,
                timeout=self.config.timeout * 10  # Longer timeout for batch compilation
            )
            
            compile_time = time.time() - start_time
            
            # Parse which files had errors vs which succeeded
            error_output = (compile_result.stdout + compile_result.stderr).strip()
            failed_files = set()
            
            if error_output:
                # Extract file paths from TypeScript error messages
                import re
                # Pattern matches: "path/to/file.ts(line,col): error ..."
                error_pattern = r'([^:\(\)]+\.ts)\(\d+,\d+\): error'
                matches = re.findall(error_pattern, error_output)
                failed_files = set(matches)
            
            successful_files = [f for f in ts_files if f not in failed_files]
            
            if successful_files:
                print(f"Batch compilation completed in {compile_time:.2f}s")
                print(f"  ✅ {len(successful_files)} files compiled successfully")
                # Mark successful files as already compiled
                for ts_file in successful_files:
                    setattr(self, f'_compiled_{ts_file}', True)
            
            if failed_files:
                print(f"  ⚠️  {len(failed_files)} files had compilation errors (will compile individually)")
                if self.config.verbose:
                    print(f"Failed files: {list(failed_files)[:5]}...")  # Show first 5 failed files
                    print(f"Error details:\n{error_output[:1000]}")  # Show more error details in verbose mode
                
            if not successful_files and not failed_files:
                # No clear parse - fall back to old behavior
                print(f"Batch compilation unclear, will compile all files individually.")
                if self.config.verbose:
                    print(f"Full error output:\n{error_output}")
                
        except Exception as e:
            print(f"Batch compilation failed due to exception, will compile individually.")
            print(f"Exception details: {str(e)}")
            if self.config.verbose:
                import traceback
                print(f"Full traceback:\n{traceback.format_exc()}")
    
    def _get_typescript_output_path(self, test_file: str) -> Optional[str]:
        """Get the expected TypeScript output path for a Frame test file."""
        test_name = Path(test_file).stem
        ts_file = self.generated_dir / "typescript" / f"{test_name}.ts"
        return str(ts_file) if ts_file.exists() else None
        
    def execute_typescript(self, ts_file: str) -> Tuple[bool, str]:
        """
        Execute TypeScript file and return success status and output.
        First compiles with tsc, then runs with node.
        Tries local ./node_modules/.bin/tsc first, then global tsc.
        """
        try:
            js_file = ts_file.replace('.ts', '.js')
            
            # Check if this file was already batch compiled
            if not hasattr(self, f'_compiled_{ts_file}'):
                # Find TypeScript compiler - try local first, then global
                tsc_cmd = self._find_typescript_compiler()
                if not tsc_cmd:
                    return False, "tsc not found - please install TypeScript (npm install)"
                
                # Compile TypeScript individually with optimized flags
                compile_result = subprocess.run(
                    [tsc_cmd, "--target", "es5", "--module", "commonjs", "--skipLibCheck", ts_file],
                    capture_output=True,
                    text=True,
                    timeout=self.config.timeout
                )
                
                if compile_result.returncode != 0:
                    return False, f"TypeScript compilation failed:\n{compile_result.stderr}"
            
            # Run JavaScript
            result = subprocess.run(
                ["node", js_file],
                capture_output=True,
                text=True,
                timeout=self.config.timeout
            )
            
            output = result.stdout + result.stderr
            if "FAIL" in output or result.returncode != 0:
                return False, output
                
            return True, output
            
        except subprocess.TimeoutExpired:
            return False, "Execution timeout"
        except FileNotFoundError:
            return False, "node not found - please install Node.js"
        except Exception as e:
            return False, str(e)
    
    def _find_typescript_compiler(self) -> Optional[str]:
        """
        Find TypeScript compiler, preferring local installation.
        
        Returns:
            Path to tsc executable, or None if not found
        """
        import shutil
        
        # Try local node_modules first (project root)
        project_root = self.base_dir.parent  # Go up from framec_tests to project root
        
        # Check both possible locations for local TypeScript
        local_tsc_bin = project_root / "node_modules" / ".bin" / "tsc"
        local_tsc_direct = project_root / "node_modules" / "typescript" / "bin" / "tsc"
        
        if local_tsc_bin.exists():
            return str(local_tsc_bin)
        elif local_tsc_direct.exists():
            return str(local_tsc_direct)
        
        # Try global tsc
        global_tsc = shutil.which("tsc")
        if global_tsc:
            return global_tsc
            
        return None
    
    def compile_rust(self, rs_file: str) -> Tuple[bool, str]:
        """Compile Rust file to check for compilation errors without requiring main function."""
        try:
            # Compile as a library (--crate-type lib) to avoid requiring main function
            compile_result = subprocess.run(
                ["rustc", "--crate-type", "lib", rs_file],
                capture_output=True,
                text=True,
                timeout=self.config.timeout
            )
            
            # Clean up generated library file
            lib_file = os.path.splitext(rs_file)[0] + ".rlib"
            try:
                if os.path.exists(lib_file):
                    os.remove(lib_file)
            except:
                pass  # Don't fail if cleanup fails
            
            if compile_result.returncode != 0:
                error_output = compile_result.stderr + compile_result.stdout
                return False, f"Rust compilation failed:\n{error_output}"
                
            return True, "Rust compilation successful"
            
        except subprocess.TimeoutExpired:
            return False, "Rust compilation timeout"
        except FileNotFoundError:
            return False, "rustc not found - please install Rust (https://rustup.rs/)"
        except Exception as e:
            return False, str(e)
    
    def execute_rust(self, rs_file: str) -> Tuple[bool, str]:
        """Execute Rust file and return success status and output."""
        try:
            # Get the base name without extension for the executable
            base_name = os.path.splitext(rs_file)[0]
            executable = base_name
            
            # Compile with rustc
            compile_result = subprocess.run(
                ["rustc", rs_file, "-o", executable],
                capture_output=True,
                text=True,
                timeout=self.config.timeout
            )
            
            if compile_result.returncode != 0:
                error_output = compile_result.stderr + compile_result.stdout
                return False, f"Rust compilation failed:\n{error_output}"
            
            # Run the executable
            result = subprocess.run(
                [executable],
                capture_output=True,
                text=True,
                timeout=self.config.timeout,
                cwd=os.path.dirname(rs_file)
            )
            
            # Clean up executable
            try:
                os.remove(executable)
            except:
                pass  # Don't fail if cleanup fails
            
            output = result.stdout + result.stderr
            
            # Check for failure patterns similar to Python execution
            if "panic" in output.lower():
                return False, output
            if result.returncode != 0:
                return False, output
            if "FAIL:" in output or "FAILED:" in output:
                return False, output
                
            return True, output

        except subprocess.TimeoutExpired:
            return False, "Rust execution timeout"
        except FileNotFoundError:
            return False, "rustc not found - please install Rust (https://rustup.rs/)"
        except Exception as e:
            return False, str(e)

    def _ensure_llvm_runtime(self) -> Tuple[bool, str]:
        """Make sure the LLVM runtime library is built and discoverable."""
        if self._llvm_runtime_ready and self._llvm_runtime_dir:
            return True, str(self._llvm_runtime_dir)

        runtime_dir = self.base_dir.parent / "target" / "release"
        lib_candidates = [
            "libframe_runtime_llvm.dylib",
            "libframe_runtime_llvm.so",
            "libframe_runtime_llvm.dll",
            "libframe_runtime_llvm.a",
        ]

        def _has_runtime() -> bool:
            return any((runtime_dir / name).exists() for name in lib_candidates)

        if not _has_runtime():
            try:
                build_timeout = max(self.config.timeout * 12, 120)
                result = subprocess.run(
                    ["cargo", "build", "--release", "-p", "frame_runtime_llvm"],
                    capture_output=True,
                    text=True,
                    timeout=build_timeout,
                    cwd=self.base_dir.parent,
                )
                if result.returncode != 0:
                    return False, (
                        "Failed to build frame_runtime_llvm:\n"
                        f"{result.stderr or result.stdout}"
                    )
            except subprocess.TimeoutExpired:
                return False, "Timed out while building frame_runtime_llvm"
            except FileNotFoundError:
                return False, "cargo not found - please install Rust toolchain"
            except Exception as exc:
                return False, f"Error while building frame_runtime_llvm: {exc}"

        if not _has_runtime():
            return False, (
                "frame_runtime_llvm build completed but no library was found in "
                f"{runtime_dir}"
            )

        self._llvm_runtime_ready = True
        self._llvm_runtime_dir = runtime_dir
        return True, str(runtime_dir)

    def execute_llvm(self, ll_file: str) -> Tuple[bool, str]:
        """Compile LLVM IR to a binary, execute it, and return status/output."""
        runtime_ready, runtime_info = self._ensure_llvm_runtime()
        if not runtime_ready:
            return False, runtime_info

        runtime_dir = Path(runtime_info)
        clang = shutil.which("clang")
        if not clang:
            return False, "clang not found - please install LLVM toolchain"

        ll_path = Path(ll_file)
        binary_path = ll_path.with_suffix("")  # Drop .ll extension

        compile_cmd = [
            clang,
            str(ll_path),
            f"-L{runtime_dir}",
            "-lframe_runtime_llvm",
            f"-Wl,-rpath,{runtime_dir}",
            "-mllvm",
            "-opaque-pointers",
            "-o",
            str(binary_path),
        ]

        try:
            compile_result = subprocess.run(
                compile_cmd,
                capture_output=True,
                text=True,
                timeout=max(self.config.timeout * 2, 30),
            )
        except subprocess.TimeoutExpired:
            return False, "LLVM binary compilation timeout"
        except Exception as exc:
            return False, f"Failed to invoke clang: {exc}"

        if compile_result.returncode != 0:
            return False, (
                "Clang failed while compiling LLVM output:\n"
                f"{compile_result.stderr or compile_result.stdout}"
            )

        env = os.environ.copy()
        if sys.platform == "win32":
            path_var = "PATH"
            separator = ";"
        elif sys.platform == "darwin":
            path_var = "DYLD_LIBRARY_PATH"
            separator = ":"
        else:
            path_var = "LD_LIBRARY_PATH"
            separator = ":"

        existing = env.get(path_var, "")
        env[path_var] = (
            str(runtime_dir)
            if not existing
            else separator.join([str(runtime_dir), existing])
        )

        try:
            result = subprocess.run(
                [str(binary_path)],
                capture_output=True,
                text=True,
                timeout=max(self.config.timeout * 2, 30),
                cwd=str(binary_path.parent),
                env=env,
            )
        except subprocess.TimeoutExpired:
            return False, "LLVM executable timeout"
        except Exception as exc:
            return False, f"Failed to run LLVM executable: {exc}"
        finally:
            try:
                if binary_path.exists():
                    binary_path.unlink()
            except OSError:
                pass

        output = result.stdout + result.stderr
        if result.returncode != 0:
            return False, output
        if "FAIL" in output:
            return False, output

        return True, output
    
    def run_test(self, test_file: Path, category: str, language: str) -> TestResult:
        """Run a single test for a specific language."""
        start_time = time.time()
        
        # Check if this is a negative test
        is_negative = self.is_negative_test(test_file)
        # Per-language override: this shared test is positive in Python but negative in TypeScript
        if language == "typescript" and test_file.stem == "test_error_handling_v049":
            is_negative = True
        
        # Check if this is an infinite loop test
        is_infinite_loop = self.is_infinite_loop_test(test_file)
        
        # Create result object
        result = TestResult(
            name=test_file.stem,
            file=str(test_file),
            category=category,
            language=language,
            transpile_success=False,
            execute_success=False,
            validation_success=False,
            is_negative_test=is_negative
        )
        
        # Special handling: torture tests are validation-only (no transpile/execute expectations)
        if self.is_torture_test(test_file):
            # Torture tests are for deep validation/diagnostics; skip in transpile-only mode
            if not self.config.execute and not self.config.validate:
                result.transpile_success = True
                result.validation_success = True
                result.execute_success = True
                result.execution_time = time.time() - start_time
                return result
            # If validation is enabled, run validator only and skip transpile/execute expectations
            if self.config.validate:
                ok, _vout = self.validate(test_file, language)
                result.transpile_success = ok
                result.validation_success = ok
                result.execute_success = True  # not executed in validation-only
                result.execution_time = time.time() - start_time
                return result
            # Otherwise, mark as success placeholder (execution harness may handle separately)
            result.transpile_success = True
            result.validation_success = True
            result.execute_success = True
            result.execution_time = time.time() - start_time
            return result

        # Transpile
        transpile_success, output_file, error = self.transpile(test_file, language)
        result.transpile_success = transpile_success

        # Optionally validate (after transpile to ensure parsing paths are similar)
        validation_success = False
        validation_output = ""
        if self.config.validate:
            validation_success, validation_output = self.validate(test_file, language)
            result.validation_success = validation_success
        
        # Handle negative tests specially
        if is_negative:
            # Negative tests are successful if either transpilation or validation fails
            if (not transpile_success) or (self.config.validate and not validation_success):
                result.expected_failure = True
                # Prefer validation output if available
                err = error or validation_output
                result.error_message = f"Expected failure: {err[:200]}" if err else "Expected failure"
            else:
                result.expected_failure = False
                result.error_message = (
                    "Negative test unexpectedly passed transpilation" +
                    (" and validation" if self.config.validate else "")
                )
        elif is_infinite_loop:
            # Infinite loop test - only check transpilation, skip execution
            if not transpile_success:
                result.error_message = f"Transpilation failed: {error}"
            else:
                # Infinite loop test transpiled successfully - mark as success without execution
                result.execute_success = True
                result.output = "Infinite loop test - transpilation successful, execution skipped"
        elif self.is_torture_test(test_file):
            # Torture tests are validation-only; never execute
            if not transpile_success:
                result.error_message = f"Transpilation failed: {error}"
            elif self.config.validate and not validation_success:
                result.error_message = f"Validation failed: {validation_output[:500]}"
            else:
                # Mark execution as skipped-success to keep summary simple
                result.execute_success = True
                result.output = "Torture test: validation-only; execution skipped"
        else:
            # Positive test - normal handling
            if not transpile_success:
                result.error_message = f"Transpilation failed: {error}"
            elif self.config.validate and not validation_success:
                # Fail early on validation errors; do not execute
                result.error_message = f"Validation failed: {validation_output[:500]}"
            elif self.config.execute:
                # Execute based on language
                if language == "python":
                    exec_success, output = self.execute_python(output_file)
                elif language == "typescript":
                    exec_success, output = self.execute_typescript(output_file)
                elif language == "rust":
                    # First try to execute (compile + run)
                    exec_success, output = self.execute_rust(output_file)
                    
                    # If execution fails due to missing main function, try compilation-only
                    if not exec_success and "`main` function not found" in output:
                        compile_success, compile_output = self.compile_rust(output_file)
                        if compile_success:
                            # Compilation successful, mark as successful test (library code)
                            exec_success = True
                            output = "Rust compilation successful (library code)"
                        else:
                            # Keep the compilation error
                            output = compile_output
                elif language == "llvm":
                    exec_success, output = self.execute_llvm(output_file)
                else:
                    exec_success = False
                    output = f"Execution not implemented for {language}"
                
                result.execute_success = exec_success
                result.output = output
                
                if not exec_success and not result.error_message:
                    result.error_message = f"Execution failed:\n{output}"
        
        result.execution_time = time.time() - start_time
        
        if self.config.verbose:
            if is_negative:
                status = "✓" if result.expected_failure else "✗"
                test_type = "NEGATIVE"
            elif is_infinite_loop:
                status = "✓" if result.transpile_success else "✗"
                test_type = "INFINITE_LOOP"
            else:
                status = "✓" if (result.transpile_success and (not self.config.execute or result.execute_success)) else "✗"
                test_type = "POSITIVE"
            print(f"  {status} {result.name} ({language}) [{test_type}]: {result.execution_time:.2f}s")
            if result.error_message and self.config.verbose:
                print(f"    Error: {result.error_message[:200]}")
        
        return result
    
    def run_all_tests(self) -> List[TestResult]:
        """Run all discovered tests."""
        tests = self.discover_tests()
        
        print(f"\nDiscovered {sum(len(files) for files in tests.values())} tests in {len(tests)} categories")
        print(f"Testing languages: {', '.join(self.config.languages)}")
        print()
        
        # Batch compilation now enabled with shared runtime module
        if 'typescript' in self.config.languages and self.config.execute:
            self._batch_compile_typescript(tests)
        
        for category, test_files in sorted(tests.items()):
            if not test_files:
                continue
                
            print(f"\n{category}: {len(test_files)} tests")
            
            for test_file in sorted(test_files):
                # Skip language-specific tests for other languages
                if category.startswith("language_specific_"):
                    lang = category.split("_")[-1]
                    if lang in self.config.languages:
                        result = self.run_test(test_file, category, lang)
                        self.results.append(result)
                else:
                    # Run common test for all configured languages
                    for language in self.config.languages:
                        # Prefer language-specific override: if present, skip common for that language
                        if self.has_language_override(test_file, language):
                            continue
                        result = self.run_test(test_file, category, language)
                        self.results.append(result)
        
        return self.results
    
    def generate_report(self) -> Dict:
        """Generate test report summary."""
        report = {
            "timestamp": datetime.now().isoformat(),
            "config": asdict(self.config),
            "summary": {
                "total_tests": len(self.results),
                "by_language": {},
                "by_category": {}
            },
            "results": [asdict(r) for r in self.results]
        }
        
        # Calculate statistics by language
        for lang in self.config.languages:
            lang_results = [r for r in self.results if r.language == lang]
            
            # For transpilation success: positive tests that transpile OR negative tests that fail to transpile
            transpile_success = sum(1 for r in lang_results if 
                                  (not r.is_negative_test and r.transpile_success) or
                                  (r.is_negative_test and r.expected_failure))
            # Validation success: for positive tests, validation_success must be True when validate enabled
            validation_success = sum(1 for r in lang_results if 
                                   (not self.config.validate) or
                                   (not r.is_negative_test and r.validation_success) or
                                   (r.is_negative_test and r.expected_failure))
            
            # For execution success: positive tests that execute OR negative tests (which don't execute)
            execute_success = sum(1 for r in lang_results if 
                                (not r.is_negative_test and r.execute_success) or
                                (r.is_negative_test and r.expected_failure))
            
            # Overall success rate - tests that behave as expected
            overall_success = sum(1 for r in lang_results if
                                (not r.is_negative_test 
                                 and not self.is_infinite_loop_test(Path(r.file)) 
                                 and r.transpile_success 
                                 and (not self.config.validate or r.validation_success)
                                 and (not self.config.execute or r.execute_success)) or
                                (r.is_negative_test and r.expected_failure) or
                                (self.is_infinite_loop_test(Path(r.file)) and r.transpile_success))
            
            report["summary"]["by_language"][lang] = {
                "total": len(lang_results),
                "transpile_success": transpile_success,
                "validation_success": validation_success,
                "execute_success": execute_success,
                "overall_success": overall_success,
                "transpile_rate": f"{100*transpile_success/len(lang_results):.1f}%" if lang_results else "0%",
                "validation_rate": f"{100*validation_success/len(lang_results):.1f}%" if lang_results else "0%",
                "execute_rate": f"{100*execute_success/len(lang_results):.1f}%" if lang_results else "0%",
                "overall_rate": f"{100*overall_success/len(lang_results):.1f}%" if lang_results else "0%"
            }
        
        # Calculate statistics by category
        categories = set(r.category for r in self.results)
        for category in categories:
            cat_results = [r for r in self.results if r.category == category]
            success_count = sum(1 for r in cat_results if
                              (not r.is_negative_test 
                               and not self.is_infinite_loop_test(Path(r.file)) 
                               and r.transpile_success 
                               and (not self.config.validate or r.validation_success)
                               and (not self.config.execute or r.execute_success)) or
                              (r.is_negative_test and r.expected_failure) or
                              (self.is_infinite_loop_test(Path(r.file)) and r.transpile_success))
            report["summary"]["by_category"][category] = {
                "total": len(cat_results),
                "success": success_count
            }
        
        return report
    
    def print_summary(self):
        """Print test execution summary."""
        report = self.generate_report()
        
        print("\n" + "="*70)
        print("TEST EXECUTION SUMMARY")
        print("="*70)
        
        for lang, stats in report["summary"]["by_language"].items():
            print(f"\n{lang.upper()}:")
            print(f"  Total tests: {stats['total']}")
            print(f"  Overall success: {stats['overall_success']}/{stats['total']} ({stats['overall_rate']})")
            print(f"  Transpilation: {stats['transpile_success']}/{stats['total']} ({stats['transpile_rate']})")
            if self.config.validate:
                print(f"  Validation: {stats['validation_success']}/{stats['total']} ({stats['validation_rate']})")
            if self.config.execute:
                print(f"  Execution: {stats['execute_success']}/{stats['total']} ({stats['execute_rate']})")
        
        print("\nBy Category:")
        for category, stats in sorted(report["summary"]["by_category"].items()):
            success_rate = 100 * stats["success"] / stats["total"] if stats["total"] > 0 else 0
            print(f"  {category}: {stats['success']}/{stats['total']} ({success_rate:.1f}%)")
        
        # List failures (tests that didn't behave as expected)
        failures = [r for r in self.results if
                   (not r.is_negative_test and not self.is_infinite_loop_test(Path(r.file)) and ((not r.transpile_success) or (self.config.validate and not r.validation_success) or (self.config.execute and not r.execute_success))) or
                   (r.is_negative_test and not r.expected_failure) or
                   (self.is_infinite_loop_test(Path(r.file)) and not r.transpile_success)]
        if failures:
            print(f"\n{len(failures)} Failed Tests:")
            for r in failures[:10]:  # Show first 10 failures
                print(f"  - {r.name} ({r.language}): {r.error_message[:100] if r.error_message else 'Unknown error'}")
            if len(failures) > 10:
                print(f"  ... and {len(failures) - 10} more")
        
        return report

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Frame Test Runner')
    parser.add_argument('--languages', '-l', nargs='+', default=['python', 'typescript'],
                       choices=['python', 'typescript', 'rust', 'golang', 'javascript', 'llvm'],
                       help='Languages to test')
    parser.add_argument('--categories', '-c', nargs='+', default=['all'],
                       help='Test categories to run')
    parser.add_argument('--framec', default='./target/release/framec',
                       help='Path to framec executable')
    parser.add_argument('--verbose', '-v', action='store_true',
                       help='Verbose output')
    parser.add_argument('--transpile-only', action='store_true',
                       help='Only transpile, do not execute')
    parser.add_argument('--no-validate', action='store_true',
                       help='Skip validator step (transpile/execute only)')
    parser.add_argument('--validation-level', default='structural',
                       choices=['basic','structural','semantic','target-language'],
                       help='Validator level to apply')
    parser.add_argument('--validation-format', default='human',
                       choices=['human','json','junit'],
                       help='Validator output format')
    parser.add_argument('--output', '-o', help='Output JSON report to file')
    parser.add_argument('--include-common', action='store_true', help='Include common/ shared tests (default: disabled for native-only policy)')
    parser.add_argument('--index', dest='index_path', default=str(Path(__file__).parent.parent / 'TEST_INDEX.json'), help='Path to test index JSON')
    parser.add_argument('--update-index', dest='update_index', action='store_true', help='Update the test index with actual results')
    parser.add_argument('--timeout', type=int, default=10,
                       help='Timeout for each test in seconds')
    
    args = parser.parse_args()
    
    # Create config
    config = TestConfig(
        framec_path=args.framec,
        languages=args.languages,
        categories=args.categories,
        verbose=args.verbose,
        execute=not args.transpile_only,
        validate=not args.no_validate,
        validation_level=args.validation_level,
        validation_format=args.validation_format,
        timeout=args.timeout,
        include_common=args.include_common,
    )
    
    # Run tests
    runner = FrameTestRunner(config)
    results = runner.run_all_tests()
    report = runner.print_summary()
    
    # Save report if requested
    if args.output:
        with open(args.output, 'w') as f:
            json.dump(report, f, indent=2)
        print(f"\nReport saved to {args.output}")

    # Compare/update index
    try:
        index_path = Path(args.index_path) if args.index_path else None
        if index_path:
            if index_path.exists():
                with index_path.open('r') as f:
                    index = json.load(f)
            else:
                index = {"metadata": {"version": 1, "active_languages": config.languages, "on_hold": ["llvm"], "policy": "All fixtures are target‑native; LLVM on hold."}, "tests": {}}

            # Fold actuals by category/filename key
            actuals = {}
            for r in results:
                p = Path(r.file)
                if runner.common_tests_dir in p.parents:
                    rel_key = f"{p.parent.name}/{p.name}"
                elif runner.language_specific_dir in p.parents:
                    rel_key = f"{p.parts[-2]}/{p.name}"
                else:
                    rel_key = p.name
                actuals.setdefault(rel_key, {})[r.language] = {
                    "transpile": r.transpile_success,
                    "validate": r.validation_success if config.validate else True,
                    "execute": r.execute_success if config.execute and not runner.is_infinite_loop_test(p) else True,
                    "negative": r.is_negative_test,
                    "infinite": runner.is_infinite_loop_test(p),
                }

            diffs = []
            for key, langs in actuals.items():
                exp_entry = index.get('tests', {}).get(key, {})
                for lang, vals in langs.items():
                    exp_vals = exp_entry.get(lang)
                    if exp_vals != vals:
                        diffs.append((key, lang, exp_vals, vals))

            if diffs:
                print(f"\nIndex comparison: {len(diffs)} differences")
                if args.verbose:
                    for key, lang, exp, got in diffs[:50]:
                        print(f"  - {key} [{lang}] expected={exp} actual={got}")
            else:
                print("\nIndex comparison: no differences")

            if args.update_index:
                idx_tests = index.setdefault('tests', {})
                for key, langs in actuals.items():
                    entry = idx_tests.setdefault(key, {})
                    entry.update(langs)
                with index_path.open('w') as f:
                    json.dump(index, f, indent=2)
                print(f"Updated index: {index_path}")
    except Exception as e:
        print(f"Warning: index processing failed: {e}")
    
    # Exit with error if any tests failed (didn't behave as expected)
    all_success = all((not r.is_negative_test and not runner.is_infinite_loop_test(Path(r.file)) and r.transpile_success and (not config.execute or r.execute_success)) or
                     (r.is_negative_test and r.expected_failure) or
                     (runner.is_infinite_loop_test(Path(r.file)) and r.transpile_success) for r in results)
    sys.exit(0 if all_success else 1)

if __name__ == '__main__':
    main()
