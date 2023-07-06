// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package state_vars

import "golang/framelang"


func NewStateVars() StateVars {
    m := &stateVarsStruct{}
    
    // Validate interfaces
    var _ StateVars = m
    var _ StateVars_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewStateVarsCompartment(StateVarsState_Init)
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type StateVarsState uint

const (
    StateVarsState_Init StateVarsState = iota
    StateVarsState_A
    StateVarsState_B
)

type StateVars interface {
    X() 
    Y() 
    Z() 
}

type StateVars_actions interface {
}


type stateVarsStruct struct {
    _compartment_ *StateVarsCompartment
    _nextCompartment_ *StateVarsCompartment
}

//===================== Interface Block ===================//

func (m *stateVarsStruct) X()  {
    e := framelang.FrameEvent{Msg:"X"}
    m._mux_(&e)
}

func (m *stateVarsStruct) Y()  {
    e := framelang.FrameEvent{Msg:"Y"}
    m._mux_(&e)
}

func (m *stateVarsStruct) Z()  {
    e := framelang.FrameEvent{Msg:"Z"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *stateVarsStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case StateVarsState_Init:
        m._StateVarsState_Init_(e)
    case StateVarsState_A:
        m._StateVarsState_A_(e)
    case StateVarsState_B:
        m._StateVarsState_B_(e)
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

func (m *stateVarsStruct) _StateVarsState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        compartment := NewStateVarsCompartment(StateVarsState_A)
        compartment.StateVars["x"] = 0
        
        m._transition_(compartment)
        
        return
    }
}

func (m *stateVarsStruct) _StateVarsState_A_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "X":
        m._compartment_.StateVars["x"] = m._compartment_.StateVars["x"].(int) + 1
        
        return
    case "Y":
        compartment := NewStateVarsCompartment(StateVarsState_B)
        compartment.StateVars["y"] = 10
        
        compartment.StateVars["z"] = 100
        
        m._transition_(compartment)
        
        return
    case "Z":
        compartment := NewStateVarsCompartment(StateVarsState_B)
        compartment.StateVars["y"] = 10
        
        compartment.StateVars["z"] = 100
        
        m._transition_(compartment)
        
        return
    }
}

func (m *stateVarsStruct) _StateVarsState_B_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "X":
        compartment := NewStateVarsCompartment(StateVarsState_A)
        compartment.StateVars["x"] = 0
        
        m._transition_(compartment)
        
        return
    case "Y":
        m._compartment_.StateVars["y"] = m._compartment_.StateVars["y"].(int) + 1
        
        return
    case "Z":
        m._compartment_.StateVars["z"] = m._compartment_.StateVars["z"].(int) + 1
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *stateVarsStruct) _transition_(compartment *StateVarsCompartment) {
    m._nextCompartment_ = compartment
}

func (m *stateVarsStruct) _do_transition_(nextCompartment *StateVarsCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions


********************************************************/

//=============== Compartment ==============//

type StateVarsCompartment struct {
    State StateVarsState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewStateVarsCompartment(state StateVarsState) *StateVarsCompartment {
    c := &StateVarsCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}