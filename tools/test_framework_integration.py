#!/usr/bin/env python3
"""
Frame Test Framework Integration for Source Map Validation
Integrates source map validation as source of truth into existing test framework
"""
import json
import os
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional

# Import our validation tools
sys.path.append(os.path.dirname(os.path.abspath(__file__)))
from source_map_test_integration import SourceMapTestRunner

class TestFrameworkIntegration:
    """Integration layer between Frame test framework and source map validation"""
    
    def __init__(self, config_path: str = None):
        self.config_path = config_path or os.path.join(os.path.dirname(__file__), "source_map_config.json")
        self.config = self._load_config()
        
    def _load_config(self) -> Dict:
        """Load validation configuration"""
        try:
            with open(self.config_path, 'r') as f:
                return json.load(f)
        except FileNotFoundError:
            # Default configuration if file missing
            return {
                "quality_standards": {
                    "minimum_executable_coverage": 95,
                    "maximum_acceptable_duplicates": 5
                },
                "test_integration": {
                    "enabled_by_default": False,
                    "generate_reports_for_vscode": True
                }
            }
    
    def should_run_validation(self, explicit_request: bool = False, ci_mode: bool = False) -> bool:
        """Determine if source map validation should run"""
        if explicit_request:
            return True
        if ci_mode and self.config.get("test_integration", {}).get("run_on_ci", True):
            return True
        return self.config.get("test_integration", {}).get("enabled_by_default", False)
    
    def validate_test_suite(self, test_dir: str, framec_path: str, 
                          output_format: str = "console") -> Dict:
        """Run source map validation on test suite"""
        print("🔍 Running source map validation (source of truth)...")
        
        runner = SourceMapTestRunner(framec_path)
        passed, failed = runner.validate_test_suite(test_dir)
        
        # Generate comprehensive report
        quality_report = runner.generate_quality_report()
        
        # Output based on format
        if output_format == "console":
            self._print_console_summary(quality_report, passed, failed)
        elif output_format == "json":
            return quality_report
        elif output_format == "ci":
            self._print_ci_summary(quality_report, passed, failed)
            
        # Generate VS Code extension report if configured
        if self.config.get("test_integration", {}).get("generate_reports_for_vscode", True):
            vscode_report_path = self.config.get("test_integration", {}).get(
                "report_output_path", "/tmp/frame_source_map_quality.json"
            )
            with open(vscode_report_path, 'w') as f:
                json.dump(quality_report, f, indent=2)
            print(f"📄 VS Code extension report: {vscode_report_path}")
        
        return quality_report
    
    def _print_console_summary(self, report: Dict, passed: int, failed: int):
        """Print console summary of validation results"""
        print(f"\n📊 SOURCE MAP VALIDATION SUMMARY ({report['transpiler_version']})")
        print(f"   Quality Classification: {report['quality_classification']}")
        print(f"   Pass Rate: {report['pass_rate']:.1f}% ({passed}/{passed+failed})")
        print(f"   Average Coverage: {report['average_coverage']:.1f}%")
        print(f"   Total Duplicates: {report['total_duplicates']}")
        
        if report.get('failed_files'):
            print(f"   Failed Files: {len(report['failed_files'])}")
            for failure in report['failed_files'][:3]:  # Show first 3
                print(f"     - {failure['file']}: {failure['summary']}")
            if len(report['failed_files']) > 3:
                print(f"     ... and {len(report['failed_files']) - 3} more")
        
        if report.get('recommendations'):
            print(f"   Recommendations:")
            for rec in report['recommendations']:
                print(f"     • {rec}")
    
    def _print_ci_summary(self, report: Dict, passed: int, failed: int):
        """Print CI-friendly summary"""
        status = "✅ PASS" if report['quality_classification'] in ['EXCELLENT', 'GOOD'] else "⚠️ WARN"
        print(f"{status} Source Map Quality: {report['quality_classification']} "
              f"({report['pass_rate']:.0f}% pass rate, {report['total_duplicates']} duplicates)")
    
    def validate_single_file(self, frm_file: str, framec_path: str) -> Dict:
        """Validate source map for single file (for VS Code extension)"""
        runner = SourceMapTestRunner(framec_path)
        result = runner.validate_file(frm_file)
        
        return {
            "file": frm_file,
            "passed": result.passed,
            "summary": result.get_summary(),
            "analysis": result.analysis,
            "timestamp": runner.generate_quality_report()["timestamp"],
            "transpiler_version": runner._get_transpiler_version()
        }
    
    def get_validation_status_for_vscode(self, test_dir: str, framec_path: str) -> Dict:
        """Get current validation status for VS Code extension consumption"""
        quality_report = self.validate_test_suite(test_dir, framec_path, "json")
        
        # Format for VS Code extension
        return {
            "status": "ready",
            "source_of_truth": True,
            "validation_tool": "Frame Transpiler Source Map Validator",
            "version": self.config.get("validation_config", {}).get("version", "1.0.0"),
            "quality": {
                "classification": quality_report['quality_classification'],
                "pass_rate": quality_report['pass_rate'],
                "coverage": quality_report['average_coverage'],
                "duplicates": quality_report['total_duplicates']
            },
            "standards": self.config.get("quality_standards", {}),
            "last_validation": quality_report['timestamp'],
            "transpiler_version": quality_report['transpiler_version'],
            "integration_ready": True
        }

def main():
    """Command line interface for test framework integration"""
    parser = argparse.ArgumentParser(description="Frame Test Framework Source Map Integration")
    parser.add_argument("--test-dir", default="/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src",
                       help="Test directory")
    parser.add_argument("--framec", default="./target/release/framec", help="Path to framec binary")
    parser.add_argument("--mode", choices=["console", "json", "ci", "vscode-status"], 
                       default="console", help="Output mode")
    parser.add_argument("--file", help="Validate single file")
    parser.add_argument("--ci", action="store_true", help="CI mode")
    parser.add_argument("--config", help="Config file path")
    
    args = parser.parse_args()
    
    integration = TestFrameworkIntegration(args.config)
    
    if args.file:
        # Single file validation for VS Code extension
        result = integration.validate_single_file(args.file, args.framec)
        print(json.dumps(result, indent=2))
    elif args.mode == "vscode-status":
        # VS Code extension status query
        status = integration.get_validation_status_for_vscode(args.test_dir, args.framec)
        print(json.dumps(status, indent=2))
    else:
        # Full test suite validation
        integration.validate_test_suite(args.test_dir, args.framec, args.mode)

if __name__ == "__main__":
    main()