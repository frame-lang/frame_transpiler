// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package hierarchical_guard

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *hierarchicalGuardStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

/* Test that basic conditional transitions work properly. In particular,
that control propagates to a parent handler if a child handler does
not transition and ends in a continue (`:>`). */

func TestPropagateToParent(t *testing.T) {
	sm := NewHierarchicalGuard()
	x := sm.(*hierarchicalGuardStruct)
	sm.A(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	sm.A(20)
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	assert.Equal(t, []string{"S0.A"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.A(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	sm.A(-5)
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S0.A", "S.A"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.A(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	sm.B(-5)
	assert.Equal(t, HierarchicalGuardState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S0.B"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.A(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	sm.B(5)
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
}

/* Test that control propagates across across multiple levels if a
transition is not initiated. */
func TestPropagateMultipleLevels(t *testing.T) {
	sm := NewHierarchicalGuard()
	x := sm.(*hierarchicalGuardStruct)
	sm.B(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	sm.A(7)
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	assert.Equal(t, []string{"S2.A", "S1.A"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.B(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	sm.A(-5)
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S2.A", "S1.A", "S0.A", "S.A"}, x.tape)

}

/* Test that propagation of control skips levels that do not contain a
given handler. */
func TestPropagateSkipsLevels(t *testing.T) {
	sm := NewHierarchicalGuard()
	x := sm.(*hierarchicalGuardStruct)
	sm.B(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	sm.B(-5)
	assert.Equal(t, HierarchicalGuardState_S1, x._compartment_.State)
	assert.Equal(t, []string{"S2.B", "S0.B"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.B(0)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	sm.B(5)
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	assert.Equal(t, []string{"S2.B", "S0.B", "S.B"}, x.tape)
}

/* Test that conditional returns prevent propagation to parents. */
func TestContitionalReturn(t *testing.T) {
	sm := NewHierarchicalGuard()
	x := sm.(*hierarchicalGuardStruct)
	sm.B(20)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	sm.A(5)
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	assert.Equal(t, []string{"S3.A", "stop"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.B(20)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	sm.A(-5)
	assert.Equal(t, HierarchicalGuardState_S0, x._compartment_.State)
	assert.Equal(t, []string{"S3.A", "continue", "S.A"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.B(20)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	sm.B(-5)
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	assert.Equal(t, []string{"S3.B", "stop"}, x.tape)

	sm = NewHierarchicalGuard()
	x = sm.(*hierarchicalGuardStruct)
	sm.B(20)
	x.tape = x.tape[:0]
	assert.Equal(t, HierarchicalGuardState_S3, x._compartment_.State)
	sm.B(5)
	assert.Equal(t, HierarchicalGuardState_S2, x._compartment_.State)
	assert.Equal(t, []string{"S3.B", "continue", "S.B"}, x.tape)

}
