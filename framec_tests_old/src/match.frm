#Match
    -interface-
    Empty
    Simple
    Multi
    Nested
    Child
    OnInt [i:i32]
    OnString [s:String]

    -machine-
    $Init
        |Empty|  -> $EmptyMatch ^
        |Simple| -> $SimpleMatch ^
        |Multi|  -> $MultiMatch ^
        |Nested| -> $NestedMatch ^
        |Child|  -> $ChildMatch ^

    $EmptyMatch
        |OnString| [s:String]
            s ?~
                /|foo/  // TODO: matching only the empty string is broken
                    log("empty")
                :   log("?")
            :: ^

    $SimpleMatch
        |OnInt| [i:i32]
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

        |OnString| [s:String]
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
        |OnInt| [i:i32]
            i ?#
                /3|-7/
                    log("3|-7") :>
                /-4|5|6/
                    log("-4|5|6")
                :   log("?")
            :: ^

        |OnString| [s:String]
            s ?~
                /$10|12.5%|@#*!/
                    log("symbols") :>
                / |  |\t|\n/
                    log("whitespace")
                :   log("?")
            :: ^

    $NestedMatch
        |OnInt| [i:i32]
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

        |OnString| [s:String]
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
        |OnInt| [i:i32]
            i ?#
                /0/  -> $Final :>
                /3/  log("3") :>
                /4/  log("4") ^ :>
                /42/ log("42 in child") :>
                /5/  log("5") -> $Final
                :    log("no match in child")
                :: :>

        |OnString| [s:String]
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
    log [msg:String]

    -domain-
    var tape:Log = `vec![]`
##
