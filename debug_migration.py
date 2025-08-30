#!/usr/bin/env python3
import re

content = '''fn main() {
    var sys = TestSystem()
    sys.process()
}

system TestSystem {
    interface:
        process()
        
    machine:
        $Ready {
            process() {
                doWork()
                return
            }
        }
        
    actions:
        doWork() {
            print("Working with total: " + str(self.total))
        }
        
    domain:
        var total: int = 0
}'''

lines = content.split('\n')
current_block = None

for line_num, line in enumerate(lines, 1):
    stripped = line.strip()
    print(f"Line {line_num}: '{line}' -> stripped: '{stripped}'")
    
    if 'actions:' in line:
        current_block = 'actions'
        print(f"  -> Set current_block to 'actions'")
    elif current_block == 'actions':
        # Debug the regex
        match = re.match(r'\s+(\w+)\s*\(', stripped)
        print(f"  -> current_block = {current_block}, regex match: {match}")
        if match:
            print(f"  -> Match found: '{match.group(1)}'")
        # Check the actual pattern
        if re.match(r'\s*(\w+)\s*\(', stripped):  # Allow no leading space in stripped
            match2 = re.match(r'\s*(\w+)\s*\(', stripped)
            print(f"  -> Alternative pattern match: '{match2.group(1)}'")
        # Check on original line
        if re.match(r'\s+(\w+)\s*\(', line):
            match3 = re.match(r'\s+(\w+)\s*\(', line)
            print(f"  -> Original line pattern match: '{match3.group(1)}'")
    
    if line.strip().startswith('}'):
        if current_block:
            print(f"  -> End of {current_block} block")
        current_block = None