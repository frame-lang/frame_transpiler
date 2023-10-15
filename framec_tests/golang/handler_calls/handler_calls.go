// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewHandlerCalls() HandlerCalls {
    m := &handlerCallsStruct{}
    
    // Validate interfaces
    var _ HandlerCalls = m
    var _ HandlerCalls_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewHandlerCallsCompartment(HandlerCallsState_Init)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type HandlerCallsState uint

const (
    HandlerCallsState_Init HandlerCallsState = iota
    HandlerCallsState_NonRecursive
    HandlerCallsState_SelfRecursive
    HandlerCallsState_MutuallyRecursive
    HandlerCallsState_Final
)

type HandlerCalls interface {
    NonRec() 
    SelfRec() 
    MutRec() 
    Call(event string,arg int) 
    Foo(arg int) 
    Bar(arg int) 
}

type HandlerCalls_actions interface {
    log(from string,val int) 
}


type handlerCallsStruct struct {
    _compartment_ *HandlerCallsCompartment
    _nextCompartment_ *HandlerCallsCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *handlerCallsStruct) NonRec()  {
    e := framelang.FrameEvent{Msg:"NonRec"}
    m._mux_(&e)
}

func (m *handlerCallsStruct) SelfRec()  {
    e := framelang.FrameEvent{Msg:"SelfRec"}
    m._mux_(&e)
}

func (m *handlerCallsStruct) MutRec()  {
    e := framelang.FrameEvent{Msg:"MutRec"}
    m._mux_(&e)
}

func (m *handlerCallsStruct) Call(event string,arg int)  {
    params := make(map[string]interface{})
    params["event"] = event
    params["arg"] = arg
    e := framelang.FrameEvent{Msg:"Call", Params:params}
    m._mux_(&e)
}

func (m *handlerCallsStruct) Foo(arg int)  {
    params := make(map[string]interface{})
    params["arg"] = arg
    e := framelang.FrameEvent{Msg:"Foo", Params:params}
    m._mux_(&e)
}

func (m *handlerCallsStruct) Bar(arg int)  {
    params := make(map[string]interface{})
    params["arg"] = arg
    e := framelang.FrameEvent{Msg:"Bar", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *handlerCallsStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case HandlerCallsState_Init:
        m._HandlerCallsState_Init_(e)
    case HandlerCallsState_NonRecursive:
        m._HandlerCallsState_NonRecursive_(e)
    case HandlerCallsState_SelfRecursive:
        m._HandlerCallsState_SelfRecursive_(e)
    case HandlerCallsState_MutuallyRecursive:
        m._HandlerCallsState_MutuallyRecursive_(e)
    case HandlerCallsState_Final:
        m._HandlerCallsState_Final_(e)
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

func (m *handlerCallsStruct) _HandlerCallsState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "NonRec":
        compartment := NewHandlerCallsCompartment(HandlerCallsState_NonRecursive)
        compartment.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        
        m._transition_(compartment)
        
        return
    case "SelfRec":
        compartment := NewHandlerCallsCompartment(HandlerCallsState_SelfRecursive)
        compartment.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        
        m._transition_(compartment)
        
        return
    case "MutRec":
        compartment := NewHandlerCallsCompartment(HandlerCallsState_MutuallyRecursive)
        compartment.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        
        m._transition_(compartment)
        
        return
    }
}

func (m *handlerCallsStruct) _HandlerCallsState_NonRecursive_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Foo":
        m.log("Foo",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        m.Bar(e.Params["arg"].(int) * 2)
        return
        m.log("Unreachable",0)
        
        return
      //  the front-end should report the next line as a static error
    case "Bar":
        m.log("Bar",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        compartment := NewHandlerCallsCompartment(HandlerCallsState_Final)
        compartment.StateArgs["counter"] = m._compartment_.StateVars["counter"].(int)
        
        m._transition_(compartment)
        
        return
    case "Call":
        if e.Params["event"].(string) == "Foo" {
            m.Foo(e.Params["arg"].(int))
            return
        } else if e.Params["event"].(string) == "Bar" {
            m.Bar(e.Params["arg"].(int))
            return
        } else {
            m.Call("Foo",1000)
            return
        }
        
        return
    }
}

func (m *handlerCallsStruct) _HandlerCallsState_SelfRecursive_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Foo":
        m.log("Foo",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        if (m._compartment_.StateVars["counter"].(int)) < 100 {
            m.Foo(e.Params["arg"].(int) * 2)
            return
        } else {
            compartment := NewHandlerCallsCompartment(HandlerCallsState_Final)
            compartment.StateArgs["counter"] = m._compartment_.StateVars["counter"].(int)
            
            m._transition_(compartment)
            return
        }
        
        return
    case "Bar":
        m.log("Bar",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        compartment := NewHandlerCallsCompartment(HandlerCallsState_Final)
        compartment.StateArgs["counter"] = m._compartment_.StateVars["counter"].(int)
        
        m._transition_(compartment)
        
        return
    case "Call":
        if e.Params["event"].(string) == "Foo" {
            m.Foo(e.Params["arg"].(int))
            return
        } else if e.Params["event"].(string) == "Bar" {
            m.Bar(e.Params["arg"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *handlerCallsStruct) _HandlerCallsState_MutuallyRecursive_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Foo":
        m.log("Foo",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        if (m._compartment_.StateVars["counter"].(int)) > 100 {
            compartment := NewHandlerCallsCompartment(HandlerCallsState_Final)
            compartment.StateArgs["counter"] = m._compartment_.StateVars["counter"].(int)
            
            m._transition_(compartment)
            return
        } else {
            m.Bar(e.Params["arg"].(int) * 2)
            return
        }
        
        return
    case "Bar":
        m.log("Bar",e.Params["arg"].(int))
        m._compartment_.StateVars["counter"] = m._compartment_.StateVars["counter"].(int) + e.Params["arg"].(int)
        if e.Params["arg"].(int) == 4 {
            m.Foo(e.Params["arg"].(int))
            return
        } else if e.Params["arg"].(int) == 8 {
            m.Foo(e.Params["arg"].(int) * 2)
            return
        } else {
            m.Foo(e.Params["arg"].(int) * 3)
            return
        }
        
        return
    case "Call":
        if e.Params["event"].(string) == "Foo" {
            m.Foo(e.Params["arg"].(int))
            return
        } else if e.Params["event"].(string) == "Bar" {
            m.Bar(e.Params["arg"].(int))
            return
        } else {
        }
        
        return
    }
}

func (m *handlerCallsStruct) _HandlerCallsState_Final_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("Final",(m._compartment_.StateArgs["counter"].(int)))
        compartment := NewHandlerCallsCompartment(HandlerCallsState_Init)
        m._transition_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *handlerCallsStruct) _transition_(compartment *HandlerCallsCompartment) {
    m._nextCompartment_ = compartment
}

func (m *handlerCallsStruct) _do_transition_(nextCompartment *HandlerCallsCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *handlerCallsStruct) log(from string,val int)  {}

********************************************************/

//=============== Compartment ==============//

type HandlerCallsCompartment struct {
    State HandlerCallsState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewHandlerCallsCompartment(state HandlerCallsState) *HandlerCallsCompartment {
    c := &HandlerCallsCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}