// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package simple_handler_calls

import "golang/framelang"


func NewSimpleHandlerCalls() SimpleHandlerCalls {
    m := &simpleHandlerCallsStruct{}
    
    // Validate interfaces
    var _ SimpleHandlerCalls = m
    
    
    // Create and intialize start state compartment.
    m._compartment_ = NewSimpleHandlerCallsCompartment(SimpleHandlerCallsState_Init)
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type SimpleHandlerCallsState uint

const (
    SimpleHandlerCallsState_Init SimpleHandlerCallsState = iota
    SimpleHandlerCallsState_A
    SimpleHandlerCallsState_B
)

type SimpleHandlerCalls interface {
    A() 
    B() 
    C() 
    D() 
    E() 
}

type simpleHandlerCallsStruct struct {
    _compartment_ *SimpleHandlerCallsCompartment
    _nextCompartment_ *SimpleHandlerCallsCompartment
}

//===================== Interface Block ===================//

func (m *simpleHandlerCallsStruct) A()  {
    e := framelang.FrameEvent{Msg:"A"}
    m._mux_(&e)
}

func (m *simpleHandlerCallsStruct) B()  {
    e := framelang.FrameEvent{Msg:"B"}
    m._mux_(&e)
}

func (m *simpleHandlerCallsStruct) C()  {
    e := framelang.FrameEvent{Msg:"C"}
    m._mux_(&e)
}

func (m *simpleHandlerCallsStruct) D()  {
    e := framelang.FrameEvent{Msg:"D"}
    m._mux_(&e)
}

func (m *simpleHandlerCallsStruct) E()  {
    e := framelang.FrameEvent{Msg:"E"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *simpleHandlerCallsStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case SimpleHandlerCallsState_Init:
        m._SimpleHandlerCallsState_Init_(e)
    case SimpleHandlerCallsState_A:
        m._SimpleHandlerCallsState_A_(e)
    case SimpleHandlerCallsState_B:
        m._SimpleHandlerCallsState_B_(e)
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

func (m *simpleHandlerCallsStruct) _SimpleHandlerCallsState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        compartment := NewSimpleHandlerCallsCompartment(SimpleHandlerCallsState_A)
        m._transition_(compartment)
        
        return
    case "B":
        compartment := NewSimpleHandlerCallsCompartment(SimpleHandlerCallsState_B)
        m._transition_(compartment)
        
        return
    case "C":
        m.A()
        return
        
        return
    case "D":
        m.B()
        return
        compartment := NewSimpleHandlerCallsCompartment(SimpleHandlerCallsState_A)
        m._transition_(compartment)
        
        return
    case "E":
        m.D()
        return
        m.C()
        return
        
        return
    }
}

func (m *simpleHandlerCallsStruct) _SimpleHandlerCallsState_A_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

func (m *simpleHandlerCallsStruct) _SimpleHandlerCallsState_B_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *simpleHandlerCallsStruct) _transition_(compartment *SimpleHandlerCallsCompartment) {
    m._nextCompartment_ = compartment
}

func (m *simpleHandlerCallsStruct) _do_transition_(nextCompartment *SimpleHandlerCallsCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}


//=============== Compartment ==============//

type SimpleHandlerCallsCompartment struct {
    State SimpleHandlerCallsState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewSimpleHandlerCallsCompartment(state SimpleHandlerCallsState) *SimpleHandlerCallsCompartment {
    c := &SimpleHandlerCallsCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}