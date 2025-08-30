#!/usr/bin/env python3
"""
Frame v0.31 Migration Script: Convert implicit calls to explicit self/system syntax
"""

import re
import sys
from pathlib import Path
import argparse

class FrameMigrator:
    def __init__(self, verbose=False):
        self.verbose = verbose
        self.actions = set()
        self.operations = set()
        self.domain_vars = set()
        self.interface_methods = set()
        self.changes_made = []
    
    def log(self, message):
        if self.verbose:
            print(f"  {message}")
    
    def analyze_file(self, content):
        """First pass: collect all actions, operations, domain vars, interface methods"""
        lines = content.split('\n')
        current_block = None
        
        for line_num, line in enumerate(lines, 1):
            stripped = line.strip()
            
            # Detect block transitions
            if 'actions:' in line:
                current_block = 'actions'
                self.log(f"Line {line_num}: Found actions block")
            elif 'operations:' in line:
                current_block = 'operations'
                self.log(f"Line {line_num}: Found operations block")
            elif 'domain:' in line:
                current_block = 'domain'
                self.log(f"Line {line_num}: Found domain block")
            elif 'interface:' in line:
                current_block = 'interface'
                self.log(f"Line {line_num}: Found interface block")
            elif 'machine:' in line:
                current_block = 'machine'
                self.log(f"Line {line_num}: Found machine block")
            elif line.strip().startswith('}') or 'fn ' in line or 'system ' in line:
                if current_block:
                    self.log(f"Line {line_num}: End of {current_block} block")
                current_block = None
            
            # Collect definitions based on current block
            if current_block == 'actions':
                # Match: methodName( or methodName() {
                match = re.match(r'\s+(\w+)\s*\(', line)
                if match and not stripped.startswith('//') and '=' not in line[:line.find('(') if '(' in line else len(line)]:
                    action_name = match.group(1)
                    self.actions.add(action_name)
                    self.log(f"Line {line_num}: Found action '{action_name}'")
            
            elif current_block == 'operations':
                # Match: methodName( or methodName(): type {
                match = re.match(r'\s+(\w+)\s*\(', stripped)
                if match:
                    op_name = match.group(1)
                    self.operations.add(op_name)
                    self.log(f"Line {line_num}: Found operation '{op_name}'")
            
            elif current_block == 'domain':
                # Match: var variableName
                match = re.match(r'\s+var\s+(\w+)', stripped)
                if match:
                    var_name = match.group(1)
                    self.domain_vars.add(var_name)
                    self.log(f"Line {line_num}: Found domain variable '{var_name}'")
            
            elif current_block == 'interface':
                # Match: methodName( or methodName(): type
                match = re.match(r'\s+(\w+)\s*\(', stripped)
                if match:
                    method_name = match.group(1)
                    self.interface_methods.add(method_name)
                    self.log(f"Line {line_num}: Found interface method '{method_name}'")
    
    def migrate_line(self, line, line_num, context):
        """Migrate a single line based on context"""
        original_line = line
        
        # Skip comments and string literals (simplified)
        if '//' in line or '"' in line or "'" in line:
            return line
        
        # Only process lines in action/operation/machine blocks
        if context not in ['actions', 'operations', 'machine']:
            return line
        
        # Replace action calls with self.action()
        for action in self.actions:
            # Pattern: actionName( but not self.actionName( and not in declarations
            pattern = r'\b' + re.escape(action) + r'\s*\('
            if re.search(pattern, line) and 'self.' not in line:
                # Avoid replacing in method declarations
                if not re.match(r'\s+' + re.escape(action) + r'\s*\(', line):
                    old_line = line
                    line = re.sub(pattern, f'self.{action}(', line)
                    if line != old_line:
                        self.changes_made.append(f"Line {line_num}: {action}() -> self.{action}()")
                        self.log(f"Line {line_num}: Changed action call '{action}()' to 'self.{action}()'")
        
        # Replace operation calls with self.operation()
        for operation in self.operations:
            # Only if not already an action (actions have precedence)
            if operation not in self.actions:
                pattern = r'\b' + re.escape(operation) + r'\s*\('
                if re.search(pattern, line) and 'self.' not in line:
                    # Avoid replacing in method declarations
                    if not re.match(r'\s+' + re.escape(operation) + r'\s*\(', line):
                        old_line = line
                        line = re.sub(pattern, f'self.{operation}(', line)
                        if line != old_line:
                            self.changes_made.append(f"Line {line_num}: {operation}() -> self.{operation}()")
                            self.log(f"Line {line_num}: Changed operation call '{operation}()' to 'self.{operation}()'")
        
        # Replace domain variable access with self.variable
        for var in self.domain_vars:
            # Pattern: var = or var + etc, but not var: (type annotations) and not var variableName
            patterns = [
                (r'\b' + re.escape(var) + r'\s*=', f'self.{var} ='),  # assignment
                (r'\b' + re.escape(var) + r'\s*\+', f'self.{var} +'),  # arithmetic
                (r'\b' + re.escape(var) + r'\s*\-', f'self.{var} -'),
                (r'\b' + re.escape(var) + r'\s*\*', f'self.{var} *'),
                (r'\b' + re.escape(var) + r'\s*/', f'self.{var} /'),
                (r'\b' + re.escape(var) + r'\s*>', f'self.{var} >'),  # comparisons
                (r'\b' + re.escape(var) + r'\s*<', f'self.{var} <'),
                (r'\b' + re.escape(var) + r'\s*==', f'self.{var} =='),
                (r'\b' + re.escape(var) + r'\s*!=', f'self.{var} !='),
                (r'\bstr\(' + re.escape(var) + r'\)', f'str(self.{var})'),  # function calls
                (r'\bprint\([^)]*' + re.escape(var) + r'[^)]*\)', lambda m: m.group(0).replace(var, f'self.{var}')),
            ]
            
            for pattern, replacement in patterns:
                if 'self.' not in line and 'var ' + var not in line:  # Skip declarations and existing self.
                    old_line = line
                    if callable(replacement):
                        line = re.sub(pattern, replacement, line)
                    else:
                        line = re.sub(pattern, replacement, line)
                    if line != old_line:
                        self.changes_made.append(f"Line {line_num}: {var} -> self.{var}")
                        self.log(f"Line {line_num}: Changed domain variable access '{var}' to 'self.{var}'")
                        break  # Only apply first matching pattern
        
        return line
    
    def migrate_file(self, filepath):
        """Migrate a single .frm file"""
        self.log(f"Analyzing {filepath}...")
        content = filepath.read_text()
        
        # Reset for each file
        self.actions.clear()
        self.operations.clear()
        self.domain_vars.clear()
        self.interface_methods.clear()
        self.changes_made.clear()
        
        # First pass: analyze
        self.analyze_file(content)
        
        self.log(f"Found: {len(self.actions)} actions, {len(self.operations)} operations, " +
                f"{len(self.domain_vars)} domain vars, {len(self.interface_methods)} interface methods")
        
        # Second pass: migrate
        lines = content.split('\n')
        new_lines = []
        current_context = None
        
        for line_num, line in enumerate(lines, 1):
            # Track context
            if 'actions:' in line:
                current_context = 'actions'
            elif 'operations:' in line:
                current_context = 'operations'
            elif 'machine:' in line:
                current_context = 'machine'
            elif 'domain:' in line:
                current_context = 'domain'
            elif 'interface:' in line:
                current_context = 'interface'
            elif line.strip().startswith('}') or 'fn ' in line or 'system ' in line:
                current_context = None
            
            # Migrate the line
            new_line = self.migrate_line(line, line_num, current_context)
            new_lines.append(new_line)
        
        return '\n'.join(new_lines)
    
    def print_summary(self):
        if self.changes_made:
            print(f"Made {len(self.changes_made)} changes:")
            for change in self.changes_made:
                print(f"  {change}")
        else:
            print("No changes made.")

def main():
    parser = argparse.ArgumentParser(description='Migrate Frame files to explicit self/system syntax')
    parser.add_argument('pattern', nargs='?', default='*.frm', help='File pattern to process')
    parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be changed without modifying files')
    parser.add_argument('--backup', action='store_true', help='Create backup files (.frm.backup)')
    
    args = parser.parse_args()
    
    # Find all .frm files
    src_dir = Path('framec_tests/python/src')
    if not src_dir.exists():
        print(f"Error: Directory {src_dir} not found")
        sys.exit(1)
    
    frame_files = list(src_dir.glob(args.pattern))
    if not frame_files:
        print(f"No .frm files found matching pattern: {args.pattern}")
        return
    
    print(f"Found {len(frame_files)} Frame files to process")
    
    total_changes = 0
    for filepath in sorted(frame_files):
        print(f"\nProcessing {filepath.name}...")
        migrator = FrameMigrator(verbose=args.verbose)
        
        try:
            new_content = migrator.migrate_file(filepath)
            
            if migrator.changes_made:
                total_changes += len(migrator.changes_made)
                
                if args.dry_run:
                    print(f"  Would make {len(migrator.changes_made)} changes:")
                    migrator.print_summary()
                else:
                    # Create backup if requested
                    if args.backup:
                        backup_path = filepath.with_suffix('.frm.backup')
                        backup_path.write_text(filepath.read_text())
                        print(f"  Created backup: {backup_path.name}")
                    
                    # Write migrated version
                    filepath.write_text(new_content)
                    print(f"  Made {len(migrator.changes_made)} changes")
                    if args.verbose:
                        migrator.print_summary()
            else:
                print("  No changes needed")
                
        except Exception as e:
            print(f"  Error processing {filepath}: {e}")
    
    if args.dry_run:
        print(f"\nDry run complete. Would make {total_changes} total changes.")
    else:
        print(f"\nMigration complete. Made {total_changes} total changes.")

if __name__ == '__main__':
    main()