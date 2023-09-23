// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewStateContextStack() StateContextStack {
    m := &stateContextStackStruct{}
    
    // Validate interfaces
    var _ StateContextStack = m
    var _ StateContextStack_actions = m
    // History mechanism used in spec. Create state stack.
    m._stateStack_ = &Stack{stack: make([]StateContextStackCompartment, 0)}
    
    
    // Create and intialize start state compartment.
    m._compartment_ = NewStateContextStackCompartment(StateContextStackState_A)
    m._compartment_.StateVars["x"] = 0
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}

type Stack struct {
    stack []StateContextStackCompartment
}

func (s *Stack) Push(compartment *StateContextStackCompartment) {
    s.stack = append(s.stack, *compartment)
}

func (s *Stack) Pop() *StateContextStackCompartment {

    l := len(s.stack)
    if l == 0 {
        panic("Attempted to pop when history stack is empty")
    }
    
    res := s.stack[l-1]
    s.stack = s.stack[:l-1]
    return &res
}

type StateContextStackState uint

const (
    StateContextStackState_A StateContextStackState = iota
    StateContextStackState_B
    StateContextStackState_C
)

type StateContextStack interface {
    To_a() 
    To_b() 
    To_c() 
    Inc() 
    Value() int
    Push() 
    Pop() 
    Pop_change() 
}

type StateContextStack_actions interface {
    log(msg string) 
}


type stateContextStackStruct struct {
    _compartment_ *StateContextStackCompartment
    _nextCompartment_ *StateContextStackCompartment
    _stateStack_ *Stack
    tape []string
}

//===================== Interface Block ===================//

func (m *stateContextStackStruct) To_a()  {
    e := framelang.FrameEvent{Msg:"to_a"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) To_b()  {
    e := framelang.FrameEvent{Msg:"to_b"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) To_c()  {
    e := framelang.FrameEvent{Msg:"to_c"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) Inc()  {
    e := framelang.FrameEvent{Msg:"inc"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) Value() int {
    e := framelang.FrameEvent{Msg:"value"}
    m._mux_(&e)
    return  e.Ret.(int)
}

func (m *stateContextStackStruct) Push()  {
    e := framelang.FrameEvent{Msg:"push"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) Pop()  {
    e := framelang.FrameEvent{Msg:"pop"}
    m._mux_(&e)
}

func (m *stateContextStackStruct) Pop_change()  {
    e := framelang.FrameEvent{Msg:"pop_change"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *stateContextStackStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case StateContextStackState_A:
        m._StateContextStackState_A_(e)
    case StateContextStackState_B:
        m._StateContextStackState_B_(e)
    case StateContextStackState_C:
        m._StateContextStackState_C_(e)
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

func (m *stateContextStackStruct) _StateContextStackState_A_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("A:>")
        
        return
    case "<":
        m.log("A:<")
        
        return
    case "inc":
        m._compartment_.StateVars["x"] = m._compartment_.StateVars["x"].(int) + 1
        
        return
    case "value":
        e.Ret = (m._compartment_.StateVars["x"].(int))
        return
        
    case "to_a":
        compartment := NewStateContextStackCompartment(StateContextStackState_A)
        compartment.StateVars["x"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateContextStackCompartment(StateContextStackState_B)
        compartment.StateVars["y"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateContextStackCompartment(StateContextStackState_C)
        compartment.StateVars["z"] = 0
        
        m._transition_(compartment)
        
        return
    case "push":
        m._stateStack_push_(m._compartment_)
        
        return
    case "pop":
        compartment := m._stateStack_pop_()
        m._transition_(compartment)
        
        return
    case "pop_change":
        compartment := m._stateStack_pop_()
        m._changeState_(compartment)
        
        return
    }
}

func (m *stateContextStackStruct) _StateContextStackState_B_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("B:>")
        
        return
    case "<":
        m.log("B:<")
        
        return
    case "inc":
        m._compartment_.StateVars["y"] = m._compartment_.StateVars["y"].(int) + 5
        
        return
    case "value":
        e.Ret = (m._compartment_.StateVars["y"].(int))
        return
        
    case "to_a":
        compartment := NewStateContextStackCompartment(StateContextStackState_A)
        compartment.StateVars["x"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateContextStackCompartment(StateContextStackState_B)
        compartment.StateVars["y"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateContextStackCompartment(StateContextStackState_C)
        compartment.StateVars["z"] = 0
        
        m._transition_(compartment)
        
        return
    case "push":
        m._stateStack_push_(m._compartment_)
        
        return
    case "pop":
        compartment := m._stateStack_pop_()
        m._transition_(compartment)
        
        return
    case "pop_change":
        compartment := m._stateStack_pop_()
        m._changeState_(compartment)
        
        return
    }
}

func (m *stateContextStackStruct) _StateContextStackState_C_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("C:>")
        
        return
    case "<":
        m.log("C:<")
        
        return
    case "inc":
        m._compartment_.StateVars["z"] = m._compartment_.StateVars["z"].(int) + 10
        
        return
    case "value":
        e.Ret = (m._compartment_.StateVars["z"].(int))
        return
        
    case "to_a":
        compartment := NewStateContextStackCompartment(StateContextStackState_A)
        compartment.StateVars["x"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateContextStackCompartment(StateContextStackState_B)
        compartment.StateVars["y"] = 0
        
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateContextStackCompartment(StateContextStackState_C)
        compartment.StateVars["z"] = 0
        
        m._transition_(compartment)
        
        return
    case "push":
        m._stateStack_push_(m._compartment_)
        
        return
    case "pop":
        compartment := m._stateStack_pop_()
        m._transition_(compartment)
        
        return
    case "pop_change":
        compartment := m._stateStack_pop_()
        m._changeState_(compartment)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *stateContextStackStruct) _transition_(compartment *StateContextStackCompartment) {
    m._nextCompartment_ = compartment
}

func (m *stateContextStackStruct) _do_transition_(nextCompartment *StateContextStackCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

func (m *stateContextStackStruct) _stateStack_push_(compartment *StateContextStackCompartment) {
    m._stateStack_.Push(compartment)
}

func (m *stateContextStackStruct) _stateStack_pop_() *StateContextStackCompartment {
    compartment := m._stateStack_.Pop()
    return compartment
}

func (m *stateContextStackStruct) _changeState_(compartment *StateContextStackCompartment) {
    m._compartment_ = compartment
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *stateContextStackStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type StateContextStackCompartment struct {
    State StateContextStackState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewStateContextStackCompartment(state StateContextStackState) *StateContextStackCompartment {
    c := &StateContextStackCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}