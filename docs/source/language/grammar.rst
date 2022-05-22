
Frame Grammar
=============

.. code-block::


    frame_spec: code_block? '#' IDENTIFIER body system_params'##' code_block?

    body: iface_block? machine_block? actions_block? domain_block?

    system_params: start_state_params? start_state_enter_params? domain_override_params

    start_state_params: '$' parameter_list

    start_state_enter_params '>' start_state_enter_params

    domain_override_params: parameter_list

    iface_block: '-interface-' iface_method*

    iface_method: IDENTIFIER parameter_list? type? message_alias?

    message_alias: '@' '(' message_selector ')'

    machine_block: '-machine-' state*

    state: state_name parameter_list? dispatch_clause? def* event_handler*

    state_name: '$' IDENTIFIER

    dispatch_clause: '=>' state_name

    event_handler: message_selector parameter_list? defs_or_stmts*

    event_handler_terminator

    message_selector: '|' STRING '|'

    defs_or_stmts: def | stmt

    event_handler_terminator: return_stmt | continue_stmt

    return_stmt: '^' ( '(' expr ')' )?

    continue_stmt: ':>'

    def: ( 'var' | 'const' ) IDENTIFIER type? '=' expr

    stmt:    transition_stmt

    | change_state_stmt

    | test_stmt

    | state_stack_stmt

    | expr_stmt

    transition_stmt: exit_args? '->' enter_args? transition_label? '=>'? state_ref state_args?

    exit_args: expr_list

    enter_args: expr_list

    transition_label: STRING

    state_ref: '$' IDENTIFIER

    state_args: expr_list

    change_state_stmt: '->>' state_ref

    test_stmt: ( bool_test | string_test | number_test ) '::'

    bool_test: equality ( '?' | '?!' ) test_branch else_cont_branch* else_branch?

    else_cont_branch: ':>' test_branch

    else_branch: ':' test_branch

    test_branch: stmt* return_stmt?

    string_test: equality '?~' string_match_branch+ else_branch?

    string_match_branch: '/' str_patterns '/ stmt* ( return_stmt | ':>' )

    str_patterns: STRING ( '|' STRING )*

    number_test: equality '?#' number_match_branch+ else_branch?

    number_match_branch: '/' number_patterns '/ stmt* ( return_stmt | ':>' )

    number_patterns: NUMBER ( '|' NUMBER )*

    state_stack_stmt: state_stack_oper_expr

    expr: assignment

    assignment: equality ( '=' equality )?

    equality: comparison ( ( '!=' | '==' ) comparison )*

    comparison: term ( ( '>' | '>=' | '<' | '<=' ) term )*

    term: factor ( ( '+' | '-' ) factor )*

    factor: logical_xor ( ( '*' | '/' ) logical_xor )*

    logical_xor: logical_or ( '&|' logical_or )*

    logical_or: logical_and ( '||' logical_and )*

    logical_and: unary_expr ( '&&' unary_expr )*

    unary_expr: ( '!' | '-' ) unary_expr
        |  '(' expr_list ')
        | '#' '.' IDENTIFIER
        | '$' '[' IDENTIFIER ']

        | '$' '.' IDENTIFIER

        | '||[' IDENTIFIER ']'

        | '||.' IDENTIFIER ']'

        | '&'? variable_or_call_expr

        | literal_expr

        | state_stack_oper_expr

        | frame_event_part_expr

        | expr_list

        | call_chain_expr

    call_chain_expr: 	variable_or_call_expr ( '.' variable_or_call_expr )*

    variable_or_call_expr: IDENTIFIER expr_list?

    expr_list: '(' expr* ')'

    literal_expr: NUMBER | STRING | 'true' | 'false' | 'null' | 'nil' | inline_code_block

    state_stack_oper_expr: '$$[+]' | '$$[-]'

    frame_event_part_expr: '@' ( '||' | '[' IDENTIFIER ']' | '^' )?

    actions_block: '-actions-' action*

    action: IDENTIFIER parameter_list? type? ( {` STRING `} )?

    domain_block: '-domain-' def*

    parameter_list: '[' parameter+ ']'

    parameter: IDENTIFIER type?

    type: ':' ( IDENTIFIER | inline_code_block )

    code_block: '```' STRING '```'

    inline_code_block: '`' STRING '`'
