#[follow_rust_naming="true"]
#RustNaming
    -interface-
    snake_event [snake_param:i32]
    CamelEvent [CamelParam:i32]
    event123 [param123:i32]

    -machine-
    $Init
        |snake_event| [snake_param:i32]
            -> $snake_state(snake_param) ^

        |CamelEvent| [CamelParam:i32]
            -> $CamelState(CamelParam) ^

        |event123| [param123:i32]
            -> $state123(param123) ^

    $snake_state [snake_state_param:i32]

        --- 1100
        var snake_state_var:i32 = snake_domain_var + CamelDomainVar + domainVar123 + 100

        |snake_event| [snake_param:i32]
            var snake_local_var:i32 = snake_state_var + snake_state_param + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:i32]
            var CamelLocalVar:i32 = snake_state_var + snake_state_param + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:i32]
            var localVar123:i32 = snake_state_var + snake_state_param + param123
            action123(localVar123)
            -> $Final(localVar123) ^

    $CamelState [CamelStateParam:i32]

        --- 1200
        var CamelStateVar:i32 = snake_domain_var + CamelDomainVar + domainVar123 + 200

        |snake_event| [snake_param:i32]
            var snake_local_var:i32 = CamelStateVar + CamelStateParam + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:i32]
            var CamelLocalVar:i32 = CamelStateVar + CamelStateParam + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:i32]
            var localVar123:i32 = CamelStateVar + CamelStateParam + param123
            action123(localVar123)
            -> $Final(localVar123) ^

    $state123 [stateParam123:i32]

        --- 1300
        var stateVar123:i32 = snake_domain_var + CamelDomainVar + domainVar123 + 300

        |snake_event| [snake_param:i32]
            var snake_local_var:i32 = stateVar123 + stateParam123 + snake_param
            snake_action(snake_local_var)
            -> $Final(snake_local_var) ^

        |CamelEvent| [CamelParam:i32]
            var CamelLocalVar:i32 = stateVar123 + stateParam123 + CamelParam
            CamelAction(CamelLocalVar)
            -> $Final(CamelLocalVar) ^

        |event123| [param123:i32]
            var localVar123:i32 = stateVar123 + stateParam123 + param123
            action123(localVar123)
            -> $Final(localVar123) ^

    $Final [result:i32]
        |>|
            logFinal(result)
            -> $Init ^

    -actions-
    snake_action [snake_param:i32]
    CamelAction [CamelParam:i32]
    action123 [param123:i32]
    logFinal [r:i32]

    -domain-
    var snake_domain_var:i32 = 300
    var CamelDomainVar:i32 = 550
    var domainVar123:i32 = 150
    var snake_log:Log = `vec![]`
    var CamelLog:Log = `vec![]`
    var log123:Log = `vec![]`
    var finalLog:Log = `vec![]`
##
