# Frame V4 Error Codes

**Version:** 1.0
**Date:** February 2026
**Status:** Normative

---

## Error Code Ranges

| Range | Category |
|-------|----------|
| E0xx | Parse errors |
| E1xx | Structural errors |
| E4xx | Semantic errors |
| W4xx | Warnings |

---

## Parse Errors (E0xx)

| Code | Name | Description |
|------|------|-------------|
| E001 | `parse-error` | Malformed Frame syntax |
| E002 | `unexpected-token` | Unexpected token in Frame construct |
| E003 | `unclosed-block` | Missing closing brace or delimiter |

---

## Structural Errors (E1xx)

| Code | Name | Description |
|------|------|-------------|
| E105 | `missing-target` | `@@target` directive missing or invalid |
| E111 | `duplicate-system-param` | Duplicate parameter in system declaration |
| E113 | `section-order` | System sections out of order |
| E114 | `duplicate-section` | Section declared more than once |
| E115 | `multiple-main` | Multiple `fn main` functions |

---

## Semantic Errors (E4xx)

### State Errors (E40x)

| Code | Name | Description |
|------|------|-------------|
| E400 | `unreachable-code` | Code after terminal statement (transition, return) |
| E401 | `frame-in-action` | Frame statement in action or operation body |
| E402 | `unknown-state` | Transition targets undefined state |
| E403 | `invalid-forward` | `=> $^` in state without parent |
| E404 | `duplicate-state` | State name declared more than once |

### Parameter Errors (E405-E418)

| Code | Name | Description |
|------|------|-------------|
| E405 | `param-arity-mismatch` | Wrong number of parameters in transition |
| E406 | `interface-mismatch` | Handler signature doesn't match interface |
| E416 | `start-state-param-mismatch` | System `$(params)` don't match start state |
| E417 | `enter-param-mismatch` | System `$>(params)` don't match start state's enter handler |
| E418 | `domain-param-missing` | Domain parameter has no matching domain variable |

### State Variable Errors (E42x)

| Code | Name | Description |
|------|------|-------------|
| E420 | `duplicate-state-var` | State variable declared more than once in same state |
| E421 | `state-var-cross-access` | Accessing `$.var` from different state (if detectable) |

### HSM Errors (E43x)

| Code | Name | Description |
|------|------|-------------|
| E430 | `hsm-cycle` | Circular parent chain (A => B => A) |
| E431 | `orphan-parent` | Parent state does not exist |
| E432 | `forward-without-parent` | Default forward (`=> $^`) in state without parent |

### Stack Errors (E44x)

| Code | Name | Description |
|------|------|-------------|
| E440 | `push-disabled` | `push$` used but state stack disabled |
| E441 | `pop-disabled` | `pop$` used but state stack disabled |

### Return Errors (E45x)

| Code | Name | Description |
|------|------|-------------|
| E450 | `return-in-operation` | `system.return` used in operation |
| E451 | `return-type-mismatch` | Handler return type doesn't match interface |

### Codegen Errors (E46x)

| Code | Name | Description |
|------|------|-------------|
| E460 | `unknown-codegen-key` | Unknown key in `@@codegen` block |
| E461 | `invalid-codegen-value` | Invalid value for codegen key |

---

## Warnings (W4xx)

| Code | Name | Description |
|------|------|-------------|
| W400 | `auto-enable-frame-event` | `frame_event` auto-enabled due to feature usage |
| W401 | `auto-enable-state-stack` | `state_stack` auto-enabled due to `push$`/`pop$` |
| W410 | `unreachable-state` | State has no incoming transitions (except start state) |
| W411 | `dead-handler` | Handler is never called |
| W420 | `shadowed-handler` | Child handler shadows parent handler |

---

## Error Message Format

```
[E402] Unknown state 'NonExistent' in transition
  --> file.frm:25:17
   |
25 |             -> $NonExistent
   |                 ^^^^^^^^^^^ state not defined
   |
   = help: Did you mean '$Existing'?
```

---

## Implementation Status

| Code | Implemented | Notes |
|------|-------------|-------|
| E001 | Yes | Parser errors |
| E105 | Yes | Target validation |
| E111 | Yes | System param validation |
| E113 | Partial | Section order checking |
| E114 | Partial | Duplicate section checking |
| E400 | No | Unreachable code detection |
| E401 | Partial | Frame-in-action checking |
| E402 | Yes | Unknown state validation |
| E403 | No | Forward validation disabled |
| E404 | Yes | Duplicate state validation |
| E405 | Yes | Parameter arity checking |
| E406 | Partial | Interface matching |
| E416 | Yes | Start state param validation |
| E417 | Yes | Enter param validation |
| E418 | Yes | Domain param validation |
| E420-E461 | No | Not implemented |
| W4xx | No | Warnings not implemented |

---

## Adding New Error Codes

When adding a new error code:

1. Choose the appropriate range based on category
2. Add to this document with name and description
3. Implement in `frame_validator.rs`
4. Add test cases in `test-frames/v4/validation/`
5. Update implementation status table above
