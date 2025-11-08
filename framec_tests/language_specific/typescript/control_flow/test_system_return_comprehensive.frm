# TS override: Comprehensive system.return semantics

system SystemReturnTest {
    interface:
        getDefault() : int = 42
        getOverride() : int = 10
        getFromAction() : string = "default"
        getNoDefault() : int

    machine:
        $Start {
            getDefault() {
                return;
            }
            getOverride() : int = 99 {
                // Handler default overrides interface default
                return;
            }
            getFromAction() {
                callAction();
                return;
            }
            getNoDefault() {
                return;
            }
        }

    actions:
        callAction() {
            system.return = "from_action";
            return;
        }
}

fn main() {
    var tester = SystemReturnTest()
    var failures = 0

    var result1 = tester.getDefault()
    console.log("Test 1 - Interface default: " + String(result1));
    if (result1 == 42) {
        console.log("  PASS: Got interface default 42");
    } else {
        console.log("  FAIL: Expected 42, got " + String(result1));
        failures = failures + 1
    }

    var result2 = tester.getOverride()
    console.log("Test 2 - Handler override: " + String(result2));
    if (result2 == 99) {
        console.log("  PASS: Got handler override 99");
    } else {
        console.log("  FAIL: Expected 99, got " + String(result2));
        failures = failures + 1
    }

    var result3 = tester.getFromAction()
    console.log("Test 3 - Action sets return: " + String(result3));
    if (result3 == "from_action") {
        console.log("  PASS: Got action value 'from_action'");
    } else {
        console.log("  FAIL: Expected 'from_action', got " + String(result3));
        failures = failures + 1
    }

    var result4 = tester.getNoDefault()
    console.log("Test 4 - No default: " + String(result4));
    if (result4 == null) {
        console.log("  PASS: Got null as expected");
    } else {
        console.log("  FAIL: Expected null, got " + String(result4));
        failures = failures + 1
    }

    console.log("\n=== Test Summary ===");
    if (failures == 0) {
        console.log("All tests PASSED");
    } else {
        console.log("FAILED: " + String(failures) + " test(s) failed");
        // Force failure
        var failed_tests = []
        var index = failed_tests[999]
    }
}

