

@@system Foo {

    interface:
        bar(a:int,b:str) : MyType = MyType(0)

    machine:2
        $A {
            bar(a:int,b:str) : MyType = MyType(1) {
                if zoo() return MyType(2)

                -> $B22
            }
        }

        x = @@MySystem()

        @@system.a
        @@system.b 
        @@system:return
        @@system:event
        @@system:params[a]
        @@system:params[b]
        @@system:data[c]


        @@.a            # direct param access
        @@:b            # direct param access
        @@:params[x]    # indirectbut  param access                                                                                                                                                              
        @@:return       # return value default None | Null | void etc based on lang
        @@:event        # string value of event name
        @@:data[key]    # context data. empty by default


}