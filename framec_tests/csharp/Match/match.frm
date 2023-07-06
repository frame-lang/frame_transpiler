```
using FrameLang;
#nullable disable
namespace Match
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]

#Match
    -interface-
    Empty
    Simple
    Multi
    Nested
    Child
    OnInt [i:int]
    OnString [s:string]

    -machine-
    $Init
        |Empty|  -> $EmptyMatch ^
        |Simple| -> $SimpleMatch ^
        |Multi|  -> $MultiMatch ^
        |Nested| -> $NestedMatch ^
        |Child|  -> $ChildMatch ^

    $EmptyMatch
        |OnString| [s:string]
            s ?~
                /|foo/  --- TODO: matching only the empty string is broken
                    log("empty")
                :   log("?")
            :: ^

    $SimpleMatch
        |OnInt| [i:int]
            i ?#
                /0/
                    log("0") :>
                /42/
                    log("42") :>
                /42/
                    log("!!!") :>
                /-200/
                    log("-200")
                :   log("?")
            :: ^

        |OnString| [s:string]
            s ?~
                /hello/
                    log("hello") :>
                /hello/
                    log("!!!") :>
                /goodbye/
                    log("goodbye") :>
                /Testing 1, 2, 3.../
                    log("testing") :>
                /$10!/
                    log("money")
                :   log("?")
            :: ^

    $MultiMatch
        |OnInt| [i:int]
            i ?#
                /3|-7/
                    log("3|-7") :>
                /-4|5|6/
                    log("-4|5|6")
                :   log("?")
            :: ^

        |OnString| [s:string]
            s ?~
                /$10|12.5%|@#*!/
                    log("symbols") :>
                / |  |\t|\n/
                    log("whitespace")
                :   log("?")
            :: ^

    $NestedMatch
        |OnInt| [i:int]
            i > 0 ?
                i ?#
                    /1|2|3/
                        log("1-3")
                        i ?#
                            /1/ log("1") :>
                            /2/ log("2")
                            :   log("3")
                        :: :>
                    /4|5/
                        log("4-5")
                        i == 4 ?
                            log("4")
                        :
                            log("5")
                        ::
                    :   log("too big")
                ::
            : log("too small")
            :: ^

        |OnString| [s:string]
            s ?~
                /hello|hola|bonjour/
                    log("greeting")
                    s ?~
                        /hello/ log("English") :>
                        /hola/  log("Spanish")
                        :       log("French")
                        :: :>
                /goodbye|adios|au revoir/
                    log("farewell")
                    s ?~
                        /goodbye/ log("English") :>
                        /adios/   log("Spanish")
                        :         log("French")
                        ::
                : log("?")
            :: ^

    $ChildMatch => $SimpleMatch
        |OnInt| [i:int]
            i ?#
                /0/  -> $Final :>
                /3/  log("3") :>
                /4/  log("4") ^ :>
                /42/ log("42 in child") :>
                /5/  log("5") -> $Final
                :    log("no match in child")
                :: :>

        |OnString| [s:string]
            s ?~
                /hello/
                    log("hello in child") :>
                /goodbye/
                    -> $Final :>
                /Testing 1, 2, 3.../
                    log("testing in child") ^
                :   log("no match in child")
                :: :>

    $Final

    -actions-
    log [msg:string] {`this.tape.Add(msg);`}

    -domain-
    var tape:`List<string>` = `new List<string>()`
##
