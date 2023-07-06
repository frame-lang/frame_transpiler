```
package framec_tests.java.Handler_calls;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
#HandlerCalls
    -interface-
    NonRec
    SelfRec
    MutRec
    Call [event:String arg:int]
    Foo [arg:int]
    Bar [arg:int]

    -machine-
    $Init
        |NonRec|  -> $NonRecursive ^
        |SelfRec| -> $SelfRecursive ^
        |MutRec|  -> $MutuallyRecursive ^

    $NonRecursive
        var counter:int = 0

        |Foo| [arg:int]
            log("Foo" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            Bar(arg*2)
            --- the front-end should report the next line as a static error
            --- need to handle the case for unreachable code
            ---log("Unreachable" 0)
            ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            -> $Final(counter) ^

        |Call| [event:String arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : Call("Foo" 1000)
                :: ^

    $SelfRecursive
        var counter:int = 0

        |Foo| [arg:int]
            log("Foo" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            counter < 100 ?
                Foo(arg*2)
            :
                -> $Final(counter)
            :: ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            -> $Final(counter) ^

        |Call| [event:String arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $MutuallyRecursive
        var counter:int = 0

        |Foo| [arg:int]
            log("Foo" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            counter > 100 ?
                -> $Final(counter)
            :
                Bar(arg*2)
            :: ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(String.valueOf(counter))
            arg ?#
                /4/ Foo(arg) :>
                /8/ Foo(arg*2)
                :   Foo(arg*3)
            :: ^

        |Call| [event:String arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $Final [counter:int]
        |>|
            log("Final" counter)
            -> $Init ^

    -actions-
        print[s:String] {`System.out.println(s);`}
        log [from:String val:int]

    -domain-
    var tape:`ArrayList<String>` = `new ArrayList<String>()`

##
