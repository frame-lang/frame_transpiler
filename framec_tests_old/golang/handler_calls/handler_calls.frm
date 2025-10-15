```
package handler_calls

import "golang/framelang"

```

#HandlerCalls
    -interface-
    NonRec
    SelfRec
    MutRec
    Call [event:string, arg:int]
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
            log("Foo", arg)
            counter = counter + arg
            Bar(arg*2)
            --- the front-end should report the next line as a static error
            log("Unreachable", 0)
            ^

        |Bar| [arg:int]
            log("Bar", arg)
            counter = counter + arg
            -> $Final(counter) ^

        |Call| [event:string, arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : Call("Foo", 1000)
                :: ^

    $SelfRecursive
        var counter:int = 0

        |Foo| [arg:int]
            log("Foo", arg)
            counter = counter + arg
            counter < 100 ?
                Foo(arg*2)
            :
                -> $Final(counter)
            :: ^

        |Bar| [arg:int]
            log("Bar", arg)
            counter = counter + arg
            -> $Final(counter) ^

        |Call| [event:string, arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $MutuallyRecursive
        var counter:int = 0

        |Foo| [arg:int]
            log("Foo", arg)
            counter = counter + arg
            counter > 100 ?
                -> $Final(counter)
            :
                Bar(arg*2)
            :: ^

        |Bar| [arg:int]
            log("Bar", arg)
            counter = counter + arg
            arg ?#
                /4/ Foo(arg) :>
                /8/ Foo(arg*2)
                :   Foo(arg*3)
            :: ^

        |Call| [event:string, arg:int]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $Final [counter:int]
        |>|
            log("Final", counter)
            -> $Init ^

    -actions-
        log [from:string, val:int]

    -domain-
    var tape:`[]string` = `[]string{}`
##
