#!/usr/bin/env python3
"""
Source Map Test Framework Integration
Integrates source map validation into Frame test suite as source of truth
"""
import json
import os
import sys
from pathlib import Path
import subprocess
from typing import Dict, List, Tuple, Optional

# Import the validation tool
sys.path.append(os.path.dirname(os.path.abspath(__file__)))
from source_map_validator import analyze_source_map

class SourceMapTestResult:
    def __init__(self, file_path: str, analysis: Dict):
        self.file_path = file_path
        self.analysis = analysis
        self.passed = self._assess_quality()
        
    def _assess_quality(self) -> bool:
        """Determine if source map quality passes minimum standards"""
        if not self.analysis:
            return False
            
        # Check for critical issues
        if self.analysis.get('main_function_analysis'):
            main = self.analysis['main_function_analysis']
            unmapped_executable = [line for line in main.get('unmapped_lines', [])
                                 if not line['content'].startswith('#') and line['content'] not in ['}', '{']]
            executable_coverage = (main['mapped_lines'] / 
                                 (main['mapped_lines'] + len(unmapped_executable))) * 100
            
            # Minimum standard: 95% executable coverage
            if executable_coverage < 95:
                return False
                
        # Check duplicate mappings - allow up to 3 minor duplicates
        duplicates = len(self.analysis.get('duplicates', []))
        if duplicates > 5:  # More than 5 duplicates is problematic
            return False
            
        return True
        
    def get_summary(self) -> str:
        """Get concise summary for test runner output"""
        if not self.analysis:
            return "❌ FAILED: Analysis failed"
            
        if not self.analysis.get('main_function_analysis'):
            return "⚠️  SKIPPED: No main function to analyze"
            
        main = self.analysis['main_function_analysis']
        unmapped_executable = [line for line in main.get('unmapped_lines', [])
                             if not line['content'].startswith('#') and line['content'] not in ['}', '{']]
        executable_coverage = (main['mapped_lines'] / 
                             (main['mapped_lines'] + len(unmapped_executable))) * 100
        
        duplicates = len(self.analysis.get('duplicates', []))
        status = "✅ PASS" if self.passed else "❌ FAIL"
        
        return f"{status}: {executable_coverage:.0f}% coverage, {duplicates} duplicates"

class SourceMapTestRunner:
    def __init__(self, framec_path: str = "./target/release/framec"):
        self.framec_path = framec_path
        self.results: List[SourceMapTestResult] = []
        
    def validate_file(self, frm_file: str) -> SourceMapTestResult:
        """Validate source map for a single .frm file"""
        try:
            analysis = analyze_source_map(frm_file)
            result = SourceMapTestResult(frm_file, analysis)
            self.results.append(result)
            return result
        except Exception as e:
            print(f"Error validating {frm_file}: {e}")
            result = SourceMapTestResult(frm_file, None)
            self.results.append(result)
            return result
            
    def validate_test_suite(self, test_dir: str) -> Tuple[int, int]:
        """Validate source maps for entire test suite"""
        test_files = list(Path(test_dir).glob("**/*.frm"))
        print(f"🔍 Validating source maps for {len(test_files)} test files...")
        
        passed = 0
        failed = 0
        
        for frm_file in test_files:
            # Skip negative test files that are expected to fail transpilation
            if "negative_tests" in str(frm_file):
                continue
                
            result = self.validate_file(str(frm_file))
            
            if result.passed:
                passed += 1
            else:
                failed += 1
                print(f"   {os.path.basename(frm_file)}: {result.get_summary()}")
                
        return passed, failed
        
    def generate_quality_report(self) -> Dict:
        """Generate comprehensive quality report for VS Code extension"""
        total_files = len(self.results)
        passed_files = sum(1 for r in self.results if r.passed)
        
        # Aggregate statistics
        total_duplicates = sum(len(r.analysis.get('duplicates', [])) for r in self.results if r.analysis)
        avg_coverage = sum(
            r.analysis['main_function_analysis'].get('coverage', 0) 
            for r in self.results 
            if r.analysis and r.analysis.get('main_function_analysis')
        ) / max(1, sum(1 for r in self.results if r.analysis and r.analysis.get('main_function_analysis')))
        
        # Quality classification
        if passed_files / total_files >= 0.95:
            quality = "EXCELLENT"
        elif passed_files / total_files >= 0.9:
            quality = "GOOD"
        elif passed_files / total_files >= 0.8:
            quality = "FAIR"
        else:
            quality = "POOR"
            
        return {
            "timestamp": subprocess.run(['date', '-Iseconds'], capture_output=True, text=True).stdout.strip(),
            "transpiler_version": self._get_transpiler_version(),
            "total_files": total_files,
            "passed_files": passed_files,
            "failed_files": total_files - passed_files,
            "pass_rate": passed_files / total_files * 100,
            "total_duplicates": total_duplicates,
            "average_coverage": avg_coverage,
            "quality_classification": quality,
            "recommendations": self._generate_recommendations(),
            "failed_files": [
                {
                    "file": os.path.basename(r.file_path),
                    "summary": r.get_summary()
                }
                for r in self.results if not r.passed
            ]
        }
        
    def _get_transpiler_version(self) -> str:
        """Get transpiler version"""
        try:
            result = subprocess.run([self.framec_path, '--version'], 
                                  capture_output=True, text=True)
            return result.stdout.strip()
        except:
            return "unknown"
            
    def _generate_recommendations(self) -> List[str]:
        """Generate improvement recommendations"""
        recommendations = []
        
        failed_results = [r for r in self.results if not r.passed]
        if len(failed_results) > len(self.results) * 0.1:
            recommendations.append("High failure rate detected - review source mapping generation")
            
        high_duplicate_files = [r for r in self.results 
                              if r.analysis and len(r.analysis.get('duplicates', [])) > 5]
        if high_duplicate_files:
            recommendations.append("Multiple files with excessive duplicate mappings - review CodeBuilder usage")
            
        return recommendations

def main():
    """Command line interface for source map testing"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Frame Source Map Validation Test Runner")
    parser.add_argument("--test-dir", default="/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src",
                       help="Directory containing test files")
    parser.add_argument("--framec", default="./target/release/framec",
                       help="Path to framec binary")
    parser.add_argument("--report", help="Generate JSON quality report for VS Code extension")
    parser.add_argument("--file", help="Validate single .frm file")
    parser.add_argument("--fail-on-quality", action="store_true",
                       help="Exit with non-zero code if quality standards not met")
    
    args = parser.parse_args()
    
    runner = SourceMapTestRunner(args.framec)
    
    if args.file:
        # Single file validation
        result = runner.validate_file(args.file)
        print(f"{os.path.basename(args.file)}: {result.get_summary()}")
        sys.exit(0 if result.passed else 1)
    else:
        # Test suite validation
        passed, failed = runner.validate_test_suite(args.test_dir)
        
        print(f"\n📊 Source Map Quality Summary:")
        print(f"   Passed: {passed}")
        print(f"   Failed: {failed}")
        print(f"   Success Rate: {passed/(passed+failed)*100:.1f}%")
        
        # Generate quality report for VS Code extension
        if args.report:
            report = runner.generate_quality_report()
            with open(args.report, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"   Quality report saved to: {args.report}")
            
        # Fail if quality standards not met and flag is set
        if args.fail_on_quality and failed > 0:
            print("❌ Source map quality standards not met")
            sys.exit(1)
        else:
            print("✅ Source map validation complete")

if __name__ == "__main__":
    main()