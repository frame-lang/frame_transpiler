--- try on https://frame-lang.org

#World

    -interface-

    start @(|>>|)
    stop @(|<<|)

    -machine-

    $Begin
        |>>|
            -> "start" $Working ^

    $Working => $Default
        |>|
            print("Hello World") ^

    $End
        |>|
            print("End of the World") ^

    $Default
        |<<|
            -> "stop" $End ^

    -actions-

    print[msg:string]:void

##
