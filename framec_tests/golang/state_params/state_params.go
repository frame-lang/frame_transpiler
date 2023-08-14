// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package state_params

import "golang/framelang"


func NewStateParams() StateParams {
    m := &stateParamsStruct{}
    
    // Validate interfaces
    var _ StateParams = m
    var _ StateParams_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewStateParamsCompartment(StateParamsState_Init)
    
    // Override domain variables.
    m.param_log = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type StateParamsState uint

const (
    StateParamsState_Init StateParamsState = iota
    StateParamsState_Split
    StateParamsState_Merge
)

type StateParams interface {
    Next() 
    Prev() 
    Log() 
}

type StateParams_actions interface {
    got_param(name string,val int) 
}


type stateParamsStruct struct {
    _compartment_ *StateParamsCompartment
    _nextCompartment_ *StateParamsCompartment
    param_log []string
}

//===================== Interface Block ===================//

func (m *stateParamsStruct) Next()  {
    e := framelang.FrameEvent{Msg:"Next"}
    m._mux_(&e)
}

func (m *stateParamsStruct) Prev()  {
    e := framelang.FrameEvent{Msg:"Prev"}
    m._mux_(&e)
}

func (m *stateParamsStruct) Log()  {
    e := framelang.FrameEvent{Msg:"Log"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *stateParamsStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case StateParamsState_Init:
        m._StateParamsState_Init_(e)
    case StateParamsState_Split:
        m._StateParamsState_Split_(e)
    case StateParamsState_Merge:
        m._StateParamsState_Merge_(e)
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

func (m *stateParamsStruct) _StateParamsState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Next":
        compartment := NewStateParamsCompartment(StateParamsState_Split)
        compartment.StateArgs["val"] = 1
        
        m._transition_(compartment)
        
        return
    }
}

func (m *stateParamsStruct) _StateParamsState_Split_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Next":
        compartment := NewStateParamsCompartment(StateParamsState_Merge)
        compartment.StateArgs["left"] = m._compartment_.StateArgs["val"].(int)
        
        compartment.StateArgs["right"] = m._compartment_.StateArgs["val"].(int) + 1
        
        m._transition_(compartment)
        
        return
    case "Prev":
        compartment := NewStateParamsCompartment(StateParamsState_Merge)
        compartment.StateArgs["left"] = m._compartment_.StateArgs["val"].(int) + 1
        
        compartment.StateArgs["right"] = m._compartment_.StateArgs["val"].(int)
        
        m._transition_(compartment)
        
        return
    case "Log":
        m.got_param("val",(m._compartment_.StateArgs["val"].(int)))
        
        return
    }
}

func (m *stateParamsStruct) _StateParamsState_Merge_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Next":
        compartment := NewStateParamsCompartment(StateParamsState_Split)
        compartment.StateArgs["val"] = m._compartment_.StateArgs["left"].(int) + m._compartment_.StateArgs["right"].(int)
        
        m._transition_(compartment)
        
        return
    case "Prev":
        compartment := NewStateParamsCompartment(StateParamsState_Split)
        compartment.StateArgs["val"] = m._compartment_.StateArgs["left"].(int) * m._compartment_.StateArgs["right"].(int)
        
        m._transition_(compartment)
        
        return
    case "Log":
        m.got_param("left",(m._compartment_.StateArgs["left"].(int)))
        m.got_param("right",(m._compartment_.StateArgs["right"].(int)))
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *stateParamsStruct) _transition_(compartment *StateParamsCompartment) {
    m._nextCompartment_ = compartment
}

func (m *stateParamsStruct) _do_transition_(nextCompartment *StateParamsCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *stateParamsStruct) got_param(name string,val int)  {}

********************************************************/

//=============== Compartment ==============//

type StateParamsCompartment struct {
    State StateParamsState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewStateParamsCompartment(state StateParamsState) *StateParamsCompartment {
    c := &StateParamsCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}