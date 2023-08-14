// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package transition

import (
    "golang/framelang"
    )


func NewTransitionSm() TransitionSm {
    m := &transitionSmStruct{}
    
    // Validate interfaces
    var _ TransitionSm = m
    var _ TransitionSm_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewTransitionSmCompartment(TransitionSmState_S0)
    
    // Override domain variables.
    m.enters = []string{}
    m.exits = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type TransitionSmState uint

const (
    TransitionSmState_S0 TransitionSmState = iota
    TransitionSmState_S1
    TransitionSmState_S2
    TransitionSmState_S3
    TransitionSmState_S4
)

type TransitionSm interface {
    Transit() 
    Change() 
}

type TransitionSm_actions interface {
    enter(state string) 
    exit(state string) 
}


type transitionSmStruct struct {
    _compartment_ *TransitionSmCompartment
    _nextCompartment_ *TransitionSmCompartment
    enters []string
    exits []string
}

//===================== Interface Block ===================//

func (m *transitionSmStruct) Transit()  {
    e := framelang.FrameEvent{Msg:"transit"}
    m._mux_(&e)
}

func (m *transitionSmStruct) Change()  {
    e := framelang.FrameEvent{Msg:"change"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *transitionSmStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case TransitionSmState_S0:
        m._TransitionSmState_S0_(e)
    case TransitionSmState_S1:
        m._TransitionSmState_S1_(e)
    case TransitionSmState_S2:
        m._TransitionSmState_S2_(e)
    case TransitionSmState_S3:
        m._TransitionSmState_S3_(e)
    case TransitionSmState_S4:
        m._TransitionSmState_S4_(e)
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

func (m *transitionSmStruct) _TransitionSmState_S0_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S0")
        
        return
    case "<":
        m.exit("S0")
        
        return
    case "transit":
        compartment := NewTransitionSmCompartment(TransitionSmState_S1)
        m._transition_(compartment)
        
        return
    case "change":
        compartment := NewTransitionSmCompartment(TransitionSmState_S1)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitionSmStruct) _TransitionSmState_S1_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S1")
        
        return
    case "<":
        m.exit("S1")
        
        return
    case "transit":
        compartment := NewTransitionSmCompartment(TransitionSmState_S2)
        m._transition_(compartment)
        
        return
    case "change":
        compartment := NewTransitionSmCompartment(TransitionSmState_S2)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitionSmStruct) _TransitionSmState_S2_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S2")
        compartment := NewTransitionSmCompartment(TransitionSmState_S3)
        m._transition_(compartment)
        
        return
    case "<":
        m.exit("S2")
        
        return
    case "transit":
        compartment := NewTransitionSmCompartment(TransitionSmState_S3)
        m._transition_(compartment)
        
        return
    case "change":
        compartment := NewTransitionSmCompartment(TransitionSmState_S3)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitionSmStruct) _TransitionSmState_S3_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S3")
        
        return
    case "<":
        m.exit("S3")
        
        return
    case "transit":
        compartment := NewTransitionSmCompartment(TransitionSmState_S4)
        m._transition_(compartment)
        
        return
    case "change":
        compartment := NewTransitionSmCompartment(TransitionSmState_S4)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitionSmStruct) _TransitionSmState_S4_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S4")
        compartment := NewTransitionSmCompartment(TransitionSmState_S0)
        
        m._changeState_(compartment)
        
        return
    case "<":
        m.exit("S4")
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *transitionSmStruct) _transition_(compartment *TransitionSmCompartment) {
    m._nextCompartment_ = compartment
}

func (m *transitionSmStruct) _do_transition_(nextCompartment *TransitionSmCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

func (m *transitionSmStruct) _changeState_(compartment *TransitionSmCompartment) {
    m._compartment_ = compartment
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *transitionSmStruct) enter(state string)  {}
func (m *transitionSmStruct) exit(state string)  {}

********************************************************/

//=============== Compartment ==============//

type TransitionSmCompartment struct {
    State TransitionSmState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewTransitionSmCompartment(state TransitionSmState) *TransitionSmCompartment {
    c := &TransitionSmCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}