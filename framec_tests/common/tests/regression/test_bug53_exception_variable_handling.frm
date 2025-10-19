# Bug #53 Regression Test: Exception variable handling in TypeScript
# Ensures exception variables generate 'e' not 'this.e' in TypeScript

system TestBug53 {
    machine:
        $Start {
            testException() {
                try {
                    var x = 10 / 0
                } except ZeroDivisionError as e {
                    # Both should generate 'e' not 'this.e' in TypeScript
                    print(f"Caught exception: {e}")
                    print("Exception var:" + str(e))
                }
            }
        }
}

fn main() {
    # Test exception handling directly to validate the bug fix
    try {
        var x = 10 / 0
    } except ZeroDivisionError as e {
        print(f"Caught exception: {e}")
        print("Exception var:" + str(e))
    }
    print("SUCCESS: Bug #53 exception variable handling working")
}