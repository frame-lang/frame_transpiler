// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package branch
import ( "golang/framelang")

func NewBranch() Branch {
    m := &branchStruct{}
    
    // Validate interfaces
    var _ Branch = m
    var _ Branch_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewBranchCompartment(BranchState_I)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type BranchState uint

const (
    BranchState_I BranchState = iota
    BranchState_SimpleIf
    BranchState_NegatedIf
    BranchState_Precedence
    BranchState_NestedIf
    BranchState_GuardedTransition
    BranchState_NestedGuardedTransition
    BranchState_F1
    BranchState_F2
    BranchState_F3
)

type Branch interface {
    A() 
    B() 
    C() 
    D() 
    E() 
    F() 
    OnBool(b bool) 
    OnInt(i int16) 
}

type Branch_actions interface {
    log(msg string) 
}


type branchStruct struct {
    _compartment_ *BranchCompartment
    _nextCompartment_ *BranchCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *branchStruct) A()  {
    e := framelang.FrameEvent{Msg:"A"}
    m._mux_(&e)
}

func (m *branchStruct) B()  {
    e := framelang.FrameEvent{Msg:"B"}
    m._mux_(&e)
}

func (m *branchStruct) C()  {
    e := framelang.FrameEvent{Msg:"C"}
    m._mux_(&e)
}

func (m *branchStruct) D()  {
    e := framelang.FrameEvent{Msg:"D"}
    m._mux_(&e)
}

func (m *branchStruct) E()  {
    e := framelang.FrameEvent{Msg:"E"}
    m._mux_(&e)
}

func (m *branchStruct) F()  {
    e := framelang.FrameEvent{Msg:"F"}
    m._mux_(&e)
}

func (m *branchStruct) OnBool(b bool)  {
    params := make(map[string]interface{})
    params["b"] = b
    e := framelang.FrameEvent{Msg:"OnBool", Params:params}
    m._mux_(&e)
}

func (m *branchStruct) OnInt(i int16)  {
    params := make(map[string]interface{})
    params["i"] = i
    e := framelang.FrameEvent{Msg:"OnInt", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *branchStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case BranchState_I:
        m._BranchState_I_(e)
    case BranchState_SimpleIf:
        m._BranchState_SimpleIf_(e)
    case BranchState_NegatedIf:
        m._BranchState_NegatedIf_(e)
    case BranchState_Precedence:
        m._BranchState_Precedence_(e)
    case BranchState_NestedIf:
        m._BranchState_NestedIf_(e)
    case BranchState_GuardedTransition:
        m._BranchState_GuardedTransition_(e)
    case BranchState_NestedGuardedTransition:
        m._BranchState_NestedGuardedTransition_(e)
    case BranchState_F1:
        m._BranchState_F1_(e)
    case BranchState_F2:
        m._BranchState_F2_(e)
    case BranchState_F3:
        m._BranchState_F3_(e)
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

func (m *branchStruct) _BranchState_I_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "A":
        compartment := NewBranchCompartment(BranchState_SimpleIf)
        m._transition_(compartment)
        
        return
    case "B":
        compartment := NewBranchCompartment(BranchState_NegatedIf)
        m._transition_(compartment)
        
        return
    case "C":
        compartment := NewBranchCompartment(BranchState_Precedence)
        m._transition_(compartment)
        
        return
    case "D":
        compartment := NewBranchCompartment(BranchState_NestedIf)
        m._transition_(compartment)
        
        return
    case "E":
        compartment := NewBranchCompartment(BranchState_GuardedTransition)
        m._transition_(compartment)
        
        return
    case "F":
        compartment := NewBranchCompartment(BranchState_NestedGuardedTransition)
        m._transition_(compartment)
        
        return
    }
}

func (m *branchStruct) _BranchState_SimpleIf_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnBool":
        if e.Params["b"].(bool) {
            m.log("then 1")
        } else {
        }
        if e.Params["b"].(bool) {
        } else {
            m.log("else 1")
        }
        if e.Params["b"].(bool) {
            m.log("then 2")
        } else {
            m.log("else 2")
        }
        if e.Params["b"].(bool) {
            compartment := NewBranchCompartment(BranchState_F1)
            m._transition_(compartment)
            return
        } else {
            compartment := NewBranchCompartment(BranchState_F2)
            m._transition_(compartment)
            return
        }
        
        return
    case "OnInt":
        if e.Params["i"].(int16) > 5 {
            m.log("> 5")
        } else {
            m.log("<= 5")
        }
        if e.Params["i"].(int16) < 10 {
            m.log("< 10")
        } else {
            m.log(">= 10")
        }
        if e.Params["i"].(int16) == 7 {
            m.log("== 7")
            compartment := NewBranchCompartment(BranchState_F1)
            m._transition_(compartment)
            return
        } else {
            m.log("!= 7")
            compartment := NewBranchCompartment(BranchState_F2)
            m._transition_(compartment)
            return
        }
        
        return
    }
}

func (m *branchStruct) _BranchState_NegatedIf_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnBool":
        if !(e.Params["b"].(bool)) {
            m.log("then 1")
        } else {
        }
        if !(e.Params["b"].(bool)) {
        } else {
            m.log("else 1")
        }
        if !(e.Params["b"].(bool)) {
            m.log("then 2")
        } else {
            m.log("else 2")
        }
        if !(e.Params["b"].(bool)) {
            compartment := NewBranchCompartment(BranchState_F1)
            m._transition_(compartment)
            return
        } else {
            compartment := NewBranchCompartment(BranchState_F2)
            m._transition_(compartment)
            return
        }
        
        return
    case "OnInt":
        if !(e.Params["i"].(int16) >= 5) {
            m.log("< 5")
        } else {
            m.log(">= 5")
        }
        if !(e.Params["i"].(int16) <= 10) {
            m.log("> 10")
        } else {
            m.log("<= 10")
        }
        if !(e.Params["i"].(int16) != 7) {
            m.log("== 7")
            compartment := NewBranchCompartment(BranchState_F1)
            m._transition_(compartment)
            return
        } else {
            m.log("!= 7")
            compartment := NewBranchCompartment(BranchState_F2)
            m._transition_(compartment)
            return
        }
        
        return
    }
}

func (m *branchStruct) _BranchState_Precedence_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if -e.Params["i"].(int16) >= 0 && -e.Params["i"].(int16) <= 5 {
            m.log("then 1")
        } else {
            m.log("else 1")
        }
        if !(e.Params["i"].(int16) >= -5 && e.Params["i"].(int16) <= 5) && (e.Params["i"].(int16) >= -10 && e.Params["i"].(int16) <= 10) {
            m.log("then 2")
        } else {
            m.log("else 2")
        }
        if e.Params["i"].(int16) >= 0 && e.Params["i"].(int16) <= 5 || e.Params["i"].(int16) >= 10 && e.Params["i"].(int16) <= 20 {
            m.log("then 3")
        } else {
            m.log("else 3")
        }
        if !((e.Params["i"].(int16) < 0 || e.Params["i"].(int16) > 10) && e.Params["i"].(int16) + 5 < 20) {
            m.log("then 4")
        } else {
            m.log("else 4")
        }
        
        return
    }
}

func (m *branchStruct) _BranchState_NestedIf_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int16) > 0 {
            m.log("> 0")
            if e.Params["i"].(int16) < 100 {
                m.log("< 100")
                compartment := NewBranchCompartment(BranchState_F1)
                m._transition_(compartment)
                return
            } else {
                m.log(">= 100")
            }
        } else {
            m.log("<= 0")
            if e.Params["i"].(int16) > -10 {
                m.log("> -10")
            } else {
                m.log("<= -10")
                compartment := NewBranchCompartment(BranchState_F2)
                m._transition_(compartment)
                return
            }
        }
        
        return
    }
}

func (m *branchStruct) _BranchState_GuardedTransition_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int16) > 100 {
            m.log("-> $F1")
            compartment := NewBranchCompartment(BranchState_F1)
            m._transition_(compartment)
            return
        } else {
        }
        if !(e.Params["i"].(int16) > 10) {
        } else {
            m.log("-> $F2")
            compartment := NewBranchCompartment(BranchState_F2)
            m._transition_(compartment)
            return
        }
        m.log("-> $F3")
        compartment := NewBranchCompartment(BranchState_F3)
        m._transition_(compartment)
        
        return
    }
}

func (m *branchStruct) _BranchState_NestedGuardedTransition_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int16) > 10 {
            if e.Params["i"].(int16) > 100 {
                m.log("-> $F1")
                compartment := NewBranchCompartment(BranchState_F1)
                m._transition_(compartment)
                return
            } else {
            }
            if e.Params["i"].(int16) > 50 {
            } else {
                m.log("-> $F2")
                compartment := NewBranchCompartment(BranchState_F2)
                m._transition_(compartment)
                return
            }
        } else {
        }
        m.log("-> $F3")
        compartment := NewBranchCompartment(BranchState_F3)
        m._transition_(compartment)
        
        return
    }
}

func (m *branchStruct) _BranchState_F1_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

func (m *branchStruct) _BranchState_F2_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

func (m *branchStruct) _BranchState_F3_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *branchStruct) _transition_(compartment *BranchCompartment) {
    m._nextCompartment_ = compartment
}

func (m *branchStruct) _do_transition_(nextCompartment *BranchCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *branchStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type BranchCompartment struct {
    State BranchState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewBranchCompartment(state BranchState) *BranchCompartment {
    c := &BranchCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}