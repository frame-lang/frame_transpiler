import Person.proto

--- <@> is the channel token

var global_channel:<@> = <@>(<config>)

#CommExample

    -machine-

    $S1
        |e1|
            var channel:<@> = <@>(<config>)
            var sendPerson:@Person = @Person(
                first_name:Mark
                last_name:Truluck
            )

            var recvPerson:@Person

            --- <@ and @> are the "I/O" tokens

            channel <@ sendPerson --- send operation
            sendPerson @> global_channel --- send operation
            channel @> recvPerson --- receive operation
            recvPerson <@ global_channel --- receive operation
            ^
##

#EnumExample

    -machine-

    $S1
        |e1|
            var dir:Direction = getDirection()
            dir ?:
                /DOWN/ goDown() ^ :>
                /UP/ goUp() ^ :>
                /LEFT|RIGHT/ goSideways() ::

    var upDir:Direction = Direction.UP
    var downDir:Direction = DOWN

    -actions-

    getDirection:Direction

    -domain-

    :[UP DOWN LEFT RIGHT]:Direction
##
