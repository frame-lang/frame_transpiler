@@target python_3

# TODO: This test demonstrates a current v4 limitation:
# V4 currently only compiles the FIRST system in a multi-system file.
# Full multi-system support with interleaved native code is planned
# but not yet implemented.

# Native function before system
def native_before():
    print("Native function before system")

@@system FirstSystem {
    interface:
        doFirst()
    
    machine:
        $Ready {
            doFirst() {
                print("First system ready")
            }
        }
}

# Native function between systems
def native_between():
    print("Native function between systems")

@@system SecondSystem {
    interface:
        doSecond()
    
    machine:
        $Idle {
            doSecond() {
                print("Second system idle")
            }
        }
}

# Native function after systems
def native_after():
    print("Native function after systems")
    # Test both systems work
    first = FirstSystem()
    second = SecondSystem()
    first.doFirst()
    second.doSecond()
    print("SUCCESS: Mixed native/Frame code working")

native_after()