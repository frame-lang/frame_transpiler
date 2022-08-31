package handler_calls

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *handlerCallsStruct) log(from string, val int) {
	value := from + "(" + strconv.Itoa(val) + ")"
	m.tape = append(m.tape, value)
}

func TestCallTerminateHandler(t *testing.T) {
	sm := NewHandlerCalls()
	x := sm.(*handlerCallsStruct)

	sm.NonRec()
	sm.Foo(10)
	assert.NotContains(t, x.tape, "Unreachable(0)", "Handler calls unreachable statement")
}

func TestNonRecursive(t *testing.T) {
	sm := NewHandlerCalls()
	x := sm.(*handlerCallsStruct)
	sm.NonRec()
	sm.Foo(10)
	assert.Equal(t, []string{"Foo(10)", "Bar(20)", "Final(30)"}, x.tape)

}

func TestSelfRecursive(t *testing.T) {
	sm := NewHandlerCalls()
	x := sm.(*handlerCallsStruct)
	sm.SelfRec()
	sm.Foo(10)
	assert.Equal(t, []string{"Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)"}, x.tape)

}

func TestMutuallyRecursive(t *testing.T) {
	sm := NewHandlerCalls()
	x := sm.(*handlerCallsStruct)
	sm.MutRec()
	sm.Foo(2)
	assert.Equal(t, []string{
		"Foo(2)",
		"Bar(4)",
		"Foo(4)",
		"Bar(8)",
		"Foo(16)",
		"Bar(32)",
		"Foo(96)",
		"Final(162)"},
		x.tape)

}

func TestStringMatchCall(t *testing.T) {
	sm := NewHandlerCalls()
	x := sm.(*handlerCallsStruct)
	sm.NonRec()
	sm.Call("Foo", 5)
	assert.Equal(t, []string{"Foo(5)", "Bar(10)", "Final(15)"}, x.tape)
	x.tape = x.tape[:0]

	sm.NonRec()
	sm.Call("Bar", 20)
	assert.Equal(t, []string{"Bar(20)", "Final(20)"}, x.tape)
	x.tape = x.tape[:0]

	sm.NonRec()
	sm.Call("Qux", 37)
	assert.Equal(t, []string{"Foo(1000)", "Bar(2000)", "Final(3000)"}, x.tape)
}
