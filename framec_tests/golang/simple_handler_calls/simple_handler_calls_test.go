package simple_handler_calls

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/* Test a basic handler call. */
func TestSimpleCall(t *testing.T) {
	sm := NewSimpleHandlerCalls()
	x := sm.(*simpleHandlerCallsStruct)
	sm.C()
	assert.Equal(t, SimpleHandlerCallsState_A, x._compartment_.State)
}

/* Test that a handler call terminates the current handler. */
func TestCallsTerminateHandler(t *testing.T) {
	sm := NewSimpleHandlerCalls()
	x := sm.(*simpleHandlerCallsStruct)
	sm.D()
	assert.Equal(t, SimpleHandlerCallsState_B, x._compartment_.State)

	sm = NewSimpleHandlerCalls()
	x = sm.(*simpleHandlerCallsStruct)
	sm.E()
	assert.Equal(t, SimpleHandlerCallsState_B, x._compartment_.State)

}
