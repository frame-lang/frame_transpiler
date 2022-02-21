#DemoMachine

    -interface-

    start @(|>>|)
    stop @(|<<|)
    machineReading [machineValue:f32]

    -machine-

    $InitialDecision
        |>>|
            print("*** Starting State Machine ***")
            setAvailability(false)
        ^

        |machineReading| [machineValue:f32]
            machineValue > AVAIL_MIN ?
                -> "We can!" $Available
            :
                -> "Nope" $Unavailable
            ::
        ^

    $Unavailable
        |>|
            print("ENTER STATE: Unavailable")
            setAvailability(true)
        ^

        |machineReading|[machineValue:f32]
            machineValue > AVAIL_MIN ?
                -> "We Finally Can!" $Available
            ::
        ^

    $Available
        |>|
            print("ENTER STATE: Available")
        ^

        |machineReading|[machineValue:f32]
            machineValue > AVAIL_MIN ?!
                -> "Awwww..." $Unavailable
            ::
        ^

    -actions-

    print [msg:&str] {
        `println!("{}", msg)`
    }

    setAvailability [available:bool] {
        `self.available = available;`
    }

    gt [f1:f32 f2:f32] : bool {
        `f1 > f2`
    }

    -domain-
    var available:bool = false
    var AVAIL_MIN:f32 = 0.0

##
