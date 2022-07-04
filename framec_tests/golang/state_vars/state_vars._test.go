package state_vars

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSingleVariable(t *testing.T) {
	sm := NewStateVars()
	x := sm.(*stateVarsStruct)
	assert.Equal(t, StateVarsState_A, x._compartment_.State)
	sm.X() // increment x
	sm.X() // increment x
	assert.Equal(t, 2, x._compartment_.StateVars["x"].(int))
}

func TestMultipleVariables(t *testing.T) {
	sm := NewStateVars()
	x := sm.(*stateVarsStruct)
	sm.Y()
	assert.Equal(t, StateVarsState_B, x._compartment_.State)
	assert.Equal(t, x._compartment_.StateVars["y"].(int), 10)
	assert.Equal(t, x._compartment_.StateVars["z"].(int), 100)
	sm.Y()
	sm.Y()
	sm.Z()
	sm.Y()
	assert.Equal(t, x._compartment_.StateVars["y"].(int), 13)
	assert.Equal(t, x._compartment_.StateVars["z"].(int), 101)

}

func TestVariablesAreReset(t *testing.T) {
	sm := NewStateVars()
	x := sm.(*stateVarsStruct)
	sm.X() // increment x
	sm.X() // increment x
	assert.Equal(t, x._compartment_.StateVars["x"].(int), 2)
	sm.Z() // transition to B
	sm.Z() // increment z
	sm.Y() // increment y
	sm.Z() // increment z
	assert.Equal(t, x._compartment_.StateVars["y"].(int), 11)
	assert.Equal(t, x._compartment_.StateVars["z"].(int), 102)
	sm.X() // transition to A
	assert.Equal(t, x._compartment_.StateVars["x"].(int), 0)
	sm.Y() // transition to B
	assert.Equal(t, x._compartment_.StateVars["y"].(int), 10)
	assert.Equal(t, x._compartment_.StateVars["z"].(int), 100)

}
