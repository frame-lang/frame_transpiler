#[feature:codegen.rust.features.runtime_support="true"]
#StateVars
    -interface-
    X
    Y
    Z

    -machine-
    $Init
        |>| -> $A ^

    $A
        var x:u32 = 0
        |X| x = x + 1 ^
        |Y| -> $B ^
        |Z| -> $B ^

    $B
        var y:u32 = 10
        var z:u32 = 100
        |X| -> $A ^
        |Y| y = y + 1 ^
        |Z| z = z + 1 ^

    -actions-

    -domain-
##
