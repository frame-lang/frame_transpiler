// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package hierarchical_guard

import "golang/framelang"


func NewHierarchicalGuard() HierarchicalGuard {
    m := &hierarchicalGuardStruct{}
    
    // Validate interfaces
    var _ HierarchicalGuard = m
    var _ HierarchicalGuard_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewHierarchicalGuardCompartment(HierarchicalGuardState_I)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type HierarchicalGuardState uint

const (
    HierarchicalGuardState_I HierarchicalGuardState = iota
    HierarchicalGuardState_S
    HierarchicalGuardState_S0
    HierarchicalGuardState_S1
    HierarchicalGuardState_S2
    HierarchicalGuardState_S3
    HierarchicalGuardState_S4
)

type HierarchicalGuard interface {
    A(i int) 
    B(i int) 
}

type HierarchicalGuard_actions interface {
    log(msg string) 
}


type hierarchicalGuardStruct struct {
    _compartment_ *HierarchicalGuardCompartment
    _nextCompartment_ *HierarchicalGuardCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *hierarchicalGuardStruct) A(i int)  {
    params := make(map[string]interface{})
    params["i"] = i
    e := framelang.FrameEvent{Msg:"A", Params:params}
    m._mux_(&e)
}

func (m *hierarchicalGuardStruct) B(i int)  {
    params := make(map[string]interface{})
    params["i"] = i
    e := framelang.FrameEvent{Msg:"B", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *hierarchicalGuardStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case HierarchicalGuardState_I:
        m._HierarchicalGuardState_I_(e)
    case HierarchicalGuardState_S:
        m._HierarchicalGuardState_S_(e)
    case HierarchicalGuardState_S0:
        m._HierarchicalGuardState_S0_(e)
    case HierarchicalGuardState_S1:
        m._HierarchicalGuardState_S1_(e)
    case HierarchicalGuardState_S2:
        m._HierarchicalGuardState_S2_(e)
    case HierarchicalGuardState_S3:
        m._HierarchicalGuardState_S3_(e)
    case HierarchicalGuardState_S4:
        m._HierarchicalGuardState_S4_(e)
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

func (m *hierarchicalGuardStruct) _HierarchicalGuardState_I_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S)
        m._transition_(compartment)
        
        return
    }
}

func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        m.log("S.A")
        if e.Params["i"].(int) < 10 {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S0)
            m._transition_(compartment)
            return
        } else {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S1)
            m._transition_(compartment)
            return
        }
        
        return
    case "B":
        m.log("S.B")
        if e.Params["i"].(int) < 10 {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S2)
            m._transition_(compartment)
            return
        } else {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S3)
            m._transition_(compartment)
            return
        }
        
        return
    }
}

func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S0_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        m.log("S0.A")
        if e.Params["i"].(int) > 0 {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S2)
            m._transition_(compartment)
            return
        } else {
        }
        
      //  fall through else branch
    case "B":
        m.log("S0.B")
        if e.Params["i"].(int) > 0 {
        } else {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S1)
            m._transition_(compartment)
            return
        }
        
    }
    m._HierarchicalGuardState_S_(e)
    
}  //  fall through then branch


func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S1_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        m.log("S1.A")
        if e.Params["i"].(int) > 5 {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S3)
            m._transition_(compartment)
            return
        } else {
        }
        
    }
    m._HierarchicalGuardState_S0_(e)
    
}  //  fall through else branch


func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S2_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        m.log("S2.A")
        if e.Params["i"].(int) > 10 {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S4)
            m._transition_(compartment)
            return
        } else {
        }
        
      //  fall through then branch
    case "B":
        m.log("S2.B")
        if !(e.Params["i"].(int) > 10) {
        } else {
            compartment := NewHierarchicalGuardCompartment(HierarchicalGuardState_S4)
            m._transition_(compartment)
            return
        }
        
    }
    m._HierarchicalGuardState_S1_(e)
    
}  //  fall through then branch


func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S3_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        m.log("S3.A")
        if e.Params["i"].(int) > 0 {
            m.log("stop")
            
            return
        } else {
            m.log("continue")
        }
        
    case "B":
        m.log("S3.B")
        if e.Params["i"].(int) > 0 {
            m.log("continue")
        } else {
            m.log("stop")
            
            return
        }
        
    }
    m._HierarchicalGuardState_S_(e)
    
}

func (m *hierarchicalGuardStruct) _HierarchicalGuardState_S4_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *hierarchicalGuardStruct) _transition_(compartment *HierarchicalGuardCompartment) {
    m._nextCompartment_ = compartment
}

func (m *hierarchicalGuardStruct) _do_transition_(nextCompartment *HierarchicalGuardCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *hierarchicalGuardStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type HierarchicalGuardCompartment struct {
    State HierarchicalGuardState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewHierarchicalGuardCompartment(state HierarchicalGuardState) *HierarchicalGuardCompartment {
    c := &HierarchicalGuardCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}