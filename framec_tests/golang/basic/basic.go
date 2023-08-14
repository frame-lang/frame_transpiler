// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package basic
import ( "golang/framelang")

func NewBasic() Basic {
    m := &basicStruct{}
    
    // Validate interfaces
    var _ Basic = m
    var _ Basic_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewBasicCompartment(BasicState_S0)
    
    // Override domain variables.
    m.entry_log = []string{}
    m.exit_log = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type BasicState uint

const (
    BasicState_S0 BasicState = iota
    BasicState_S1
)

type Basic interface {
    A() 
    B() 
}

type Basic_actions interface {
    entered(msg string) 
    left(msg string) 
}


type basicStruct struct {
    _compartment_ *BasicCompartment
    _nextCompartment_ *BasicCompartment
    entry_log []string
    exit_log []string
}

//===================== Interface Block ===================//

func (m *basicStruct) A()  {
    e := framelang.FrameEvent{Msg:"A"}
    m._mux_(&e)
}

func (m *basicStruct) B()  {
    e := framelang.FrameEvent{Msg:"B"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *basicStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case BasicState_S0:
        m._BasicState_S0_(e)
    case BasicState_S1:
        m._BasicState_S1_(e)
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

func (m *basicStruct) _BasicState_S0_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.entered("S0")
        
        return
    case "<":
        m.left("S0")
        
        return
    case "A":
        // ooh
        compartment := NewBasicCompartment(BasicState_S1)
        m._transition_(compartment)
        
        return
    }
}

func (m *basicStruct) _BasicState_S1_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.entered("S1")
        
        return
    case "<":
        m.left("S1")
        
        return
    case "B":
        // aah
        compartment := NewBasicCompartment(BasicState_S0)
        m._transition_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *basicStruct) _transition_(compartment *BasicCompartment) {
    m._nextCompartment_ = compartment
}

func (m *basicStruct) _do_transition_(nextCompartment *BasicCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *basicStruct) entered(msg string)  {}
func (m *basicStruct) left(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type BasicCompartment struct {
    State BasicState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewBasicCompartment(state BasicState) *BasicCompartment {
    c := &BasicCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}