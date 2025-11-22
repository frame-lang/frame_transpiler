# Frame Language V3 — Minimal Non‑Native Grammar (Going Native)

Purpose
- Define only the Frame‑syntax that still exists when handler/action/operation bodies are native and arguments in bodies are validated in native syntax via Stage 07 (facade). Everything else (statements inside bodies, types, classes/structs/functions) is native.

Scope (What remains Frame)
- File prolog selecting target: `@target <language>`
- System and state outline (declarations and headers only)
- Action/Operation headers (bodies are native)
- Special state handlers ($>() and <$()) — headers only; bodies are native
- Top‑level Frame functions, including a single `fn main` per module (headers only; bodies are native / Frame‑mixed)
- SOL‑anchored Frame statements embedded in native bodies:
  - Transition: `-> $State(args?)`
  - Parent forward: `=> $^`
  - Stack ops: `$$[+]` and `$$[-]`

Not Frame (Native)
- All bodies (handler/action/operation) are native text
- All top‑level function bodies are native text (with optional embedded Frame statements, parsed via the same DPDA/MIR pipeline as handlers)
- All argument expressions inside native bodies are native syntax (parsed/validated optionally via Stage 07)
- Classes/structs/types/enums declared for the target language are native (see Decision below)
- `system.return` is a native pseudo‑variable rewritten by visitors; not a Frame syntax form

File Prolog
```
prolog       ::= '@target' WS+ language NL
language     ::= 'python' | 'typescript' | 'csharp' | 'c' | 'cpp' | 'java' | 'rust'
```
- Must be the first non‑whitespace token in the file.

Top‑Level Outline and Outer Pipeline
```
module       ::= prolog module_item*
module_item  ::= system_decl | function_decl | native_item

system_decl  ::= 'system' IDENT system_params? '{' system_item* '}'
system_item  ::= states_block | interface_block | actions_block | operations_block | native_item

system_params   ::= '(' system_param_list ')'
system_param_list
              ::= system_param (',' system_param)*
system_param ::= start_state_param | enter_event_param | domain_param
start_state_param
              ::= '$(' param_list? ')'
enter_event_param
              ::= '$>' '(' param_list? ')'
domain_param  ::= IDENT                 // domain object injected at construction time

native_item  ::= /* any target‑language construct not recognized as Frame; ignored by Frame grammar */
```

### Outer Parsing and AST

V3 uses a dedicated, Frame‑only outer pipeline to interpret the outline:

- `SystemParserV3`:
  - Scans for `system` headers and builds a `ModuleAst`:
    - `SystemAst { name, params: SystemParamsAst, sections: SystemSectionsAst, section_order: [SystemSectionKind] }`.
  - Locates per‑system block spans:
    - `operations:`, `interface:`, `machine:`, `actions:`, and `domain:`.
  - Parses `system_params` into:
    - `start_params` (`$(...)`), `enter_params` (`$>(...)`), and `domain_params` (identifiers).

- `MachineParserV3`:
  - Operates within a system’s `machine:` span.
  - Finds `$State` headers and `$>()` entry handlers (`$>() { … }`) for the start state.
  - Supplies state/entry parameter names for system‑parameter semantics (E416/E417).

- `DomainBlockScannerV3`:
  - Operates within a system’s `domain:` span.
  - Enforces that each non‑blank, non‑comment line is a declaration‐shaped statement:
    - `var ident = <expr>` or `ident = <expr>`.
  - Reports E419 if a line does not match this form.

- `ModuleAst` + `Arcanum`:
  - `ModuleAst` is the primary outer AST; `Arcanum` is a symbol table built from
    `ModuleAst` + `machine:` spans:
    - Systems → machines → states (`StateDecl { name, parent, params, span }`).
  - The validator uses this to:
    - Enforce system block ordering and uniqueness (E113/E114).
    - Validate state parameter arity vs transitions (E405).
    - Check state header form and placement (E112/E404).

The outer pipeline is **DPDA‑based and language‑agnostic**: it only understands
Frame tokens and structural markers (`system`, section headers, `$State`, etc.).
All native syntax inside bodies is delegated to per‑target native scanners and,
optionally, native parsers (facades).

States and Handlers (headers are Frame; bodies are native)
```
states_block   ::= 'machine:' state_decl*
state_decl     ::= '$' IDENT state_params? ( '=>' '$' IDENT )? '{' handler_decl* '}'

handler_decl   ::= IDENT '(' param_list? ')' native_body
enter_handler  ::= '$>' '(' param_list? ')' native_body
exit_handler   ::= '<$' '(' param_list? ')' native_body

param_list     ::= IDENT (',' IDENT)*
state_params   ::= '(' param_list? ')'

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

Functions (including `fn main`)
-------------------------------

Frame functions are peer artifacts alongside systems. In the V3 “going native” model they provide
named entry points and helpers, but their bodies are treated as native code (with the same
SOL‑anchored Frame statement embedding rules as handlers).

```
function_decl ::= attributes? 'fn' IDENT '(' param_list? ')' native_body
```

Notes
- The function body is `native_body`: it is scanned and spliced using the same DPDA + MIR + expander
  pipeline as handler bodies, so embedded Frame statements (`->`, `=> $^`, `$$[+/-]`) inside functions
  are handled identically to those in handlers.
- Functions may appear anywhere after the prolog, interleaved with systems and native items.
- `fn main` rule:
  - At most one function named `main` is allowed per module; additional `fn main` declarations are a
    validation error.
  - `fn main` is optional. When present, codegen maps it to a host‑language entry point
    (e.g., `def main():` in Python) that uses the same runtime kernel as system code.

Embedded Frame Statements inside Native Bodies (SOL‑anchored)
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
- Statements (embedded ops): `->` (transition), `=> $^` (parent forward), `$$[+]`, `$$[-]` (stack ops)
- Note: “handler” is a concept only; there is no `handler` keyword in the grammar

System Return and System Calls (Native Patterns)
-----------------------------------------------

- `system.return` is a **native** pseudo‑variable used to set interface return
  values from handlers, actions, and non‑static operations. It is not a
  separate Frame statement; it appears inside native bodies and is rewritten
  by target‑language visitors to access a per‑call return slot.
- Handler and operation headers may optionally initialize the per‑call slot
  using the same header form as interfaces:

  - Interface: `status(a, b): Type = Expr`
  - Handler: `status(a, b) = Expr`

  The initializer expression `Expr` is evaluated in the handler’s native
  context at entry and used as the initial `system.return` value for that
  call.
- Calls of the form `system.methodName(...)` inside handlers/actions/operations
  are also treated as native and are used to invoke interface methods or other
  system‑level helpers. The Frame grammar does not special‑case this syntax;
  it is validated and enforced by target‑language codegen and runtime logic.

Decision: Classes/Structs Are Native
- V3 keeps classes/structs/type definitions as native. Frame does not introduce a separate class/type grammar.
- Rationale: preserves idiomatic target usage, avoids semantic duplication, and aligns with Going Native where bodies and expressions are native.
- Implication: any class/struct definitions should appear as native items inside system blocks or in separate native modules/files according to project conventions.

- Structural: terminal‑last; no Frame statements in actions/ops
- Optional strict/native: Stage 07 can parse the spliced native body to validate argument expressions and surface mapped diagnostics

- MixedBody/MIR is authoritative for embedded Frame statements and mapping
- `splice_map` provides dual‑origin mapping for diagnostics; source maps are composed in Stage 08

Status
- This file is the authoritative minimal grammar for Frame constructs in V3; all other syntax is native and documented per target under target‑language docs.
