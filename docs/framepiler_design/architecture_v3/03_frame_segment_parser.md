# Stage 3 — Frame Statement Parser (Tiny)

Purpose
- Parse SOL‑anchored Frame statement slices into MIR items. Only three Frame statements are supported in native regions:
  - Transition: `(exit_args)? -> (enter_args)? $State(state_params?)`
  - Parent forward: `=> $^`
  - Stack ops: `$$[+]` / `$$[-]`
  
Inline separation policy
- Frame statements follow the host language’s multi‑statement rules when they share a physical line with native code:
  - Python: end of Frame statement is LF or a top‑level semicolon `;` or start of a `#` comment.
  - TypeScript/C#/C/C++/Java/Rust: end of Frame statement is LF or a top‑level semicolon `;` or start of a line comment `//` (block `/* ... */` opens a native region and is not part of the Frame segment).
- When an inline separator is present, the parser only consumes the Frame statement portion; the remainder of the line is emitted as a native segment by the scanner.
- Without a valid separator, non‑whitespace tokens following a Frame statement on the same line are invalid.

Out of scope
- `system.return` is native‑only and rewritten by visitors; it is not parsed here.

Inputs
- `FrameSegment { start, end, kind_hint, indent }` and source bytes.

Outputs
- `MirItem::{ Transition{ target: StateRef, exit_args: Vec<ExprText>, enter_args: Vec<ExprText>, state_args: Vec<ExprText>, span }, Forward{ span }, StackPush{ span }, StackPop{ span } }`

Grammar (BNF‑ish)
```
transition   ::= exit_opt "-" ">" enter_opt label_opt? "$" state_ident state_params_opt
exit_opt     ::= /* empty */ | "(" arg_text ")"
enter_opt    ::= /* empty */ | "(" arg_text ")"
state_params_opt ::= /* empty */ | "(" arg_text ")"
label_opt    ::= /* empty */ | ident WS*
forward      ::= "=" ">" WS+ "$" "^"
stackpush    ::= "$" "$" "[" "+" "]"
stackpop     ::= "$" "$" "[" "-" "]"
state_ident  ::= [A-Za-z_][A-Za-z0-9_]*  /* align with common grammar for $State */
WS           ::= space | tab
```

Arg Text Handling
- The parser does not parse Python/TS expressions; it slices the raw text between the balanced parentheses for later visitor consumption. All three buckets (exit/enter/state) are balanced and string‑aware.
- Parentheses are balanced and string‑aware (respect quotes inside arg list to avoid early close).
- An optional trailing semicolon is tolerated for languages that use `;` as a statement terminator.

Errors
- Unbalanced parentheses in `args_opt`.
- Missing `$` or invalid state identifier after `->`.
- Trailing non‑whitespace tokens after a Frame statement line when no inline separator is present.
- Malformed heads for non‑transition statements: any forward head other than `$^` is invalid; any stack head other than `$$[+]` or `$$[-]` is invalid.

Validation notes (negatives)
- Identifier rule tightened: `state_ident` must begin with a letter or underscore and continue with `[A-Za-z0-9_]*`. Fixtures cover invalid starts (digits, symbols) and empty names.
- Malformed head coverage extends to all Frame statements, not only transitions:
  - Forward negatives such as `=> $B` (must be `=> $^`).
  - Stack negatives for any head not exactly `$$[+]` or `$$[-]`.
- Inline separation is enforced by the scanner; without a top‑level separator (`;` or line comment), any trailing tokens on the same line produce a trailing‑tokens error.

Complexity
- O(length of segment); overall sum O(n).

Test Hooks
- Empty args, single arg, multiple args with commas.
- Quotes inside args; nested parentheses in function calls.
- Whitespace variants and indentation unaffected.
- Inline forms accepted:
  - Python: `=> $^; x = 1  # ok`
  - TS/C#/C/CPP/Java/Rust: `=> $^; native(); // ok`
- Inline without separator rejected: `=> $^ native()`.
