> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Testing Infrastructure

This document describes the testing infrastructure and test pragmas for Frame V4.

---

## Overview

Frame V4 testing uses a shared test environment with Docker-based test runners for each target language. Tests are Frame source files that compile to native code, execute, and validate behavior.

---

## Test Environment

### Directory Structure

```
framepiler_test_env/
├── common/
│   └── test-frames/
│       └── v4/
│           └── prt/           # PRT language tests
│               ├── 01_minimal.frm
│               ├── 02_interface.frm
│               └── ...
├── framepiler/
│   └── docker/
│       └── target/release/
│           └── frame-docker-runner   # Test runner binary
```

### Running Tests

```bash
# Set environment
export FRAMEPILER_TEST_ENV=$(pwd)/framepiler_test_env

# Run a specific test
frame-docker-runner python_3 01_minimal --framec ./target/release/framec

# Run all tests for a language
frame-docker-runner python_3 --all --framec ./target/release/framec

# Run tests for all PRT languages
for lang in python_3 typescript rust; do
    frame-docker-runner $lang --all --framec ./target/release/framec
done
```

---

## Test File Structure

A test file is a standard Frame source file with a test harness in native code:

```frame
@@target python_3

@@system TestSystem {
    interface:
        doSomething(): str

    machine:
        $Start {
            doSomething(): str {
                return "success"
            }
        }
}

# Test harness (native Python)
if __name__ == '__main__':
    s = TestSystem()
    result = s.doSomething()
    if result == "success":
        print("PASS: doSomething returned expected value")
    else:
        print(f"FAIL: expected 'success', got '{result}'")
        raise AssertionError("Test failed")
```

### Test Conventions

1. **Print PASS/FAIL messages** — Clear output for test runner
2. **Raise exception on failure** — Non-zero exit code signals failure
3. **Self-contained** — Each test file is independent
4. **Minimal** — Test one feature per file

---

## Test Pragmas

Test pragmas are Frame annotations that control test behavior.

### @@expect (Planned)

Declares expected compiler behavior for negative tests.

```frame
@@expect(error: E402)
@@target python_3

@@system BadSystem {
    machine:
        $Start {
            go() {
                -> $NonExistent   // Should produce E402
            }
        }
}
```

| Parameter | Description |
|-----------|-------------|
| `error: E###` | Expect specific error code |
| `warning: W###` | Expect specific warning |
| `success` | Expect successful compilation |

### @@skip (Planned)

Skip test for specific languages or conditions.

```frame
@@skip(languages: [rust])
@@target python_3

@@system PythonOnly {
    // Test uses Python-specific features
}
```

---

## Test Categories

### 1. Structural Tests

Validate Frame parsing and system structure.

```frame
@@target python_3

@@system StructureTest {
    interface:
        method1()

    machine:
        $State1 {
            method1() { }
        }

    actions:
        helper() { }

    domain:
        x = 0
}
```

### 2. Transition Tests

Validate state transitions and lifecycle.

```frame
@@target python_3

@@system TransitionTest {
    interface:
        go()
        getState(): str

    machine:
        $A {
            $>() { self.log.append("enter A") }
            $<() { self.log.append("exit A") }
            go() { -> $B }
            getState(): str { return "A" }
        }

        $B {
            $>() { self.log.append("enter B") }
            getState(): str { return "B" }
        }

    domain:
        log = []
}

if __name__ == '__main__':
    t = TransitionTest()
    assert t.getState() == "A"
    t.go()
    assert t.getState() == "B"
    assert t.log == ["enter A", "exit A", "enter B"]
    print("PASS")
```

### 3. HSM Tests

Validate hierarchical state machine features.

```frame
@@target python_3

@@system HSMTest {
    interface:
        event1()
        event2()

    machine:
        $Parent {
            event1() {
                self.handled_by = "parent"
            }
        }

        $Child => $Parent {
            event2() {
                self.handled_by = "child"
            }

            => $^   // Forward unhandled to parent
        }

    domain:
        handled_by = ""
}
```

### 4. Stack Tests

Validate push/pop operations.

```frame
@@target python_3

@@system StackTest {
    interface:
        push_and_go()
        return_back()
        getState(): str

    machine:
        $Main {
            push_and_go() {
                push$
                -> $Temporary
            }
            getState(): str { return "Main" }
        }

        $Temporary {
            return_back() {
                -> pop$
            }
            getState(): str { return "Temporary" }
        }
}
```

### 5. Native Code Tests

Validate native code preservation.

```frame
@@target python_3

import json

@@system NativeTest {
    interface:
        process(data: str): dict

    machine:
        $Ready {
            process(data: str): dict {
                # All of this is native Python
                parsed = json.loads(data)
                result = {
                    "count": len(parsed),
                    "keys": list(parsed.keys())
                }
                return result
            }
        }
}
```

### 6. Persistence Tests

Validate `@@persist` code generation.

```frame
@@target python_3

@@persist
@@system PersistTest {
    interface:
        setValue(v: int)
        getValue(): int

    machine:
        $Active {
            setValue(v: int) {
                self.value = v
            }
            getValue(): int {
                return self.value
            }
        }

    domain:
        value = 0
}

if __name__ == '__main__':
    p1 = PersistTest()
    p1.setValue(42)

    # Save state
    snapshot = p1._save()

    # Restore to new instance
    p2 = PersistTest._restore(snapshot)
    assert p2.getValue() == 42
    print("PASS")
```

---

## Validation Tests

Tests that verify compiler error detection.

### Expected Error Tests

```frame
@@expect(error: E402)
@@target python_3

@@system UnknownStateTest {
    machine:
        $Start {
            go() {
                -> $DoesNotExist
            }
        }
}
```

### Expected Success Tests

```frame
@@expect(success)
@@target python_3

@@system ValidSystem {
    machine:
        $Start {
            go() {
                -> $End
            }
        }
        $End { }
}
```

---

## Test Runner Output

```
Running tests for python_3...
  01_minimal ................ PASS
  02_interface .............. PASS
  03_transition ............. PASS
  04_native_code ............ PASS
  05_enter_exit ............. PASS
  06_domain_vars ............ PASS
  07_params ................. PASS
  08_hsm .................... PASS
  09_stack .................. PASS

Total: 9 passed, 0 failed
```

---

## Writing New Tests

### 1. Create Test File

```bash
touch framepiler_test_env/common/test-frames/v4/prt/10_new_feature.frm
```

### 2. Write Frame Code

```frame
@@target python_3

@@system NewFeatureTest {
    // Test the new feature
}

if __name__ == '__main__':
    # Test harness
    t = NewFeatureTest()
    # assertions
    print("PASS")
```

### 3. Run Test

```bash
frame-docker-runner python_3 10_new_feature --framec ./target/release/framec
```

### 4. Add to All Languages

Copy and adapt for TypeScript and Rust targets.

---

## Debugging Failed Tests

### 1. View Generated Code

```bash
./target/release/framec test.frm -l python_3 -o /tmp/test.py
cat /tmp/test.py
```

### 2. Run Manually

```bash
python3 /tmp/test.py
```

### 3. Check Compiler Output

```bash
./target/release/framec test.frm -l python_3 --verbose
```

---

## CI Integration

Tests run automatically on PR/push:

```yaml
# .github/workflows/test.yml
- name: Run PRT Tests
  run: |
    for lang in python_3 typescript rust; do
      frame-docker-runner $lang --all --framec ./target/release/framec
    done
```
