package hierarchical

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *hierarchicalStruct) enter(msg string) {
	m.enters = append(m.enters, msg)
}
func (m *hierarchicalStruct) exit(msg string) {
	m.exits = append(m.exits, msg)
}
func (m *hierarchicalStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

/// Test that a continue (`:>`) in a child enter handler calls the parent enter handler.
func TestEnterContinue(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	x.enters = x.enters[:0]
	sm.A()
	assert.Equal(t, []string{"S0", "S"}, x.enters)
	x.enters = x.enters[:0]
	sm.C()
	assert.Equal(t, []string{"S2", "S0", "S"}, x.enters)
}

/// Test that a continue (`:>`) in a child exit handler calls the parent exit handler.
func TestExitContinue(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	sm.A()
	x.exits = x.exits[:0]
	sm.C()
	assert.Equal(t, []string{"S0", "S"}, x.exits)
	x.exits = x.exits[:0]
	sm.A()
	assert.Equal(t, []string{"S2", "S0", "S"}, x.exits)

}

/// Test that a return (`^`) in a child enter handler *does not* call the parent enter handler.
func TestEnterReturn(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	x.enters = x.enters[:0]
	sm.B()
	assert.Equal(t, []string{"S1"}, x.enters)
	sm = NewHierarchical()
	x = sm.(*hierarchicalStruct)
	sm.A()
	sm.A()
	assert.Equal(t, HierarchicalState_T, x._compartment_.State)
	x.enters = x.enters[:0]
	sm.C()
	assert.Equal(t, []string{"S3", "S1"}, x.enters)

}

/// Test that a return (`^`) in a child exit handler *does not* call the parent exit handler.
func TestExitReturn(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	x.exits = x.exits[:0]
	sm.A()
	assert.Equal(t, []string{"S1"}, x.exits)
	sm = NewHierarchical()
	x = sm.(*hierarchicalStruct)
	sm.A()
	sm.A()
	sm.C()
	assert.Equal(t, HierarchicalState_S3, x._compartment_.State)
	x.exits = x.exits[:0]
	sm.B()
	assert.Equal(t, []string{"S3", "S1"}, x.exits)

}

/// Test that location in a hierarchical state is represented correctly. In this test, all
/// state transitions are performed by the immediately matching handler.
func TestCurrentStateSimple(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	assert.Equal(t, HierarchicalState_S, x._compartment_.State)
	sm.A()
	assert.Equal(t, HierarchicalState_S0, x._compartment_.State)
	sm.A()
	assert.Equal(t, HierarchicalState_T, x._compartment_.State)
	sm.C()
	assert.Equal(t, HierarchicalState_S3, x._compartment_.State)
	sm.B()
	assert.Equal(t, HierarchicalState_S2, x._compartment_.State)
}

/* Test that location in a hierarchical state is represented correctly. In this test, several
state transitions propagate message handling to parents, either by implicit fall-through or
explicit continues. */
func TestCurrentStateWithPropagation(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	assert.Equal(t, HierarchicalState_S, x._compartment_.State)
	sm.A()
	assert.Equal(t, HierarchicalState_S0, x._compartment_.State)
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	sm.C()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	sm.A()
	assert.Equal(t, HierarchicalState_S0, x._compartment_.State)
	sm.C()
	assert.Equal(t, HierarchicalState_S2, x._compartment_.State)
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
}

/* Test that a handler in a child overrides the parent handler if the child handler ends with
a return. */
func TestOverrideParentHandler(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	sm.A()
	x.tape = x.tape[:0]
	sm.A()
	assert.Equal(t, HierarchicalState_T, x._compartment_.State)
	assert.Equal(t, []string{"S0.A"}, x.tape)
	sm.C()
	x.tape = x.tape[:0]
	sm.B()
	assert.Equal(t, HierarchicalState_S2, x._compartment_.State)
	assert.Equal(t, []string{"S3.B"}, x.tape)
}

// Test that a handler in a child propagates control to the parent handler if the child
// handler ends with a continue.

func TestBeforeParentHandle(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)

	sm.A()
	x.tape = x.tape[:0]
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S0.B", "S.B"}, x.tape)
	x.tape = x.tape[:0]
	x.exits = x.exits[:0]
	x.enters = x.enters[:0]

	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S1.B", "S.B"}, x.tape)
	assert.Equal(t, []string{"S1"}, x.exits)
	assert.Equal(t, []string{"S1"}, x.enters)

	sm = NewHierarchical()
	x = sm.(*hierarchicalStruct)

	sm.A()
	sm.C()
	assert.Equal(t, HierarchicalState_S2, x._compartment_.State)
	x.tape = x.tape[:0]
	x.exits = x.exits[:0]
	x.enters = x.enters[:0]
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S2.B", "S0.B", "S.B"}, x.tape)
	assert.Equal(t, []string{"S2", "S0", "S"}, x.exits)
	assert.Equal(t, []string{"S1"}, x.enters)

}

/* Test that missing event handlers in children automatically propagate to parents.
 */
func TestDeferToParentHandler(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)

	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	x.tape = x.tape[:0]
	sm.A()
	assert.Equal(t, HierarchicalState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S.A"}, x.tape)
	sm.A()
	sm.C()
	assert.Equal(t, HierarchicalState_S3, x._compartment_.State)
	x.tape = x.tape[:0]
	sm.A()
	assert.Equal(t, HierarchicalState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S.A"}, x.tape)
}

/* Test that propagating control to a parent handler that doesn't handle the current message
is a no-op. */

func TestBeforeMissingHandler(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)
	sm.B()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	x.tape = x.tape[:0]
	x.enters = x.enters[:0]
	x.exits = x.exits[:0]
	sm.C()
	assert.Equal(t, HierarchicalState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S1.C"}, x.tape)
	assert.Empty(t, x.exits)
	assert.Empty(t, x.enters)
}

/* Test that a continue after a transition statement is ignored. */

func TestContinueAfterTransitionIgnored(t *testing.T) {
	sm := NewHierarchical()
	x := sm.(*hierarchicalStruct)

	sm.A()
	sm.C()
	assert.Equal(t, HierarchicalState_S2, x._compartment_.State)
	x.tape = x.tape[:0]
	x.enters = x.enters[:0]
	sm.C()
	assert.Equal(t, HierarchicalState_T, x._compartment_.State)
	assert.Equal(t, []string{"T"}, x.enters)
	assert.Equal(t, []string{"S2.C"}, x.tape)
}
