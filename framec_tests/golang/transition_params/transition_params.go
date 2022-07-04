// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package transition_params

import (
    "strconv"
    "golang/framelang")


func NewTransitParams() TransitParams {
    m := &transitParamsStruct{}
    
    // Validate interfaces
    var _ TransitParams = m
    var _ TransitParams_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewTransitParamsCompartment(TransitParamsState_Init)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type TransitParamsState uint

const (
    TransitParamsState_Init TransitParamsState = iota
    TransitParamsState_A
    TransitParamsState_B
)

type TransitParams interface {
    Next() 
    Change() 
}

type TransitParams_actions interface {
    log(msg string) 
}


type transitParamsStruct struct {
    _compartment_ *TransitParamsCompartment
    _nextCompartment_ *TransitParamsCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *transitParamsStruct) Next()  {
    e := framelang.FrameEvent{Msg:"Next"}
    m._mux_(&e)
}

func (m *transitParamsStruct) Change()  {
    e := framelang.FrameEvent{Msg:"Change"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *transitParamsStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case TransitParamsState_Init:
        m._TransitParamsState_Init_(e)
    case TransitParamsState_A:
        m._TransitParamsState_A_(e)
    case TransitParamsState_B:
        m._TransitParamsState_B_(e)
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

func (m *transitParamsStruct) _TransitParamsState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Next":
        compartment := NewTransitParamsCompartment(TransitParamsState_A)
        compartment.EnterArgs["msg"] = "hi A"
        m._transition_(compartment)
        
        return
    case "Change":
        compartment := NewTransitParamsCompartment(TransitParamsState_A)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitParamsStruct) _TransitParamsState_A_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log(e.Params["msg"].(string))
        
        return
    case "<":
        m.log("bye A")
        
        return
    case "Next":
        compartment := NewTransitParamsCompartment(TransitParamsState_B)
        compartment.EnterArgs["msg"] = "hi B"
        compartment.EnterArgs["val"] = 42
        m._transition_(compartment)
        
        return
    case "Change":
        compartment := NewTransitParamsCompartment(TransitParamsState_B)
        
        m._changeState_(compartment)
        
        return
    }
}

func (m *transitParamsStruct) _TransitParamsState_B_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log(e.Params["msg"].(string))
        m.log(strconv.Itoa(e.Params["val"].(int)))
        
        return
    case "<":
        m.log(strconv.FormatBool(e.Params["val"].(bool)))
        m.log(e.Params["msg"].(string))
        
        return
    case "Next":
        m._compartment_.ExitArgs["val"] = true
        m._compartment_.ExitArgs["msg"] = "bye B"
        compartment := NewTransitParamsCompartment(TransitParamsState_A)
        compartment.EnterArgs["msg"] = "hi again A"
        m._transition_(compartment)
        
        return
    case "Change":
        compartment := NewTransitParamsCompartment(TransitParamsState_A)
        
        m._changeState_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *transitParamsStruct) _transition_(compartment *TransitParamsCompartment) {
    m._nextCompartment_ = compartment
}

func (m *transitParamsStruct) _do_transition_(nextCompartment *TransitParamsCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

func (m *transitParamsStruct) _changeState_(compartment *TransitParamsCompartment) {
    m._compartment_ = compartment
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *transitParamsStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type TransitParamsCompartment struct {
    State TransitParamsState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewTransitParamsCompartment(state TransitParamsState) *TransitParamsCompartment {
    c := &TransitParamsCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}