package state_params

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *stateParamsStruct) got_param(name string, val int) {
	m.param_log = append(m.param_log, name+"="+strconv.Itoa(val))
}

func TestSingleParameter(t *testing.T) {
	sm := NewStateParams()
	x := sm.(*stateParamsStruct)
	sm.Next()
	sm.Log()
	assert.Equal(t, []string{"val=1"}, x.param_log)
}

func TestMultipleParameters(t *testing.T) {
	sm := NewStateParams()
	x := sm.(*stateParamsStruct)
	sm.Next()
	sm.Next()
	sm.Log()
	assert.Equal(t, []string{"left=1", "right=2"}, x.param_log)
}

func TestSeveralPasses(t *testing.T) {
	sm := NewStateParams()
	x := sm.(*stateParamsStruct)

	sm.Next() // val=1
	sm.Next() // left=1 right=2
	sm.Next() // val=3
	sm.Log()
	sm.Prev() // left=4 right=3
	sm.Log()
	sm.Prev() // val=12
	sm.Log()
	assert.Equal(t, []string{"val=3", "left=4", "right=3", "val=12"}, x.param_log)
}
