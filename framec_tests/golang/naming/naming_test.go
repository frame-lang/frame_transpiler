package naming

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *namingStruct) snake_action(snake_param int) {
	m.snake_log = append(m.snake_log, snake_param)
}
func (m *namingStruct) CamelAction(CamelParam int) {
	m.CamelLog = append(m.CamelLog, CamelParam)
}
func (m *namingStruct) action123(param123 int) {
	m.log123 = append(m.log123, param123)
}
func (m *namingStruct) logFinal(r int) {
	m.finalLog = append(m.finalLog, r)
}

/* Test that the generated state machine works and that events are
named as expected. */
func TestFollowNamingWorks(t *testing.T) {
	sm := NewNaming()
	x := sm.(*namingStruct)
	sm.Snake_event(1)
	assert.Equal(t, NamingState_snake_state, x._compartment_.State)
	sm.Snake_event(2)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.Snake_event(1)
	assert.Equal(t, NamingState_snake_state, x._compartment_.State)
	sm.CamelEvent(3)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.Snake_event(1)
	assert.Equal(t, NamingState_snake_state, x._compartment_.State)
	sm.Event123(4)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	assert.Equal(t, []int{1103, 1104, 1105}, x.finalLog)
	x.finalLog = x.finalLog[:0]

	sm.CamelEvent(11)
	assert.Equal(t, NamingState_CamelState, x._compartment_.State)
	sm.Snake_event(2)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.CamelEvent(11)
	assert.Equal(t, NamingState_CamelState, x._compartment_.State)
	sm.CamelEvent(3)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.CamelEvent(11)
	assert.Equal(t, NamingState_CamelState, x._compartment_.State)
	sm.Event123(4)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	assert.Equal(t, []int{1213, 1214, 1215}, x.finalLog)
	x.finalLog = x.finalLog[:0]

	sm.Event123(21)
	assert.Equal(t, NamingState_state123, x._compartment_.State)
	sm.Snake_event(2)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.Event123(21)
	assert.Equal(t, NamingState_state123, x._compartment_.State)
	sm.CamelEvent(3)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	sm.Event123(21)
	assert.Equal(t, NamingState_state123, x._compartment_.State)
	sm.Event123(4)
	assert.Equal(t, NamingState_Init, x._compartment_.State)
	assert.Equal(t, []int{1323, 1324, 1325}, x.finalLog)
	assert.Equal(t, []int{1103, 1213, 1323}, x.snake_log)
	assert.Equal(t, []int{1104, 1214, 1324}, x.CamelLog)
	assert.Equal(t, []int{1105, 1215, 1325}, x.log123)
}

/* Test that dynamic interface calls are renamed correctly. */

func TestInterfaceCalls(t *testing.T) {
	sm := NewNaming()
	x := sm.(*namingStruct)

	sm.Call("snake_event", 1)
	sm.Call("CamelEvent", 2)
	sm.Call("event123", 3)
	sm.Call("snake_event", 4)
	sm.Call("CamelEvent", 5)
	sm.Call("event123", 6)

	assert.Equal(t, []int{1103, 1307, 1211}, x.finalLog)
	assert.Equal(t, []int{1307}, x.snake_log)
	assert.Equal(t, []int{1103}, x.CamelLog)
	assert.Equal(t, []int{1211}, x.log123)

}
