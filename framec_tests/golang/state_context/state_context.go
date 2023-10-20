// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewStateContextSm() StateContextSm {
    m := &stateContextSmStruct{}
    
    // Validate interfaces
    var _ StateContextSm = m
    var _ StateContextSm_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewStateContextSmCompartment(StateContextSmState_Init)
    m._compartment_.StateVars["w"] = m._compartment_.StateVars["w"].(int) + 1
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type StateContextSmState uint

const (
    StateContextSmState_Init StateContextSmState = iota
    StateContextSmState_Foo
    StateContextSmState_Bar
)

type StateContextSm interface {
    Start() 
    LogState() 
    Inc() int
    Next(arg int) 
    Change(arg int) 
}

type StateContextSm_actions interface {
    log(name string,val int) 
}


type stateContextSmStruct struct {
    _compartment_ *StateContextSmCompartment
    _nextCompartment_ *StateContextSmCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *stateContextSmStruct) Start()  {
    e := framelang.FrameEvent{Msg:"Start"}
    m._mux_(&e)
}

func (m *stateContextSmStruct) LogState()  {
    e := framelang.FrameEvent{Msg:"LogState"}
    m._mux_(&e)
}

func (m *stateContextSmStruct) Inc() int {
    e := framelang.FrameEvent{Msg:"Inc"}
    m._mux_(&e)
    return  e.Ret.(int)
}

func (m *stateContextSmStruct) Next(arg int)  {
    params := make(map[string]interface{})
    params["arg"] = arg
    e := framelang.FrameEvent{Msg:"Next", Params:params}
    m._mux_(&e)
}

func (m *stateContextSmStruct) Change(arg int)  {
    params := make(map[string]interface{})
    params["arg"] = arg
    e := framelang.FrameEvent{Msg:"Change", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *stateContextSmStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case StateContextSmState_Init:
        m._StateContextSmState_Init_(e)
    case StateContextSmState_Foo:
        m._StateContextSmState_Foo_(e)
    case StateContextSmState_Bar:
        m._StateContextSmState_Bar_(e)
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

func (m *stateContextSmStruct) _StateContextSmState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m._compartment_.StateVars["w"] = 3
        m.log("w",(m._compartment_.StateVars["w"].(int)))
        
        return
    case "Inc":
        m._compartment_.StateVars["w"] = m._compartment_.StateVars["w"].(int) + 1
        m.log("w",(m._compartment_.StateVars["w"].(int)))
        e.Ret = (m._compartment_.StateVars["w"].(int))
        return
        
    case "LogState":
        m.log("w",(m._compartment_.StateVars["w"].(int)))
        
        return
    case "Start":
        compartment := NewStateContextSmCompartment(StateContextSmState_Foo)
        compartment.EnterArgs["a"] = 3
        compartment.EnterArgs["b"] = m._compartment_.StateVars["w"].(int)
        compartment.StateVars["x"] = m._compartment_.StateVars["x"].(int) + 1
        
        m._transition_(compartment)
        
        return
    }
}

func (m *stateContextSmStruct) _StateContextSmState_Foo_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("a",e.Params["a"].(int))
        m.log("b",e.Params["b"].(int))
        m._compartment_.StateVars["x"] = e.Params["a"].(int) * e.Params["b"].(int)
        m.log("x",(m._compartment_.StateVars["x"].(int)))
        
        return
    case "<":
        m.log("c",e.Params["c"].(int))
        m._compartment_.StateVars["x"] = m._compartment_.StateVars["x"].(int) + e.Params["c"].(int)
        m.log("x",(m._compartment_.StateVars["x"].(int)))
        
        return
    case "LogState":
        m.log("x",(m._compartment_.StateVars["x"].(int)))
        
        return
    case "Inc":
        m._compartment_.StateVars["x"] = m._compartment_.StateVars["x"].(int) + 1
        m.log("x",(m._compartment_.StateVars["x"].(int)))
        e.Ret = (m._compartment_.StateVars["x"].(int))
        return
        
    case "Next":
        var tmp  = e.Params["arg"].(int) * 10
        m._compartment_.ExitArgs["c"] = 10
        compartment := NewStateContextSmCompartment(StateContextSmState_Bar)
        compartment.EnterArgs["a"] = tmp
        compartment.StateArgs["y"] = m._compartment_.StateVars["x"].(int)
        
        compartment.StateVars["z"] = m._compartment_.StateVars["z"].(int) + 1
        
        m._transition_(compartment)
        
        return
      // FIXME: Swapping this to 10 * arg causes a parse error!
    case "Change":
        var tmp  = m._compartment_.StateVars["x"].(int) + e.Params["arg"].(int)
        
        return
    }
}  // ->> $Bar(tmp)


func (m *stateContextSmStruct) _StateContextSmState_Bar_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("a",e.Params["a"].(int))
        m.log("y",(m._compartment_.StateArgs["y"].(int)))
        m._compartment_.StateVars["z"] = e.Params["a"].(int) + m._compartment_.StateArgs["y"].(int)
        m.log("z",(m._compartment_.StateVars["z"].(int)))
        
        return
    case "LogState":
        m.log("y",(m._compartment_.StateArgs["y"].(int)))
        m.log("z",(m._compartment_.StateVars["z"].(int)))
        
        return
    case "Inc":
        m._compartment_.StateVars["z"] = m._compartment_.StateVars["z"].(int) + 1
        m.log("z",(m._compartment_.StateVars["z"].(int)))
        e.Ret = (m._compartment_.StateVars["z"].(int))
        return
        
    case "Change":
        var tmp  = m._compartment_.StateArgs["y"].(int) + m._compartment_.StateVars["z"].(int) + e.Params["arg"].(int)
        m.log("tmp",tmp)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *stateContextSmStruct) _transition_(compartment *StateContextSmCompartment) {
    m._nextCompartment_ = compartment
}

func (m *stateContextSmStruct) _do_transition_(nextCompartment *StateContextSmCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *stateContextSmStruct) log(name string,val int)  {}

********************************************************/

//=============== Compartment ==============//

type StateContextSmCompartment struct {
    State StateContextSmState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewStateContextSmCompartment(state StateContextSmState) *StateContextSmCompartment {
    c := &StateContextSmCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}