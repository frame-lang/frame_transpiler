#TerminatorsInBranch
    -interface-
    Foo

    -machine-

    $S1
        |Foo|
            t ?~
                /a/  xoo ? -> $S2 ^ : -> $S3 ^  ::   :>
                /b/ noo ?~ /a/ -> $S2 ^ :> /b/ -> $S3 ^  : -> $S3 ^ ::
                : c() ^
            ::


        |x| ^

    $S2
    $S3

    -actions-
    -domain-
##

#Terminators2

    -machine-

      $TransitionTermination

          |A| -> $S2 :> --- this should be invalid
          |B| -> $S3 ^
          |C| -> $S4 ^(42)
          |D|
            t ? -> $S2 :>
            u ? -> $S3 :>
            v ? -> $S4 ^(0) :>
            w ? a() :>
            x ? b() ^ :>
              : f()
            :: ^(42)
          |E|
            t ? -> $S2
            : ^ ::
          |F|
            t ? -> $S2 ^
            : -> $S3 ::
##