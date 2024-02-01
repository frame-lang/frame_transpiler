`import sys`
`from enum import Enum`


#MatchTests

    -machine-

    $A
        |>|

            // Enum Tests
            matchFruit(Fruit.PEACH)
            matchFruit(Fruit.pear)
            matchFruit(Fruit.Banana)
            matchFruit(Fruit.Lemon)

            // String Tests
            matchString("!@#$%^&*()")
            matchString("a")
            matchString("")
            matchString(null)
            matchString("b")
            matchString("c")

            // Number Tests
            matchNumber(1001.5)
            matchNumber(0.12)
            matchNumber(0.5)
            matchNumber(0.111)
            matchNumber(1001)


            // output
            print(tape)
        ^

    -actions-


    matchFruit[x:Fruit]:Fruit {
        x ?:(Fruit)
            :/Banana|Watermelon/  log("Matched Banana or Watermelon") :>
            :/PEACH/ log("Matched PEACH") :>
            :/pear/ log("Matched pear") :>
            :/Banana/  log("Matched Banana") :>
            : log("no enum match")
        :|
    }

    matchString [s] {
        // validate comment token // doesn't interfere with match patterns
        s ?~
            ~/%|^|!@#$%^&*()/ log("matched " + s) :> // comment test
            ~/a|b/ log("matched a|b") :> // comment test
            ~// log("matched empty string") :>  // comment test
            !// log("matched null") // comment test
            : log("no string match") // comment test
        :|
        ^
    }

    matchNumber [n] {
        // validate comment token // doesn't interfere with match patterns
        n ?#
            #/1001.5|0.12/ log("Matched 1001.5 or 0.12") :>
            #/0.5/ log("Matched .5") :>
            #/0.111/ log("Matched .111") :>
            : log("no number match")
        :|
        ^
    }

    // syntax tests validate correct syntax and should fully explore
    // the grammar. These are not run to test behavior (but could be promoted
    // to do so).

    syntaxTests {

        // dangling else-continue. Both versions are permitted.
        x ?~  ~/a/ : :|
        x ?~  ~/a/ :> :  :|

        // other valid grammar
        x ?~  ~/a/ :|
        x ?~  ~/a/ foo() :|

        // explict scope in branches
        x ?~  ~/a/ foo() : bar()  :|
        x ?~  ~/a/ foo() :> : bar()  :|
        x ?~  ~/a/ {} :> : {}  :|
        x ?~  ~/a/ {foo()} : {bar()}  :|
        x ?~  ~/a/ {foo()} :> : {bar()}  :|

        // Negative tests - these should be manually used to test
        // error reporting.
        // x ?~  ~/a/ -> $B :| // transitions not permitted


        ^
    }

    log [msg] {
        tape.append(msg)
    }

    -domain-

    enum Fruit {
        PEACH
        pear = 2
        Banana = 3
        Watermelon
        Lemon
    }

    var tape = `[]`
##
