# Frame Language V3 — Minimal Non‑Native Grammar (Going Native)

Purpose
- Define only the Frame‑syntax that still exists when handler/action/operation bodies are native and arguments in bodies are validated in native syntax via Stage 07 (facade). Everything else (statements inside bodies, types, classes/structs) is native.

Scope (What remains Frame)
- File prolog selecting target: `@target <language>`
- System and state outline (declarations and headers only)
- Action/Operation headers (bodies are native)
- Special state handlers ($>() and <$()) — headers only; bodies are native
- SOL‑anchored Frame directives embedded in native bodies:
  - Transition: `-> $State(args?)`
  - Parent forward: `=> $^`
  - Stack ops: `$$[+]` and `$$[-]`

Not Frame (Native)
- All bodies (handler/action/operation) are native text
- All argument expressions inside native bodies are native syntax (parsed/validated optionally via Stage 07)
- Classes/structs/types/enums declared for the target language are native (see Decision below)
- `system.return` is a native pseudo‑variable rewritten by visitors; not a Frame syntax form

File Prolog
```
prolog       ::= '@target' WS+ language NL
language     ::= 'python' | 'typescript' | 'csharp' | 'c' | 'cpp' | 'java' | 'rust'
```
- Must be the first non‑whitespace token in the file.

Top‑Level Outline
```
module       ::= prolog module_item*
module_item  ::= system_decl | (native items – ignored by Frame)

system_decl  ::= 'system' IDENT '{' system_item* '}'
system_item  ::= states_block | interface_block | actions_block | operations_block | (native items)
```

States and Handlers (headers are Frame; bodies are native)
```
states_block   ::= 'machine:' state_decl*
state_decl     ::= '$' IDENT ( '=>' '$' IDENT )? '{' handler_decl* '}'

handler_decl   ::= IDENT '(' param_list? ')' native_body
enter_handler  ::= '$>' '(' param_list? ')' native_body
exit_handler   ::= '<$' '(' param_list? ')' native_body

param_list     ::= IDENT (',' IDENT)*

native_body    ::= '{' …native text… '}'   // Body text is parsed by per‑target DPDA closers; content is not Frame‑parsed
```

Actions and Operations (headers only)
```
actions_block    ::= 'actions:' action_decl*
action_decl      ::= IDENT '(' param_list? ')' ( type_and_default? ) native_body

operations_block ::= 'operations:' operation_decl*
operation_decl   ::= (attribute* ) IDENT '(' param_list? ')' ( type_and_default? ) native_body

attribute        ::= '@' IDENT                // target‑neutral attribute token; semantics are target‑specific

type_and_default ::= ':' IDENT ( '=' expr_stub )?
expr_stub        ::= /* raw substring until matching brace depth; not Frame‑parsed */
```

Interface Methods (headers only)
```
interface_block  ::= 'interface:' interface_decl*
interface_decl   ::= IDENT '(' param_list? ')' ( type_and_default? ) native_body
```

Embedded Frame Directives inside Native Bodies (SOL‑anchored)
- Recognized only at start‑of‑line (indentation allowed), outside strings/comments/templates per target.
- No Frame tokenization occurs inside protected regions.
```
frame_stmt     ::= sol ( transition | parent_forward | stack_op )
sol            ::= (SP|TAB)*

transition     ::= '->' WS* '$' IDENT args_opt
args_opt       ::= '(' arg_bytes* ')' | /* empty */

parent_forward ::= '=>' WS* '$^'

stack_op       ::= '$$[+]' | '$$[-]'
```
Notes
- `arg_bytes*` is not Frame‑parsed; Stage 03 performs balanced‑paren + string‑aware splitting; Stage 07 (runtime‑optional) may parse args natively for diagnostics.
- Terminal‑last rule: transitions/parent‑forward/stack ops are terminal within a handler.

Lexical/Scanning Policy
- SOL‑anchored detection for Frame statements (indentation allowed)
- Per‑language scanners are DPDA‑based and string/comment/template aware; no regex for language syntax
- Body closers are DPDA‑based per language; downstream stages never “re‑close” bodies

Reserved Terms (Frame)
- Keywords (outline): `@target`, `system`, `machine:`, `interface:`, `actions:`, `operations:`
- Header symbols (states/handlers): `$` (state), `$>`, `<$`, `=>` (state inheritance)
- Directives (embedded ops): `->` (transition), `=> $^` (parent forward), `$$[+]`, `$$[-]` (stack ops)
- Note: “handler” is a concept only; there is no `handler` keyword in the grammar

Decision: Classes/Structs Are Native
- V3 keeps classes/structs/type definitions as native. Frame does not introduce a separate class/type grammar.
- Rationale: preserves idiomatic target usage, avoids semantic duplication, and aligns with Going Native where bodies and expressions are native.
- Implication: any class/struct definitions should appear as native items inside system blocks or in separate native modules/files according to project conventions.

Validation
- Structural: terminal‑last; no Frame directives in actions/ops
- Optional strict/native: Stage 07 can parse the spliced native body to validate argument expressions and surface mapped diagnostics

Interoperability
- MixedBody/MIR is authoritative for embedded Frame directives and mapping
- `splice_map` provides dual‑origin mapping for diagnostics; source maps are composed in Stage 08

Status
- This file is the authoritative minimal grammar for Frame constructs in V3; all other syntax is native and documented per target under target‑language docs.
