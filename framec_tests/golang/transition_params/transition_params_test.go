package transition_params

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *transitParamsStruct) log(msg string) {

	m.tape = append(m.tape, msg)
}

func TestEnter(t *testing.T) {
	sm := NewTransitParams()
	x := sm.(*transitParamsStruct)
	sm.Next()
	assert.Equal(t, []string{"hi A"}, x.tape)
}

func TestEnterAndExit(t *testing.T) {
	sm := NewTransitParams()
	x := sm.(*transitParamsStruct)
	sm.Next()
	x.tape = x.tape[:0]
	sm.Next()
	assert.Equal(t, []string{"bye A", "hi B", "42"}, x.tape)
	x.tape = x.tape[:0]
	sm.Next()
	assert.Equal(t, []string{"true", "bye B", "hi again A"}, x.tape)

}

func TestChangeState(t *testing.T) {
	sm := NewTransitParams()
	x := sm.(*transitParamsStruct)
	assert.Equal(t, TransitParamsState_Init, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitParamsState_A, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitParamsState_B, x._compartment_.State)
	sm.Change()
	assert.Equal(t, TransitParamsState_A, x._compartment_.State)
	assert.Empty(t, x.tape)

}

func TestChangeAndTransition(t *testing.T) {
	sm := NewTransitParams()
	x := sm.(*transitParamsStruct)
	sm.Change()
	assert.Equal(t, TransitParamsState_A, x._compartment_.State)
	assert.Empty(t, x.tape)
	sm.Next()
	assert.Equal(t, TransitParamsState_B, x._compartment_.State)
	assert.Equal(t, []string{"bye A", "hi B", "42"}, x.tape)
	x.tape = x.tape[:0]
	sm.Change()
	assert.Equal(t, TransitParamsState_A, x._compartment_.State)
	assert.Empty(t, x.tape)
	sm.Change()
	sm.Next()
	assert.Equal(t, TransitParamsState_A, x._compartment_.State)
	assert.Equal(t, []string{"true", "bye B", "hi again A"}, x.tape)

}
