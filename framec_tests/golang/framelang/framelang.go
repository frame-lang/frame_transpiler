package framelang

type FrameState uint

type FrameEvent struct {
	Msg    string
	Params map[string]interface{}
	Ret    interface{}
}

// type StateContext struct {
// 	State     FrameState
// 	StateArgs map[string]interface{}
// 	StateVars map[string]interface{}
// 	EnterArgs map[string]interface{}
// }

// func NewStateContext(state FrameState) *StateContext {
// 	sc := new(StateContext)
// 	sc.State = state
// 	sc.StateArgs = make(map[string]interface{})
// 	sc.StateVars = make(map[string]interface{})
// 	sc.EnterArgs = make(map[string]interface{})
// 	return sc
// }

// func (sc *StateContext) AddStateArg(name string, value interface{}) {
// 	sc.StateArgs[name] = value
// }

// func (sc *StateContext) SetStateArg(name string, value interface{}) {
// 	sc.StateArgs[name] = value
// }

// func (sc *StateContext) GetStateArg(name string) interface{} {
// 	return sc.StateArgs[name]
// }

// func (sc *StateContext) AddStateVar(name string, value interface{}) {
// 	sc.StateVars[name] = value
// }

// func (sc *StateContext) SetStateVar(name string, value interface{}) {
// 	sc.StateVars[name] = value
// }

// func (sc *StateContext) GetStateVar(name string) interface{} {
// 	return sc.StateVars[name]
// }

// func (sc *StateContext) AddEnterArg(name string, value interface{}) {
// 	sc.EnterArgs[name] = value
// }

// func (sc *StateContext) SetEnterArg(name string, value interface{}) {
// 	sc.EnterArgs[name] = value
// }

// func (sc *StateContext) GetEnterArg(name string) interface{} {
// 	return sc.EnterArgs[name]
// }

// func (sc *StateContext) GetEnterArgs() map[string]interface{} {
// 	return sc.EnterArgs
// }
