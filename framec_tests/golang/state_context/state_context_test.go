package state_context

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *stateContextSmStruct) log(name string, val int) {
	m.tape = append(m.tape, name+"="+strconv.Itoa(val))
}

func TestInitialState(t *testing.T) {
	sm := NewStateContextSm()
	x := sm.(*stateContextSmStruct)
	r := sm.Inc()
	assert.Equal(t, r, 4)
	sm.LogState()
	assert.Equal(t, x.tape, []string{"w=3", "w=4", "w=4"})
}

func TestTransition(t *testing.T) {
	sm := NewStateContextSm()
	x := sm.(*stateContextSmStruct)
	sm.Inc()
	sm.Inc()
	x.tape = x.tape[:0]

	sm.Start()
	assert.Equal(t, []string{"a=3", "b=5", "x=15"}, x.tape)
	x.tape = x.tape[:0]

	sm.Inc()
	r := sm.Inc()
	assert.Equal(t, r, 17)
	assert.Equal(t, []string{"x=16", "x=17"}, x.tape)
	x.tape = x.tape[:0]

	sm.Next(3)
	assert.Equal(t, []string{"c=10", "x=27", "a=30", "y=17", "z=47"}, x.tape)
	x.tape = x.tape[:0]

	sm.Inc()
	sm.Inc()
	r = sm.Inc()
	assert.Equal(t, r, 50)
	assert.Equal(t, []string{"z=48", "z=49", "z=50"}, x.tape)
}

func TestChangeState(t *testing.T) {
	sm := NewStateContextSm()
	x := sm.(*stateContextSmStruct)
	sm.Inc()
	sm.Inc()
	sm.Start()
	x.tape = x.tape[:0]

	sm.Inc()
	assert.Equal(t, []string{"x=16"}, x.tape)
	x.tape = x.tape[:0]

	sm.Change(10)
	sm.LogState()
	assert.Equal(t, []string{"y=26", "z=0"}, x.tape)
	x.tape = x.tape[:0]

	sm.Inc()
	sm.Change(100)
	sm.LogState()
	assert.Equal(t, StateContextSmState_Init, x._compartment_.State)
	assert.Equal(t, []string{"z=1", "tmp=127", "w=0"}, x.tape)

}
