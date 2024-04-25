
#[codegen.python.code.public_state_info:bool="true"]

#ForwardEvents

    -interface-

    GotoS1
    GotoS2
    ReturnFromS1
    ReturnFromS2

    -machine-

    $S0
        |>| log("Enter $S0") ^
        |<| log("Exit $S0") ^
        |GotoS1| log("Recieved |GotoS1|")
                 -> $S1 ^
        |GotoS2| log("Recieved |GotoS2|")
                 $$[+] -> $S2 ^
        |ReturnFromS1| log("|ReturnFromS1| Forwarded") ^
        |ReturnFromS2| log("|ReturnFromS2| Forwarded") ^

    $S1
        |>| log("Enter $S1") ^
        |<| log("Exit $S1") ^
        |ReturnFromS1| -> => $S0 ^

    $S2
        |>| log("Enter $S2") ^
        |<| log("Exit $S2") ^
        |ReturnFromS2| -> => $$[-] ^

    -actions-

    log [msg:str] {
        ```
        self.tape.append(f'{msg}')
        ```
    }

    -domain-

    var tape = `[]`

##
