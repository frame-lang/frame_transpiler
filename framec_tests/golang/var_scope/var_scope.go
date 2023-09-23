// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files


func NewVarScope() VarScope {
    m := &varScopeStruct{}
    
    // Validate interfaces
    var _ VarScope = m
    var _ VarScope_actions = m
    
    // Create and intialize start state compartment.
    m._compartment_ = NewVarScopeCompartment(VarScopeState_Init)
    
    // Override domain variables.
    m.a = "#.a"
    m.x = "#.x"
    m.tape = []string{}
    
    // Send system start event
    e := framelang.FrameEvent{Msg:">"}
    m._mux_(&e)
    return m
}


type VarScopeState uint

const (
    VarScopeState_Init VarScopeState = iota
    VarScopeState_NN
    VarScopeState_NY
    VarScopeState_YN
    VarScopeState_YY
)

type VarScope interface {
    To_nn() 
    To_ny() 
    To_yn() 
    To_yy() 
    Nn(d string) 
    Ny(d string) 
    Yn(d string,x string) 
    Yy(d string,x string) 
    Sigils(x string) 
}

type VarScope_actions interface {
    log(s string) 
}


type varScopeStruct struct {
    _compartment_ *VarScopeCompartment
    _nextCompartment_ *VarScopeCompartment
    a string
    x string
    tape []string
}

//===================== Interface Block ===================//

func (m *varScopeStruct) To_nn()  {
    e := framelang.FrameEvent{Msg:"to_nn"}
    m._mux_(&e)
}

func (m *varScopeStruct) To_ny()  {
    e := framelang.FrameEvent{Msg:"to_ny"}
    m._mux_(&e)
}

func (m *varScopeStruct) To_yn()  {
    e := framelang.FrameEvent{Msg:"to_yn"}
    m._mux_(&e)
}

func (m *varScopeStruct) To_yy()  {
    e := framelang.FrameEvent{Msg:"to_yy"}
    m._mux_(&e)
}

func (m *varScopeStruct) Nn(d string)  {
    params := make(map[string]interface{})
    params["d"] = d
    e := framelang.FrameEvent{Msg:"nn", Params:params}
    m._mux_(&e)
}

func (m *varScopeStruct) Ny(d string)  {
    params := make(map[string]interface{})
    params["d"] = d
    e := framelang.FrameEvent{Msg:"ny", Params:params}
    m._mux_(&e)
}

func (m *varScopeStruct) Yn(d string,x string)  {
    params := make(map[string]interface{})
    params["d"] = d
    params["x"] = x
    e := framelang.FrameEvent{Msg:"yn", Params:params}
    m._mux_(&e)
}

func (m *varScopeStruct) Yy(d string,x string)  {
    params := make(map[string]interface{})
    params["d"] = d
    params["x"] = x
    e := framelang.FrameEvent{Msg:"yy", Params:params}
    m._mux_(&e)
}

func (m *varScopeStruct) Sigils(x string)  {
    params := make(map[string]interface{})
    params["x"] = x
    e := framelang.FrameEvent{Msg:"sigils", Params:params}
    m._mux_(&e)
}

//====================== Multiplexer ====================//

func (m *varScopeStruct) _mux_(e *framelang.FrameEvent) {
    switch m._compartment_.State {
    case VarScopeState_Init:
        m._VarScopeState_Init_(e)
    case VarScopeState_NN:
        m._VarScopeState_NN_(e)
    case VarScopeState_NY:
        m._VarScopeState_NY_(e)
    case VarScopeState_YN:
        m._VarScopeState_YN_(e)
    case VarScopeState_YY:
        m._VarScopeState_YY_(e)
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

func (m *varScopeStruct) _VarScopeState_Init_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "to_nn":
        compartment := NewVarScopeCompartment(VarScopeState_NN)
        compartment.StateArgs["b"] = "$NN[b]"
        
        compartment.StateVars["c"] = "$NN.c"
        
        m._transition_(compartment)
        
        return
    case "to_ny":
        compartment := NewVarScopeCompartment(VarScopeState_NY)
        compartment.StateArgs["b"] = "$NY[b]"
        
        compartment.StateVars["c"] = "$NY.c"
        
        compartment.StateVars["x"] = "$NY.x"
        
        m._transition_(compartment)
        
        return
    case "to_yn":
        compartment := NewVarScopeCompartment(VarScopeState_YN)
        compartment.StateArgs["b"] = "$YN[b]"
        
        compartment.StateArgs["x"] = "$YN[x]"
        
        compartment.StateVars["c"] = "$YN.c"
        
        m._transition_(compartment)
        
        return
    case "to_yy":
        compartment := NewVarScopeCompartment(VarScopeState_YY)
        compartment.StateArgs["b"] = "$YY[b]"
        
        compartment.StateArgs["x"] = "$YY[x]"
        
        compartment.StateVars["c"] = "$YY.c"
        
        compartment.StateVars["x"] = "$YY.x"
        
        m._transition_(compartment)
        
        return
    }
}

func (m *varScopeStruct) _VarScopeState_NN_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "nn":
        var et string = "|nn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(m.x)
        
        return
    case "ny":
        var et string = "|ny|.e"
        var x string = "|ny|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "yn":
        var et string = "|yn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(e.Params["x"].(string))
        
        return
    case "yy":
        var et string = "|yy|.e"
        var x string = "|yy|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "sigils":
        m.log(m.x)
        
        return
    }
}  //  var x:string = "|sigils|.x"
  //  log(||[x])
  //  log(||.x)


func (m *varScopeStruct) _VarScopeState_NY_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "nn":
        var et string = "|nn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log((m._compartment_.StateVars["x"].(string)))
        
        return
    case "ny":
        var et string = "|ny|.e"
        var x string = "|ny|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "yn":
        var et string = "|yn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(e.Params["x"].(string))
        
        return
    case "yy":
        var et string = "|yy|.e"
        var x string = "|yy|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "sigils":
        m.log(m.x)
        
        return
    }
}  //  var x:string = "|sigils|.x"
  //  log($.x)
  //  log(||[x])
  //  log(||.x)


func (m *varScopeStruct) _VarScopeState_YN_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "nn":
        var et string = "|nn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log((m._compartment_.StateArgs["x"].(string)))
        
        return
    case "ny":
        var et string = "|ny|.e"
        var x string = "|ny|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "yn":
        var et string = "|yn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(e.Params["x"].(string))
        
        return
    case "yy":
        var et string = "|yy|.e"
        var x string = "|yy|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "sigils":
        m.log(m.x)
        
        return
    }
}  //  var x:string = "|sigils|.x"
  //  log($[x])
  //  log(||[x])
  //  log(||.x)


func (m *varScopeStruct) _VarScopeState_YY_(e *framelang.FrameEvent) {
    switch e.Msg {
    case "nn":
        var et string = "|nn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log((m._compartment_.StateVars["x"].(string)))
        
        return
    case "ny":
        var et string = "|ny|.e"
        var x string = "|ny|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "yn":
        var et string = "|yn|.e"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(e.Params["x"].(string))
        
        return
    case "yy":
        var et string = "|yy|.e"
        var x string = "|yy|.x"
        m.log(m.a)
        m.log((m._compartment_.StateArgs["b"].(string)))
        m.log((m._compartment_.StateVars["c"].(string)))
        m.log(e.Params["d"].(string))
        m.log(et)
        m.log(x)
        
        return
    case "sigils":
        m.log(m.x)
        
        return
    }
}

//=============== Machinery and Mechanisms ==============//

func (m *varScopeStruct) _transition_(compartment *VarScopeCompartment) {
    m._nextCompartment_ = compartment
}

func (m *varScopeStruct) _do_transition_(nextCompartment *VarScopeCompartment) {
    m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
    m._compartment_ = nextCompartment
    m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
}

//===================== Actions Block ===================//


/********************************************************

// Unimplemented Actions

func (m *varScopeStruct) log(s string)  {}

********************************************************/

//=============== Compartment ==============//

type VarScopeCompartment struct {
    State VarScopeState
    StateArgs map[string]interface{}
    StateVars map[string]interface{}
    EnterArgs map[string]interface{}
    ExitArgs map[string]interface{}
    _forwardEvent_ *framelang.FrameEvent
}

func NewVarScopeCompartment(state VarScopeState) *VarScopeCompartment {
    c := &VarScopeCompartment{State: state}
    c.StateArgs = make(map[string]interface{})
    c.StateVars = make(map[string]interface{})
    c.EnterArgs = make(map[string]interface{})
    c.ExitArgs = make(map[string]interface{})
    return c
}