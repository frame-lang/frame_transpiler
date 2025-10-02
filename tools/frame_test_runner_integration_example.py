#!/usr/bin/env python3
"""
Example: How to integrate source map validation into existing frame_test_runner.py

This shows how the existing test framework can optionally call source map validation
"""
import sys
import os

# Add source map validation tools to path
sys.path.append(os.path.join(os.path.dirname(__file__)))
from test_framework_integration import TestFrameworkIntegration

class EnhancedFrameTestRunner:
    """Example of existing frame_test_runner.py enhanced with source map validation"""
    
    def __init__(self, framec_path="./target/release/framec", test_dir="framec_tests/python/src"):
        self.framec_path = framec_path
        self.test_dir = test_dir
        self.source_map_integration = TestFrameworkIntegration()
        
    def run_tests_with_optional_validation(self, 
                                         validate_source_maps=False, 
                                         force_validation=False,
                                         ci_mode=False):
        """Enhanced test runner with optional source map validation"""
        
        print("🧪 Running Frame transpiler tests...")
        
        # Run existing tests (simplified for example)
        test_results = self._run_existing_tests()
        
        # Determine if source map validation should run
        should_validate = (
            force_validation or 
            validate_source_maps or
            self.source_map_integration.should_run_validation(
                explicit_request=validate_source_maps,
                ci_mode=ci_mode
            )
        )
        
        validation_results = None
        if should_validate:
            print("\n🔍 Running source map validation (source of truth)...")
            validation_results = self.source_map_integration.validate_test_suite(
                self.test_dir, 
                self.framec_path,
                output_format="ci" if ci_mode else "console"
            )
            
        return test_results, validation_results
    
    def _run_existing_tests(self):
        """Placeholder for existing test runner logic"""
        # This would contain the existing frame_test_runner.py logic
        return {
            "total": 378,
            "passed": 378,
            "failed": 0,
            "success_rate": 100.0
        }

def main():
    """Example command line interface with source map validation"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Enhanced Frame Test Runner")
    parser.add_argument("--all", action="store_true", help="Run all tests")
    parser.add_argument("--validate-source-maps", action="store_true", 
                       help="Run source map validation")
    parser.add_argument("--force-validation", action="store_true",
                       help="Force source map validation regardless of config")
    parser.add_argument("--ci", action="store_true", help="CI mode")
    parser.add_argument("--framec", default="./target/release/framec", 
                       help="Path to framec binary")
    
    args = parser.parse_args()
    
    if args.all:
        runner = EnhancedFrameTestRunner(framec_path=args.framec)
        test_results, validation_results = runner.run_tests_with_optional_validation(
            validate_source_maps=args.validate_source_maps,
            force_validation=args.force_validation,
            ci_mode=args.ci
        )
        
        print(f"\n📊 FINAL SUMMARY")
        print(f"   Tests: {test_results['success_rate']:.1f}% pass rate")
        
        if validation_results:
            print(f"   Source Maps: {validation_results['quality_classification']} quality")
            
            # Exit with appropriate code for CI
            if args.ci and validation_results['quality_classification'] == 'POOR':
                print("❌ Source map quality below CI threshold")
                sys.exit(1)
            else:
                print("✅ All checks complete")
    else:
        print("Use --all to run tests with optional source map validation")

if __name__ == "__main__":
    main()