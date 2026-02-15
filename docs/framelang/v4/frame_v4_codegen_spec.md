# Frame V4 Codegen Specification

**Version:** 1.0  
**Date:** February 2026  
**Audience:** Implementation team (backend authors)  
**Status:** Normative — this is the source of truth for what the Framepiler emits.

---

## 1. Scope

This document specifies the exact generated code for every Frame construct in every supported target language. All examples show final emitted code, not intermediate CodegenNode representations.

**Target languages:** Python 3, TypeScript, Rust

---

## 2. Naming Conventions

All generated identifiers follow these patterns:

| Concept | Pattern | Example |
|---------|---------|---------|
| State identifier string | `__<sys>_state_<State>` | `__foo_state_Idle` |
| State dispatch function | `__<sys>_state_<State>` | `__foo_state_Idle` |
| Handler method | `__<sys>_state_<State>_<event>` | `__foo_state_Idle_start` |
| Enter handler | `__<sys>_state_<State>_enter` | `__foo_state_Idle_enter` |
| Exit handler | `__<sys>_state_<State>_exit` | `__foo_state_Idle_exit` |
| Compartment class | `<Sys>Compartment` | `FooCompartment` |
| FrameEvent class | `FrameEvent` | `FrameEvent` |
| Action method | `__<actionName>` | `__validate` |

`<sys>` is the system name lowercased. `<Sys>` is the system name as declared.

---

## 3. Runtime Classes

### 3.1 FrameEvent (when `frame_event = on`)

**Python:**
```python
class FrameEvent:
    def __init__(self, message, parameters=None):
        self._message = message
        self._parameters = parameters or {}
```

**TypeScript:**
```typescript
class FrameEvent {
    public message: string;
    public parameters: Record<string, any>;
    constructor(message: string, parameters: Record<string, any> = {}) {
        this.message = message;
        this.parameters = parameters;
    }
}
```

**Rust:**
```rust
struct FrameEvent {
    message: String,
    parameters: HashMap<String, Box<dyn Any>>,
}
impl FrameEvent {
    fn new(message: &str) -> Self {
        FrameEvent { message: message.to_string(), parameters: HashMap::new() }
    }
    fn with_params(message: &str, parameters: HashMap<String, Box<dyn Any>>) -> Self {
        FrameEvent { message: message.to_string(), parameters }
    }
}
```

### 3.2 Compartment (always generated)

**Python:**
```python
class FooCompartment:
    def __init__(self, state):
        self.state = state
        self.state_args = {}
        self.state_vars = {}
        self.enter_args = {}
        self.exit_args = {}
        self.forward_event = None
```

**TypeScript:**
```typescript
class FooCompartment {
    state: string;
    stateArgs: Record<string, any> = {};
    stateVars: Record<string, any> = {};
    enterArgs: Record<string, any> = {};
    exitArgs: Record<string, any> = {};
    forwardEvent: FrameEvent | null = null;
    constructor(state: string) { this.state = state; }
}
```

**Rust:**
```rust
struct FooCompartment {
    state: String,
    state_args: HashMap<String, Box<dyn Any>>,
    state_vars: HashMap<String, Box<dyn Any>>,
    enter_args: HashMap<String, Box<dyn Any>>,
    exit_args: HashMap<String, Box<dyn Any>>,
    forward_event: Option<FrameEvent>,
}
impl FooCompartment {
    fn new(state: &str) -> Self {
        FooCompartment {
            state: state.to_string(),
            state_args: HashMap::new(),
            state_vars: HashMap::new(),
            enter_args: HashMap::new(),
            exit_args: HashMap::new(),
            forward_event: None,
        }
    }
}
```

---

## 4. Constructor

**Python:**
```python
def __init__(self):
    # State stack (if state_stack = on)
    self.__state_stack: list = []

    # Return stack (always)
    self.__return_stack: list = []

    # Create start state compartment
    self.__compartment = FooCompartment('__foo_state_Idle')
    self.__next_compartment = None

    # Start state's state_args (if system has $(params))
    # self.__compartment.state_args["param"] = arg_value

    # Start state's state_vars (from $.var declarations in start state)
    # self.__compartment.state_vars["count"] = 0

    # Domain variables
    self.count = 0
    self.label = "default"

    # Enter start state
    e = FrameEvent("$>", self.__compartment.enter_args)
    self.__kernel(e)
```

**TypeScript:**
```typescript
constructor() {
    this.#stateStack = [];
    this.#returnStack = [];
    this.#compartment = new FooCompartment('__foo_state_Idle');
    this.#nextCompartment = null;
    this.count = 0;
    this.label = "default";
    const e = new FrameEvent("$>", this.#compartment.enterArgs);
    this.#kernel(e);
}
```

**Rust:**
```rust
pub fn new() -> Self {
    let compartment = FooCompartment::new("__foo_state_Idle");
    // init state_vars on compartment
    let mut foo = Foo {
        state_stack: Vec::new(),
        return_stack: Vec::new(),
        compartment,
        next_compartment: None,
        count: 0,
        label: String::from("default"),
    };
    let e = FrameEvent::new("$>");
    foo.kernel(e);
    foo
}
```

---

## 5. Kernel

Frame V4 always uses the kernel runtime for deferred transitions.

**Python:**
```python
def __kernel(self, e):
    self.__router(e)
    while self.__next_compartment is not None:
        next_compartment = self.__next_compartment
        self.__next_compartment = None
        exit_event = FrameEvent("$<", self.__compartment.exit_args)
        self.__router(exit_event)
        self.__compartment = next_compartment
        if next_compartment.forward_event is None:
            enter_event = FrameEvent("$>", self.__compartment.enter_args)
            self.__router(enter_event)
        else:
            forwarded = next_compartment.forward_event
            next_compartment.forward_event = None
            if forwarded._message == "$>":
                self.__router(forwarded)
            else:
                enter_event = FrameEvent("$>", self.__compartment.enter_args)
                self.__router(enter_event)
                self.__router(forwarded)
```

**TypeScript:**
```typescript
#kernel(e: FrameEvent): void {
    this.#router(e);
    while (this.#nextCompartment !== null) {
        const nextCompartment = this.#nextCompartment!;
        this.#nextCompartment = null;
        const exitEvent = new FrameEvent("$<", this.#compartment.exitArgs);
        this.#router(exitEvent);
        this.#compartment = nextCompartment;
        if (nextCompartment.forwardEvent === null) {
            const enterEvent = new FrameEvent("$>", this.#compartment.enterArgs);
            this.#router(enterEvent);
        } else {
            const forwarded = nextCompartment.forwardEvent;
            nextCompartment.forwardEvent = null;
            if (forwarded.message === "$>") {
                this.#router(forwarded);
            } else {
                const enterEvent = new FrameEvent("$>", this.#compartment.enterArgs);
                this.#router(enterEvent);
                this.#router(forwarded);
            }
        }
    }
}
```

**Rust:**
```rust
fn kernel(&mut self, e: FrameEvent) {
    self.router(e);
    while let Some(next_compartment) = self.next_compartment.take() {
        let exit_event = FrameEvent::with_params("$<",
            std::mem::take(&mut self.compartment.exit_args));
        self.router(exit_event);
        self.compartment = next_compartment;
        if self.compartment.forward_event.is_none() {
            let enter_event = FrameEvent::with_params("$>",
                std::mem::take(&mut self.compartment.enter_args));
            self.router(enter_event);
        } else {
            let forwarded = self.compartment.forward_event.take().unwrap();
            if forwarded.message == "$>" {
                self.router(forwarded);
            } else {
                let enter_event = FrameEvent::with_params("$>",
                    std::mem::take(&mut self.compartment.enter_args));
                self.router(enter_event);
                self.router(forwarded);
            }
        }
    }
}
```

---

## 6. Router

**Python:**
```python
def __router(self, e):
    if self.__compartment.state == '__foo_state_Idle':
        self.__foo_state_Idle(e)
    elif self.__compartment.state == '__foo_state_Active':
        self.__foo_state_Active(e)
```

**TypeScript:**
```typescript
#router(e: FrameEvent): void {
    switch (this.#compartment.state) {
        case '__foo_state_Idle': this.#fooStateIdle(e); break;
        case '__foo_state_Active': this.#fooStateActive(e); break;
    }
}
```

**Rust:**
```rust
fn router(&mut self, e: FrameEvent) {
    match self.compartment.state.as_str() {
        "__foo_state_Idle" => self.foo_state_idle(e),
        "__foo_state_Active" => self.foo_state_active(e),
        _ => {}
    }
}
```

---

## 7. State Dispatch Functions

One per state. Routes event messages to handler methods.

### 7.1 Without HSM Parent

**Python:**
```python
def __foo_state_Idle(self, e):
    if e._message == "$>":
        self.__foo_state_Idle_enter(e)
    elif e._message == "$<":
        self.__foo_state_Idle_exit(e)
    elif e._message == "start":
        self.__foo_state_Idle_start(e)
```

**TypeScript:**
```typescript
#fooStateIdle(e: FrameEvent): void {
    switch (e.message) {
        case "$>": this.#fooStateIdleEnter(e); break;
        case "$<": this.#fooStateIdleExit(e); break;
        case "start": this.#fooStateIdleStart(e); break;
    }
}
```

**Rust:**
```rust
fn foo_state_idle(&mut self, e: FrameEvent) {
    match e.message.as_str() {
        "$>" => self.foo_state_idle_enter(e),
        "$<" => self.foo_state_idle_exit(e),
        "start" => self.foo_state_idle_start(e),
        _ => {}
    }
}
```

### 7.2 With HSM Parent

Unhandled events fall through to parent dispatch:

**Python:**
```python
def __foo_state_Child(self, e):
    if e._message == "specific":
        self.__foo_state_Child_specific(e)
    else:
        self.__foo_state_Parent(e)
```

Enter and exit events are NOT forwarded to parent. Only user-defined events.

### 7.3 With Default Forward (`=> $^`)

When the state has `=> $^` at the end, the state dispatch calls parent for all unmatched events. This is the same codegen as 7.2 — the `=> $^` declaration in the Frame source controls whether the `else` clause is generated.

---

## 8. Transition Method

**Python:**
```python
def __transition(self, next_compartment):
    self.__next_compartment = next_compartment
```

**TypeScript:**
```typescript
#transition(nextCompartment: FooCompartment): void {
    this.#nextCompartment = nextCompartment;
}
```

**Rust:**
```rust
fn transition(&mut self, next_compartment: FooCompartment) {
    self.next_compartment = Some(next_compartment);
}
```

---

## 9. State Stack Methods

### 9.1 Push

**Python:**
```python
def __state_stack_push(self, compartment):
    self.__state_stack.append(compartment)
```

**TypeScript:**
```typescript
#stateStackPush(compartment: FooCompartment): void {
    this.#stateStack.push(compartment);
}
```

**Rust:**
```rust
fn state_stack_push(&mut self, compartment: FooCompartment) {
    self.state_stack.push(compartment);
}
```

### 9.2 Pop

**Python:**
```python
def __state_stack_pop(self):
    return self.__state_stack.pop()
```

**TypeScript:**
```typescript
#stateStackPop(): FooCompartment {
    return this.#stateStack.pop()!;
}
```

**Rust:**
```rust
fn state_stack_pop(&mut self) -> FooCompartment {
    self.state_stack.pop().expect("state stack underflow")
}
```

---

## 10. Interface Methods

### 10.1 No Return

**Python:**
```python
def start(self):
    e = FrameEvent("start", None)
    self.__kernel(e)
```

**TypeScript:**
```typescript
start(): void {
    const e = new FrameEvent("start");
    this.#kernel(e);
}
```

**Rust:**
```rust
pub fn start(&mut self) {
    let e = FrameEvent::new("start");
    self.kernel(e);
}
```

### 10.2 With Parameters

**Python:**
```python
def process(self, data, priority):
    e = FrameEvent("process", {"data": data, "priority": priority})
    self.__kernel(e)
```

### 10.3 With Return (No Default)

**Python:**
```python
def get_status(self) -> str:
    self.__return_stack.append(None)
    e = FrameEvent("get_status", None)
    self.__kernel(e)
    return self.__return_stack.pop()
```

**TypeScript:**
```typescript
getStatus(): string {
    this.#returnStack.push(null);
    const e = new FrameEvent("get_status");
    this.#kernel(e);
    return this.#returnStack.pop();
}
```

**Rust:**
```rust
pub fn get_status(&mut self) -> String {
    self.return_stack.push(Box::new(None::<String>));
    let e = FrameEvent::new("get_status");
    self.kernel(e);
    let val = self.return_stack.pop().unwrap();
    *val.downcast::<Option<String>>().unwrap()
        .unwrap_or_default()
}
```

### 10.4 With Default Return

```frame
interface:
    getDecision(): str = "yes"
```

**Python:**
```python
def get_decision(self) -> str:
    self.__return_stack.append("yes")    # push default
    e = FrameEvent("get_decision", None)
    self.__kernel(e)
    return self.__return_stack.pop()
```

---

## 11. Splicer Expansion Rules

These rules define how each Frame construct within a handler body is expanded into generated code. The splicer operates on the native code regions identified by the NativeRegionScanner.

### 11.1 Simple Transition: `-> $Next`

**Python:**
```python
next_compartment = FooCompartment('__foo_state_Next')
next_compartment.state_vars["x"] = 0          # one line per $.var in $Next
next_compartment.state_vars["y"] = "default"
self.__transition(next_compartment)
return
```

**TypeScript:**
```typescript
const nextCompartment = new FooCompartment('__foo_state_Next');
nextCompartment.stateVars["x"] = 0;
nextCompartment.stateVars["y"] = "default";
this.#transition(nextCompartment);
return;
```

**Rust:**
```rust
let mut next_compartment = FooCompartment::new("__foo_state_Next");
next_compartment.state_vars.insert("x".to_string(), Box::new(0i32));
next_compartment.state_vars.insert("y".to_string(), Box::new(String::from("default")));
self.transition(next_compartment);
return;
```

**Rule:** Every transition expansion ends with `return`. Non-negotiable.

**Rule:** State variable initializers come from the target state's `$.var` declarations. If the target state has no state variables, no init lines are emitted.

### 11.2 Transition with State Args: `-> $Next(job_id)`

Same as 11.1 plus:
```python
next_compartment.state_args["job_id"] = job_id
```

State args are set BEFORE state var inits (order: create → state_args → state_vars → transition → return).

### 11.3 Transition with Enter Args: `-> (msg, count) $Next`

Same as 11.1 plus:
```python
next_compartment.enter_args["msg"] = msg
next_compartment.enter_args["count"] = count
```

### 11.4 Transition with Exit Args: `(reason) -> $Next`

Exit args are set on the CURRENT compartment, before creating the next:
```python
self.__compartment.exit_args["reason"] = reason
next_compartment = FooCompartment('__foo_state_Next')
# ... state var inits ...
self.__transition(next_compartment)
return
```

### 11.5 Full Transition: `(exit) -> (enter) $Next(state)`

```python
self.__compartment.exit_args["exit"] = exit
next_compartment = FooCompartment('__foo_state_Next')
next_compartment.state_args["state"] = state
next_compartment.enter_args["enter"] = enter
next_compartment.state_vars["x"] = 0
self.__transition(next_compartment)
return
```

**Order:** exit_args on current → create compartment → state_args → enter_args → state_vars → transition → return

### 11.6 Event Forwarding: `-> => $Next`

Same as 11.1 plus:
```python
next_compartment.forward_event = e    # stash current event
```

Set AFTER state var inits, BEFORE transition call.

### 11.7 Transition to Popped State: `-> pop$`

```python
next_compartment = self.__state_stack_pop()
self.__transition(next_compartment)
return
```

No state var initialization — the popped compartment already has its preserved variables.

### 11.8 Forward to Parent: `=> $^`

**Python:**
```python
self.__foo_state_Parent(e)
return
```

**TypeScript:**
```typescript
this.#fooStateParent(e);
return;
```

**Rust:**
```rust
self.foo_state_parent(e);
return;
```

### 11.9 Stack Push (Current): `push$`

**Python:**
```python
self.__state_stack_push(self.__compartment)
```

**TypeScript:**
```typescript
this.#stateStackPush(this.#compartment);
```

**Rust:**
```rust
self.state_stack_push(std::mem::replace(&mut self.compartment,
    FooCompartment::new("")));  // placeholder, will be replaced by transition
```

Note: Rust requires ownership transfer. The exact pattern depends on whether a transition follows immediately.

### 11.10 Stack Push (Named): `push$ $Fallback`

**Python:**
```python
__fallback_compartment = FooCompartment('__foo_state_Fallback')
__fallback_compartment.state_vars["x"] = 0    # init state vars
self.__state_stack_push(__fallback_compartment)
```

### 11.11 Stack Pop (Discard): `pop$`

**Python:**
```python
self.__state_stack_pop()
```

### 11.12 State Variable Read: `$.counter`

**Python:**
```python
self.__compartment.state_vars["counter"]
```

**TypeScript:**
```typescript
this.#compartment.stateVars["counter"]
```

**Rust:**
```rust
*self.compartment.state_vars.get("counter").unwrap()
    .downcast_ref::<i32>().unwrap()
```

This is an inline substitution — the `$.counter` token in native code is replaced with the compartment access expression. The surrounding native code is preserved.

### 11.13 State Variable Write: `$.counter = expr`

**Python:**
```python
self.__compartment.state_vars["counter"] = expr
```

**TypeScript:**
```typescript
this.#compartment.stateVars["counter"] = expr;
```

**Rust:**
```rust
self.compartment.state_vars.insert("counter".to_string(), Box::new(expr));
```

The `expr` is native code and passes through unchanged.

### 11.14 System Return Assign: `system.return = expr`

**Python:**
```python
self.__return_stack[-1] = expr
```

**TypeScript:**
```typescript
this.#returnStack[this.#returnStack.length - 1] = expr;
```

**Rust:**
```rust
*self.return_stack.last_mut().unwrap() = Box::new(expr);
```

### 11.15 System Return Read: `system.return`

**Python:**
```python
self.__return_stack[-1]
```

Inline substitution.

### 11.16 Return Value Sugar (in Handler): `return expr`

**Python:**
```python
self.__return_stack[-1] = expr
return
```

**TypeScript:**
```typescript
this.#returnStack[this.#returnStack.length - 1] = expr;
return;
```

### 11.17 Return Value (in Action): `return expr`

Pass through unchanged. No transformation.

### 11.18 Bare Return: `return`

Pass through unchanged in all contexts.

---

## 12. Actions

Actions are private methods. Their bodies are entirely native code except for `system.return` access.

**Python:**
```python
def __validate(self, data):
    # native code from action body
    # system.return references are rewritten
    # return <expr> is NOT rewritten (native function return)
```

**TypeScript:**
```typescript
#validate(data: any): any {
    // native code
}
```

**Rust:**
```rust
fn validate(&mut self, data: ...) -> ... {
    // native code
}
```

---

## 13. Operations

Operations are public methods. Bodies are entirely native code. No Frame construct rewriting.

**Python:**
```python
def get_temp(self):
    return self.temp

@staticmethod
def add(a, b):
    return a + b
```

**TypeScript:**
```typescript
getTemp(): number {
    return this.temp;
}

static add(a: number, b: number): number {
    return a + b;
}
```

**Rust:**
```rust
pub fn get_temp(&self) -> f64 {
    self.temp
}

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
```

---

## 14. Persistence Methods (when `@@persist`)

### 14.1 Save

**Python:**
```python
def _save(self) -> str:
    import json
    snapshot = {
        "schemaVersion": 1,
        "systemName": "Foo",
        "state": self.__compartment.state,
        "stateArgs": dict(self.__compartment.state_args),
        "stateVars": dict(self.__compartment.state_vars),
        "domain": {
            "count": self.count,
            "label": self.label,
        },
        "stack": [
            {
                "state": c.state,
                "stateArgs": dict(c.state_args),
                "stateVars": dict(c.state_vars),
            }
            for c in self.__state_stack
        ]
    }
    return json.dumps(snapshot)
```

### 14.2 Restore

**Python:**
```python
@classmethod
def _restore(cls, data: str) -> 'Foo':
    import json
    snapshot = json.loads(data)
    instance = object.__new__(cls)
    instance.__state_stack = []
    instance.__return_stack = []
    instance.__next_compartment = None
    instance.__compartment = FooCompartment(snapshot["state"])
    instance.__compartment.state_args = dict(snapshot.get("stateArgs", {}))
    instance.__compartment.state_vars = dict(snapshot.get("stateVars", {}))
    instance.count = snapshot["domain"]["count"]
    instance.label = snapshot["domain"]["label"]
    for entry in snapshot.get("stack", []):
        c = FooCompartment(entry["state"])
        c.state_args = dict(entry.get("stateArgs", {}))
        c.state_vars = dict(entry.get("stateVars", {}))
        instance.__state_stack.append(c)
    return instance
```

Note: `_restore` does NOT call the constructor or trigger enter events. The machine is reconstituted at rest.

---

## 15. Complete Generated Example

**Input:**

```frame
@@target python_3
@@codegen {
    frame_event: on,
    runtime: kernel
}

@@system Counter {
    interface:
        increment()
        getCount(): int = 0

    machine:
        $Counting {
            $.count: int = 0

            $>() {
                print("Entered Counting")
            }

            increment() {
                $.count = $.count + 1
                if $.count >= 10:
                    -> $Full
            }

            getCount(): int {
                return $.count
            }
        }

        $Full {
            $>() {
                print("Counter is full!")
            }

            getCount(): int {
                return 10
            }
        }

    domain:
        var label: str = "default"
}
```

**Output:**

```python
class Counter:

    class FrameEvent:
        def __init__(self, message, parameters=None):
            self._message = message
            self._parameters = parameters or {}

    class CounterCompartment:
        def __init__(self, state):
            self.state = state
            self.state_args = {}
            self.state_vars = {}
            self.enter_args = {}
            self.exit_args = {}
            self.forward_event = None

    def __init__(self):
        self.__state_stack = []
        self.__return_stack = []
        self.__compartment = Counter.CounterCompartment('__counter_state_Counting')
        self.__next_compartment = None
        self.__compartment.state_vars["count"] = 0
        self.label = "default"
        e = Counter.FrameEvent("$>", self.__compartment.enter_args)
        self.__kernel(e)

    # ---- Runtime Infrastructure ----

    def __kernel(self, e):
        self.__router(e)
        while self.__next_compartment is not None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            exit_event = Counter.FrameEvent("$<", self.__compartment.exit_args)
            self.__router(exit_event)
            self.__compartment = next_compartment
            if next_compartment.forward_event is None:
                enter_event = Counter.FrameEvent("$>", self.__compartment.enter_args)
                self.__router(enter_event)
            else:
                forwarded = next_compartment.forward_event
                next_compartment.forward_event = None
                if forwarded._message == "$>":
                    self.__router(forwarded)
                else:
                    enter_event = Counter.FrameEvent("$>", self.__compartment.enter_args)
                    self.__router(enter_event)
                    self.__router(forwarded)

    def __router(self, e):
        if self.__compartment.state == '__counter_state_Counting':
            self.__counter_state_Counting(e)
        elif self.__compartment.state == '__counter_state_Full':
            self.__counter_state_Full(e)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

    # ---- State Dispatch ----

    def __counter_state_Counting(self, e):
        if e._message == "$>":
            self.__counter_state_Counting_enter(e)
        elif e._message == "increment":
            self.__counter_state_Counting_increment(e)
        elif e._message == "getCount":
            self.__counter_state_Counting_getCount(e)

    def __counter_state_Full(self, e):
        if e._message == "$>":
            self.__counter_state_Full_enter(e)
        elif e._message == "getCount":
            self.__counter_state_Full_getCount(e)

    # ---- Interface Methods ----

    def increment(self):
        e = Counter.FrameEvent("increment", None)
        self.__kernel(e)

    def getCount(self) -> int:
        self.__return_stack.append(0)
        e = Counter.FrameEvent("getCount", None)
        self.__kernel(e)
        return self.__return_stack.pop()

    # ---- Handler Methods ----

    def __counter_state_Counting_enter(self, e):
        print("Entered Counting")

    def __counter_state_Counting_increment(self, e):
        self.__compartment.state_vars["count"] = self.__compartment.state_vars["count"] + 1
        if self.__compartment.state_vars["count"] >= 10:
            next_compartment = Counter.CounterCompartment('__counter_state_Full')
            self.__transition(next_compartment)
            return

    def __counter_state_Counting_getCount(self, e):
        self.__return_stack[-1] = self.__compartment.state_vars["count"]
        return

    def __counter_state_Full_enter(self, e):
        print("Counter is full!")

    def __counter_state_Full_getCount(self, e):
        self.__return_stack[-1] = 10
        return
```

---

## 16. Test Requirements

Every backend must pass these test patterns:

| # | Test | Validates |
|---|------|-----------|
| 1 | Simple transition A → B | Compartment creation, exit/enter fire, state changes |
| 2 | Transition with state args | `state_args` populated, accessible in target |
| 3 | Transition with enter args | `enter_args` passed to enter handler |
| 4 | Transition with exit args | `exit_args` passed to exit handler |
| 5 | Event forwarding `-> =>` | Forwarded event dispatched after enter |
| 6 | HSM implicit forward | Unhandled event bubbles to parent |
| 7 | HSM explicit `=> $^` | Explicit forward in handler |
| 8 | State var init + modify | `$.x` initialized on entry, modified in handler |
| 9 | State var reset on reentry | `$.x` reinitialized when state re-entered |
| 10 | State var preserved by push/pop | `push$` → `-> pop$` preserves `$.x` value |
| 11 | `system.return` basic | Set in handler, returned to caller |
| 12 | `system.return` in action | Action sets it, interface returns it |
| 13 | `system.return` chain | Set across transition, last writer wins |
| 14 | `system.return` default | Default returned when handler doesn't set |
| 15 | Return stack reentrancy | Nested interface calls maintain separate returns |
| 16 | Service pattern | Enter-handler transition chains, no stack overflow |
| 17 | System params (all 3 groups) | State params, enter params, domain overrides |
| 18 | Operations bypass | Operations don't trigger state machine |
| 19 | Static operations | No self/this |
| 20 | `return value` sugar in handler | Sets system.return + exits |
| 21 | `return value` in action | Normal function return |
| 22 | `@@codegen` auto-enable | Features auto-enabled when needed |
| 23 | Push named state `push$ $X` | Creates new compartment with state vars |
| 24 | Persistence save/restore | Roundtrip serialize/deserialize |