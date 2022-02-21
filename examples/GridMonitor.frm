
#GridMonitor
    -interface-

    start @(|>>|)
    stop @(|<<|)
    gridReading [gridVoltage:f32]


    -machine-

    $DetermineGridAvailability
        |>>|
            print(&"*** Starting GridMonitor State Machine ***")
        ---    gridAvailability = `GridAvailability::Unknown`
        ^

        |gridReading| [gridVoltage:f32]
            print(&"gridReading: Determine State")
            printFloat(gridVoltage)
            gridVoltage > GRID_VOLT_MIN ?
                -> "gridVoltagePresent" $GridAvailable
            :
                -> "!gridVoltagePresent" $GridUnavailable
            ::
        ^

    $GridUnavailable
        |>|
        ---    gridAvailability = `GridAvailability::Unavailable`
            print(&"ENTER STATE: Grid Unavailable")
        ^

        |gridReading| [gridVoltage:f32]
            print(&"gridReading: Unavailable State")
            printFloat(gridVoltage)
            gridVoltage > GRID_VOLT_MIN ? -> "gridVoltagePresent" $GridAvailable ::
        ^

    $GridAvailable
        |>|
        ---    gridAvailability = `GridAvailability::Available`
            print(&"ENTER STATE: Grid Available")
        ^

        |gridReading| [gridVoltage:f32]
            print(&"gridReading: Available State")
            printFloat(gridVoltage)
            gridVoltage <= GRID_VOLT_MIN ?
                -> "!gridVoltagePresent" $GridUnavailable
            ::
        ^


    -actions-

    --- Application Methods ---
    ---setGridAvailability [gridAvailability:GridAvailability]
    ---getGridAvailability : GridAvailability {
---        `self.gridAvailability`
   --- }

    --- System Methods ---

    print [msg:&str] {`
        println!("{}", msg);
    `}

    printFloat [val:f32] {`
        println!("Value: {}", val);
    `}

    toString [msg:&str] : String {`
        return msg.to_string();
    `}

    -domain-
    const GRID_VOLT_MIN:f32 = 95.0      --- Units: Volts
    ---vvar gridAvailability:GridAvailability = `GridAvailability::Unknown`

    --- var sm_info:`SmInfo<'a>` = `SmInfo::default()`
##
