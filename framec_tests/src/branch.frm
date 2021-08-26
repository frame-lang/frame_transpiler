#Branch
    -interface-
    A
    B
    C
    D
    E
    OnBool [b:bool]
    OnInt [i:i16]
    
    -machine-
    $I
        |A| -> $SimpleIf ^
        |B| -> $NegatedIf ^
        |C| -> $Precedence ^
        |D| -> $NestedIf ^
        |E| -> $TransitReturns ^
    
    $SimpleIf
        |OnBool| [b:bool]
            b ? log("then 1") : ::
            b ? : log("else 1") :: 
            b ? log("then 2") : log("else 2") :: b ? -> $F1 : -> $F2 :: ^
        
        |OnInt| [i:i16]
            i > 5 ? log("> 5") : log("<= 5") ::
            i < 10 ? log("< 10") : log(">= 10") ::
            i == 7 ?
                log("== 7")
                -> $F1
            :
                log("!= 7")
                -> $F2
            ::
            ^
    
    $NegatedIf
        |OnBool| [b:bool]
            b ?! log("then 1") : ::
            b ?! : log("else 1") :: 
            b ?! log("then 2") : log("else 2") :: 
            b ?! -> $F1 : -> $F2 ::
            ^
        
        |OnInt| [i:i16]
            i >= 5 ?! log("< 5") : log(">= 5") ::
            i <= 10 ?! log("> 10") : log("<= 10") ::
            i != 7 ?!
                log("== 7")
                -> $F1
            :
                log("!= 7")
                -> $F2
            ::
            ^

    $Precedence
        |OnInt| [i:i16]
            -i >= 0 && -i <= 5 ?
                log("then 1")
            :
                log("else 1")
            ::
            !(i >= -5 && i <= 5) && (i >= -10 && i <= 10) ?
                log("then 2")
            :
                log("else 2")
            ::
            i >= 0 && i <= 5 || i >= 10 && i <= 20 ?
                log("then 3")
            :
                log("else 3")
            ::
            (i < 0 || i > 10) && i+5 < 20 ?!
                log("then 4")
            :
                log("else 4")
            ::
            ^

    $NestedIf
        |OnInt| [i:i16]
            i > 0 ?
                log("> 0")
                i < 100 ?
                    log("< 100")
                    -> $F1
                :
                    log(">= 100")
                ::
            :
                log("<= 0")
                i > -10 ?
                    log("> -10")
                :
                    log("<= -10")
                    -> $F2
                ::
            ::
            ^
              
      $TransitReturns
          |OnInt| [i:i16]
              i > 100 ?
                  log("-> $F1")
                  -> $F1
              : ::
              i > 10 ?!
              :
                  log("-> $F2")
                  -> $F2
              ::
              log("-> $F3")
              -> $F3
              ^

    $F1
    $F2
    $F3
    
    -actions-
    log[msg:String]

    -domain-
    var tape:Log = `vec![]`
##
