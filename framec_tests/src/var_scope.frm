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
            var e:String = "|nn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |ny| [d:String]
            var e:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yn| [d:String x:String]
            var e:String = "|yn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yy| [d:String x:String]
            var e:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |sigils| [x:String]
            var x:String = "|sigils|.x"
            log(#.x.clone())
            --- log(||[x].clone())
            --- log(||.x.clone())
            ^

    $NY [b:String]
        var c:String = "$NY.c"
        var x:String = "$NY.x"

        |nn| [d:String]
            var e:String = "|nn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |ny| [d:String]
            var e:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yn| [d:String x:String]
            var e:String = "|yn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yy| [d:String x:String]
            var e:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |sigils| [x:String]
            var x:String = "|sigils|.x"
            log(#.x.clone())
            --- log($.x.clone())
            --- log(||[x].clone())
            --- log(||.x.clone())
            ^

    $YN [b:String x:String]
        var c:String = "$YN.c"

        |nn| [d:String]
            var e:String = "|nn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |ny| [d:String]
            var e:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yn| [d:String x:String]
            var e:String = "|yn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yy| [d:String x:String]
            var e:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |sigils| [x:String]
            var x:String = "|sigils|.x"
            log(#.x.clone())
            --- log($[x].clone())
            --- log(||[x].clone())
            --- log(||.x.clone())
            ^

    $YY [b:String x:String]
        var c:String = "$YY.c"
        var x:String = "$YY.x"

        |nn| [d:String]
            var e:String = "|nn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |ny| [d:String]
            var e:String = "|ny|.e"
            var x:String = "|ny|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yn| [d:String x:String]
            var e:String = "|yn|.e"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |yy| [d:String x:String]
            var e:String = "|yy|.e"
            var x:String = "|yy|.x"
            log(a.clone())
            log(b.clone())
            log(c.clone())
            log(d.clone())
            log(e.clone())
            log(x.clone())
            ^

        |sigils| [x:String]
            var x:String = "|sigils|.x"
            log(#.x.clone())
            --- log($[x].clone())
            --- log($.x.clone())
            --- log(||[x].clone())
            --- log(||.x.clone())
            ^

    -actions-
    log [s:String]

    -domain-
    var a:String = "#.a"
    var x:String = "#.x"
    var tape:Log = `vec![]`
##
