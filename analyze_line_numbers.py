#!/usr/bin/env python3
"""
Analyze Frame transpiler AST nodes for line number tracking.
Identifies nodes that lack line number information for debugging support.
"""

import re
import sys

def analyze_ast_file(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find all struct definitions
    struct_pattern = r'pub struct (\w+Node|\w+Expr|\w+Stmt)\s*\{([^}]*)\}'
    structs = re.findall(struct_pattern, content, re.DOTALL)
    
    has_line = []
    no_line = []
    
    for struct_name, struct_body in structs:
        if 'pub line:' in struct_body:
            has_line.append(struct_name)
        else:
            no_line.append(struct_name)
    
    # Categorize by type
    statement_nodes = [n for n in no_line if 'Stmt' in n]
    expression_nodes = [n for n in no_line if 'Expr' in n] 
    other_nodes = [n for n in no_line if 'Stmt' not in n and 'Expr' not in n]
    
    print("=== NODES WITH LINE NUMBERS ===")
    for node in sorted(has_line):
        print(f"✅ {node}")
    
    print(f"\n=== STATEMENT NODES MISSING LINE NUMBERS ({len(statement_nodes)}) ===")
    for node in sorted(statement_nodes):
        print(f"❌ {node}")
    
    print(f"\n=== EXPRESSION NODES MISSING LINE NUMBERS ({len(expression_nodes)}) ===")
    for node in sorted(expression_nodes):
        print(f"❌ {node}")
    
    print(f"\n=== OTHER NODES MISSING LINE NUMBERS ({len(other_nodes)}) ===")
    for node in sorted(other_nodes):
        print(f"❌ {node}")
    
    print(f"\n=== SUMMARY ===")
    print(f"Total nodes with line numbers: {len(has_line)}")
    print(f"Total nodes missing line numbers: {len(no_line)}")
    print(f"  - Statement nodes: {len(statement_nodes)}")
    print(f"  - Expression nodes: {len(expression_nodes)}")
    print(f"  - Other nodes: {len(other_nodes)}")

if __name__ == "__main__":
    analyze_ast_file("framec/src/frame_c/ast.rs")