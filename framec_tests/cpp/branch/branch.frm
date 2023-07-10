
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
#Branch
    -interface-
    A
    B
    C
    D
    E
    F
    OnBool [b:Boolean]
    OnInt [i:int]

    -machine-
    $I
        |A| -> $SimpleIf ^
        |B| -> $NegatedIf ^
        |C| -> $Precedence ^
        |D| -> $NestedIf ^
        |E| -> $GuardedTransition ^
        |F| -> $NestedGuardedTransition ^

    $SimpleIf
        |OnBool| [b:Boolean]
            b ? log("then 1") : ::
            b ? : log("else 1") ::
            b ? log("then 2") : log("else 2") :: b ? -> $F1 : -> $F2 :: ^

        |OnInt| [i:int]
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
        |OnBool| [b:Boolean]
            b ?! log("then 1") : ::
            b ?! : log("else 1") ::
            b ?! log("then 2") : log("else 2") ::
            b ?! -> $F1 : -> $F2 ::
            ^

        |OnInt| [i:int]
            i < 5 ? log("< 5") : log(">= 5") ::
            i > 10 ? log("> 10") : log("<= 10") ::
            i == 7 ?
                log("== 7")
                -> $F1
            :
                log("!= 7")
                -> $F2
            ::
            ^

    $Precedence
        |OnInt| [i:int]
            -i >= 0 && -i <= 5 ?
                log("then 1")
            :
                log("else 1")
            ::
            (i > -5 && i > 5) && (i > -10 && i < 10) ?
                log("then 2")
            :
                log("else 2")
            ::
            i >= 0 && i <= 5 || i >= 10 && i <= 20 ?
                log("then 3")
            :
                log("else 3")
            ::
            (i >= 0 && i < 10) && i+5 < 20 ?
                log("then 4")
            :
                log("else 4")
            ::
            ^

    $NestedIf
        |OnInt| [i:int]
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

      $GuardedTransition
          |OnInt| [i:int]
              i > 100 ?
                  log("-> $F1")
                  -> $F1
                  ^
              : ::
              i < 10 ?
              :
                  log("-> $F2")
                  -> $F2
                  ^
              ::
              log("-> $F3")
              -> $F3
              ^

      $NestedGuardedTransition
          |OnInt| [i:int]
              i > 10 ?
                  i > 100 ?
                      log("-> $F1")
                      -> $F1
                      ^
                  : ::
                  i > 50 ?
                  :
                      log("-> $F2")
                      -> $F2
                      ^
                  ::
              : ::
              log("-> $F3")
              -> $F3
              ^

    $F1
    $F2
    $F3

    -actions-
    log[msg:String]

    -domain-
    var tape:`ArrayList<String>` = `new ArrayList<String>();`
##
