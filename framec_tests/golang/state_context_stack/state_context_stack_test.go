package state_context_stack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *stateContextStackStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

/* Test that a pop restores a pushed state. */
func TestPushPop(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)

	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Push()
	sm.To_b()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
}

/* Test that multiple states can be pushed and subsequently restored by pops, LIFO style. */
func TestMultiplePushPops(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Push()
	sm.To_c()
	sm.Push()
	sm.To_a()
	sm.Push()
	sm.Push()
	sm.To_c() // no push
	sm.To_b()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, B, A, A, C, A
	sm.To_a()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	sm.To_a()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	sm.To_b()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, B, A
	sm.To_a()
	sm.To_b()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)

}

/*  Test that pop transitions trigger enter/exit events. */
func TestPopTransitionEvents(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)
	sm.To_b()
	sm.Push()
	sm.To_a()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, A, B
	sm.To_a()
	x.tape = x.tape[:0]
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, []string{"A:<", "C:>"}, x.tape)
	x.tape = x.tape[:0]
	sm.Pop()
	sm.Pop()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	assert.Equal(t, []string{"C:<", "A:>", "A:<", "B:>"}, x.tape)
	assert.Equal(t, []string{"C:<", "A:>", "A:<", "B:>"}, x.tape)
}

/* Test that pop change-states do not trigger enter/exit events. */
func TestPopChangeStateNoEvents(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)
	sm.To_b()
	sm.Push()
	sm.To_a()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, A, B
	sm.To_a()
	x.tape = x.tape[:0]
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	sm.Pop_change()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Empty(t, x.tape)
	sm.Pop()
	sm.Pop_change()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	assert.Equal(t, []string{"C:<", "A:>"}, x.tape)
}

/* Test that state variables are restored after pop. */
func TestPopRestoresStateVariables(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)

	sm.Inc()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 2, sm.Value())
	sm.To_b()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	assert.Equal(t, sm.Value(), 5)
	sm.To_c()
	sm.Inc()
	sm.Inc()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, 30, sm.Value())
	sm.To_a()
	sm.Inc()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 1, sm.Value())
	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, 30, sm.Value())
	sm.Pop()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	assert.Equal(t, 5, sm.Value())
	sm.To_a()
	sm.Inc()
	sm.Inc()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 3, sm.Value())
	sm.To_c()
	sm.Inc()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, 10, sm.Value())
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 3, sm.Value())
	sm.Pop()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 2, sm.Value())

}

/* Test that push stores a snapshot of the current values of state variables. Any changes to
state variables after a push should not be reflected after that state is popped. */

func TestPushStoresStateVariableSnapshot(t *testing.T) {
	sm := NewStateContextStack()
	x := sm.(*stateContextStackStruct)
	sm.Inc()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 2, sm.Value())
	sm.Inc()
	sm.Inc()
	assert.Equal(t, 4, sm.Value())

	sm.To_b()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_B, x._compartment_.State)
	assert.Equal(t, 5, sm.Value())
	sm.Inc()
	sm.Inc()
	assert.Equal(t, 15, sm.Value()) // these changes should be forgotten

	sm.To_c()
	sm.Inc()
	sm.Inc()
	sm.Inc()
	sm.Push()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, 30, sm.Value())
	sm.Inc()
	assert.Equal(t, 40, sm.Value()) // forgotten

	sm.To_a()
	sm.Inc()
	assert.Equal(t, StateContextStackState_A, x._compartment_.State)
	assert.Equal(t, 1, sm.Value())

	sm.Pop()
	assert.Equal(t, StateContextStackState_C, x._compartment_.State)
	assert.Equal(t, 30, sm.Value())

}
