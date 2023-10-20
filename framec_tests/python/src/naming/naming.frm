```
from framelang.framelang import FrameEvent
```
#[codegen.python.code.public_state_info:bool="true"]

#Naming
    -interface-
    snake_event [snake_param:int]
    CamelEvent [CamelParam:int]
    event123 [param123:int]
    call [event:str, param:int]

    -machine-
    $Init
        |snake_event| [snake_param:int]
            -> $snake_state(snake_param) ^

        |CamelEvent| [CamelParam:int]
            -> $CamelState(CamelParam) ^

        |event123| [param123:int]
            -> $state123(param123) ^

        |call| [event:str, param:int]
            event ?~
                /snake_event/ snake_event(param) :>
                /CamelEvent/ CamelEvent(param) :>
                /event123/ event123(param)
                : :: ^

    $snake_state [snake_state_param:int]

        // 1100
        var snake_state_var:int = snake_domain_var + CamelDomainVar + domainVar123 + 100

        |snake_event| [snake_param:int]
            var snake_local_var:int = snake_state_var + snake_state_param + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:int]
            var CamelLocalVar:int = snake_state_var + snake_state_param + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:int]
            var localVar123:int = snake_state_var + snake_state_param + param123
            action123(localVar123)
            -> $Final(localVar123) ^

        |call| [event:str, param:int]
            event ?~
                /snake_event/ snake_event(param) :>
                /CamelEvent/ CamelEvent(param) :>
                /event123/ event123(param)
                : :: ^

    $CamelState [CamelStateParam:int]

        // 1200
        var CamelStateVar:int = snake_domain_var + CamelDomainVar + domainVar123 + 200

        |snake_event| [snake_param:int]
            var snake_local_var:int = CamelStateVar + CamelStateParam + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:int]
            var CamelLocalVar:int = CamelStateVar + CamelStateParam + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:int]
            var localVar123:int = CamelStateVar + CamelStateParam + param123
            action123(localVar123)
            -> $Final(localVar123) ^

        |call| [event:str, param:int]
            event ?~
                /snake_event/ snake_event(param) :>
                /CamelEvent/ CamelEvent(param) :>
                /event123/ event123(param)
                : :: ^

    $state123 [stateParam123:int]

        // 1300
        var stateVar123:int = snake_domain_var + CamelDomainVar + domainVar123 + 300

        |snake_event| [snake_param:int]
            var snake_local_var:int = stateVar123 + stateParam123 + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:int]
            var CamelLocalVar:int = stateVar123 + stateParam123 + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:int]
            var localVar123:int = stateVar123 + stateParam123 + param123
            action123(localVar123)
            -> $Final(localVar123) ^

        |call| [event:str, param:int]
            event ?~
                /snake_event/ snake_event(param) :>
                /CamelEvent/ CamelEvent(param) :>
                /event123/ event123(param)
                : :: ^

    $Final [result:int]
        |>|
            logFinal(result)
            -> $Init ^

    -actions-
    snake_action [snake_param:int]
    CamelAction [CamelParam:int]
    action123 [param123:int]
    logFinal [r:int]

    -domain-
    var snake_domain_var:int = 300
    var CamelDomainVar:int = 550
    var domainVar123:int = 150
    var snake_log = `[]`
    var CamelLog = `[]`
    var log123 = `[]`
    var finalLog = `[]`
##
