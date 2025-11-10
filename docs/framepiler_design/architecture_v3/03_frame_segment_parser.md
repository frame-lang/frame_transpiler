# Stage 3 — Frame Statement Parser (Tiny)

Purpose
- Parse SOL‑anchored Frame statement slices into MIR items. Only three Frame statements are supported in native regions:
  - Transition: `-> $State(args?)`
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
- `MirItem::{ Transition{ target: StateRef, args: Option<Vec<ExprText>>, span }, Forward{ span }, StackPush{ span }, StackPop{ span } }`

Grammar (BNF‑ish)
```
transition  ::= "-" ">" WS+ "$" state_ident args_opt
args_opt     ::= /* empty */ | "(" arg_text ")"
forward      ::= "=" ">" WS+ "$" "^"
stackpush    ::= "$" "$" "[" "+" "]"
stackpop     ::= "$" "$" "[" "-" "]"
state_ident  ::= [A-Za-z_][A-Za-z0-9_]*  /* align with common grammar for $State */
WS           ::= space | tab
```

Arg Text Handling
- The parser does not parse Python/TS expressions; it slices the raw text between the balanced parentheses for later visitor consumption.
- Parentheses are balanced and string‑aware (respect quotes inside arg list to avoid early close).
- An optional trailing semicolon is tolerated for languages that use `;` as a statement terminator.

Errors
- Unbalanced parentheses in `args_opt`.
- Missing `$` or invalid state identifier after `->`.
- Trailing non‑whitespace tokens after a Frame statement line when no inline separator is present.
- Malformed heads for non‑transition statements: any forward head other than `$^` is invalid; any stack head other than `$$[+]` or `$$[-]` is invalid.

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
