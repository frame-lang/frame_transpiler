```
package basic
import ( "golang/framelang")
```
#Basic
    -interface-
    A
    B
    -machine-

    $S0
        |>| entered("S0") ^
        |<| left("S0") ^
        |A| -> "ooh" $S1 ^

    $S1
        |>| entered("S1") ^
        |<| left("S1") ^
        |B| -> "aah" $S0 ^

    -actions-
    entered[msg:string]
    left[msg:string] 

    -domain-
    var entry_log:`[]string` = `[]string{}`
    var exit_log:`[]string` = `[]string{}`
##
