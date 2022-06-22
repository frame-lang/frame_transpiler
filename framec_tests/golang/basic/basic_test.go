package basic

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *basicStruct) entered(msg string) {
	m.entry_log = append(m.entry_log, msg)
}

func (m *basicStruct) left(msg string) {
	m.exit_log = append(m.exit_log, msg)
}

func TestIntialEnterEvent(t *testing.T) {
	sm := NewBasic()
	x := sm.(*basicStruct)
	expected := []string{"S0"}
	assert.Equal(t, x.entry_log, expected)

}

func TestTransitionEnterEvents(t *testing.T) {
	sm := NewBasic()
	x := sm.(*basicStruct)
	x.entry_log = x.entry_log[:0]
	sm.A()
	sm.B()

	expected := []string{"S1", "S0"}
	assert.Equal(t, x.entry_log, expected)

}

func TestTransitionExitEvents(t *testing.T) {
	sm := NewBasic()
	x := sm.(*basicStruct)
	sm.A()
	sm.B()

	expected := []string{"S0", "S1"}
	assert.Equal(t, expected, x.exit_log)

}

func TestCurrentState(t *testing.T) {
	sm := NewBasic()
	x := sm.(*basicStruct)
	assert.Equal(t, BasicState_S0, x._compartment_.State)
	sm.A()
	assert.Equal(t, BasicState_S1, x._compartment_.State)
	sm.B()
	assert.Equal(t, BasicState_S0, x._compartment_.State)
}
