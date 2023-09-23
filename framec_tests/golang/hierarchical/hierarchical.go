// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewHierarchical() Hierarchical {
    m := &hierarchicalStruct{}
    
    // Validate interfaces
    var _ Hierarchical = m
    var _ Hierarchical_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewHierarchicalCompartment(HierarchicalState_I)
    
    // Override domain variables.
    m.enters = []string{}
    m.exits = []string{}
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type HierarchicalState uint

const (
    HierarchicalState_I HierarchicalState = iota
    HierarchicalState_S
    HierarchicalState_S0
    HierarchicalState_S1
    HierarchicalState_S2
    HierarchicalState_S3
    HierarchicalState_T
)

type Hierarchical interface {
    A() 
    B() 
    C() 
}

type Hierarchical_actions interface {
    enter(msg string) 
    exit(msg string) 
    log(msg string) 
}


type hierarchicalStruct struct {
    _compartment_ *HierarchicalCompartment
    _nextCompartment_ *HierarchicalCompartment
    enters []string
    exits []string
    tape []string
}

//===================== Interface Block ===================//

func (m *hierarchicalStruct) A()  {
    e := framelang.FrameEvent{Msg:"A"}
    m._mux_(&e)
}

func (m *hierarchicalStruct) B()  {
    e := framelang.FrameEvent{Msg:"B"}
    m._mux_(&e)
}

func (m *hierarchicalStruct) C()  {
    e := framelang.FrameEvent{Msg:"C"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *hierarchicalStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case HierarchicalState_I:
        m._HierarchicalState_I_(e)
    case HierarchicalState_S:
        m._HierarchicalState_S_(e)
    case HierarchicalState_S0:
        m._HierarchicalState_S0_(e)
    case HierarchicalState_S1:
        m._HierarchicalState_S1_(e)
    case HierarchicalState_S2:
        m._HierarchicalState_S2_(e)
    case HierarchicalState_S3:
        m._HierarchicalState_S3_(e)
    case HierarchicalState_T:
        m._HierarchicalState_T_(e)
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

func (m *hierarchicalStruct) _HierarchicalState_I_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        compartment := NewHierarchicalCompartment(HierarchicalState_S)
        m._transition_(compartment)
        
        return
    }
}

func (m *hierarchicalStruct) _HierarchicalState_S_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S")
        
        return
    case "<":
        m.exit("S")
        
        return
    case "A":
        m.log("S.A")
        compartment := NewHierarchicalCompartment(HierarchicalState_S0)
        m._transition_(compartment)
        
        return
    case "B":
        m.log("S.B")
        compartment := NewHierarchicalCompartment(HierarchicalState_S1)
        m._transition_(compartment)
        
        return
    }
}

func (m *hierarchicalStruct) _HierarchicalState_S0_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S0")
        
    case "<":
        m.exit("S0")
        
      //  override parent handler
    case "A":
        m.log("S0.A")
        compartment := NewHierarchicalCompartment(HierarchicalState_T)
        m._transition_(compartment)
        
        return
      //  do this, then parent handler
    case "B":
        m.log("S0.B")
        
      //  extend parent handler
    case "C":
        m.log("S0.C")
        compartment := NewHierarchicalCompartment(HierarchicalState_S2)
        m._transition_(compartment)
        
        return
    }
    m._HierarchicalState_S_(e)
    
}

func (m *hierarchicalStruct) _HierarchicalState_S1_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S1")
        
        return
    case "<":
        m.exit("S1")
        
        return
      //  defer to parent for A
      //  do this, then parent, which transitions here
    case "B":
        m.log("S1.B")
        
      //  propagate message not handled by parent
    case "C":
        m.log("S1.C")
        
    }
    m._HierarchicalState_S_(e)
    
}

func (m *hierarchicalStruct) _HierarchicalState_S2_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S2")
        
    case "<":
        m.exit("S2")
        
      //  will propagate to S0 and S
    case "B":
        m.log("S2.B")
        
    case "C":
        m.log("S2.C")
        compartment := NewHierarchicalCompartment(HierarchicalState_T)
        m._transition_(compartment)
        
        return
    }
    m._HierarchicalState_S0_(e)
    
}  //  continue after transition (should be ignored)


func (m *hierarchicalStruct) _HierarchicalState_S3_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("S3")
        
    case "<":
        m.exit("S3")
        
      //  defer to grandparent for A
      //  override and move to sibling
    case "B":
        m.log("S3.B")
        compartment := NewHierarchicalCompartment(HierarchicalState_S2)
        m._transition_(compartment)
        
        return
    }
    m._HierarchicalState_S1_(e)
    
}

func (m *hierarchicalStruct) _HierarchicalState_T_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.enter("T")
        
        return
    case "<":
        m.exit("T")
        
        return
    case "A":
        m.log("T.A")
        compartment := NewHierarchicalCompartment(HierarchicalState_S)
        m._transition_(compartment)
        
        return
    case "B":
        m.log("T.B")
        compartment := NewHierarchicalCompartment(HierarchicalState_S2)
        m._transition_(compartment)
        
        return
    case "C":
        m.log("T.C")
        compartment := NewHierarchicalCompartment(HierarchicalState_S3)
        m._transition_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *hierarchicalStruct) _transition_(compartment *HierarchicalCompartment) {
    m._nextCompartment_ = compartment
}

func (m *hierarchicalStruct) _do_transition_(nextCompartment *HierarchicalCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *hierarchicalStruct) enter(msg string)  {}
func (m *hierarchicalStruct) exit(msg string)  {}
func (m *hierarchicalStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type HierarchicalCompartment struct {
    State HierarchicalState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewHierarchicalCompartment(state HierarchicalState) *HierarchicalCompartment {
    c := &HierarchicalCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}