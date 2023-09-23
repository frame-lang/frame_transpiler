// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewMatch() Match {
    m := &matchStruct{}
    
    // Validate interfaces
    var _ Match = m
    var _ Match_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewMatchCompartment(MatchState_Init)
    
    // Override domain variables.
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type MatchState uint

const (
    MatchState_Init MatchState = iota
    MatchState_EmptyMatch
    MatchState_SimpleMatch
    MatchState_MultiMatch
    MatchState_NestedMatch
    MatchState_ChildMatch
    MatchState_Final
)

type Match interface {
    Empty() 
    Simple() 
    Multi() 
    Nested() 
    Child() 
    OnInt(i int) 
    Onstring(s string) 
}

type Match_actions interface {
    log(msg string) 
}


type matchStruct struct {
    _compartment_ *MatchCompartment
    _nextCompartment_ *MatchCompartment
    tape []string
}

//===================== Interface Block ===================//

func (m *matchStruct) Empty()  {
    e := framelang.FrameEvent{Msg:"Empty"}
    m._mux_(&e)
}

func (m *matchStruct) Simple()  {
    e := framelang.FrameEvent{Msg:"Simple"}
    m._mux_(&e)
}

func (m *matchStruct) Multi()  {
    e := framelang.FrameEvent{Msg:"Multi"}
    m._mux_(&e)
}

func (m *matchStruct) Nested()  {
    e := framelang.FrameEvent{Msg:"Nested"}
    m._mux_(&e)
}

func (m *matchStruct) Child()  {
    e := framelang.FrameEvent{Msg:"Child"}
    m._mux_(&e)
}

func (m *matchStruct) OnInt(i int)  {
    params := make(map[string]interface{})
    params["i"] = i
    e := framelang.FrameEvent{Msg:"OnInt", Params:params}
    m._mux_(&e)
}

func (m *matchStruct) Onstring(s string)  {
    params := make(map[string]interface{})
    params["s"] = s
    e := framelang.FrameEvent{Msg:"Onstring", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *matchStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case MatchState_Init:
        m._MatchState_Init_(e)
    case MatchState_EmptyMatch:
        m._MatchState_EmptyMatch_(e)
    case MatchState_SimpleMatch:
        m._MatchState_SimpleMatch_(e)
    case MatchState_MultiMatch:
        m._MatchState_MultiMatch_(e)
    case MatchState_NestedMatch:
        m._MatchState_NestedMatch_(e)
    case MatchState_ChildMatch:
        m._MatchState_ChildMatch_(e)
    case MatchState_Final:
        m._MatchState_Final_(e)
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

func (m *matchStruct) _MatchState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Empty":
        compartment := NewMatchCompartment(MatchState_EmptyMatch)
        m._transition_(compartment)
        
        return
    case "Simple":
        compartment := NewMatchCompartment(MatchState_SimpleMatch)
        m._transition_(compartment)
        
        return
    case "Multi":
        compartment := NewMatchCompartment(MatchState_MultiMatch)
        m._transition_(compartment)
        
        return
    case "Nested":
        compartment := NewMatchCompartment(MatchState_NestedMatch)
        m._transition_(compartment)
        
        return
    case "Child":
        compartment := NewMatchCompartment(MatchState_ChildMatch)
        m._transition_(compartment)
        
        return
    }
}

func (m *matchStruct) _MatchState_EmptyMatch_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "Onstring":
        if e.Params["s"].(string) == "" || e.Params["s"].(string) == "foo" {
            m.log("empty")
        } else {
            m.log("?")
        }
        
        return
    }
}  //  TODO: matching only the empty string is broken


func (m *matchStruct) _MatchState_SimpleMatch_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int) == 0 {
            m.log("0")
        } else if e.Params["i"].(int) == 42 {
            m.log("42")
        } else if e.Params["i"].(int) == 42 {
            m.log("!!!")
        } else if e.Params["i"].(int) == -200 {
            m.log("-200")
        } else {
            m.log("?")
        }
        
        return
    case "Onstring":
        if e.Params["s"].(string) == "hello" {
            m.log("hello")
        } else if e.Params["s"].(string) == "hello" {
            m.log("!!!")
        } else if e.Params["s"].(string) == "goodbye" {
            m.log("goodbye")
        } else if e.Params["s"].(string) == "Testing 1, 2, 3..." {
            m.log("testing")
        } else if e.Params["s"].(string) == "$10!" {
            m.log("money")
        } else {
            m.log("?")
        }
        
        return
    }
}

func (m *matchStruct) _MatchState_MultiMatch_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int) == 3 || e.Params["i"].(int) == -7 {
            m.log("3|-7")
        } else if e.Params["i"].(int) == -4 || e.Params["i"].(int) == 5 || e.Params["i"].(int) == 6 {
            m.log("-4|5|6")
        } else {
            m.log("?")
        }
        
        return
    case "Onstring":
        if e.Params["s"].(string) == "$10" || e.Params["s"].(string) == "12.5%" || e.Params["s"].(string) == "@#*!" {
            m.log("symbols")
        } else if e.Params["s"].(string) == " " || e.Params["s"].(string) == "  " || e.Params["s"].(string) == "\t" || e.Params["s"].(string) == "\n" {
            m.log("whitespace")
        } else {
            m.log("?")
        }
        
        return
    }
}

func (m *matchStruct) _MatchState_NestedMatch_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int) > 0 {
            if e.Params["i"].(int) == 1 || e.Params["i"].(int) == 2 || e.Params["i"].(int) == 3 {
                m.log("1-3")
                if e.Params["i"].(int) == 1 {
                    m.log("1")
                } else if e.Params["i"].(int) == 2 {
                    m.log("2")
                } else {
                    m.log("3")
                }
            } else if e.Params["i"].(int) == 4 || e.Params["i"].(int) == 5 {
                m.log("4-5")
                if e.Params["i"].(int) == 4 {
                    m.log("4")
                } else {
                    m.log("5")
                }
            } else {
                m.log("too big")
            }
        } else {
            m.log("too small")
        }
        
        return
    case "Onstring":
        if e.Params["s"].(string) == "hello" || e.Params["s"].(string) == "hola" || e.Params["s"].(string) == "bonjour" {
            m.log("greeting")
            if e.Params["s"].(string) == "hello" {
                m.log("English")
            } else if e.Params["s"].(string) == "hola" {
                m.log("Spanish")
            } else {
                m.log("French")
            }
        } else if e.Params["s"].(string) == "goodbye" || e.Params["s"].(string) == "adios" || e.Params["s"].(string) == "au revoir" {
            m.log("farewell")
            if e.Params["s"].(string) == "goodbye" {
                m.log("English")
            } else if e.Params["s"].(string) == "adios" {
                m.log("Spanish")
            } else {
                m.log("French")
            }
        } else {
            m.log("?")
        }
        
        return
    }
}

func (m *matchStruct) _MatchState_ChildMatch_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "OnInt":
        if e.Params["i"].(int) == 0 {
            compartment := NewMatchCompartment(MatchState_Final)
            m._transition_(compartment)
            return
        } else if e.Params["i"].(int) == 3 {
            m.log("3")
        } else if e.Params["i"].(int) == 4 {
            m.log("4")
            
            return
        } else if e.Params["i"].(int) == 42 {
            m.log("42 in child")
        } else if e.Params["i"].(int) == 5 {
            m.log("5")
            compartment := NewMatchCompartment(MatchState_Final)
            m._transition_(compartment)
            return
        } else {
            m.log("no match in child")
        }
        
    case "Onstring":
        if e.Params["s"].(string) == "hello" {
            m.log("hello in child")
        } else if e.Params["s"].(string) == "goodbye" {
            compartment := NewMatchCompartment(MatchState_Final)
            m._transition_(compartment)
            return
        } else if e.Params["s"].(string) == "Testing 1, 2, 3..." {
            m.log("testing in child")
            
            return
        } else {
            m.log("no match in child")
        }
        
    }
    m._MatchState_SimpleMatch_(e)
    
}

func (m *matchStruct) _MatchState_Final_(e *framelang.FrameEvent) {
    switch e.Msg {
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *matchStruct) _transition_(compartment *MatchCompartment) {
    m._nextCompartment_ = compartment
}

func (m *matchStruct) _do_transition_(nextCompartment *MatchCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *matchStruct) log(msg string)  {}

********************************************************/

//=============== Compartment ==============//

type MatchCompartment struct {
    State MatchState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewMatchCompartment(state MatchState) *MatchCompartment {
    c := &MatchCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}