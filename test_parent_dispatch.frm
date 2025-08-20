// Test for => $^ parent dispatch syntax
// This test validates:
// 1. => $^ as a statement (not terminator) - can appear anywhere in event handler
// 2. Parent dispatch functionality
// 3. Transition detection after dispatch
// 4. Statements can follow => $^ dispatch

fn main() {
    var hsm = ParentDispatchTest()
    
    // Test basic parent dispatch
    hsm.test1()
    
    // Test parent dispatch with statements after
    hsm.test2() 
    
    // Test parent dispatch with transition in parent
    hsm.test3()
    
    // Test parent dispatch in enter/exit handlers
    hsm.next()
}

system ParentDispatchTest {
    
    interface:
        test1()
        test2() 
        test3()
        next()
    
    machine:
        
        // Parent state with shared behavior
        $Parent {
            test1() {
                print("test1 handled in parent")
                return
            }
            
            test2() {
                print("test2 handled in parent")
                return
            }
            
            test3() {
                print("test3 parent triggers transition")
                -> $Child2
                return
            }
        }
        
        // Child state that demonstrates => $^ dispatch
        $Child1 => $Parent {
            
            // Test 1: Simple parent dispatch (like old @:> behavior)
            test1() {
                => $^
            }
            
            // Test 2: Parent dispatch with statements after
            test2() {
                print("test2 in child before dispatch")
                => $^
                print("test2 in child after dispatch - should execute")
            }
            
            // Test 3: Parent dispatch where parent transitions
            test3() {
                print("test3 in child before dispatch")
                => $^
                print("test3 in child after dispatch - should NOT execute due to transition")
            }
            
            // Test enter/exit handlers with parent dispatch
            $>() {
                print("enter child1")
                => $^
                print("enter child1 - after parent dispatch")
            }
            
            <$() {
                print("exit child1")
                => $^
                print("exit child1 - after parent dispatch")
            }
            
            next() {
                -> $Child2
                return
            }
        }
        
        $Child2 => $Parent {
            $>() {
                print("enter child2")
                => $^
            }
            
            <$() {
                print("exit child2")  
                => $^
            }
            
            next() {
                -> $Child1
                return
            }
        }
}