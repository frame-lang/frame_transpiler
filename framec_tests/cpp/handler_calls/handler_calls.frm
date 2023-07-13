```
#include <unordered_map>
#include <stdexcept>
#include <string>
#include <iostream>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"
```

#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
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
            print(`std::to_string`(counter))
            Bar(arg*2)
            --- the front-end should report the next line as a static error
            --- need to handle the case for unreachable code
            ---log("Unreachable" 0)
            ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(`std::to_string`(counter))
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
            print(`std::to_string`(counter))
            counter < 100 ?
                Foo(arg*2)
            :
                -> $Final(counter)
            :: ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(`std::to_string`(counter))
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
            print(`std::to_string`(counter))
            counter > 100 ?
                -> $Final(counter)
            :
                Bar(arg*2)
            :: ^

        |Bar| [arg:int]
            log("Bar" arg)
            counter = counter + arg
            print(`std::to_string`(counter))
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
        print[s:`const std::string&`] {`std::cout << s << std::endl;`}
        log [from:`const std::string&` val:int] {
        `std::string value = from + "(" + std::to_string(val) + ")";
    this->tape.push_back(value);`
    }

    -domain-
    var tape:`std::vector<std::string>` =``

##
