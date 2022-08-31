// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package transition

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *transitionSmStruct) enter(state string) {
	m.enters = append(m.enters, state)
}
func (m *transitionSmStruct) exit(state string) {
	m.exits = append(m.exits, state)
}

//  for clearing all values
func (m *transitionSmStruct) clearAll() {
	m.enters = m.enters[:0]
	m.exits = m.exits[:0]
}

/// Test that transition works and triggers enter and exit events.
func TestTransitionEvents(t *testing.T) {
	sm := NewTransitionSm()
	x := sm.(*transitionSmStruct)
	x.clearAll()
	sm.Transit()
	assert.Equal(t, TransitionSmState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S0"}, x.exits)
	assert.Equal(t, []string{"S1"}, x.enters)
}

/// Test that change-state works and does not trigger events.
func TestChangeStateNoEvents(t *testing.T) {
	sm := NewTransitionSm()
	x := sm.(*transitionSmStruct)
	x.clearAll()
	sm.Change()
	assert.Equal(t, TransitionSmState_S1, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitionSmState_S2, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitionSmState_S3, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitionSmState_S4, x._compartment_.State)
	assert.Empty(t, x.exits)
	assert.Empty(t, x.enters)

}

/// Test transition that triggers another transition in an enter event handler.

func TestCascadingTransition(t *testing.T) {
	sm := NewTransitionSm()
	x := sm.(*transitionSmStruct)
	sm.Change()
	x.clearAll()
	assert.Equal(t, TransitionSmState_S1, x._compartment_.State)
	sm.Transit()
	assert.Equal(t, TransitionSmState_S3, x._compartment_.State)
	assert.Equal(t, []string{"S1", "S2"}, x.exits)
	assert.Equal(t, []string{"S2", "S3"}, x.enters)
}

/// Test transition that triggers a change-state from an enter event handler.

func TestCascadingChangeState(t *testing.T) {
	sm := NewTransitionSm()
	x := sm.(*transitionSmStruct)
	sm.Change()
	sm.Change()
	sm.Change()
	x.clearAll()
	assert.Equal(t, TransitionSmState_S3, x._compartment_.State)
	sm.Transit()
	assert.Equal(t, TransitionSmState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S3"}, x.exits)
	assert.Equal(t, []string{"S4"}, x.enters)
}
