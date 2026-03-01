# Frame V4 HSM Compartment Architecture

**Version:** 0.1 (Draft)
**Date:** February 2026
**Status:** Design Document - Pre-Implementation

---

## 1. Overview

This document specifies the compartment architecture for Hierarchical State Machines (HSM) in Frame V4. The key insight is that each state in an HSM hierarchy requires its own compartment, linked via `parent_compartment` references.

---

## 2. Core Architecture

### 2.1 Compartment Chain

When a state has a parent (`$Child => $Parent`), the runtime maintains a chain of compartments:

```
Child Compartment
├── state: "Child"
├── state_args: {child's params}
├── state_vars: {child's vars}
└── parent_compartment ──────► Parent Compartment
                               ├── state: "Parent"
                               ├── state_args: {parent's params}
                               ├── state_vars: {parent's vars}
                               └── parent_compartment: null (root)
```

### 2.2 Compartment Creation Helpers

Each state with HSM relationships gets a helper function:

```python
def _create_Child_compartment(self, state_args=None, enter_args=None, parent_comp=None):
    # If no explicit parent_comp provided, create default parent chain
    if parent_comp is None:
        parent_comp = self._create_Parent_compartment()

    comp = Compartment("Child", parent_compartment=parent_comp)
    comp.state_args = state_args or {}
    comp.enter_args = enter_args or {}
    # Initialize state vars (can reference state_args)
    comp.state_vars = {
        "child_count": 0,
        "derived": comp.state_args.get("x", 0) * 2  # Can use state params
    }
    return comp
```

### 2.3 Nested Call Pattern for Transitions

For a transition with explicit chain parameters:

```frame
-> $S1(1,"foo") => $S2(2,false) => $S3("bar",1.0)
```

**Generated code (build from root to leaf):**

```python
# Build chain: S3 (root) -> S2 -> S1 (leaf/current)
__comp_S3 = self._create_S3_compartment(state_args={"m": "bar", "n": 1.0})
__comp_S2 = self._create_S2_compartment(state_args={"x": 2, "y": False}, parent_comp=__comp_S3)
__comp_S1 = self._create_S1_compartment(state_args={"a": 1, "s": "foo"}, parent_comp=__comp_S2)
self.__transition(__comp_S1)
```

---

## 3. State Parameters

### 3.1 Declaration Syntax

States can declare parameters in their definition:

```frame
$StateName(param1: type1, param2: type2) => $Parent {
    // state body
}
```

### 3.2 Transition Chain Syntax

When transitioning, the full ancestor chain with parameters must be specified if ANY ancestor has parameters:

```frame
-> $Leaf(leaf_args) => $Middle(middle_args) => $Root(root_args)
```

### 3.3 Default Values

State parameters can have default values:

```frame
$State(required: int, optional: str = "default") {}
```

**Rules:**
- If param has default, it's optional in transition
- If param has NO default, it's required in transition
- Positional args fill left-to-right

### 3.4 Required Chain Rule

**RULE:** If any ancestor state has required parameters (no defaults), the transition MUST specify the full chain.

**Valid:**
```frame
$A => $B(x: int) {}
$B(x: int) {}

-> $A => $B(42)    // Chain specified because B has required param
```

**Valid with defaults:**
```frame
$A => $B(x: int = 0) {}
$B(x: int = 0) {}

-> $A              // Valid: B's param has default, chain not required
-> $A => $B(42)    // Also valid: explicit override
```

**Invalid:**
```frame
$A => $B(x: int) {}  // No default
$B(x: int) {}

-> $A              // ERROR: Ancestor B has required param, chain required
-> $A => $B        // ERROR: B requires param value
```

### 3.5 No-Param Ancestors

If no ancestor has parameters, simple transition syntax is allowed:

```frame
$A => $B {}
$B => $C {}
$C {}

-> $A              // Valid: no ancestor has params, helper creates default chain
```

---

## 4. State Variable Initialization

### 4.1 Initialization Timing

State variables are initialized when the compartment is **created**, not when `$>()` runs.

```frame
$State(multiplier: int) {
    $.base: int = 10
    $.computed: int = $.base * multiplier   // Can use state params

    $>() {
        // Runs ONLY when transitioning TO this state
        // NOT when this is a parent in HSM hierarchy
    }
}
```

### 4.2 Scope During Initialization

State variable initializers can access:
- Literal values
- State parameters (from `state_args`)
- Domain variables (`self.domain_var`)
- Function calls (`self.compute()`, external functions)
- Other state variables declared earlier in same state (`$.earlier_var`)

### 4.3 Same-Named Variables in Hierarchy

Parent and child can have state variables with the same name. They are independent:

```frame
$Child => $Parent {
    $.count: int = 0      // Child's count
}

$Parent {
    $.count: int = 100    // Parent's count (separate compartment)
}
```

---

## 5. Enter/Exit Handler Rules

### 5.1 Signature Matching

If a child state defines an enter or exit handler, its signature MUST match the parent's handler signature (if parent has one).

**Valid:**
```frame
$Child => $Parent {
    $>(a, b) {}          // Matches Parent's $>(a, b)
}

$Parent {
    $>(a, b) {}
}
```

**Invalid:**
```frame
$Child => $Parent {
    $>(a) {}             // ERROR: Parent has $>(a, b)
}

$Parent {
    $>(a, b) {}
}
```

### 5.2 Handler Omission

Child can omit handlers that parent defines. The parent's handler will be invoked via forwarding.

```frame
$Child => $Parent {
    // No $>() defined - OK
    // No <$() defined - OK
}

$Parent {
    $>(a, b) { /* runs when entering Child or Parent */ }
    <$(x) { /* runs when exiting Child or Parent */ }
}
```

### 5.3 When Enter/Exit Handlers Run

- `$>()` runs ONLY when transitioning TO that specific state
- `$>()` does NOT run for parent states in HSM hierarchy during child's enter
- Same for `<$()` - only runs when exiting that specific state

**Example:**
```frame
-> $Child  // Child's $>() runs. Parent's $>() does NOT run.
```

---

## 6. Event Forwarding (`=> $^`)

### 6.1 Compartment Context During Forward

When child forwards to parent via `=> $^`, the parent handler executes with access to the **parent's compartment**:

```python
def _state_Child(self, __e):
    if __e._message == "get_parent_count":
        # Forward to parent
        self._state_Parent(__e)  # Parent accesses parent_compartment

def _state_Parent(self, __e):
    if __e._message == "get_parent_count":
        # Access parent's state vars via parent_compartment
        parent_comp = self.__compartment.parent_compartment
        return parent_comp.state_vars["parent_count"]
```

### 6.2 State-Level Default Forward

The `=> $^` at state level forwards ALL unhandled events:

```frame
$Child => $Parent {
    specific_event() { /* handled by child */ }
    => $^   // All other events forward to parent
}
```

---

## 7. Stack Operations (push$/pop$)

### 7.1 Pushing HSM State

When `push$` is called, the ENTIRE compartment chain is preserved:

```python
def _state_stack_push(self):
    # Copy the current compartment (includes parent_compartment reference)
    self._state_stack.append(self.__compartment.copy())
```

### 7.2 Popping HSM State

When `-> pop$` is called, the entire chain is restored:

```python
def _state_stack_pop(self):
    restored = self._state_stack.pop()
    # restored.parent_compartment is intact
    self.__compartment = restored
```

**Note:** The parent_compartment references are preserved, maintaining the full hierarchy.

---

## 8. Transition Scenarios

### 8.1 Parent Transitioning to Child

Valid - creates fresh compartment chain:

```frame
$Parent {
    event() {
        -> $Child   // Creates new Child compartment with new Parent compartment
    }
}
$Child => $Parent {}
```

### 8.2 Child Transitioning to Sibling

Both children share same parent declaration but get separate parent compartments:

```frame
$ChildA => $Parent {
    switch() {
        -> $ChildB   // New ChildB compartment with new Parent compartment
    }
}
$ChildB => $Parent {}
$Parent {}
```

### 8.3 Forward Transition with HSM

`-> =>` (transition with event forwarding) works with HSM:

```frame
$StateA {
    event() {
        -> => $Child   // Transition to Child, forward event
    }
}
$Child => $Parent {}
```

---

## 9. Validation Requirements

The compiler (AST/Arcanum) must validate:

| Check | Error |
|-------|-------|
| Ancestor has params but transition doesn't specify chain | E4XX |
| Chain specifies wrong number of params for a state | E4XX |
| Enter handler signature doesn't match parent's | E4XX |
| Exit handler signature doesn't match parent's | E4XX |
| Circular inheritance (`$A => $B`, `$B => $A`) | E4XX |
| Reference to undefined parent state | E402 |

---

## 10. Implementation Checklist

### Phase 1: Foundation
- [ ] Create compartment chain on system initialization
- [ ] Generate `_create_<State>_compartment()` helpers
- [ ] Parent handlers access `parent_compartment.state_vars`
- [ ] Test 40 passes (HSM parent state vars)

### Phase 2: Transition Chain Syntax
- [ ] Parse `-> $A => $B(params) => $C(params)` syntax
- [ ] Generate nested compartment creation calls
- [ ] Validate chain completeness when ancestor has params

### Phase 3: Handler Signature Validation
- [ ] Validate enter handler signatures match
- [ ] Validate exit handler signatures match
- [ ] Allow handler omission in child

### Phase 4: Stack Operations
- [ ] Verify push$ preserves parent_compartment chain
- [ ] Verify pop$ restores parent_compartment chain

---

## 11. Resolved Design Decisions

1. **Actions and state vars**: Actions are system-level and have NO access to state variables (`$.`). They can only access domain variables.

2. **State param defaults**: State params can have default values using Frame notation:
   ```frame
   $State(required: int, optional: str = "default") {}
   ```
   - If param has default, it's optional in transition
   - If param has NO default, it's required
   - Positional args fill left-to-right

3. **Deep hierarchies (3+ levels)**: Same nested call pattern scales naturally.

4. **Performance**: Creating compartment chain on every transition is acceptable - compartments are lightweight.

---

## Appendix: Example Generated Code

**Frame source:**
```frame
@@system HSM {
    machine:
        $Child(x: int) => $Parent(y: int) {
            $.child_val: int = x * 2

            get_parent_val(): int {
                => $^
            }
        }

        $Parent(y: int) {
            $.parent_val: int = y + 100

            get_parent_val(): int {
                return $.parent_val
            }
        }
}
```

**Generated Python:**
```python
def _create_Parent_compartment(self, state_args=None, enter_args=None, parent_comp=None):
    comp = HSMCompartment("Parent", parent_compartment=parent_comp)
    comp.state_args = state_args or {}
    comp.enter_args = enter_args or {}
    y = comp.state_args.get("y", 0)
    comp.state_vars = {"parent_val": y + 100}
    return comp

def _create_Child_compartment(self, state_args=None, enter_args=None, parent_comp=None):
    if parent_comp is None:
        parent_comp = self._create_Parent_compartment()
    comp = HSMCompartment("Child", parent_compartment=parent_comp)
    comp.state_args = state_args or {}
    comp.enter_args = enter_args or {}
    x = comp.state_args.get("x", 0)
    comp.state_vars = {"child_val": x * 2}
    return comp

def _state_Parent(self, __e):
    # Access via parent_compartment when forwarded from child
    comp = self.__compartment
    if comp.state == "Child":
        comp = comp.parent_compartment

    if __e._message == "get_parent_val":
        self._context_stack[-1]._return = comp.state_vars["parent_val"]
        return

def _state_Child(self, __e):
    if __e._message == "get_parent_val":
        self._state_Parent(__e)  # Forward to parent
```
