#!/usr/bin/env python3
"""
Model-Driven Transpiler Workflow Implementation
Integrates with existing test framework to support the model creation -> transpiler fix workflow
"""

import os
import shutil
from pathlib import Path
import subprocess
import sys

script_dir = Path(__file__).parent
src_dir = script_dir.parent / "src"
models_dir = script_dir.parent / "models"

def create_models_from_fixes():
    """
    Step 2 of workflow: Save current hand-fixed Python files as models
    """
    models_dir.mkdir(exist_ok=True)
    
    # Find Python files that have been hand-fixed
    py_files = list(src_dir.glob("*.py"))
    py_files = [f for f in py_files if 'Test.py' in f.name or f.name.startswith('test_')]
    
    saved_count = 0
    
    print("🎯 Creating Models from Hand-Fixed Python Files")
    print("=" * 50)
    
    for py_file in py_files:
        # Test if the file runs successfully
        try:
            result = subprocess.run([sys.executable, str(py_file)], 
                                  capture_output=True, text=True, 
                                  cwd=str(src_dir), timeout=5)
            
            if result.returncode == 0:
                # This file works - save it as a model
                model_file = models_dir / f"{py_file.stem}.model.py"
                shutil.copy2(py_file, model_file)
                print(f"✅ Saved model: {model_file.name}")
                saved_count += 1
            else:
                print(f"❌ Skipped (still broken): {py_file.name}")
        except subprocess.TimeoutExpired:
            print(f"⏱️ Skipped (timeout): {py_file.name}")
        except Exception as e:
            print(f"❌ Error testing {py_file.name}: {e}")
    
    print(f"\n📊 Created {saved_count} model files in {models_dir}")
    return saved_count

def regenerate_and_compare():
    """
    Step 4 of workflow: Regenerate from transpiler and compare with models
    """
    if not models_dir.exists():
        print("❌ No models directory found. Run create_models_from_fixes() first.")
        return False
    
    print("🔄 Regenerating Files and Comparing with Models")
    print("=" * 50)
    
    model_files = list(models_dir.glob("*.model.py"))
    matches = 0
    differences = 0
    
    for model_file in model_files:
        # Get corresponding .frm file
        base_name = model_file.stem.replace('.model', '')
        frm_file = src_dir / f"{base_name}.frm"
        
        if not frm_file.exists():
            print(f"⚠️ No .frm file found for {model_file.name}")
            continue
        
        # Regenerate Python from .frm
        generated_file = src_dir / f"{base_name}.py"
        try:
            result = subprocess.run([
                "/Users/marktruluck/projects/frame_transpiler/target/debug/framec",
                "-l", "python_3", str(frm_file)
            ], capture_output=True, text=True, cwd=str(src_dir))
            
            if result.returncode != 0:
                print(f"❌ Failed to regenerate {base_name}.py")
                print(f"   Error: {result.stderr}")
                continue
            
            # Write generated content to file
            with open(generated_file, 'w') as f:
                f.write(result.stdout)
            
            # Compare with model
            with open(model_file, 'r') as f:
                model_content = f.read()
            
            with open(generated_file, 'r') as f:
                generated_content = f.read()
            
            if model_content.strip() == generated_content.strip():
                print(f"✅ MATCH: {base_name}.py matches model")
                matches += 1
            else:
                print(f"❌ DIFF: {base_name}.py differs from model")
                differences += 1
                
                # Save diff for analysis
                diff_file = models_dir / f"{base_name}.diff.txt"
                with open(diff_file, 'w') as f:
                    f.write(f"=== DIFF: {base_name} ===\n")
                    f.write("MODEL VERSION:\n")
                    f.write(model_content[:500] + "...\n\n")
                    f.write("GENERATED VERSION:\n")
                    f.write(generated_content[:500] + "...\n\n")
                
        except Exception as e:
            print(f"❌ Error processing {base_name}: {e}")
    
    print(f"\n📊 Comparison Results:")
    print(f"✅ Matches: {matches}")
    print(f"❌ Differences: {differences}")
    
    if differences > 0:
        print(f"\n🔍 Check {models_dir}/*.diff.txt for detailed comparisons")
    
    return differences == 0

def show_current_status():
    """
    Show current status of models vs generated files
    """
    print("📊 Model-Driven Workflow Status")
    print("=" * 50)
    
    # Count working Python files
    py_files = list(src_dir.glob("*.py"))
    py_files = [f for f in py_files if 'Test.py' in f.name or f.name.startswith('test_')]
    
    working_count = 0
    broken_count = 0
    
    for py_file in py_files:
        try:
            result = subprocess.run([sys.executable, str(py_file)], 
                                  capture_output=True, text=True, 
                                  cwd=str(src_dir), timeout=5)
            if result.returncode == 0:
                working_count += 1
            else:
                broken_count += 1
        except:
            broken_count += 1
    
    total = working_count + broken_count
    success_rate = (working_count / total * 100) if total > 0 else 0
    
    print(f"Generated Python Files: {working_count}/{total} working ({success_rate:.1f}%)")
    
    # Count model files
    model_count = len(list(models_dir.glob("*.model.py"))) if models_dir.exists() else 0
    print(f"Model Files Available: {model_count}")
    
    # Recommend next step
    if broken_count > 5 and model_count == 0:
        print(f"\n💡 RECOMMENDATION: Run create_models_from_fixes() - many files broken, need models")
    elif model_count > 0 and broken_count > 0:
        print(f"\n💡 RECOMMENDATION: Fix transpiler using models, then run regenerate_and_compare()")
    elif model_count > 0 and broken_count == 0:
        print(f"\n💡 RECOMMENDATION: Run regenerate_and_compare() to validate transpiler fixes")
    else:
        print(f"\n✅ All files working - no model workflow needed")

def main():
    print("🎯 Model-Driven Transpiler Workflow")
    print("Usage:")
    print("  python3 model_workflow.py status        # Show current status")
    print("  python3 model_workflow.py create-models # Save working Python as models")
    print("  python3 model_workflow.py compare       # Regenerate and compare with models")
    
    if len(sys.argv) > 1:
        command = sys.argv[1]
        if command == "status":
            show_current_status()
        elif command == "create-models":
            create_models_from_fixes()
        elif command == "compare":
            regenerate_and_compare()
        else:
            print(f"Unknown command: {command}")
    else:
        show_current_status()

if __name__ == "__main__":
    main()