1. dfa?
2. pda?
3. peek$

```frame
 

@@system NestedStateSyntax {

    interface:
        * { 
            if not validateHeaders() {
                fail()
                return
            }
            
            => $
        }
        foo() : int = 0 {
            if validateUser() => $ else { fail() }
        }
        
    machine:

        $Grandparent(name,rank) {
        
            $>>() {} # 2) state compartment constructor
            <<$() {} # state compartment destructor      
              
            $>() {}
            <$() {}
                
            $Parent(x,y) { 
                $>>() {} # 3) state compartment constructor
                <<$() {} # state compartment destructor            
                $>() {}
                <$() {}
                        
                # 1) State param initilization chain. done first       
                $Child1(a,b) : (0,1) : (self.name,"E5") {
                    $>>() {} # 4) state compartment constructor
                    <<$() {} # 7) state compartment destructor
                    
                    $>() { # 5) enter
                        => $^ 
                        self.init()
                    }
                    ^$>() {}
                    
                    <$() {  # 6) exit
                        self.deleteAll()
                        => $^
                    }
                    
                    ^foo() : int = 42 {
                        print("$Child1.foo()")
                    }
                }
                
                $Child2(a,b) {}
                
                
                foo() {
                    print("$Parent.foo()")
                }
            }
            
        $Child1(a,b) => $Parent(0,1) {
        }

        $Child2(a,b) => $Parent(0,1) {
        }
                
        $Parent(x,y) => $Grandparent2(self.name,"E5") {
        
        }
}

 ```

