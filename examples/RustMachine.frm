```
#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;
```
#Machine

    -interface-

    start @(|>>|)
    stop @(|<<|)
    toggle[msg:String] : String

    -machine-

    $Begin
        |>>|
            print("Begin::>>")
            -> $S1 ^
        |<|
            print("exit $Begin") ^

    $S1 => $Default
        |>|
            print("enter $S1") ^
        |<|
            print("exit $S1") ^
        |toggle|[msg:String] : String
            var s:String = msg.clone()
            -> $S2 ^(`format!("{}) $S1",s)`)

    $S2 => $Default
        |>|
            print("enter $S2") ^
        |<|
            print("exit $S2") ^
        |toggle|[msg:String] : String
            var s:String = msg.clone()
            -> $S1 ^(`format!("{}) $S2",s)`)

    $End
        |>|
            print("enter $End") ^

    $Default
        |<<|
            -> $End ^

    -actions-

    print [msg:&str] {`
        println!("{}", msg);
    `}

    toString [msg:&str] : String {`
        return msg.to_string();
    `}

    -domain-

    var s:String = `String::from("hi")`
 ---   var x:`&str` = `String::from("")`
##