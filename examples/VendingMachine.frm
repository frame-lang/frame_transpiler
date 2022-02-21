--- answer to question https://www.reddit.com/r/learnmath/comments/ns5s00/finite_state_machinesfsm_vending_machine/

#VendingMachine

    -interface-

    AddC25
    AddC50
    Cancel

    -machine-

    $C0 => $Cancel
        |AddC25| -> "Add\n$0.25" $C25 ^
        |AddC50| -> "Add\n$0.50" $C50 ^

    $C25 => $Cancel
        |AddC25| -> "Add\n$0.25" $C50 ^
        |AddC50| -> "Add\n$0.50" $C75 ^

    $C50 => $Cancel
        |AddC25| -> "Add\n$0.25" $C75 ^
        |AddC50| -> "Add\n$0.50" $C100 ^

    $C75 => $Cancel
        |AddC25| -> "Add\n$0.25" $C100 ^
        |AddC50| -> "Add\n$0.50" $C125 ^

    $C100 => $DispenseC100Drink
        |AddC25| -> "Add\n$0.25" $C125 ^
        |AddC50| -> "Add\n$0.50" $C150 ^

    $C125 => $DispenseC125Drink
        |AddC25| -> "Add\n$0.25" $C150 ^
        |AddC50| -> "Add\n$0.50" $C175 ^

    $C150 => $DispenseC125Drink
        |AddC25| -> "Add\n$0.25" $C175 ^
        |AddC50| -> "Add\n$0.50" $C200 ^

    $C175 => $DispenseC125Drink
        |AddC25| -> "Add\n$0.25" $C200 ^
        |AddC50| -> "Add\n$0.50" $C200 ^

    $C200 => $DispenseC125Drink
        |DispenseC200Drink|
            dispenseC200Drink()
            -> "Dispense\n $2.00 Drink" $C0 ^

    $DispenseC100Drink => $Cancel
        |DispenseC100Drink|
            dispenseC100Drink()
            -> "Dispense\n $1.00 Drink" $C0 ^

    $DispenseC125Drink => $DispenseC100Drink
        |DispenseC125Drink|
            dispenseC200Drink()
            -> "Dispense\n $1.25 Drink" $C0 ^

    $Cancel
        |Cancel| -> "Cancel" $C0 ^

    -actions-

    dispenseC100Drink
    dispenseC125Drink
    dispenseC200Drink

##
