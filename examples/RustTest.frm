```
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_imports)]

```

--- This spec demonstrates the different scopes for variables. (Rust version)

--- see Rust Playground:
--- https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=87f5ce4097ee75572d85639f8daf1efd

#ScopeIdentifiers

    -interface-

    start @(|>>|)

    -machine-

    $Begin
        |>>| -> (2) $Scopes(4) ^

    $Scopes[d:i32] --- 4

        var c:i32 = 3

        |>| [b:i32] --- 2

            var a:i32 = 1

            output(a b c d e)
            output(||.a ||[b] $.c $[d] #.e) ^

    -actions-

    output[a:i32 b:i32 c:i32 d:i32 e:i32] {`
         println!("{}", &format!("{}, {}, {}, {}, {}",a,b,c,d,e));
    `}

    -domain-

    var e:i32 = 5
##