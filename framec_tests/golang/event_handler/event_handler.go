// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package event_handler
import ( "golang/framelang")

func NewEventHandler() EventHandler {
    m := &eventHandlerStruct{}
    
    // Validate interfaces
    var _ EventHandler = m
    var _ EventHandler_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewEventHandlerCompartment(EventHandlerState_S1)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type EventHandlerState uint

const (
    EventHandlerState_S1 EventHandlerState = iota
    EventHandlerState_S2
)

type EventHandler interface {
    LogIt(x int) 
    LogAdd(a int,b int) 
    LogReturn(a int,b int) int
    PassAdd(a int,b int) 
    PassReturn(a int,b int) int
}

type EventHandler_actions interface {
    log(msg string,val int) 
}


type eventHandlerStruct struct {
    _compartment_ *EventHandlerCompartment
    _nextCompartment_ *EventHandlerCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *eventHandlerStruct) LogIt(x int)  {
    params := make(map[string]interface{})
    params["x"] = x
    e := framelang.FrameEvent{Msg:"LogIt", Params:params}
    m._mux_(&e)
}

func (m *eventHandlerStruct) LogAdd(a int,b int)  {
    params := make(map[string]interface{})
    params["a"] = a
    params["b"] = b
    e := framelang.FrameEvent{Msg:"LogAdd", Params:params}
    m._mux_(&e)
}

func (m *eventHandlerStruct) LogReturn(a int,b int) int {
    params := make(map[string]interface{})
    params["a"] = a
    params["b"] = b
    e := framelang.FrameEvent{Msg:"LogReturn", Params:params}
    m._mux_(&e)
    return  e.Ret.(int)
}

func (m *eventHandlerStruct) PassAdd(a int,b int)  {
    params := make(map[string]interface{})
    params["a"] = a
    params["b"] = b
    e := framelang.FrameEvent{Msg:"PassAdd", Params:params}
    m._mux_(&e)
}

func (m *eventHandlerStruct) PassReturn(a int,b int) int {
    params := make(map[string]interface{})
    params["a"] = a
    params["b"] = b
    e := framelang.FrameEvent{Msg:"PassReturn", Params:params}
    m._mux_(&e)
    return  e.Ret.(int)
}

//====================== Multiplexer ====================//

func (m *eventHandlerStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case EventHandlerState_S1:
        m._EventHandlerState_S1_(e)
    case EventHandlerState_S2:
        m._EventHandlerState_S2_(e)
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

func (m *eventHandlerStruct) _EventHandlerState_S1_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "LogIt":
        m.log("x",e.Params["x"].(int))
        
        return
    case "LogAdd":
        m.log("a",e.Params["a"].(int))
        m.log("b",e.Params["b"].(int))
        m.log("a+b",e.Params["a"].(int) + e.Params["b"].(int))
        
        return
    case "LogReturn":
        m.log("a",e.Params["a"].(int))
        m.log("b",e.Params["b"].(int))
        var r  = e.Params["a"].(int) + e.Params["b"].(int)
        m.log("r",r)
        compartment := NewEventHandlerCompartment(EventHandlerState_S2)
        m._transition_(compartment)
        e.Ret = r
        return
        
    case "PassAdd":
        compartment := NewEventHandlerCompartment(EventHandlerState_S2)
        compartment.StateArgs["p"] = e.Params["a"].(int) + e.Params["b"].(int)
        
        m._transition_(compartment)
        
        return
    case "PassReturn":
        var r  = e.Params["a"].(int) + e.Params["b"].(int)
        m.log("r",r)
        compartment := NewEventHandlerCompartment(EventHandlerState_S2)
        compartment.StateArgs["p"] = r
        
        m._transition_(compartment)
        e.Ret = r
        return
        
    }
}

func (m *eventHandlerStruct) _EventHandlerState_S2_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("p",(m._compartment_.StateArgs["p"].(int)))
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *eventHandlerStruct) _transition_(compartment *EventHandlerCompartment) {
    m._nextCompartment_ = compartment
}

func (m *eventHandlerStruct) _do_transition_(nextCompartment *EventHandlerCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *eventHandlerStruct) log(msg string,val int)  {}

********************************************************/

//=============== Compartment ==============//

type EventHandlerCompartment struct {
    State EventHandlerState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewEventHandlerCompartment(state EventHandlerState) *EventHandlerCompartment {
    c := &EventHandlerCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}