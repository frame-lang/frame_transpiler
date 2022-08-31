```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]

#VarScope
    -interface-
    to_nn
    to_ny
    to_yn
    to_yy
    nn [d:str]
    ny [d:str]
    yn [d:str x:str]
    yy [d:str x:str]
    sigils [x:str]

    -machine-
    $Init
        |to_nn| -> $NN ("$NN[b]") ^
        |to_ny| -> $NY ("$NY[b]") ^
        |to_yn| -> $YN ("$YN[b]" "$YN[x]") ^
        |to_yy| -> $YY ("$YY[b]" "$YY[x]") ^

    $NN [b:str]
        var c:str = "$NN.c"

        |nn| [d:str]
            var et:str = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:str]
            var et:str = "|ny|.e"
            var x:str = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:str x:str]
            var et:str = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:str x:str]
            var et:str = "|yy|.e"
            var x:str = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:str]
            --- var x:str = "|sigils|.x"
            log(#.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $NY [b:str]
        var c:str = "$NY.c"
        var x:str = "$NY.x"

        |nn| [d:str]
            var et:str = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:str]
            var et:str = "|ny|.e"
            var x:str = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:str x:str]
            var et:str = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:str x:str]
            var et:str = "|yy|.e"
            var x:str = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:str]
            --- var x:str = "|sigils|.x"
            log(#.x)
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $YN [b:str x:str]
        var c:str = "$YN.c"

        |nn| [d:str]
            var et:str = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:str]
            var et:str = "|ny|.e"
            var x:str = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:str x:str]
            var et:str = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:str x:str]
            var et:str = "|yy|.e"
            var x:str = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:str]
            --- var x:str = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log(||[x])
            --- log(||.x)
            ^

    $YY [b:str x:str]
        var c:str = "$YY.c"
        var x:str = "$YY.x"

        |nn| [d:str]
            var et:str = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:str]
            var et:str = "|ny|.e"
            var x:str = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:str x:str]
            var et:str = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:str x:str]
            var et:str = "|yy|.e"
            var x:str = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:str]
            --- var x:str = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    -actions-
    log [s:str]

    -domain-
    var a:str = "#.a"
    var x:str = "#.x"
    var tape = `[]`
##
