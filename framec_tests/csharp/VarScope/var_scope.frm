```
using FrameLang;
#nullable disable
namespace VarScope
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]
#VarScope
    -interface-
    to_nn
    to_ny
    to_yn
    to_yy
    nn [d:string]
    ny [d:string]
    yn [d:string x:string]
    yy [d:string x:string]
    sigils [x:string]

    -machine-
    $Init
        |to_nn| -> $NN ("$NN[b]") ^
        |to_ny| -> $NY ("$NY[b]") ^
        |to_yn| -> $YN ("$YN[b]" "$YN[x]") ^
        |to_yy| -> $YY ("$YY[b]" "$YY[x]") ^

    $NN [b:string]
        var c:string = "$NN.c"

        |nn| [d:string]
            var et:string = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:string]
            var et:string = "|ny|.e"
            var x:string = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:string x:string]
            var et:string = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:string x:string]
            var et:string = "|yy|.e"
            var x:string = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:string]
            --- var x:string = "|sigils|.x"
            log(#.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $NY [b:string]
        var c:string = "$NY.c"
        var x:string = "$NY.x"

        |nn| [d:string]
            var et:string = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:string]
            var et:string = "|ny|.e"
            var x:string = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:string x:string]
            var et:string = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:string x:string]
            var et:string = "|yy|.e"
            var x:string = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:string]
            --- var x:string = "|sigils|.x"
            log(#.x)
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $YN [b:string x:string]
        var c:string = "$YN.c"

        |nn| [d:string]
            var et:string = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:string]
            var et:string = "|ny|.e"
            var x:string = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:string x:string]
            var et:string = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:string x:string]
            var et:string = "|yy|.e"
            var x:string = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:string]
            --- var x:string = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log(||[x])
            --- log(||.x)
            ^

    $YY [b:string x:string]
        var c:string = "$YY.c"
        var x:string = "$YY.x"

        |nn| [d:string]
            var et:string = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:string]
            var et:string = "|ny|.e"
            var x:string = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:string x:string]
            var et:string = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:string x:string]
            var et:string = "|yy|.e"
            var x:string = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:string]
            --- var x:string = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    -actions-
    log [s:string] {`this.tape.Add(s);`}

    -domain-
    var a:string = "#.a"
    var x:string = "#.x"
    var tape:`List<string>` = `new List<string>()`
##
