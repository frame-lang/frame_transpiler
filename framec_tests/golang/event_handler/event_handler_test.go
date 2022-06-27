package event_handler

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *eventHandlerStruct) log(msg string, val int) {
	value := msg + "=" + strconv.Itoa(val)
	m.tape = append(m.tape, value)
}

func TestSingleParameter(t *testing.T) {
	sm := NewEventHandler()
	x := sm.(*eventHandlerStruct)

	sm.LogIt(2)
	assert.Equal(t, []string{"x=2"}, x.tape)
}

func TestComputeTwoParameter(t *testing.T) {
	sm := NewEventHandler()
	x := sm.(*eventHandlerStruct)

	sm.LogAdd(-3, 10)
	assert.Equal(t, []string{"a=-3", "b=10", "a+b=7"}, x.tape)
}

func TestReturnLocalVariable(t *testing.T) {
	sm := NewEventHandler()
	x := sm.(*eventHandlerStruct)

	ret := sm.LogReturn(13, 21)
	assert.Equal(t, []string{"a=13", "b=21", "r=34"}, x.tape)
	assert.Equal(t, 34, ret)
}

func TestPassResult(t *testing.T) {
	sm := NewEventHandler()
	x := sm.(*eventHandlerStruct)

	sm.PassAdd(5, -12)
	assert.Equal(t, []string{"p=-7"}, x.tape)

}

func TestPassAndReturnResult(t *testing.T) {
	sm := NewEventHandler()
	x := sm.(*eventHandlerStruct)

	ret := sm.PassReturn(101, -59)
	assert.Equal(t, []string{"r=42", "p=42"}, x.tape)
	assert.Equal(t, 42, ret)
}
