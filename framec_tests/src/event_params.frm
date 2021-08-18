#EventParams
    -interface-
    Hello
    Goodbye
    Both
    
    -machine-
    $A
        |>| [msg:String val:i16]
            entered(msg val) ^
        |<| [val:bool msg:String]
            exited(val msg) ^
        |Hello|
            -> ("hello B" 42) $B ^
        |Goodbye|
            (true "goodbye A") -> $B ^
        |Both|
            (false "bye A") -> ("hi B" -42) $B ^

    $B
        |>| [msg:String val:i16]
            entered(msg val) ^
        |<| [val:bool msg:String]
            exited(val msg) ^
        |Hello|
            -> ("howdy A" 0) $A ^
        |Goodbye|
            (false "tootles B") -> $B ^
        |Both|
            (true "ciao B") -> ("sup A" 101) $B ^

    -actions-
    entered [msg:String val:i16]
    exited [val:bool msg:String]

    -domain-
    var enter_log:Log = `vec![]`
    var exit_log:Log = `vec![]`
##
