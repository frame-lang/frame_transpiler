#[codegen.javascript.code.public_domain:bool="true"]
#[codegen.javascript.code.public_state_info:bool="true"]
#[codegen.javascript.code.generate_import_export:bool="true"]
#HandlerCalls
    -interface-
    NonRec
    SelfRec
    MutRec
    Call [event:String, arg:i32]
    Foo [arg:i32]
    Bar [arg:i32]

    -machine-
    $Init
        |NonRec|  -> $NonRecursive ^
        |SelfRec| -> $SelfRecursive ^
        |MutRec|  -> $MutuallyRecursive ^

    $NonRecursive
        var counter:i32 = 0

        |Foo| [arg:i32]
            log("Foo", arg)
            counter = counter + arg
            Bar(arg*2)
            --- the front-end should report the next line as a static error
            log("Unreachable", 0)
            ^

        |Bar| [arg:i32]
            log("Bar", arg)
            counter = counter + arg
            -> $Final(counter) ^

        |Call| [event:String, arg:i32]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : Call("Foo", 1000)
                :: ^

    $SelfRecursive
        var counter:i32 = 0

        |Foo| [arg:i32]
            log("Foo", arg)
            counter = counter + arg
            counter < 100 ?
                Foo(arg*2)
            :
                -> $Final(counter)
            :: ^

        |Bar| [arg:i32]
            log("Bar", arg)
            counter = counter + arg
            -> $Final(counter) ^

        |Call| [event:String, arg:i32]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $MutuallyRecursive
        var counter:i32 = 0

        |Foo| [arg:i32]
            log("Foo", arg)
            counter = counter + arg
            counter > 100 ?
                -> $Final(counter)
            :
                Bar(arg*2)
            :: ^

        |Bar| [arg:i32]
            log("Bar", arg)
            counter = counter + arg
            arg ?#
                /4/ Foo(arg) :>
                /8/ Foo(arg*2)
                :   Foo(arg*3)
            :: ^

        |Call| [event:String, arg:i32]
            event ?~
                /Foo/ Foo(arg) :>
                /Bar/ Bar(arg)
                : :: ^

    $Final [counter:i32]
        |>|
            log("Final", counter)
            -> $Init ^

    -actions-
        log [from:String, val:i32]

    -domain-
    var tape:Log = `[]`
##
