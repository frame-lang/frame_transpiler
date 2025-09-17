#!/usr/bin/env python3
"""
Categorize Frame AST nodes by debugging importance.
"""

# Node categories for debugging importance
HIGH_PRIORITY = {
    # Primary executable statements that a debugger would step through
    'CONTROL_FLOW': [
        'IfStmtNode',
        'ForStmtNode', 
        'WhileStmtNode',
        'MatchStmtNode',
        'TryStmtNode',
        'WithStmtNode'
    ],
    'FUNCTION_CALLS': [
        'CallStmtNode',
        'ActionCallStmtNode',
        'CallExprNode',
        'ActionCallExprNode',
        'InterfaceMethodCallExprNode',
        'OperationCallExprNode'
    ],
    'ASSIGNMENTS': [
        'AssignmentStmtNode',  # Note: AssignmentExprNode already has line numbers
        'VariableStmtNode'
    ],
    'CONTROL_STATEMENTS': [
        'ReturnStmtNode',
        'ReturnAssignStmtNode',
        'BreakStmtNode',
        'ContinueStmtNode',
        'RaiseStmtNode',
        'DelStmtNode',
        'AssertStmtNode'
    ],
    'STATE_MACHINE': [
        'TransitionExprNode',
        'TransitionStatementNode'
    ]
}

MEDIUM_PRIORITY = {
    # Expression nodes that represent evaluatable code
    'EXPRESSIONS': [
        'BinaryExprNode',
        'UnaryExprNode', 
        'LiteralExprNode',
        'AwaitExprNode',
        'YieldExprNode',
        'YieldFromExprNode',
        'LambdaExprNode',
        'GeneratorExprNode'
    ],
    'LOOP_VARIANTS': [
        'LoopStmtNode',
        'LoopInStmtNode',
        'LoopForStmtNode',
        'LoopInfiniteStmtNode'
    ],
    'SYSTEM_EXPRESSIONS': [
        'SystemInstanceStmtNode',
        'SystemTypeStmtNode',
        'SystemInstanceExprNode',
        'SystemTypeExprNode'
    ]
}

LOW_PRIORITY = {
    # Structural nodes and containers
    'CONTAINERS': [
        'BlockStmtNode',
        'ExprListStmtNode',
        'ListStmtNode',
        'BinaryStmtNode'
    ],
    'LITERALS': [
        'CallChainStmtNode',
        'CallChainExprNode',
        'CallChainLiteralExprNode',
        'EnumeratorStmtNode',
        'EnumeratorExprNode',
        'SelfExprNode'
    ],
    'COLLECTION_OPS': [
        'StarExprNode',
        'UnpackExprNode',
        'DictUnpackExprNode'
    ],
    'CALL_HELPERS': [
        'CallExprListNode',
        'ExprListNode',
        'OperationRefExprNode'
    ]
}

STRUCTURAL_ONLY = {
    # Purely structural - generally don't need line numbers for debugging
    'DECLARATIONS': [
        'ActionNode',
        'OperationNode', 
        'VariableDeclNode',
        'LoopVariableDeclNode',
        'VariableNode',
        'EnumDeclNode',
        'EnumeratorDeclNode'
    ],
    'BLOCKS': [
        'ActionsBlockNode',
        'OperationsBlockNode',
        'MachineBlockNode',
        'DomainBlockNode',
        'InterfaceBlockNode'
    ],
    'TYPE_SYSTEM': [
        'TypeNode',
        'ParameterNode',
        'InterfaceMethodNode'
    ],
    'PATTERNS': [
        'CaseNode',
        'ExceptClauseNode',
        'BoolTestNode',
        'BoolTestConditionalBranchNode',
        'BoolTestElseBranchNode',
        'StringMatchTestNode',
        'StringMatchTestMatchBranchNode',
        'StringMatchTestElseBranchNode',
        'StringMatchTestPatternNode',
        'NumberMatchTestNode',
        'NumberMatchTestMatchBranchNode', 
        'NumberMatchTestElseBranchNode',
        'NumberMatchTestPatternNode',
        'EnumMatchTestNode',
        'EnumMatchTestMatchBranchNode',
        'EnumMatchTestElseBranchNode',
        'EnumMatchTestPatternNode'
    ],
    'COLLECTIONS': [
        'DictLiteralNode',
        'SetLiteralNode',
        'TupleLiteralNode',
        'ListNode',
        'ListElementNode',
        'SliceNode',
        'ListComprehensionNode',
        'DictComprehensionNode',
        'SetComprehensionNode'
    ],
    'REFERENCES': [
        'StateRefNode',
        'TargetStateContextNode',
        'PropertyNode',
        'ModuleNode'
    ],
    'RUNTIME_INFO': [
        'KernelNode',
        'RouterNode',
        'StateDispatcherNode',
        'TransitionNode',
        'TestStatementNode',
        'StateStackOperationNode',
        'StateStackOperationStatementNode'
    ]
}

def categorize_nodes():
    print("=== FRAME AST DEBUGGING PRIORITY ANALYSIS ===\n")
    
    print("🔴 HIGH PRIORITY - Critical for step-through debugging:")
    total_high = 0
    for category, nodes in HIGH_PRIORITY.items():
        print(f"\n  {category}:")
        for node in sorted(nodes):
            print(f"    • {node}")
            total_high += 1
    
    print(f"\n🟡 MEDIUM PRIORITY - Helpful for expression debugging:")
    total_medium = 0
    for category, nodes in MEDIUM_PRIORITY.items():
        print(f"\n  {category}:")
        for node in sorted(nodes):
            print(f"    • {node}")
            total_medium += 1
    
    print(f"\n🟠 LOW PRIORITY - Less critical but still useful:")
    total_low = 0
    for category, nodes in LOW_PRIORITY.items():
        print(f"\n  {category}:")
        for node in sorted(nodes):
            print(f"    • {node}")
            total_low += 1
    
    print(f"\n⚪ STRUCTURAL ONLY - Generally don't need line numbers:")
    total_structural = 0
    for category, nodes in STRUCTURAL_ONLY.items():
        print(f"\n  {category}:")
        for node in sorted(nodes):
            print(f"    • {node}")
            total_structural += 1
    
    print(f"\n=== IMPLEMENTATION RECOMMENDATION ===")
    print(f"High Priority ({total_high} nodes): Implement first - essential for debugging")
    print(f"Medium Priority ({total_medium} nodes): Implement second - valuable for expression debugging")
    print(f"Low Priority ({total_low} nodes): Implement third - completeness")
    print(f"Structural ({total_structural} nodes): Consider only if needed for specific use cases")
    
    print(f"\nTotal nodes analyzed: {total_high + total_medium + total_low + total_structural}")

if __name__ == "__main__":
    categorize_nodes()