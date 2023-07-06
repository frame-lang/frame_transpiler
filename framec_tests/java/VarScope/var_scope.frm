```
package framec_tests.java.VarScope;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
#VarScope
    -interface-
    to_nn
    to_ny
    to_yn
    to_yy
    nn [d:String]
    ny [d:String]
    yn [d:String x:String]
    yy [d:String x:String]
    sigils [x:String]

    -machine-
    $Init
        |to_nn| -> $NN ("$NN[b]") ^
        |to_ny| -> $NY ("$NY[b]") ^
        |to_yn| -> $YN ("$YN[b]" "$YN[x]") ^
        |to_yy| -> $YY ("$YY[b]" "$YY[x]") ^

    $NN [b:String]
        var c:String = "$NN.c"

        |nn| [d:String]
            var et:String = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:String]
            var et:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:String x:String]
            var et:String = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:String x:String]
            var et:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:String]
            --- var x:String = "|sigils|.x"
            log(#.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $NY [b:String]
        var c:String = "$NY.c"
        var x:String = "$NY.x"

        |nn| [d:String]
            var et:String = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:String]
            var et:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:String x:String]
            var et:String = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:String x:String]
            var et:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:String]
            --- var x:String = "|sigils|.x"
            log(#.x)
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    $YN [b:String x:String]
        var c:String = "$YN.c"

        |nn| [d:String]
            var et:String = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:String]
            var et:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:String x:String]
            var et:String = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:String x:String]
            var et:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:String]
            --- var x:String = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log(||[x])
            --- log(||.x)
            ^

    $YY [b:String x:String]
        var c:String = "$YY.c"
        var x:String = "$YY.x"

        |nn| [d:String]
            var et:String = "|nn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |ny| [d:String]
            var et:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yn| [d:String x:String]
            var et:String = "|yn|.e"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |yy| [d:String x:String]
            var et:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a)
            log(b)
            log(c)
            log(d)
           log(et)
            log(x)
            ^

        |sigils| [x:String]
            --- var x:String = "|sigils|.x"
            log(#.x)
            --- log($[x])
            --- log($.x)
            --- log(||[x])
            --- log(||.x)
            ^

    -actions-
    log [s:String]

    -domain-
    var a:String = "#.a"
    var x:String = "#.x"
    var tape:`ArrayList<String>` = `new ArrayList<String>()`
##
