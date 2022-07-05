package state_stack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *stateStackStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

/// Test that a pop restores a pushed state.
func TestPushPop(t *testing.T) {
	sm := NewStateStack()
	x := sm.(*stateStackStruct)
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Push()
	sm.To_b()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_A, x._compartment_.State)
}

/// Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
func TestMultiPushPops(t *testing.T) {
	sm := NewStateStack()
	x := sm.(*stateStackStruct)
	assert.Equal(t, StateStackState_A, x._compartment_.State)
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
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_C, x._compartment_.State)
	sm.To_a()
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_C, x._compartment_.State)
	sm.To_b()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, B, A
	sm.To_a()
	sm.To_b()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_C, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_A, x._compartment_.State)

}

/// Test that pop transitions trigger enter/exit events.
func TestPopTransitionEvents(t *testing.T) {
	sm := NewStateStack()
	x := sm.(*stateStackStruct)

	sm.To_b()
	sm.Push()
	sm.To_a()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, A, B
	sm.To_a()
	x.tape = x.tape[:0]
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop()
	assert.Equal(t, StateStackState_C, x._compartment_.State)
	assert.Equal(t, []string{"A:<", "C:>"}, x.tape)
	x.tape = x.tape[:0]
	sm.Pop()
	sm.Pop()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	assert.Equal(t, []string{"C:<", "A:>", "A:<", "B:>"}, x.tape)

}

/// Test that pop change-states do not trigger enter/exit events.
func TestPopChangeStateNoEvents(t *testing.T) {
	sm := NewStateStack()
	x := sm.(*stateStackStruct)

	sm.To_b()
	sm.Push()
	sm.To_a()
	sm.Push()
	sm.To_c()
	sm.Push() // stack top-to-bottom: C, A, B
	sm.To_a()
	x.tape = x.tape[:0]
	assert.Equal(t, StateStackState_A, x._compartment_.State)
	sm.Pop_change()
	assert.Equal(t, StateStackState_C, x._compartment_.State)
	assert.Empty(t, x.tape)
	sm.Pop()
	sm.Pop_change()
	assert.Equal(t, StateStackState_B, x._compartment_.State)
	assert.Equal(t, []string{"C:<", "A:>"}, x.tape)

}
