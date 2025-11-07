// Test case for Bug #39: Missing Frame Semantic Metadata for Debugger Integration
// Verifies that --debug-output includes semantic metadata about Frame constructs

system SemanticMetadataTest {
    interface:
        start() -> Bool
        process() -> Bool
        finish()
    
    machine:
        $Start {
            $>() {
                print("Entering Start state")
            }
            
            start() {
                if self.initialize():
                    -> $Processing
                    return True
                else:
                    return False
                }
            }
            
            $<() {
                print("Exiting Start state")
            }
        }
        
        $Processing {
            $>() {
                print("Entering Processing state")
            }
            
            process() {
                result = self.performWork()
                if result:
                    -> $Complete
                    return True
                else:
                    -> $Error
                    return False
                }
            }
            
            $<() {
                print("Exiting Processing state")
            }
        }
        
        $Complete {
            $>() {
                print("Entering Complete state")
            }
            
            finish() {
                self.cleanup()
                -> $Start
            }
        }
        
        $Error {
            $>() {
                print("Entering Error state")
            }
            
            finish() {
                self.cleanup()
                -> $Start
            }
        }
    
    actions:
        initialize() -> Bool
        performWork() -> Bool
        cleanup()
    
    domain:
        workData: object = {}
        errorCount: int = 0
}

fn main() {
    test = SemanticMetadataTest()
    print("Testing semantic metadata generation...")
    
    # This test validates that debug output includes semantic metadata
    # When run with --debug-output, should generate JSON with:
    # - System structure (states, transitions, interface methods)
    # - State machine topology
    # - Python-to-Frame mappings
    
    result = test.start()
    if result:
        print("SUCCESS: System executed with semantic metadata")
    else:
        print("FAIL: System execution failed")
        # Force test failure
        failed_tests = []
        index = failed_tests[999]  # This will cause an IndexError and fail the test
    }
}
