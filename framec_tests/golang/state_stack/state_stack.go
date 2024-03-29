// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewStateStack() StateStack {
    m := &stateStackStruct{}
    
    // Validate interfaces
    var _ StateStack = m
    var _ StateStack_actions = m
    // History mechanism used in spec. Create state stack.
    m._stateStack_ = &Stack{stack: make([]StateStackCompartment, 0)}
    
    
    // Create and intialize start state compartment.
    m._compartment_ = NewStateStackCompartment(StateStackState_A)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}

type Stack struct {
    stack []StateStackCompartment
}

func (s *Stack) Push(compartment *StateStackCompartment) {
    s.stack = append(s.stack, *compartment)
}

func (s *Stack) Pop() *StateStackCompartment {

    l := len(s.stack)
    if l == 0 {
        panic("Attempted to pop when history stack is empty")
    }
    
    res := s.stack[l-1]
    s.stack = s.stack[:l-1]
    return &res
}

type StateStackState uint

const (
    StateStackState_A StateStackState = iota
    StateStackState_B
    StateStackState_C
)

type StateStack interface {
    To_a() 
    To_b() 
    To_c() 
    Push() 
    Pop() 
    Pop_change() 
}

type StateStack_actions interface {
    log(msg string) 
}


type stateStackStruct struct {
    _compartment_ *StateStackCompartment
    _nextCompartment_ *StateStackCompartment
    _stateStack_ *Stack
    tape []string
}

//===================== Interface Block ===================//

func (m *stateStackStruct) To_a()  {
    e := framelang.FrameEvent{Msg:"to_a"}
    m._mux_(&e)
}

func (m *stateStackStruct) To_b()  {
    e := framelang.FrameEvent{Msg:"to_b"}
    m._mux_(&e)
}

func (m *stateStackStruct) To_c()  {
    e := framelang.FrameEvent{Msg:"to_c"}
    m._mux_(&e)
}

func (m *stateStackStruct) Push()  {
    e := framelang.FrameEvent{Msg:"push"}
    m._mux_(&e)
}

func (m *stateStackStruct) Pop()  {
    e := framelang.FrameEvent{Msg:"pop"}
    m._mux_(&e)
}

func (m *stateStackStruct) Pop_change()  {
    e := framelang.FrameEvent{Msg:"pop_change"}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *stateStackStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case StateStackState_A:
        m._StateStackState_A_(e)
    case StateStackState_B:
        m._StateStackState_B_(e)
    case StateStackState_C:
        m._StateStackState_C_(e)
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

func (m *stateStackStruct) _StateStackState_A_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("A:>")
        
        return
    case "<":
        m.log("A:<")
        
        return
    case "to_a":
        compartment := NewStateStackCompartment(StateStackState_A)
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateStackCompartment(StateStackState_B)
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateStackCompartment(StateStackState_C)
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
        
        return
    }
}  // ->> $$[-]


func (m *stateStackStruct) _StateStackState_B_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("B:>")
        
        return
    case "<":
        m.log("B:<")
        
        return
    case "to_a":
        compartment := NewStateStackCompartment(StateStackState_A)
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateStackCompartment(StateStackState_B)
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateStackCompartment(StateStackState_C)
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
        
        return
    }
}  // ->> $$[-]


func (m *stateStackStruct) _StateStackState_C_(e *framelang.FrameEvent) {
    switch e.Msg {
    case ">":
        m.log("C:>")
        
        return
    case "<":
        m.log("C:<")
        
        return
    case "to_a":
        compartment := NewStateStackCompartment(StateStackState_A)
        m._transition_(compartment)
        
        return
    case "to_b":
        compartment := NewStateStackCompartment(StateStackState_B)
        m._transition_(compartment)
        
        return
    case "to_c":
        compartment := NewStateStackCompartment(StateStackState_C)
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
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *stateStackStruct) _transition_(compartment *StateStackCompartment) {
    m._nextCompartment_ = compartment
}

func (m *stateStackStruct) _do_transition_(nextCompartment *StateStackCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

func (m *stateStackStruct) _stateStack_push_(compartment *StateStackCompartment) {
    m._stateStack_.Push(compartment)
}

func (m *stateStackStruct) _stateStack_pop_() *StateStackCompartment {
    compartment := m._stateStack_.Pop()
    return compartment
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *stateStackStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type StateStackCompartment struct {
    State StateStackState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewStateStackCompartment(state StateStackState) *StateStackCompartment {
    c := &StateStackCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}