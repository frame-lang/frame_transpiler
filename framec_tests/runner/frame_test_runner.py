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
import re
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import re
import random

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
    validation_errors: Optional[List[str]] = None
    skipped: Optional[str] = None

@dataclass 
class TestConfig:
    """Configuration for test execution."""
    framec_path: str = "./target/release/framec"
    languages: List[str] = None
    categories: List[str] = None
    verbose: bool = False
    execute: bool = True  # If False, only build (no run)
    validate: bool = True  # Run compiler validation after build
    validation_level: str = "structural"  # basic|structural|semantic|target-language
    validation_format: str = "human"  # human|json|junit
    parallel: bool = False
    timeout: int = 30
    include_common: bool = False
    strict_negatives: bool = True  # Negatives must fail compiler validation (not just build)
    require_error_codes: bool = True  # Negatives must include one or more E### codes
    include_flaky: bool = False  # Skip @flaky tests unless explicitly included
    expected_error_mode: str = 'superset'  # or 'equal'
    include_patterns: Optional[List[str]] = None
    exclude_patterns: Optional[List[str]] = None
    shuffle: bool = False
    seed: Optional[int] = None
    # Execute selected non-smoke V3 categories (python/typescript) using demo-frame exec emission
    exec_v3: bool = False
    
    def __post_init__(self):
        if self.languages is None:
            # Include all configured V3 demo languages by default; execution for v3_demos is skipped
            self.languages = ["python", "typescript", "csharp", "c", "cpp", "java", "rust"]
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

    def parse_fixture_meta(self, test_file: Path) -> Dict[str, List[str]]:
        """Parse inline metadata from the fixture header.
        Supports (first 20 lines):
          - @expect: E403 E404 (space-separated error codes)
          - @run-expect: <regex> (can appear multiple times)
          - @flaky
          - @skip-if: <token> (e.g., java-toolchain-missing)
          - @timeout: <seconds>
        """
        meta: Dict[str, List[str]] = {}
        try:
            with open(test_file, 'r') as f:
                for i, line in enumerate(f):
                    if i > 20:
                        break
                    m = re.match(r"^\s*(#|//)\s*@expect:\s*(.+)$", line)
                    if m:
                        codes = re.findall(r"E\d{3}", m.group(2))
                        if codes:
                            meta['expect'] = [c for c in codes]
                    m2 = re.match(r"^\s*(#|//)\s*@run-expect:\s*(.+)$", line)
                    if m2:
                        pat = m2.group(2).strip()
                        if pat:
                            meta.setdefault('run_expect', []).append(pat)
                    m2b = re.match(r"^\s*(#|//)\s*@run-exact:\s*(.+)$", line)
                    if m2b:
                        exact = m2b.group(2).rstrip('\n')
                        meta['run_exact'] = [exact]
                    if re.match(r"^\s*(#|//)\s*@flaky\b", line):
                        meta['flaky'] = ['1']
                    if re.match(r"^\s*(#|//)\s*@exec-ok\b", line):
                        meta['exec_ok'] = ['1']
                    m3 = re.match(r"^\s*(#|//)\s*@skip-if:\s*(.+)$", line)
                    if m3:
                        toks = [t.strip() for t in m3.group(2).split(',') if t.strip()]
                        if toks:
                            meta.setdefault('skip_if', []).extend(toks)
                    m4 = re.match(r"^\s*(#|//)\s*@timeout:\s*(\d+)\s*$", line)
                    if m4:
                        meta['timeout'] = [m4.group(2)]
        except Exception:
            pass
        return meta
        
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
        
        # V3 category helpers: demos, outline, prolog, imports
        def collect_v3_category(cat_name: str):
            for lang in self.config.languages:
                lang_dir = self.language_specific_dir / lang / cat_name
                if lang_dir.exists():
                    test_files = list(lang_dir.rglob("*.frm"))
                    if test_files:
                        tests[f"language_specific_{lang}_{cat_name}"] = test_files

        # v3_demos (explicit or per-lang)
        if "v3_demos" in self.config.categories:
            collect_v3_category("v3_demos")
        else:
            for lang in self.config.languages:
                cat = f"v3_demos_{lang}"
                if cat in self.config.categories:
                    lang_dir = self.language_specific_dir / lang / "v3_demos"
                    if lang_dir.exists():
                        demo_tests = list(lang_dir.glob("*.frm"))
                        if demo_tests:
                            tests[f"language_specific_{lang}_v3_demos"] = demo_tests

        # v3_outline, v3_prolog, v3_imports, v3_closers, v3_mir, v3_mapping, v3_expansion
        if any(cat in self.config.categories for cat in [
            "v3_outline", "v3_prolog", "v3_imports", "v3_closers", "v3_mir", "v3_mapping", "v3_expansion", "v3_validator", "v3_project", "v3_facade_smoke", "v3_exec_smoke",
            "v3_core", "v3_control_flow", "v3_data_types", "v3_operators", "v3_scoping", "v3_systems"
        ]):
            if "v3_outline" in self.config.categories:
                collect_v3_category("v3_outline")
            if "v3_prolog" in self.config.categories:
                collect_v3_category("v3_prolog")
            if "v3_imports" in self.config.categories:
                collect_v3_category("v3_imports")
            if "v3_closers" in self.config.categories:
                collect_v3_category("v3_closers")
            if "v3_mir" in self.config.categories:
                collect_v3_category("v3_mir")
            if "v3_mapping" in self.config.categories:
                collect_v3_category("v3_mapping")
            if "v3_expansion" in self.config.categories:
                collect_v3_category("v3_expansion")
            if "v3_validator" in self.config.categories:
                collect_v3_category("v3_validator")
            if "v3_project" in self.config.categories:
                collect_v3_category("v3_project")
            if "v3_facade_smoke" in self.config.categories:
                collect_v3_category("v3_facade_smoke")
            if "v3_exec_smoke" in self.config.categories:
                collect_v3_category("v3_exec_smoke")
            if "v3_core" in self.config.categories:
                collect_v3_category("v3_core")
            if "v3_control_flow" in self.config.categories:
                collect_v3_category("v3_control_flow")
            if "v3_data_types" in self.config.categories:
                collect_v3_category("v3_data_types")
            if "v3_operators" in self.config.categories:
                collect_v3_category("v3_operators")
            if "v3_scoping" in self.config.categories:
                collect_v3_category("v3_scoping")
            if "v3_systems" in self.config.categories:
                collect_v3_category("v3_systems")

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
    
    def transpile(self, test_file: Path, language: str, timeout: Optional[int] = None) -> Tuple[bool, str, str]:
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
        
        # Special handling for V3 demo tests (module partitioner demo path)
        parts_lower = [p.lower() for p in test_file.parts]
        # Treat all v3_* categories as module demo path; v3_closers uses single-body demo
        v3_categories = {"v3_demos", "v3_outline", "v3_prolog", "v3_imports", "v3_closers", "v3_mir", "v3_mapping", "v3_validator", "v3_project", "v3_facade_smoke", "v3_core", "v3_control_flow", "v3_data_types", "v3_operators", "v3_scoping", "v3_systems", "v3_exec_smoke"}
        is_v3 = any(seg in v3_categories for seg in parts_lower)
        is_v3_closers = "v3_closers" in parts_lower
        is_v3_mapping = "v3_mapping" in parts_lower
        is_v3_expansion = "v3_expansion" in parts_lower
        # Initialize optional flags to avoid UnboundLocalError
        is_v3_facade_smoke = False
        # Run transpiler - check if multifile test
        if is_v3:
            is_v3_facade_smoke = "v3_facade_smoke" in parts_lower
            if is_v3_closers or is_v3_mapping or is_v3_expansion:
                cmd = [self.config.framec_path, "demo-multi", "-l", lang_flag, str(test_file)]
                extension = ".txt"
                output_file = output_dir / (test_file.stem + extension)
            else:
                cmd = [self.config.framec_path, "demo-frame", "-l", lang_flag, str(test_file)]
                # Choose extension when we intend to execute the output
                if ("v3_exec_smoke" in parts_lower and self.config.execute) or (getattr(self.config, 'exec_v3', False) and self.config.execute and language in ("python", "typescript") and any(seg in ("v3_core", "v3_control_flow", "v3_systems") for seg in parts_lower)):
                    ext_map = {"python": ".py", "typescript": ".ts", "rust": ".rs", "c": ".c", "cpp": ".cpp", "java": ".java", "csharp": ".cs"}
                    extension = ext_map.get(language, ".txt")
                    if language in ("python", "typescript"):
                        cmd.insert(2, "--emit-exec")
                else:
                    extension = ".txt"
                output_file = output_dir / (test_file.stem + extension)
            # Include validation flag for V3 flows when enabled
            if self.config.validate:
                cmd.insert(2, "--validate")
                # Enable native validation for facade smoke fixtures
                if is_v3_facade_smoke:
                    cmd.insert(3, "--validate-native")
        elif self.is_multifile_test(test_file):
            # Use multifile flag for tests with Frame imports
            cmd = [self.config.framec_path, "-m", str(test_file), "-l", lang_flag]
        else:
            # Detect module-style files with @target and route via V3 demo-frame
            is_module_file = False
            try:
                with open(test_file, 'r') as f:
                    for line in f:
                        if line.strip().startswith('@target '):
                            is_module_file = True
                            break
            except Exception:
                is_module_file = False
            # For Python legacy fixtures without @target, synthesize a module prolog on the fly
            synthesized_path = None
            if (not is_module_file) and language == 'python':
                try:
                    content = Path(test_file).read_text()
                    if '@target ' not in content:
                        synth_dir = self.generated_dir / 'python' / 'tmp_modules'
                        synth_dir.mkdir(parents=True, exist_ok=True)
                        synthesized_path = synth_dir / (Path(test_file).stem + '__module.frm')
                        synthesized_path.write_text('@target python\n' + content)
                        is_module_file = True
                except Exception:
                    pass

            if is_module_file:
                cmd = [self.config.framec_path, "demo-frame", "-l", lang_flag, str(test_file)]
                if synthesized_path is not None:
                    cmd[-1] = str(synthesized_path)
                extension = ".py" if language == "python" else (".ts" if language == "typescript" else extension)
                output_file = output_dir / (test_file.stem + extension)
            else:
                # Standard single-file compilation (legacy pipeline)
                cmd = [self.config.framec_path, "-l", lang_flag, str(test_file)]
        
        try:
            # Set environment variables for Rust main function generation
            env = os.environ.copy()
            if language == "rust":
                env["FRAME_RUST_GENERATE_MAIN"] = "1"
            # For mapping fixtures, request trailer
            if is_v3_mapping:
                env["FRAME_MAP_TRAILER"] = "1"
            # For facade smoke fixtures, request facade expansion output
            if is_v3_facade_smoke:
                env["FRAME_FACADE_EXPANSION"] = "1"
            # Exec smoke should emit a minimal executable so we can run it end-to-end
            if "v3_exec_smoke" in parts_lower and self.config.execute:
                env["FRAME_EMIT_EXEC"] = "1"
            # For general V3 categories in Python/TS, emit a minimal executable when running
            if any(seg.startswith("v3_") for seg in parts_lower) and not is_v3_facade_smoke and language in ("python", "typescript") and self.config.execute:
                env["FRAME_EMIT_EXEC"] = "1"
            # For module files in Python/TS routed via demo-frame, also emit exec when executing
            if 'demo-frame' in cmd and language in ("python", "typescript") and self.config.execute:
                env["FRAME_EMIT_EXEC"] = "1"
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=timeout or self.config.timeout,
                env=env
            )
            
            if result.returncode == 0:
                # Optional mapping trailer validation for v3_mapping
                if is_v3_mapping:
                    out = result.stdout or ""
                    start = out.find("/*#frame-map#")
                    end = out.find("#frame-map#*/")
                    if start == -1 or end == -1 or end <= start:
                        return False, str(output_file), "Missing frame-map trailer in output"
                    trailer_text = out[start+len("/*#frame-map#"):end]
                    try:
                        import json as _json
                        payload = _json.loads(trailer_text.strip())
                        m = payload.get("map", [])
                        if not m:
                            return False, str(output_file), "Empty mapping payload"
                        # At least one origin should be frame
                        has_frame = any(item.get("origin") == "frame" for item in m if isinstance(item, dict))
                        if not has_frame:
                            return False, str(output_file), "No frame-origin entries in mapping"
                    except Exception as e:
                        return False, str(output_file), f"Invalid mapping JSON: {e}"
                # Write output to file
                out = result.stdout or ""
                output_file.write_text(out)
                # Execute demo-frame emitted executables for exec smoke (Py/TS)
                if self.config.execute and "v3_exec_smoke" in parts_lower and language in ("python", "typescript"):
                    if language == "python":
                        ok, err = self.execute_python(str(output_file))
                    else:
                        ok, err = self.execute_typescript(str(output_file))
                    if not ok:
                        return False, str(output_file), err
                if self.config.execute and is_v3_facade_smoke and language in ("typescript", "python", "rust", "c", "cpp", "java", "csharp"):
                    # Build and execute a minimal TS harness from spliced output for facade strict tests only.
                    if "__frame_transition" not in out and "__frame_forward" not in out and "__frame_stack_" not in out:
                        return False, str(output_file), "Facade wrappers not found in output"
                    if language == "typescript":
                        ok, err = self._execute_ts_harness_from_spliced(test_file.stem, out)
                    elif language == "python":
                        ok, err = self._execute_py_harness_from_spliced(test_file.stem, out)
                    elif language == "rust":
                        ok, err = self._execute_rust_harness_from_spliced(test_file.stem, out)
                    elif language == "c":
                        ok, err = self._execute_c_like_harness_from_spliced(test_file.stem, out, use_cpp=False)
                    elif language == "cpp":
                        ok, err = self._execute_c_like_harness_from_spliced(test_file.stem, out, use_cpp=True)
                    elif language == "java":
                        ok, err = self._execute_java_harness_from_spliced(test_file.stem, out)
                    else:  # csharp
                        ok, err = self._execute_csharp_harness_from_spliced(test_file.stem, out)
                    if not ok:
                        return False, str(output_file), err
                return True, str(output_file), None
            else:
                error = result.stderr or result.stdout
                return False, str(output_file), error
                
        except subprocess.TimeoutExpired:
            return False, str(output_file), "Transpilation timeout"
        except Exception as e:
            return False, str(output_file), str(e)

    def validate(self, test_file: Path, language: str) -> Tuple[bool, str]:
        """Run framec validation on a Frame file for a given target language.
        Routes v3_closers fixtures through the single-body validator (demo-multi)
        so body-closer errors map to E-codes.
        """
        lang_flag = {
            "python": "python_3",
            "typescript": "typescript",
            "rust": "rust",
            "golang": "golang",
            "javascript": "javascript",
            "llvm": "llvm",
        }.get(language, language)

        parts_lower = [p.lower() for p in test_file.parts]
        # Single-body demo categories validate via demo-multi
        use_single_body = any(seg in ("v3_closers", "v3_mapping", "v3_expansion") for seg in parts_lower)
        synthesized_single_body: Optional[Path] = None
        if use_single_body:
            # If a single-body fixture has a leading @target line, strip it into a temp file
            try:
                content = Path(test_file).read_text()
                lines = content.splitlines()
                # Find first non-empty line
                first_non_empty = next((i for i, ln in enumerate(lines) if ln.strip()), None)
                if first_non_empty is not None and lines[first_non_empty].strip().startswith('@target '):
                    stripped = '\n'.join(lines[:first_non_empty] + lines[first_non_empty+1:])
                    # Ensure single-body starts with '{' at byte 0
                    stripped = stripped.lstrip()
                    tmp_dir = self.generated_dir / language / 'tmp_single_body'
                    tmp_dir.mkdir(parents=True, exist_ok=True)
                    synthesized_single_body = tmp_dir / (Path(test_file).stem + '__body.frm')
                    synthesized_single_body.write_text(stripped)
            except Exception:
                synthesized_single_body = None
            target_path = str(synthesized_single_body or test_file)
            cmd = [
                self.config.framec_path,
                "demo-multi",
                "--language",
                lang_flag,
                "--validate",
                "--validation-only",
                target_path,
            ]
        else:
            # Align validation path with demo-frame module validator for V3
            cmd = [
                self.config.framec_path,
                "demo-frame",
                "--language",
                lang_flag,
                "--validate",
                "--validation-only",
                "--validation-level",
                self.config.validation_level,
                "--validation-format",
                self.config.validation_format,
                str(test_file),
            ]
            # Enable strict/native facade validation for facade smoke fixtures
            if "v3_facade_smoke" in parts_lower:
                # Insert before the test file path (last arg)
                insert_at = max(3, len(cmd) - 1)
                cmd.insert(insert_at, "--validate-native")

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

    def _execute_py_harness_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """
        Build and execute a minimal Python harness for facade strict tests by extracting
        wrapper-call lines from the spliced output and running them in a main() function.
        """
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition(") or s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                wrappers.append(s)
        prelude = "\n".join([
            "def __frame_transition(state, *args, **kwargs):\n    pass",
            "def __frame_forward():\n    pass",
            "def __frame_stack_push():\n    pass",
            "def __frame_stack_pop():\n    pass",
        ])
        indented = "\n".join(["    " + w for w in wrappers])
        program = f"{prelude}\n\nif __name__ == '__main__':\n{indented}\n"
        out_dir = self.generated_dir / "python"
        out_dir.mkdir(parents=True, exist_ok=True)
        py_path = out_dir / f"{test_name}__v3.py"
        py_path.write_text(program)
        return self.execute_python(str(py_path))

    def _execute_py_prod_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """Execute production-style Python expansions by wrapping the spliced body in a minimal runtime shell."""
        prelude = "\n".join([
            "class FrameEvent:\n    def __init__(self, message, parameters=None):\n        self.message=message; self.parameters=parameters",
            "class FrameCompartment:\n    def __init__(self, state):\n        self.state=state; self.forward_event=None; self.exit_args=None; self.enter_args=None; self.parent_compartment=None; self.state_args=None",
            "class M:\n    def __init__(self):\n        self._compartment = FrameCompartment('__S_state_A')\n    def _frame_transition(self, next_compartment):\n        self._compartment = next_compartment\n    def _frame_router(self, __e, compartment=None):\n        pass\n    def _frame_stack_push(self):\n        pass\n    def _frame_stack_pop(self):\n        pass",
            "def native():\n    pass",
        ])
        # Keep only production-glue lines for execution
        keep_tokens_py = [
            "FrameCompartment(",
            "compartment.exit_args",
            "next_compartment.enter_args",
            "next_compartment.state_args",
            "self._frame_transition(",
            "self._frame_router(",
            "self._frame_stack_push(",
            "self._frame_stack_pop("
        ]
        kept_py: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if any(tok in s for tok in keep_tokens_py):
                kept_py.append("    "+s)
        body = "\n".join(kept_py)
        program = f"{prelude}\n\ndef handler(self, __e, compartment):\n{body}\n\nif __name__ == '__main__':\n    m=M()\n    handler(m, FrameEvent('e'), m._compartment)\n"
        out_dir = self.generated_dir / "python"
        out_dir.mkdir(parents=True, exist_ok=True)
        py_path = out_dir / f"{test_name}__v3_prod.py"
        py_path.write_text(program)
        return self.execute_python(str(py_path))
    
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
    
    def _execute_ts_harness_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """
        Build and execute a minimal TypeScript harness by extracting wrapper-call lines
        from the spliced output and running them inside a main() function. If no wrappers
        are present (non-facade categories), execute an empty main.
        """
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition(") or s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                if not s.endswith(";"):
                    s = s + ";"
                wrappers.append(s)
        prelude = "\n".join([
            "function __frame_transition(state: string, ...args: any[]) {}",
            "function __frame_forward() {}",
            "function __frame_stack_push() {}",
            "function __frame_stack_pop() {}",
        ])
        body = "\n".join(wrappers)
        program = f"{prelude}\nfunction main() {{\n{body}\n}}\nmain();\n"
        out_dir = self.generated_dir / "typescript"
        out_dir.mkdir(parents=True, exist_ok=True)
        ts_path = out_dir / f"{test_name}__v3.ts"
        ts_path.write_text(program)
        return self.execute_typescript(str(ts_path))

    def _execute_ts_prod_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """Execute production-style TypeScript expansions by wrapping the spliced body in a minimal runtime shell."""
        prelude = "\n".join([
            "class FrameEvent { constructor(public message: string, public parameters: any|null) {} }",
            "class FrameCompartment { constructor(public state: string) {} public forwardEvent: FrameEvent|null=null; public exitArgs: any=null; public enterArgs: any=null; public parentCompartment: FrameCompartment|null=null; public stateArgs: any=null; }",
            "class M { public _compartment: FrameCompartment = new FrameCompartment('__S_state_A'); _frame_transition(n: FrameCompartment){ this._compartment=n; } _frame_router(__e: FrameEvent, c?: FrameCompartment){ } _frame_stack_push(){} _frame_stack_pop(){} }",
            "function native(): void {}",
        ])
        keep_tokens = [
            "new FrameCompartment(",
            "compartment.exitArgs",
            "nextCompartment.enterArgs",
            "nextCompartment.stateArgs",
            "this._frame_transition(",
            "this._frame_router(",
            "this._frame_stack_push(",
            "this._frame_stack_pop("
        ]
        kept: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if any(tok in s for tok in keep_tokens):
                if not s.endswith(";") and not s.endswith("{") and not s.endswith("}"):
                    s = s + ";"
                kept.append("    "+s)
        body = "\n".join(kept)
        program = prelude + "\n" \
            + "function handler(self: M, __e: FrameEvent, compartment: FrameCompartment) {\n" \
            + body + "\n}" \
            + "\n(function(){ const m=new M(); handler.call(m, m, new FrameEvent('e', null), m._compartment); })();\n"
        out_dir = self.generated_dir / "typescript"
        out_dir.mkdir(parents=True, exist_ok=True)
        ts_path = out_dir / f"{test_name}__v3_prod.ts"
        ts_path.write_text(program)
        return self.execute_typescript(str(ts_path))
    
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

    def _execute_rust_harness_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """
        Build and execute a minimal Rust harness for facade strict tests by extracting
        wrapper-call lines and rewriting them to match no-op wrapper signatures.
        """
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition("):
                # Expect __frame_transition("State" ...);
                try:
                    start = s.find('("')
                    if start == -1:
                        start = s.find('(\'')
                    if start != -1:
                        q = '"' if s.find('("') != -1 else '\''
                        # find end quote
                        endq = s.find(q, start+2)
                        if endq != -1:
                            state = s[start+2:endq]
                            s = f"__frame_transition(\"{state}\");"
                        else:
                            s = "// skipped malformed transition"
                    else:
                        s = "// skipped malformed transition"
                except Exception:
                    s = "// skipped malformed transition"
                wrappers.append(s)
            elif s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                # Keep as-is
                if not s.endswith(";"):
                    s += ";"
                wrappers.append(s)
        prelude = "\n".join([
            "fn __frame_transition(_state: &str) {}",
            "fn __frame_forward() {}",
            "fn __frame_stack_push() {}",
            "fn __frame_stack_pop() {}",
        ])
        body = "\n    ".join(wrappers)
        program = f"{prelude}\nfn main() {{\n    {body}\n}}\n"
        out_dir = self.generated_dir / "rust"
        out_dir.mkdir(parents=True, exist_ok=True)
        rs_path = out_dir / f"{test_name}__v3.rs"
        rs_path.write_text(program)
        return self.execute_rust(str(rs_path))

    def _execute_c_like_harness_from_spliced(self, test_name: str, spliced_output: str, use_cpp: bool) -> Tuple[bool, str]:
        """
        Build and execute a minimal C/C++ harness for facade strict tests by extracting
        wrapper-call lines and compiling with a system compiler (clang/gcc or clang++/g++).
        """
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition(") or s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                if not s.endswith(";"):
                    s += ";"
                wrappers.append(s)
        prelude = (
            "#include <stdarg.h>\n"
            "void __frame_transition(const char* state, ...) {}\n"
            "void __frame_forward(void) {}\n"
            "void __frame_stack_push(void) {}\n"
            "void __frame_stack_pop(void) {}\n"
        )
        body = "\n    ".join(wrappers)
        src = f"{prelude}\nint main(void) {{\n    {body}\n    return 0;\n}}\n"
        out_dir = self.generated_dir / ("cpp" if use_cpp else "c")
        out_dir.mkdir(parents=True, exist_ok=True)
        ext = ".cpp" if use_cpp else ".c"
        src_path = out_dir / f"{test_name}__v3{ext}"
        src_path.write_text(src)
        # Find compiler
        import shutil
        compiler = shutil.which("clang++" if use_cpp else "clang") or shutil.which("g++" if use_cpp else "gcc")
        if not compiler:
            return False, "C/C++ compiler not found (clang/gcc)"
        exe_path = str(src_path.with_suffix(""))
        try:
            compile_result = subprocess.run(
                [compiler, str(src_path), "-o", exe_path],
                capture_output=True,
                text=True,
                timeout=max(self.config.timeout, 10)
            )
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run = subprocess.run([exe_path], capture_output=True, text=True, timeout=self.config.timeout)
            # Cleanup
            try:
                os.remove(exe_path)
            except Exception:
                pass
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "C/C++ harness timeout"
        except Exception as e:
            return False, str(e)

    def _execute_c_like_source(self, src_file: str, use_cpp: bool) -> Tuple[bool, str]:
        """Compile and run a generated C/C++ source file and return status/output."""
        import shutil
        compiler = shutil.which("clang++" if use_cpp else "clang") or shutil.which("g++" if use_cpp else "gcc")
        if not compiler:
            return True, ("C++ compiler not found; skipping execution" if use_cpp else "C compiler not found; skipping execution")
        exe_path = os.path.splitext(src_file)[0]
        try:
            compile_result = subprocess.run([compiler, src_file, "-o", exe_path], capture_output=True, text=True, timeout=max(self.config.timeout, 10))
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run = subprocess.run([exe_path], capture_output=True, text=True, timeout=self.config.timeout)
            try:
                os.remove(exe_path)
            except Exception:
                pass
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "C/C++ execution timeout"
        except Exception as e:
            return False, str(e)

    def _execute_java_source(self, java_file: str) -> Tuple[bool, str]:
        """Compile and run a generated Java source file with a main class ExecMain."""
        import shutil
        javac = shutil.which("javac")
        java = shutil.which("java")
        if not javac or not java:
            return True, "Java toolchain not found; skipping execution"
        out_dir = os.path.dirname(java_file)
        class_name = "ExecMain"
        # Ensure file name matches public class name by writing a copy as ExecMain.java
        try:
            with open(java_file, 'r') as f:
                src_text = f.read()
            exec_path = os.path.join(out_dir, f"{class_name}.java")
            with open(exec_path, 'w') as f:
                f.write(src_text)
            java_src = exec_path
        except Exception:
            java_src = java_file
        try:
            compile_result = subprocess.run([javac, java_src], capture_output=True, text=True, timeout=max(self.config.timeout, 10), cwd=out_dir)
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run = subprocess.run([java, "-cp", out_dir, class_name], capture_output=True, text=True, timeout=self.config.timeout)
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "Java execution timeout"
        except Exception as e:
            return False, str(e)

    def _execute_csharp_source(self, cs_file: str) -> Tuple[bool, str]:
        """Compile and run a generated C# source file with class ExecMain."""
        import shutil
        csc = shutil.which("csc")
        mcs = shutil.which("mcs")
        mono = shutil.which("mono")
        exe_path = os.path.splitext(cs_file)[0] + ".exe"
        if not csc and not mcs:
            return True, "C# compiler not found; skipping execution"
        try:
            if csc:
                compile_cmd = [csc, "/nologo", f"/out:{exe_path}", cs_file]
            else:
                compile_cmd = [mcs, "-out:", exe_path, cs_file]
            compile_result = subprocess.run(compile_cmd, capture_output=True, text=True, timeout=max(self.config.timeout, 10))
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run_cmd = [exe_path]
            if mcs and not mono:
                return True, "C# compiled (mono not found; skip run)"
            if mono:
                run_cmd = [mono, exe_path]
            run = subprocess.run(run_cmd, capture_output=True, text=True, timeout=self.config.timeout)
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "C# execution timeout"
        except Exception as e:
            return False, str(e)

    def _execute_java_harness_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """
        Build and execute a minimal Java harness by extracting wrapper-call lines
        into a uniquely named class and invoking it with java.
        """
        # Sanitize class name
        base = "FrameFacade_" + "".join(ch if ch.isalnum() else "_" for ch in test_name)
        if not base[0].isalpha():
            base = "F_" + base
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition(") or s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                if not s.endswith(";"):
                    s += ";"
                wrappers.append(s)
        prelude = (
            "public class %s {\n" % base +
            "  public static void __frame_transition(String state, Object... args) {}\n" +
            "  public static void __frame_forward() {}\n" +
            "  public static void __frame_stack_push() {}\n" +
            "  public static void __frame_stack_pop() {}\n" +
            "  public static void main(String[] args) {\n"
        )
        body = "\n    ".join(wrappers) + ("\n" if wrappers else "")
        src = f"{prelude}    {body}  }}\n}}\n"
        out_dir = self.generated_dir / "java"
        out_dir.mkdir(parents=True, exist_ok=True)
        java_path = out_dir / f"{base}.java"
        java_path.write_text(src)
        import shutil
        javac = shutil.which("javac")
        java = shutil.which("java")
        if not javac or not java:
            return True, "Java toolchain not found; skipping execution"
        try:
            compile_result = subprocess.run([javac, str(java_path)], capture_output=True, text=True, timeout=max(self.config.timeout, 10), cwd=str(out_dir))
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run = subprocess.run([java, "-cp", str(out_dir), base], capture_output=True, text=True, timeout=self.config.timeout)
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "Java harness timeout"
        except Exception as e:
            return False, str(e)

    def _execute_csharp_harness_from_spliced(self, test_name: str, spliced_output: str) -> Tuple[bool, str]:
        """
        Build and execute a minimal C# harness by extracting wrapper-call lines
        and compiling with csc or mcs; run via mono if needed.
        """
        wrappers: List[str] = []
        for line in spliced_output.splitlines():
            s = line.strip()
            if s.startswith("__frame_transition(") or s.startswith("__frame_forward(") or s.startswith("__frame_stack_"):
                if not s.endswith(";"):
                    s += ";"
                wrappers.append(s)
        src = (
            "using System;\n" +
            "class Program {\n" +
            "  static void __frame_transition(string state, params object[] args) {}\n" +
            "  static void __frame_forward() {}\n" +
            "  static void __frame_stack_push() {}\n" +
            "  static void __frame_stack_pop() {}\n" +
            "  static void Main(string[] args) {\n    " + "\n    ".join(wrappers) + "\n  }\n}"
        )
        out_dir = self.generated_dir / "csharp"
        out_dir.mkdir(parents=True, exist_ok=True)
        cs_path = out_dir / f"{test_name}__v3.cs"
        cs_path.write_text(src)
        import shutil
        csc = shutil.which("csc")
        mcs = shutil.which("mcs")
        mono = shutil.which("mono")
        exe_path = str(cs_path.with_suffix(".exe"))
        if not csc and not mcs:
            return True, "C# compiler not found; skipping execution"
        try:
            if csc:
                compile_cmd = [csc, "/nologo", f"/out:{exe_path}", str(cs_path)]
            else:
                compile_cmd = [mcs, "-out:", exe_path, str(cs_path)]
            compile_result = subprocess.run(compile_cmd, capture_output=True, text=True, timeout=max(self.config.timeout, 10))
            if compile_result.returncode != 0:
                return False, compile_result.stderr or compile_result.stdout
            run_cmd = [exe_path]
            if mcs and not mono:
                # No mono available; treat compile success as pass
                return True, "C# compiled (mono not found; skip run)"
            if mono:
                run_cmd = [mono, exe_path]
            run = subprocess.run(run_cmd, capture_output=True, text=True, timeout=self.config.timeout)
            if run.returncode != 0:
                return False, run.stdout + run.stderr
            return True, run.stdout
        except subprocess.TimeoutExpired:
            return False, "C# harness timeout"
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
        meta = self.parse_fixture_meta(test_file)
        # Per-test timeout override
        eff_timeout = self.config.timeout
        if meta.get('timeout'):
            try:
                eff_timeout = max(1, int(meta['timeout'][0]))
            except Exception:
                pass
        transpile_success, output_file, error = self.transpile(test_file, language, timeout=eff_timeout)
        result.transpile_success = transpile_success

        # Optionally validate (after transpile to ensure parsing paths are similar)
        validation_success = False
        validation_output = ""
        if self.config.validate:
            validation_success, validation_output = self.validate(test_file, language)
            result.validation_success = validation_success
            # Extract and attach error codes like E400, E401, etc., from validation output
            try:
                import re
                codes = re.findall(r"\bE\d{3}\b", validation_output)
                result.validation_errors = sorted(set(codes)) if codes else []
            except Exception:
                result.validation_errors = []
        
        # Handle skip metadata
        if meta.get('flaky') and not self.config.include_flaky:
            result.skipped = 'flaky'
            result.transpile_success = True
            result.validation_success = True
            result.execute_success = True
            result.execution_time = time.time() - start_time
            return result
        if meta.get('skip_if'):
            # Very simple toolchain-based skips
            reasons = []
            for cond in meta['skip_if']:
                c = cond.lower()
                if c == 'java-toolchain-missing' and not shutil.which('javac'):
                    reasons.append('javac missing')
                if c == 'csharp-toolchain-missing' and not (shutil.which('csc') or shutil.which('mcs')):
                    reasons.append('csc/mcs missing')
                if c == 'c-toolchain-missing' and not (shutil.which('clang') or shutil.which('gcc')):
                    reasons.append('clang/gcc missing')
                if c == 'cpp-toolchain-missing' and not (shutil.which('clang++') or shutil.which('g++')):
                    reasons.append('clang++/g++ missing')
                if c == 'tsc-missing' and not shutil.which('tsc'):
                    reasons.append('tsc missing')
            if reasons:
                result.skipped = ', '.join(reasons)
                result.transpile_success = True
                result.validation_success = True
                result.execute_success = True
                result.execution_time = time.time() - start_time
                return result

        # Handle negative tests specially
        if is_negative:
            if self.config.strict_negatives:
                negative_ok = (self.config.validate and not validation_success)
            else:
                negative_ok = (not transpile_success) or (self.config.validate and not validation_success)
            # Inline expected error codes
            if 'expect' in meta:
                expected = set(meta['expect'])
                actual = set(result.validation_errors or [])
                mode = getattr(self.config, 'expected_error_mode', 'superset')
                if mode == 'equal':
                    if actual != expected:
                        negative_ok = False
                        result.error_message = f"Expected error codes {sorted(expected)}, got {sorted(actual)}\n{validation_output}"
                else:
                    if not expected.issubset(actual):
                        negative_ok = False
                        missing = sorted(expected - actual)
                        result.error_message = f"Missing expected error codes {missing}\n{validation_output}"
            # Require at least one validator code when enabled
            if negative_ok and self.config.require_error_codes:
                parts_lower = [p.lower() for p in test_file.parts]
                if "v3_facade_smoke" not in parts_lower:
                    if not (result.validation_errors or []):
                        negative_ok = False
                        result.error_message = "Negative test failed without validator error codes (E###)"
            if negative_ok:
                result.expected_failure = True
                err = error or validation_output
                result.error_message = f"Expected failure:\n{err}" if err else "Expected failure"
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
                result.error_message = f"Validation failed:\n{validation_output}"
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
                result.error_message = f"Validation failed:\n{validation_output}"
            elif self.config.execute:
                # For V3 exec smoke only, execute the emitted wrapper
                parts_lower = [p.lower() for p in test_file.parts]
                if "v3_exec_smoke" in parts_lower and language in ("python", "typescript", "c", "cpp", "java", "csharp", "rust"):
                    if language == "python":
                        exec_success, output = self.execute_python(output_file)
                    elif language == "typescript":
                        exec_success, output = self.execute_typescript(output_file)
                    elif language == "c":
                        exec_success, output = self._execute_c_like_source(output_file, use_cpp=False)
                    elif language == "cpp":
                        exec_success, output = self._execute_c_like_source(output_file, use_cpp=True)
                    elif language == "java":
                        exec_success, output = self._execute_java_source(output_file)
                    elif language == "csharp":
                        exec_success, output = self._execute_csharp_source(output_file)
                    else:
                        exec_success, output = self.execute_rust(output_file)
                    result.execute_success = exec_success
                    result.output = output
                    # For exec smoke, validate standardized markers
                    if exec_success:
                        out = output or ""
                        # Respect toolchain skips in exec smoke for non-Py/TS
                        if any(skip in out for skip in [
                            "compiler not found; skipping execution",
                            "toolchain not found; skipping execution",
                            "compiled (mono not found; skip run)"
                        ]):
                            result.skipped = "exec toolchain missing"
                            result.execution_time = time.time() - start_time
                            return result
                        name = test_file.stem
                        ok = True
                        err = ""
                        if name == "transition_basic":
                            if "TRANSITION:" not in out:
                                ok = False; err = "Missing TRANSITION marker"
                        elif name == "forward_parent":
                            if "FORWARD:PARENT" not in out:
                                ok = False; err = "Missing FORWARD:PARENT marker"
                        elif name == "stack_ops":
                            if not ("STACK:PUSH" in out and "STACK:POP" in out):
                                ok = False; err = "Missing STACK markers"
                        elif name == "mixed_ops":
                            if not ("STACK:PUSH" in out and "TRANSITION:" in out):
                                ok = False; err = "Missing MIXED markers"
                        elif name == "stack_then_transition" or name == "nested_stack_then_transition":
                            if not ("STACK:PUSH" in out and "STACK:POP" in out and "TRANSITION:" in out):
                                ok = False; err = "Missing STACK/TRANSITION markers"
                        elif name == "if_forward_else_transition":
                            if not ("FORWARD:PARENT" in out or "TRANSITION:" in out):
                                ok = False; err = "Missing FORWARD or TRANSITION marker"
                        if not ok:
                            result.execute_success = False
                            result.error_message = f"Execution output check failed: {err}\n{out[:200]}"
                        # Apply @run-expect and @run-exact assertions
                        meta = self.parse_fixture_meta(test_file)
                        if result.execute_success and meta.get('run_expect'):
                            missing = [p for p in meta['run_expect'] if not re.search(p, out, re.MULTILINE)]
                            if missing:
                                result.execute_success = False
                                result.error_message = f"Run output expectation failed: missing patterns {missing}\n{out}"
                        if result.execute_success and meta.get('run_exact'):
                            want = meta['run_exact'][0]
                            got = out.strip()
                            if got != want.strip():
                                result.execute_success = False
                                result.error_message = f"Run exact mismatch.\nWanted:\n{want}\nGot:\n{out}"
                        result.execution_time = time.time() - start_time
                        return result
                # Facade strict tests for TS/Python are executed during transpile via harness
                if "v3_facade_smoke" in parts_lower and language in ("typescript", "python", "rust", "c", "cpp"):
                    result.execute_success = True
                    result.output = "Facade harness executed"
                    result.execution_time = time.time() - start_time
                    return result
                # For other V3 categories, optionally execute selected sets for Python/TypeScript
                if any(seg.startswith("v3_") for seg in parts_lower):
                    if getattr(self.config, 'exec_v3', False) and language in ("python", "typescript") and any(seg in ("v3_core", "v3_control_flow", "v3_systems") for seg in parts_lower):
                        meta = self.parse_fixture_meta(test_file)
                        if not (meta.get('run_expect') or meta.get('exec_ok')):
                            result.execute_success = True
                            result.output = "V3 exec gated: no @run-expect/@exec-ok"
                            result.execution_time = time.time() - start_time
                            return result
                        if language == "python":
                            exec_success, output = self.execute_python(output_file)
                        else:
                            exec_success, output = self.execute_typescript(output_file)
                        result.execute_success = exec_success
                        result.output = output
                        # Apply @run-expect/@run-exact for exec-v3
                        if result.execute_success and meta.get('run_expect'):
                            missing = [p for p in meta['run_expect'] if not re.search(p, output or "", re.MULTILINE)]
                            if missing:
                                result.execute_success = False
                                result.error_message = f"Run output expectation failed: missing patterns {missing}\n{output}"
                        if result.execute_success and meta.get('run_exact'):
                            want = meta['run_exact'][0]
                            got = (output or "").strip()
                            if got != want.strip():
                                result.execute_success = False
                                result.error_message = f"Run exact mismatch.\nWanted:\n{want}\nGot:\n{output}"
                        result.execution_time = time.time() - start_time
                        return result
                    # Otherwise skip exec for remaining V3 categories
                    result.execute_success = True
                    result.output = "V3 category: execution skipped"
                    result.execution_time = time.time() - start_time
                    return result
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
                elif language == "c":
                    # Compile and run generated C program when present
                    exec_success, output = self._execute_c_like_source(output_file, use_cpp=False)
                elif language == "cpp":
                    exec_success, output = self._execute_c_like_source(output_file, use_cpp=True)
                elif language == "java":
                    exec_success, output = self._execute_java_source(output_file)
                elif language == "csharp":
                    exec_success, output = self._execute_csharp_source(output_file)
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
        
        # Optionally shuffle test order
        rng = random.Random(self.config.seed) if self.config.shuffle else None
        items = sorted(tests.items())
        if rng:
            items = list(items)
            rng.shuffle(items)

        for category, test_files in items:
            if not test_files:
                continue
                
            print(f"\n{category}: {len(test_files)} tests")
            
            files = sorted(test_files)
            if rng:
                files = list(files)
                rng.shuffle(files)
            for test_file in files:
                # include/exclude filters
                ts = str(test_file)
                if self.config.include_patterns and not any(p in ts for p in self.config.include_patterns):
                    continue
                if self.config.exclude_patterns and any(p in ts for p in self.config.exclude_patterns):
                    continue
                # Skip language-specific tests for other languages
                if category.startswith("language_specific_"):
                    # Patterns:
                    #  - language_specific_<lang>
                    #  - language_specific_<lang>_v3_demos
                    parts = category.split("_")
                    lang = parts[2] if (len(parts) >= 3 and parts[0]=="language" and parts[1]=="specific") else parts[-1]
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
            print(f"  Build: {stats['transpile_success']}/{stats['total']} ({stats['transpile_rate']})")
            if self.config.validate:
                print(f"  Compiler Validation: {stats['validation_success']}/{stats['total']} ({stats['validation_rate']})")
            if self.config.execute:
                print(f"  Run: {stats['execute_success']}/{stats['total']} ({stats['execute_rate']})")
        
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

def generate_junit(results: List[TestResult], runner: FrameTestRunner) -> str:
    """Generate a minimal JUnit XML report from test results."""
    def xml_escape(s: str) -> str:
        return (s.replace('&','&amp;').replace('<','&lt;').replace('>','&gt;')
                  .replace("'","&apos;").replace('"','&quot;'))
    # Group by language for suites
    suites: Dict[str, List[TestResult]] = {}
    for r in results:
        suites.setdefault(r.language, []).append(r)
    xml_parts: List[str] = ["<?xml version=\"1.0\" encoding=\"UTF-8\"?>", "<testsuites>"]
    for lang, group in suites.items():
        total = len(group)
        failures = 0
        skipped = 0
        for r in group:
            failed = ((not r.is_negative_test and not runner.is_infinite_loop_test(Path(r.file)) and ((not r.transpile_success) or (runner.config.validate and not r.validation_success) or (runner.config.execute and not r.execute_success))) or
                      (r.is_negative_test and not r.expected_failure))
            if failed:
                failures += 1
            if r.skipped:
                skipped += 1
        xml_parts.append(f"  <testsuite name=\"{xml_escape(lang)}\" tests=\"{total}\" failures=\"{failures}\" skipped=\"{skipped}\">")
        for r in group:
            classname = f"{Path(r.file).parent.name}.{lang}"
            name = Path(r.file).name
            xml_parts.append(f"    <testcase classname=\"{xml_escape(classname)}\" name=\"{xml_escape(name)}\">")
            if r.skipped:
                xml_parts.append(f"      <skipped message=\"{xml_escape(r.skipped)}\"/>")
            failed = ((not r.is_negative_test and not runner.is_infinite_loop_test(Path(r.file)) and ((not r.transpile_success) or (runner.config.validate and not r.validation_success) or (runner.config.execute and not r.execute_success))) or
                      (r.is_negative_test and not r.expected_failure))
            if failed:
                msg = r.error_message or "Test failed"
                xml_parts.append(f"      <failure message=\"{xml_escape(msg[:200])}\">{xml_escape(msg)}</failure>")
            xml_parts.append("    </testcase>")
        xml_parts.append("  </testsuite>")
    xml_parts.append("</testsuites>")
    return "\n".join(xml_parts)

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Frame Test Runner')
    parser.add_argument('--languages', '-l', nargs='+', default=['python', 'typescript'],
                       choices=['python', 'typescript', 'csharp', 'c', 'cpp', 'java', 'rust', 'golang', 'javascript', 'llvm'],
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
    parser.add_argument('--timeout', type=int, default=30,
                       help='Timeout for each test in seconds')
    parser.add_argument('--junit', dest='junit_path', help='Write JUnit XML report to this path')
    # Presets and selection
    parser.add_argument('--fast', action='store_true', help='Run a fast subset (outline/mir/validator + exec smoke)')
    parser.add_argument('--full', action='store_true', help='Run all V3 categories (alias for all_v3)')
    parser.add_argument('--include', action='append', dest='include_patterns', help='Only include tests matching this substring (can repeat)')
    parser.add_argument('--exclude', action='append', dest='exclude_patterns', help='Exclude tests matching this substring (can repeat)')
    parser.add_argument('--shuffle', action='store_true', help='Shuffle test order')
    parser.add_argument('--seed', type=int, help='Seed for shuffle')
    parser.add_argument('--include-flaky', action='store_true', help='Include @flaky tests')
    # Aliases for build/run terminology
    parser.add_argument('--build-only', dest='build_only', action='store_true', help='Build only (no run); alias for --transpile-only')
    parser.add_argument('--run', dest='run', action='store_true', help='Enable running generated programs (alias for execute on)')
    parser.add_argument('--no-run', dest='no_run', action='store_true', help='Disable running generated programs (alias for execute off)')
    # Negative test policies
    parser.add_argument('--strict-negatives', dest='strict_negatives', action='store_true', help='Negatives must fail compiler validation (default)')
    parser.add_argument('--no-strict-negatives', dest='no_strict_negatives', action='store_true', help='Allow negatives to pass on build failure alone')
    parser.add_argument('--require-error-codes', dest='require_error_codes', action='store_true', help='Negatives must surface validator error codes (E###) (default)')
    parser.add_argument('--no-require-error-codes', dest='no_require_error_codes', action='store_true', help='Do not require E### codes in negatives')
    parser.add_argument('--exec-v3', action='store_true', help='Execute selected non-smoke V3 categories (python/typescript: v3_core, v3_control_flow, v3_systems)')
    parser.add_argument('--expect-mode', choices=['superset','equal'], default='superset', help='How to match expected validator error codes in negatives (default: superset)')
    
    args = parser.parse_args()
    
    # Expand pseudo-category 'all_v3' and presets
    categories = args.categories[:]
    if args.full and 'all_v3' not in categories:
        categories.append('all_v3')
    if 'all_v3' in categories:
        base = ['v3_core','v3_control_flow','v3_data_types','v3_operators','v3_scoping','v3_systems']
        categories = [c for c in categories if c != 'all_v3'] + base
    if args.fast:
        fast = ['v3_outline','v3_mir','v3_validator','v3_exec_smoke']
        categories = list(dict.fromkeys(fast))

    # Create config
    # Map build/run and policy flags
    exec_flag = True
    if args.transpile_only or getattr(args, 'build_only', False) or getattr(args, 'no_run', False):
        exec_flag = False
    elif getattr(args, 'run', False):
        exec_flag = True

    strict_neg = True
    if getattr(args, 'no_strict_negatives', False):
        strict_neg = False
    elif getattr(args, 'strict_negatives', False):
        strict_neg = True

    require_codes = True
    if getattr(args, 'no_require_error_codes', False):
        require_codes = False
    elif getattr(args, 'require_error_codes', False):
        require_codes = True

    config = TestConfig(
        framec_path=args.framec,
        languages=args.languages,
        categories=categories,
        verbose=args.verbose,
        execute=exec_flag,
        validate=not args.no_validate,
        validation_level=args.validation_level,
        validation_format=args.validation_format,
        timeout=args.timeout,
        include_common=args.include_common,
        strict_negatives=strict_neg,
        require_error_codes=require_codes,
        expected_error_mode=args.expect_mode,
        include_flaky=args.include_flaky,
        include_patterns=args.include_patterns,
        exclude_patterns=args.exclude_patterns,
        shuffle=args.shuffle,
        seed=args.seed,
        exec_v3=getattr(args, 'exec_v3', False),
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

    # Write JUnit report if requested
    if args.junit_path:
        try:
            junit = generate_junit(results, runner)
            Path(args.junit_path).write_text(junit)
            print(f"\nJUnit report written to {args.junit_path}")
        except Exception as e:
            print(f"Warning: failed to write JUnit report: {e}")

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
                entry = {
                    "transpile": r.transpile_success,
                    "validate": r.validation_success if config.validate else True,
                    "execute": r.execute_success if config.execute and not runner.is_infinite_loop_test(p) else True,
                    "negative": r.is_negative_test,
                    "infinite": runner.is_infinite_loop_test(p),
                }
                if r.validation_errors is not None and (r.is_negative_test or r.validation_errors):
                    entry["errors"] = r.validation_errors
                actuals.setdefault(rel_key, {})[r.language] = entry

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
                for key, lang, exp, got in diffs[:200]:
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
