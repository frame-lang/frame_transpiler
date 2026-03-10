# Frame V4 Transpiler Architecture Specification

**Version:** 1.1
**Date:** February 2026
**Audience:** Implementation team
**Status:** Normative — Python 144/144, TypeScript 126/126, Rust 130/130, C 139/139 (539/539 total, 100%)

---

## 0. Compartment Architecture (Canonical)

The **compartment** is the central data structure in Frame's runtime model. It serves as "a closure concept for states that preserve the state itself, the data from the various scopes as well as runtime data needed for Frame machine semantics."

### 0.1 Compartment Structure (6 Fields)

Per the official Frame documentation, a compartment contains:

| Field | Type | Purpose |
|-------|------|---------|
| `state` | string | Current state identifier |
| `state_args` | dict | Arguments passed to the state (state parameters) |
| `state_vars` | dict | State variables declared with `$.varName` |
| `enter_args` | dict | Arguments passed via `-> (args) $State` |
| `exit_args` | dict | Arguments passed via `(args) -> $State` |
| `forward_event` | FrameEvent? | Runtime data for event forwarding (`-> =>`) |

### 0.2 Reference Example (Go from docs)

```go
type SomeCompartment struct {
    State          SomeState
    StateArgs      map[string]interface{}
    StateVars      map[string]interface{}
    EnterArgs      map[string]interface{}
    ExitArgs       map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}
```

### 0.3 State Variable Access

State variables (`$.varName`) are stored in the compartment's `state_vars` dict:

| Language | Access Pattern |
|----------|---------------|
| Python | `self.__compartment.state_vars["varName"]` |
| TypeScript | `this.#compartment.stateVars["varName"]` |
| Rust | `self.__compartment.state_vars.get("varName")` |

**Note:** All three languages now use double-underscore prefix (`__`) for runtime fields to indicate they are internal implementation details.

### 0.4 State Stack (History)

The state stack stores **compartments**, not just state names. This is what enables state variables to be preserved across push/pop:

```
push$           → saves entire compartment to stack
-> pop$         → restores compartment from stack (including state_vars)
```

This gives Frame the power of a **Pushdown Automaton**.

### 0.5 Key Insight: Compartment = State Closure

The compartment captures everything needed to fully describe and restore a state's context:
- Which state we're in (`state`)
- What data was passed to get here (`state_args`, `enter_args`)
- What data the state is tracking (`state_vars`)
- What to do when leaving (`exit_args`)
- Whether to forward an event after entering (`forward_event`)

---

## 1. Pipeline Overview

```
Source (.frm)
    │
    ▼
┌──────────────┐
│  Frame Parser │ → FrameAst
└──────────────┘
    │
    ▼
┌──────────────┐
│   Arcanum    │ → Symbol table (systems, states, events, variables)
└──────────────┘
    │
    ▼
┌──────────────┐
│  Validator   │ → Errors/warnings or pass
└──────────────┘
    │
    ▼
┌──────────────┐
│   Codegen    │ → CodegenNode tree
└──────────────┘
    │
    ▼
┌──────────────┐
│   Backend    │ → Target source code (.py, .ts, .rs)
└──────────────┘
```

Each stage is a pure function of its inputs. No stage mutates the output of a previous stage. Errors at any stage halt the pipeline and report diagnostics.

---

## 2. Frame Parser

**Input:** Raw source bytes + target language identifier  
**Output:** `FrameAst`

### 2.1 Responsibilities

1. Tokenize and parse all Frame constructs (`@@target`, `@@codegen`, `@@system`, states, handlers)
2. Record source spans (byte offsets) for every handler body, state variable declaration, and native code region
3. Skip native code preamble/postamble — record their spans for later pass-through
4. Parse state variable declarations (`$.varName (: type)? = expr`) at the top of each state block
5. Parse Frame statements within handler bodies (transitions, forwards, push/pop, `$.` access, `@@:return`)
6. Record but do not parse native code within handler bodies — preserve as byte spans

### 2.2 AST Definitions

```rust
pub enum FrameAst {
    Module(ModuleAst),
}

pub struct ModuleAst {
    pub preamble: Option<Span>,          // native code before @@target
    pub target: TargetLanguage,
    pub codegen_config: Option<CodegenConfig>,
    pub annotations: Vec<Annotation>,
    pub system: SystemAst,
    pub postamble: Option<Span>,         // native code after @@system
}

pub struct CodegenConfig {
    pub frame_event: Option<bool>,       // None = use default
    pub state_stack: Option<bool>,
    // Note: Frame V4 always uses kernel runtime
}

pub enum Annotation {
    Persist(PersistConfig),
}

pub struct SystemAst {
    pub name: String,
    pub system_params: Option<SystemParams>,
    pub interface: Vec<InterfaceMethodAst>,
    pub machine: MachineAst,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVarAst>,
}

pub struct SystemParams {
    pub state_params: Vec<ParamAst>,     // $(...)
    pub enter_params: Vec<ParamAst>,     // $>(...)
    pub domain_params: Vec<ParamAst>,    // bare names
}

pub struct MachineAst {
    pub states: Vec<StateAst>,
}

pub struct StateAst {
    pub name: String,
    pub parent: Option<String>,          // HSM parent state name
    pub state_vars: Vec<StateVarAst>,    // $.varName declarations
    pub handlers: Vec<HandlerAst>,
    pub enter_handler: Option<HandlerAst>,
    pub exit_handler: Option<HandlerAst>,
    pub has_default_forward: bool,       // trailing => $^
}

pub struct StateVarAst {
    pub name: String,                    // variable name (without $.)
    pub type_annotation: Option<String>, // optional type
    pub initializer: Span,              // byte span of native initializer expression
}

pub struct HandlerAst {
    pub event: String,
    pub params: Vec<ParamAst>,
    pub return_type: Option<String>,
    pub body: HandlerBody,
}

pub struct HandlerBody {
    pub span: Span,                      // byte span of entire body
    pub frame_statements: Vec<FrameStatement>,  // parsed Frame constructs with positions
}

pub enum FrameStatement {
    Transition(TransitionStmt),
    Forward,                             // => $^
    StackPush(StackPushStmt),
    StackPop,                            // bare pop$
    StateVarAccess(StateVarAccessStmt),
    StateVarAssign(StateVarAssignStmt),
    ContextReturnAssign(Span),           // @@:return = <expr_span>
    ContextReturnRead,                   // @@:return as expression
    ReturnValue(Span),                   // return <expr_span> (in handler = sugar)
}

pub struct TransitionStmt {
    pub target: TransitionTarget,
    pub exit_args: Vec<Span>,            // native expression spans
    pub enter_args: Vec<Span>,
    pub state_args: Vec<Span>,
    pub is_forwarding: bool,             // -> => flag
}

pub enum TransitionTarget {
    State(String),                       // $StateName
    Pop,                                 // pop$
}

pub struct StackPushStmt {
    pub target: Option<String>,          // None = current state, Some = named state
}

pub struct StateVarAccessStmt {
    pub var_name: String,
    pub span: Span,
}

pub struct StateVarAssignStmt {
    pub var_name: String,
    pub value_span: Span,                // native expression span
}

pub struct InterfaceMethodAst {
    pub name: String,
    pub params: Vec<ParamAst>,
    pub return_type: Option<String>,
    pub default_return: Option<Span>,    // native expression for default
}

pub struct ActionAst {
    pub name: String,
    pub params: Vec<ParamAst>,
    pub return_type: Option<String>,
    pub body: Span,                      // entirely native code
}

pub struct OperationAst {
    pub name: String,
    pub params: Vec<ParamAst>,
    pub return_type: Option<String>,
    pub body: Span,                      // entirely native code
    pub is_static: bool,
}

pub struct DomainVarAst {
    pub name: String,
    pub type_annotation: Option<String>,
    pub initializer: Span,
}

pub struct ParamAst {
    pub name: String,
    pub type_annotation: Option<String>,
}

pub struct Span {
    pub start: usize,                    // byte offset in source
    pub end: usize,                      // byte offset (exclusive)
}
```

### 2.3 Parser Behavior Notes

- **State variable region:** The parser recognizes `$.` at the beginning of a line within a state block as a state variable declaration. The region ends at the first event handler (`<ident>(`, `$>`, `$<`) or `=> $^`.
- **Handler body parsing:** The parser records the full byte span of the handler body. Within that span, it identifies Frame statements by their leading tokens (`->`, `=>`, `push$`, `pop$`, `$.`, `@@:return`, `return`). The parser does NOT attempt to fully parse native code between Frame statements.
- **String/comment awareness:** The parser must skip Frame token recognition inside string literals and comments of the target language. This requires minimal target-language awareness (string delimiters, comment syntax).

---

## 3. Arcanum (Symbol Table)

**Input:** `FrameAst`  
**Output:** `Arcanum`

### 3.1 Responsibilities

1. Catalog all systems, states, events, variables, interface methods
2. Resolve HSM parent-child relationships
3. Compute effective codegen configuration (user settings + auto-enable overrides)
4. Build per-state handler registry (which events each state handles)

### 3.2 Data Structure

```rust
pub struct Arcanum {
    pub system: SystemInfo,
    pub effective_config: EffectiveCodegenConfig,
}

pub struct SystemInfo {
    pub name: String,
    pub states: IndexMap<String, StateInfo>,  // insertion order = declaration order
    pub start_state: String,                   // first state declared
    pub interface_methods: Vec<InterfaceInfo>,
    pub actions: Vec<String>,
    pub operations: Vec<String>,
    pub domain_vars: Vec<String>,
    pub has_system_params: bool,
    pub has_persist: bool,
}

pub struct StateInfo {
    pub name: String,
    pub parent: Option<String>,
    pub state_vars: Vec<StateVarInfo>,
    pub handled_events: HashSet<String>,      // event names this state handles
    pub has_enter: bool,
    pub has_exit: bool,
    pub has_default_forward: bool,
    pub children: Vec<String>,                 // states that declare this as parent
}

pub struct StateVarInfo {
    pub name: String,
    pub type_annotation: Option<String>,
}

pub struct InterfaceInfo {
    pub name: String,
    pub has_return: bool,
    pub has_default_return: bool,
}

pub struct EffectiveCodegenConfig {
    pub frame_event: bool,
    pub state_stack: bool,
    // runtime is always kernel
    pub overrides: Vec<String>,               // warning messages for auto-enables
}
```

### 3.3 Auto-Enable Logic

The Arcanum scans the AST for features that require specific codegen flags:

```rust
fn compute_effective_config(ast: &SystemAst, user: &CodegenConfig) -> EffectiveCodegenConfig {
    let mut need_frame_event = false;
    let mut need_state_stack = false;
    let mut need_kernel = false;

    for state in &ast.machine.states {
        for handler in &state.handlers {
            for stmt in &handler.body.frame_statements {
                match stmt {
                    Transition { is_forwarding: true, .. } => need_frame_event = true,
                    Transition { exit_args, enter_args, .. }
                        if !exit_args.is_empty() || !enter_args.is_empty()
                        => need_frame_event = true,
                    StackPush(_) | StackPop => need_state_stack = true,
                    ContextReturnAssign(_) | ContextReturnRead => need_frame_event = true,
                    ReturnValue(_) => need_frame_event = true,  // sugar needs return stack
                    _ => {}
                }
            }
        }
        if state.enter_handler.as_ref().map_or(false, |h|
            h.body.frame_statements.iter().any(|s| matches!(s, Transition { .. }))
        ) {
            need_kernel = true;
        }
    }

    for iface in &ast.interface {
        if iface.has_return || iface.default_return.is_some() {
            need_frame_event = true;
        }
    }

    // Apply overrides
    // ...
}
```

---

## 4. Validator

**Input:** `FrameAst` + `Arcanum`  
**Output:** `Vec<Diagnostic>` (errors + warnings)

### 4.1 Validation Rules

| Code | Check | Severity |
|------|-------|----------|
| E001 | Parse errors | Error |
| E402 | Transition targets unknown state | Error |
| E403 | Duplicate state name | Error |
| E405 | Interface parameter arity mismatch | Error |
| E4xx | `=> $^` in state without parent | Error |
| E4xx | `$.x` references undeclared state variable | Error |
| E4xx | Transition in action body | Error |
| E4xx | Duplicate state variable name in same state | Error |
| E4xx | HSM cycle (A => B => A) | Error |
| W4xx | `@@codegen` auto-enable override | Warning |
| W4xx | Unreachable code after transition | Warning |

### 4.2 Validation Order

1. Structural validation (duplicates, missing references)
2. HSM graph validation (cycles, missing parents)
3. Statement validation (transitions in wrong context, undeclared variables)
4. Config validation (auto-enable warnings)

---

## 5. Codegen

**Input:** `FrameAst` + `Arcanum` + raw source bytes  
**Output:** `CodegenNode` tree

### 5.1 Responsibilities

1. Transform `SystemAst` into a `CodegenNode::Class` tree
2. Generate all runtime infrastructure (compartment, kernel, router, dispatch, stack)
3. Generate interface methods, handler methods, actions, operations
4. Extract native code from source using spans
5. Splice Frame statement expansions into native code regions
6. Generate state variable initialization code for each state's compartment

### 5.2 CodegenNode

The language-agnostic intermediate representation. Backends translate this to target syntax.

```rust
pub enum CodegenNode {
    // === Structural ===
    Module {
        preamble: Option<String>,
        imports: Vec<CodegenNode>,
        items: Vec<CodegenNode>,
        postamble: Option<String>,
    },
    Class {
        name: String,
        fields: Vec<Field>,
        methods: Vec<CodegenNode>,
        inner_classes: Vec<CodegenNode>,
    },

    // === Methods ===
    Constructor {
        params: Vec<Param>,
        body: Vec<CodegenNode>,
    },
    Method {
        name: String,
        params: Vec<Param>,
        return_type: Option<String>,
        body: Vec<CodegenNode>,
        visibility: Visibility,
        is_static: bool,
    },

    // === Statements ===
    VarDecl {
        name: String,
        type_annotation: Option<String>,
        init: Option<Box<CodegenNode>>,
    },
    Assignment {
        target: Box<CodegenNode>,
        value: Box<CodegenNode>,
    },
    Return {
        value: Option<Box<CodegenNode>>,
    },
    If {
        condition: Box<CodegenNode>,
        then_block: Vec<CodegenNode>,
        else_block: Option<Vec<CodegenNode>>,
    },
    IfElseChain {
        branches: Vec<(Box<CodegenNode>, Vec<CodegenNode>)>,  // (condition, body) pairs
        else_block: Option<Vec<CodegenNode>>,
    },
    Match {
        subject: Box<CodegenNode>,
        arms: Vec<(String, Vec<CodegenNode>)>,
        default: Option<Vec<CodegenNode>>,
    },

    // === Frame-Specific ===
    CompartmentCreate {
        system_name: String,
        state_name: String,
        state_arg_inits: Vec<(String, Box<CodegenNode>)>,
        state_var_inits: Vec<(String, Box<CodegenNode>)>,
        enter_arg_inits: Vec<(String, Box<CodegenNode>)>,
        set_forward_event: bool,
    },
    TransitionCall {
        compartment_var: String,            // variable name holding the compartment
    },
    StateStackPush {
        compartment_expr: Box<CodegenNode>,
    },
    StateStackPop,
    ContextReturnAssign {
        value: Box<CodegenNode>,
    },
    ContextReturnRead,
    ParentDispatch {
        parent_state: String,
        system_name: String,
    },

    // === Native Code ===
    NativeBlock {
        code: String,
    },

    // === Expressions ===
    Literal(LiteralValue),
    Ident(String),
    FieldAccess {
        receiver: Box<CodegenNode>,
        field: String,
    },
    MethodCall {
        receiver: Option<Box<CodegenNode>>,
        method: String,
        args: Vec<CodegenNode>,
    },
    IndexAccess {
        receiver: Box<CodegenNode>,
        key: Box<CodegenNode>,
    },
    StringLiteral(String),
    BoolLiteral(bool),
    NullLiteral,
    ListLiteral(Vec<CodegenNode>),
    DictLiteral(Vec<(CodegenNode, CodegenNode)>),
}

pub enum Visibility { Public, Private }
pub enum LiteralValue { Int(i64), Float(f64), String(String), Bool(bool), Null }
pub struct Field { pub name: String, pub type_annotation: Option<String>, pub visibility: Visibility }
pub struct Param { pub name: String, pub type_annotation: Option<String> }
```

### 5.3 System Codegen Algorithm

The system codegen builds the class tree in this order:

```
1. Generate inner classes:
   a. FrameEvent (if enabled)
   b. Compartment

2. Generate fields:
   a. __compartment
   b. __next_compartment
   c. __state_stack (if enabled)
   d. __return_stack
   e. Domain variables

3. Generate constructor:
   a. Initialize state stack
   b. Initialize return stack
   c. Create start state compartment
   d. Initialize start state's state_args (from system params)
   e. Initialize start state's state_vars
   f. Initialize domain variables (from system params or defaults)
   g. Send enter event to start state via kernel

4. Generate runtime infrastructure:
   a. __kernel (always)
   b. __router
   c. __transition
   d. State dispatch functions (one per state)
   e. __state_stack_push, __state_stack_pop (if enabled)

5. Generate interface methods

6. Generate handler methods (one per state+event pair):
   a. Extract native code from source using body span
   b. Scan for Frame constructs within native code
   c. Splice Frame expansions into native code
   d. Append auto-return after transitions

7. Generate actions (native code pass-through, with @@:return rewriting)

8. Generate operations (entirely native code)

9. Generate persistence methods (if @@persist)
```

### 5.4 NativeRegionScanner

Scans handler body bytes for Frame constructs within native code.

**Input:** Byte slice of handler body + target language  
**Output:** Ordered list of regions (Native or Frame construct)

```rust
pub enum Region {
    Native(Span),
    Transition(TransitionRegion),
    Forward(Span),
    StackPush(StackPushRegion),
    StackPop(Span),
    StateVarRead(StateVarRegion),
    StateVarAssign(StateVarAssignRegion),
    ContextReturnAssign(ContextReturnRegion),
    ContextReturnRead(Span),
    ReturnValue(ReturnValueRegion),
}
```

**Recognition patterns:**

| Token sequence | Region type |
|---------------|-------------|
| `->` `$<ident>` | Transition |
| `->` `=>` | Forwarding transition |
| `->` `(` | Transition with enter args (scan for `$`) |
| `(` ... `)` `->` | Transition with exit args |
| `->` `pop$` | Transition to popped state |
| `=>` `$^` | Forward to parent |
| `push$` | Stack push |
| `pop$` (not preceded by `->`) | Stack pop |
| `$.` `<ident>` `=` | State variable write |
| `$.` `<ident>` (not followed by `=`) | State variable read |
| `@@:return` `=` | Context return assign |
| `@@:return` (not followed by `=`) | Context return read |
| `return` `<expr>` (in handler context) | Return value sugar |

**Critical:** Skip recognition inside string literals and comments of the target language.

### 5.5 Splicer

The splicer takes the region list from the scanner and builds `CodegenNode` output by:

1. Emitting `NativeBlock` nodes for native regions
2. Expanding Frame regions into their `CodegenNode` equivalents
3. Appending `Return` after every transition expansion

The splicer knows the current context (handler vs action) to correctly handle `return <expr>` sugar.

---

## 6. Language Backend

**Input:** `CodegenNode` tree  
**Output:** Target language source code string

### 6.1 Backend Trait

```rust
pub trait LanguageBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;
    fn target_language(&self) -> TargetLanguage;
    fn self_keyword(&self) -> &'static str;           // "self" | "this"
    fn null_keyword(&self) -> &'static str;            // "None" | "null" | "None"
    fn true_keyword(&self) -> &'static str;
    fn false_keyword(&self) -> &'static str;
    fn runtime_imports(&self) -> Vec<String>;
    fn class_syntax(&self) -> ClassSyntax;
    fn dispatch_syntax(&self) -> DispatchSyntax;       // IfElse | Switch | Match
}

pub struct EmitContext {
    pub indent_level: usize,
    pub indent_str: String,                            // "    " or "\t"
}
```

### 6.2 Backend-Specific Dispatch Patterns

| Backend | Router | State dispatch | Variable access |
|---------|--------|---------------|-----------------|
| Python | `if/elif` on `self.__compartment.state` | `if/elif` on `e._message` | `self.__compartment.state_vars["name"]` |
| TypeScript | `switch (this.#compartment.state)` | `switch (e.message)` | `this.#compartment.stateVars["name"]` |
| Rust | `match self.compartment.state.as_str()` | `match e.message.as_str()` | `self.compartment.state_vars.get("name")` with downcast |

### 6.3 Backend-Specific Compartment

| Backend | Compartment type | State vars storage | State args storage |
|---------|-----------------|-------------------|-------------------|
| Python | Class instance | `dict` | `dict` |
| TypeScript | Class instance | `Record<string, any>` | `Record<string, any>` |
| Rust | Struct instance | `HashMap<String, Box<dyn Any>>` | `HashMap<String, Box<dyn Any>>` |

### 6.4 Generated Compartment Class (Python/TypeScript)

For Python and TypeScript, a `{System}Compartment` class is generated before the main system class:

**Python:**
```python
class FooCompartment:
    def __init__(self, state: str):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None

    def copy(self) -> 'FooCompartment':
        c = FooCompartment(self.state)
        c.state_args = self.state_args.copy()
        c.state_vars = self.state_vars.copy()
        c.enter_args = self.enter_args.copy()
        c.exit_args = self.exit_args.copy()
        c.forward_event = self.forward_event
        return c
```

**TypeScript:**
```typescript
class FooCompartment {
    state: string;
    state_args: Record<string, any> = {};
    state_vars: Record<string, any> = {};
    enter_args: Record<string, any> = {};
    exit_args: Record<string, any> = {};
    forward_event: any = null;

    constructor(state: string) {
        this.state = state;
    }

    copy(): FooCompartment {
        const c = new FooCompartment(this.state);
        c.state_args = {...this.state_args};
        c.state_vars = {...this.state_vars};
        c.enter_args = {...this.enter_args};
        c.exit_args = {...this.exit_args};
        c.forward_event = this.forward_event;
        return c;
    }
}
```

The `copy()` method enables push/pop to preserve state variables by storing entire compartment copies on the state stack.

### 6.5 Generated Runtime Infrastructure (Rust)

Rust follows the same kernel/router/transition pattern as Python/TypeScript, using Rust idioms.

#### 6.5.1 Rust FrameEvent

```rust
#[derive(Clone)]
struct FooFrameEvent {
    message: String,
    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    _return: Option<Box<dyn std::any::Any>>,
}

impl FooFrameEvent {
    fn new(message: &str, parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>) -> Self {
        Self {
            message: message.to_string(),
            parameters,
            _return: None,
        }
    }
}
```

#### 6.5.2 Rust Compartment

```rust
#[derive(Clone)]
struct FooCompartment {
    state: String,
    state_args: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    state_vars: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    enter_args: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    exit_args: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    forward_event: Option<FooFrameEvent>,
}

impl FooCompartment {
    fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
            state_args: std::collections::HashMap::new(),
            state_vars: std::collections::HashMap::new(),
            enter_args: std::collections::HashMap::new(),
            exit_args: std::collections::HashMap::new(),
            forward_event: None,
        }
    }
}
```

**Note:** Rust's `Box<dyn Any>` requires `Clone` to be implemented manually or use `dyn_clone` crate. For simplicity, state vars can use concrete types when known, or `serde_json::Value` for dynamic typing.

#### 6.5.3 Rust System Fields

```rust
pub struct Foo {
    __compartment: FooCompartment,
    __next_compartment: Option<FooCompartment>,
    _state_stack: Vec<FooCompartment>,
    _return_value: Option<Box<dyn std::any::Any>>,
    // Domain variables
    counter: i32,
}
```

#### 6.5.4 Rust Constructor

```rust
impl Foo {
    pub fn new() -> Self {
        let mut __compartment = FooCompartment::new("Idle");
        // Initialize state vars for start state
        __compartment.state_vars.insert("count".to_string(), Box::new(0i32));

        let mut this = Self {
            __compartment,
            __next_compartment: None,
            _state_stack: Vec::new(),
            _return_value: None,
            counter: 0,
        };

        // Send enter event to start state
        let enter_event = FooFrameEvent::new("$>", std::collections::HashMap::new());
        this.__kernel(enter_event);
        this
    }
}
```

#### 6.5.5 Rust Kernel

The kernel processes events and handles deferred transitions:

```rust
fn __kernel(&mut self, __e: FooFrameEvent) {
    // Route event to current state
    self.__router(&__e);

    // Process any pending transition
    while self.__next_compartment.is_some() {
        let next_compartment = self.__next_compartment.take().unwrap();

        // Exit current state
        let exit_event = FooFrameEvent::new("$<", self.__compartment.exit_args.clone());
        self.__router(&exit_event);

        // Switch to new compartment
        self.__compartment = next_compartment;

        // Enter new state (or forward event)
        if self.__compartment.forward_event.is_none() {
            let enter_event = FooFrameEvent::new("$>", self.__compartment.enter_args.clone());
            self.__router(&enter_event);
        } else {
            // There is a forwarded event
            let forward_event = self.__compartment.forward_event.take().unwrap();
            if forward_event.message == "$>" {
                // Forwarded event IS enter event - just send it
                self.__router(&forward_event);
            } else {
                // Forwarded event is NOT enter event
                // Send normal enter event first
                let enter_event = FooFrameEvent::new("$>", self.__compartment.enter_args.clone());
                self.__router(&enter_event);
                // Then forward the event to the new, initialized state
                self.__router(&forward_event);
            }
        }
    }
}
```

#### 6.5.6 Rust Router

The router dispatches events to the appropriate state handler:

```rust
fn __router(&mut self, __e: &FooFrameEvent) {
    match self.__compartment.state.as_str() {
        "Idle" => self._state_Idle(__e),
        "Working" => self._state_Working(__e),
        _ => {}
    }
}
```

#### 6.5.7 Rust State Dispatch

Each state has a dispatch function that routes to event handlers:

```rust
fn _state_Idle(&mut self, __e: &FooFrameEvent) {
    match __e.message.as_str() {
        "$>" => self._s_Idle_enter(__e),
        "$<" => self._s_Idle_exit(__e),
        "start" => self._s_Idle_start(__e),
        "process" => self._s_Idle_process(__e),
        _ => {
            // HSM: forward to parent if this state has a parent
            // self._state_Parent(__e);
        }
    }
}

fn _state_Working(&mut self, __e: &FooFrameEvent) {
    match __e.message.as_str() {
        "$>" => self._s_Working_enter(__e),
        "$<" => self._s_Working_exit(__e),
        "process" => self._s_Working_process(__e),
        _ => {}
    }
}
```

#### 6.5.8 Rust Transition

Deferred transition - caches the next compartment for the kernel to process:

```rust
fn __transition(&mut self, next_compartment: FooCompartment) {
    self.__next_compartment = Some(next_compartment);
}
```

#### 6.5.9 Rust Interface Methods

Interface methods create FrameEvents and route through kernel:

```rust
pub fn start(&mut self) {
    let __e = FooFrameEvent::new("start", std::collections::HashMap::new());
    self.__kernel(__e);
}

pub fn process(&mut self) {
    let __e = FooFrameEvent::new("process", std::collections::HashMap::new());
    self.__kernel(__e);
}

pub fn get_count(&mut self) -> i32 {
    self._return_value = None;
    let __e = FooFrameEvent::new("get_count", std::collections::HashMap::new());
    self.__kernel(__e);
    self._return_value.take()
        .and_then(|v| v.downcast::<i32>().ok())
        .map(|v| *v)
        .unwrap_or_default()
}
```

#### 6.5.10 Rust Handler Methods

Handler methods contain user code with Frame statement expansions:

```rust
fn _s_Idle_start(&mut self, __e: &FooFrameEvent) {
    // Native code
    println!("Starting...");

    // Frame expansion of -> $Working
    let mut __compartment = FooCompartment::new("Working");
    // Initialize state vars for target state
    __compartment.state_vars.insert("count".to_string(), Box::new(0i32));
    self.__transition(__compartment);
    return;

    // Unreachable after transition
}

fn _s_Working_process(&mut self, __e: &FooFrameEvent) {
    // Access state var: $.count
    let count = self.__compartment.state_vars.get("count")
        .and_then(|v| v.downcast_ref::<i32>())
        .copied()
        .unwrap_or(0);

    println!("Count: {}", count);

    // Assign state var: $.count = count + 1
    self.__compartment.state_vars.insert("count".to_string(), Box::new(count + 1));
}
```

#### 6.5.11 Rust State Stack Operations

```rust
fn _state_stack_push(&mut self) {
    self._state_stack.push(self.__compartment.clone());
}

fn _state_stack_pop(&mut self) {
    if let Some(compartment) = self._state_stack.pop() {
        // Exit current state
        let exit_event = FooFrameEvent::new("$<", self.__compartment.exit_args.clone());
        self.__router(&exit_event);
        // Restore compartment (do NOT send enter - we're restoring, not entering)
        self.__compartment = compartment;
    }
}
```

#### 6.5.12 Rust System Return

```rust
fn _s_Working_get_count(&mut self, __e: &FooFrameEvent) {
    let count = self.__compartment.state_vars.get("count")
        .and_then(|v| v.downcast_ref::<i32>())
        .copied()
        .unwrap_or(0);

    // @@:return = count
    self._return_value = Some(Box::new(count));
}
```

### 6.6 Native Code Re-indentation

When emitting `NativeBlock` nodes, the backend must adjust indentation to match the current emit context. The algorithm:

1. Determine the original indentation of the native code block (from its first non-empty line)
2. Strip that original indentation from all lines
3. Apply the current context's indentation to all lines

---

## 7. Generated Code Structure

For a system named `Foo` with states `$A` and `$B`, the generated class contains:

```
class Foo:
    # Inner classes
    class FrameEvent          (if frame_event = on)
    class FooCompartment      (always)

    # Fields
    __compartment             (always)
    __next_compartment        (always)
    __state_stack             (if state_stack = on)
    __return_stack            (always)
    <domain vars>             (from domain:)

    # Constructor
    __init__                  (always)

    # Runtime infrastructure
    __kernel                  (always)
    __router                  (always)
    __transition              (always)
    __foo_state_A             (state dispatch — one per state)
    __foo_state_B
    __state_stack_push        (if state_stack = on)
    __state_stack_pop         (if state_stack = on)

    # Interface methods        (from interface:)
    start
    stop
    getStatus

    # Handler methods          (one per state+event pair)
    __foo_state_A_enter       (if $A has $>)
    __foo_state_A_exit        (if $A has $<)
    __foo_state_A_start       (if $A handles start)
    __foo_state_B_enter
    __foo_state_B_stop

    # Actions                  (from actions:)
    __validate

    # Operations               (from operations:)
    getTemp
    static add

    # Persistence              (if @@persist)
    _save
    _restore (classmethod/static)
```

**Naming conventions:**
- State identifiers: `__<systemname>_state_<StateName>`
- State dispatch functions: `__<systemname>_state_<StateName>`
- Handler methods: `__<systemname>_state_<StateName>_<event>`
- Actions: `__<actionName>` (private)
- Operations: `<opName>` (public)

---

## 8. Three-Layer Dispatch Architecture

### 8.1 Flow

```
Interface method call
  → Create FrameEvent (if enabled) or direct call
  → Kernel (processes event + deferred transitions)
    → Router (selects state dispatch by compartment.state)
      → State dispatch (selects handler by event name)
        → Handler method (user code + Frame expansions)
```

### 8.2 Kernel

```
kernel(event):
    router(event)
    while next_compartment is not None:
        nc = next_compartment
        next_compartment = None
        router(FrameEvent("$<", compartment.exit_args))     // exit current
        compartment = nc                                     // change state
        if nc.forward_event is None:
            router(FrameEvent("$>", compartment.enter_args)) // enter new
        else:
            if nc.forward_event.message == "$>":
                router(nc.forward_event)                     // forwarded IS enter
            else:
                router(FrameEvent("$>", compartment.enter_args)) // enter first
                router(nc.forward_event)                          // then forward
            nc.forward_event = None
```

### 8.3 Router

```
router(event):
    match compartment.state:
        "__foo_state_A" → foo_state_A(event)
        "__foo_state_B" → foo_state_B(event)
```

### 8.4 State Dispatch

```
foo_state_A(event):
    match event.message:
        "$>"        → foo_state_A_enter(event)
        "$<"        → foo_state_A_exit(event)
        "start"     → foo_state_A_start(event)
        "process"   → foo_state_A_process(event)
        _           → foo_state_Parent(event)     // HSM: forward to parent
                    // or: do nothing              // no parent
```

### 8.5 Handler Method

Contains user's native code with Frame statement expansions. Example:

```
foo_state_A_start(event):
    // native code
    result = compute()
    if result > 0:
        // Frame expansion of -> $B
        nc = FooCompartment("__foo_state_B")
        nc.state_vars["count"] = 0          // $.count init
        self.__transition(nc)
        return                               // auto-generated
    // native code continues (only if transition didn't fire)
    log("staying in A")
```

---

## 9. Persistence Codegen

When `@@persist` is present, generate two additional methods.

### 9.1 JSON Schema

```json
{
    "schemaVersion": 1,
    "systemName": "<name>",
    "state": "<current_state_name>",
    "stateArgs": { ... },
    "stateVars": { ... },
    "domain": { ... },
    "stack": [ { "state": "...", "stateArgs": {...}, "stateVars": {...} }, ... ]
}
```

### 9.2 Generated Methods

**Save:** Serialize current compartment + domain + stack to JSON string.

**Restore:** Create new instance, deserialize JSON, set compartment, domain, stack. Do NOT invoke enter handler (the state is being restored, not entered).

### 9.3 Field Filtering

| Form | Behavior |
|------|---------|
| `@@persist` | All domain vars |
| `@@persist(domain=[a, b])` | Only `a`, `b` |
| `@@persist(exclude=[c])` | All except `c` |

---

## 10. File Structure

```
framec/src/frame_c/
├── v4/
│   ├── frame_parser.rs           # Frame syntax parser
│   ├── frame_ast.rs              # FrameAst definitions (Section 2.2)
│   ├── arcanum.rs                # Symbol table (Section 3)
│   ├── frame_validator.rs        # Validation (Section 4)
│   ├── native_region_scanner.rs  # Scanner (Section 5.4)
│   ├── codegen/
│   │   ├── mod.rs                # Codegen entry point
│   │   ├── ast.rs                # CodegenNode definitions (Section 5.2)
│   │   ├── system_codegen.rs     # SystemAst → CodegenNode (Section 5.3)
│   │   ├── splicer.rs            # Native code splicing (Section 5.5)
│   │   ├── backend.rs            # LanguageBackend trait (Section 6.1)
│   │   └── backends/
│   │       ├── python.rs         # Python emitter
│   │       ├── typescript.rs     # TypeScript emitter
│   │       └── rust_backend.rs   # Rust emitter
│   └── pipeline/
│       ├── config.rs             # Pipeline configuration
│       ├── compiler.rs           # Orchestrates parse → arcanum → validate → codegen → emit
│       └── traits.rs             # Pipeline traits
└── cli.rs                        # Command-line interface
```