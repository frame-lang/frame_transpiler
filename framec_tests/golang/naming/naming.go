// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewNaming() Naming {
    m := &namingStruct{}
    
    // Validate interfaces
    var _ Naming = m
    var _ Naming_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewNamingCompartment(NamingState_Init)
    
    // Override domain variables.
    m.snake_domain_var = 300
    m.CamelDomainVar = 550
    m.domainVar123 = 150
    m.snake_log = []int{}
    m.CamelLog = []int{}
    m.log123 = []int{}
    m.finalLog = []int{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type NamingState uint

const (
    NamingState_Init NamingState = iota
    NamingState_snake_state
    NamingState_CamelState
    NamingState_state123
    NamingState_Final
)

type Naming interface {
    Snake_event(snake_param int) 
    CamelEvent(CamelParam int) 
    Event123(param123 int) 
    Call(event string,param int) 
}

type Naming_actions interface {
    snake_action(snake_param int) 
    CamelAction(CamelParam int) 
    action123(param123 int) 
    logFinal(r int) 
}


type namingStruct struct {
    _compartment_ *NamingCompartment
    _nextCompartment_ *NamingCompartment
    snake_domain_var int
    CamelDomainVar int
    domainVar123 int
    snake_log []int
    CamelLog []int
    log123 []int
    finalLog []int
}

//===================== Interface Block ===================//

func (m *namingStruct) Snake_event(snake_param int)  {
    params := make(map[string]interface{})
    params["snake_param"] = snake_param
    e := framelang.FrameEvent{Msg:"snake_event", Params:params}
    m._mux_(&e)
}

func (m *namingStruct) CamelEvent(CamelParam int)  {
    params := make(map[string]interface{})
    params["CamelParam"] = CamelParam
    e := framelang.FrameEvent{Msg:"CamelEvent", Params:params}
    m._mux_(&e)
}

func (m *namingStruct) Event123(param123 int)  {
    params := make(map[string]interface{})
    params["param123"] = param123
    e := framelang.FrameEvent{Msg:"event123", Params:params}
    m._mux_(&e)
}

func (m *namingStruct) Call(event string,param int)  {
    params := make(map[string]interface{})
    params["event"] = event
    params["param"] = param
    e := framelang.FrameEvent{Msg:"call", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *namingStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case NamingState_Init:
        m._NamingState_Init_(e)
    case NamingState_snake_state:
        m._NamingState_snake_state_(e)
    case NamingState_CamelState:
        m._NamingState_CamelState_(e)
    case NamingState_state123:
        m._NamingState_state123_(e)
    case NamingState_Final:
        m._NamingState_Final_(e)
    }
    
    if m._nextCompartment_ != nil {
        nextCompartment := m._nextCompartment_
        m._nextCompartment_ = nil
        if nextCompartment._forwardEvent_ != nil && 
           nextCompartment._forwardEvent_.Msg == ">" {
            m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
            m._compartment_ = nextCompartment
            m._mux_(nextCompartment._forwardEvent_)
        } else {
            m._do_transition_(nextCompartment)
            if nextCompartment._forwardEvent_ != nil {
                m._mux_(nextCompartment._forwardEvent_)
            }
        }
        nextCompartment._forwardEvent_ = nil
    }
}

//===================== Machine Block ===================//

func (m *namingStruct) _NamingState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "snake_event":
        compartment := NewNamingCompartment(NamingState_snake_state)
        compartment.StateArgs["snake_state_param"] = e.Params["snake_param"].(int)
        
        compartment.StateVars["snake_state_var"] = m.snake_domain_var + m.CamelDomainVar + m.domainVar123 + 100
        
        m._transition_(compartment)
        
        return
    case "CamelEvent":
        compartment := NewNamingCompartment(NamingState_CamelState)
        compartment.StateArgs["CamelStateParam"] = e.Params["CamelParam"].(int)
        
        compartment.StateVars["CamelStateVar"] = m.snake_domain_var + m.CamelDomainVar + m.domainVar123 + 200
        
        m._transition_(compartment)
        
        return
    case "event123":
        compartment := NewNamingCompartment(NamingState_state123)
        compartment.StateArgs["stateParam123"] = e.Params["param123"].(int)
        
        compartment.StateVars["stateVar123"] = m.snake_domain_var + m.CamelDomainVar + m.domainVar123 + 300
        
        m._transition_(compartment)
        
        return
    case "call":
        if e.Params["event"].(string) == "snake_event" {
            m.Snake_event(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "CamelEvent" {
            m.CamelEvent(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "event123" {
            m.Event123(e.Params["param"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *namingStruct) _NamingState_snake_state_(e *framelang.FrameEvent) {
    switch e.Msg {
      //  1100
    case "snake_event":
        var snake_local_var int = m._compartment_.StateVars["snake_state_var"].(int) + m._compartment_.StateArgs["snake_state_param"].(int) + e.Params["snake_param"].(int)
        m.snake_action(snake_local_var)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = snake_local_var
        
        m._transition_(compartment)
        
        return
    case "CamelEvent":
        var CamelLocalVar int = m._compartment_.StateVars["snake_state_var"].(int) + m._compartment_.StateArgs["snake_state_param"].(int) + e.Params["CamelParam"].(int)
        m.CamelAction(CamelLocalVar)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = CamelLocalVar
        
        m._transition_(compartment)
        
        return
    case "event123":
        var localVar123 int = m._compartment_.StateVars["snake_state_var"].(int) + m._compartment_.StateArgs["snake_state_param"].(int) + e.Params["param123"].(int)
        m.action123(localVar123)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = localVar123
        
        m._transition_(compartment)
        
        return
    case "call":
        if e.Params["event"].(string) == "snake_event" {
            m.Snake_event(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "CamelEvent" {
            m.CamelEvent(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "event123" {
            m.Event123(e.Params["param"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *namingStruct) _NamingState_CamelState_(e *framelang.FrameEvent) {
    switch e.Msg {
      //  1200
    case "snake_event":
        var snake_local_var int = m._compartment_.StateVars["CamelStateVar"].(int) + m._compartment_.StateArgs["CamelStateParam"].(int) + e.Params["snake_param"].(int)
        m.snake_action(snake_local_var)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = snake_local_var
        
        m._transition_(compartment)
        
        return
    case "CamelEvent":
        var CamelLocalVar int = m._compartment_.StateVars["CamelStateVar"].(int) + m._compartment_.StateArgs["CamelStateParam"].(int) + e.Params["CamelParam"].(int)
        m.CamelAction(CamelLocalVar)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = CamelLocalVar
        
        m._transition_(compartment)
        
        return
    case "event123":
        var localVar123 int = m._compartment_.StateVars["CamelStateVar"].(int) + m._compartment_.StateArgs["CamelStateParam"].(int) + e.Params["param123"].(int)
        m.action123(localVar123)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = localVar123
        
        m._transition_(compartment)
        
        return
    case "call":
        if e.Params["event"].(string) == "snake_event" {
            m.Snake_event(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "CamelEvent" {
            m.CamelEvent(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "event123" {
            m.Event123(e.Params["param"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *namingStruct) _NamingState_state123_(e *framelang.FrameEvent) {
    switch e.Msg {
      //  1300
    case "snake_event":
        var snake_local_var int = m._compartment_.StateVars["stateVar123"].(int) + m._compartment_.StateArgs["stateParam123"].(int) + e.Params["snake_param"].(int)
        m.snake_action(snake_local_var)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = snake_local_var
        
        m._transition_(compartment)
        
        return
    case "CamelEvent":
        var CamelLocalVar int = m._compartment_.StateVars["stateVar123"].(int) + m._compartment_.StateArgs["stateParam123"].(int) + e.Params["CamelParam"].(int)
        m.CamelAction(CamelLocalVar)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = CamelLocalVar
        
        m._transition_(compartment)
        
        return
    case "event123":
        var localVar123 int = m._compartment_.StateVars["stateVar123"].(int) + m._compartment_.StateArgs["stateParam123"].(int) + e.Params["param123"].(int)
        m.action123(localVar123)
        compartment := NewNamingCompartment(NamingState_Final)
        compartment.StateArgs["result"] = localVar123
        
        m._transition_(compartment)
        
        return
    case "call":
        if e.Params["event"].(string) == "snake_event" {
            m.Snake_event(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "CamelEvent" {
            m.CamelEvent(e.Params["param"].(int))
            return
        } else if e.Params["event"].(string) == "event123" {
            m.Event123(e.Params["param"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *namingStruct) _NamingState_Final_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.logFinal((m._compartment_.StateArgs["result"].(int)))
        compartment := NewNamingCompartment(NamingState_Init)
        m._transition_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *namingStruct) _transition_(compartment *NamingCompartment) {
    m._nextCompartment_ = compartment
}

func (m *namingStruct) _do_transition_(nextCompartment *NamingCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *namingStruct) snake_action(snake_param int)  {}
func (m *namingStruct) CamelAction(CamelParam int)  {}
func (m *namingStruct) action123(param123 int)  {}
func (m *namingStruct) logFinal(r int)  {}

********************************************************/

//=============== Compartment ==============//

type NamingCompartment struct {
    State NamingState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewNamingCompartment(state NamingState) *NamingCompartment {
    c := &NamingCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}