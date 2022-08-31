package branch

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *branchStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

func TestSimpleIfBool(t *testing.T) {

	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.A()
	sm.OnBool(true)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"then 1", "then 2"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.A()
	sm.OnBool(false)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"else 1", "else 2"}, x.tape)
}

func TestSimpleIfInit(t *testing.T) {

	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.A()
	sm.OnInt(7)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"> 5", "< 10", "== 7"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.A()
	sm.OnInt(-3)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"<= 5", "< 10", "!= 7"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.A()
	sm.OnInt(12)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"> 5", ">= 10", "!= 7"}, x.tape)

}

func TestNegatedIfBool(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.B()
	sm.OnBool(true)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"else 1", "else 2"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.B()
	sm.OnBool(false)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"then 1", "then 2"}, x.tape)
}

func TestNegatedIfInt(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.B()
	sm.OnInt(7)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{">= 5", "<= 10", "== 7"}, x.tape)

	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.B()
	sm.OnInt(5)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{">= 5", "<= 10", "!= 7"}, x.tape)

	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.B()
	sm.OnInt(10)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{">= 5", "<= 10", "!= 7"}, x.tape)

	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.B()
	sm.OnInt(0)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"< 5", "<= 10", "!= 7"}, x.tape)

	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.B()
	sm.OnInt(100)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{">= 5", "> 10", "!= 7"}, x.tape)

}

func TestOperatorPrecedence(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.C()
	sm.OnInt(0)
	assert.Equal(t, []string{"then 1", "else 2", "then 3", "then 4"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(7)
	assert.Equal(t, []string{"else 1", "then 2", "else 3", "then 4"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(-3)
	assert.Equal(t, []string{"then 1", "else 2", "else 3", "else 4"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(12)
	assert.Equal(t, []string{"else 1", "else 2", "then 3", "else 4"}, x.tape)
}

func TestNestedIf(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.D()
	sm.OnInt(50)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"> 0", "< 100"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.D()
	sm.OnInt(200)
	assert.Equal(t, BranchState_NestedIf, x._compartment_.State)
	assert.Equal(t, []string{"> 0", ">= 100"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.D()
	sm.OnInt(-5)
	assert.Equal(t, BranchState_NestedIf, x._compartment_.State)
	assert.Equal(t, []string{"<= 0", "> -10"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.D()
	sm.OnInt(-10)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"<= 0", "<= -10"}, x.tape)
}

/// Test that a transition guarded by a conditional expression triggers an
/// early return from the handler.
func TestGuardedTransition(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.E()
	sm.OnInt(5)
	assert.Equal(t, BranchState_F3, x._compartment_.State)
	assert.Equal(t, []string{"-> $F3"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.E()
	sm.OnInt(15)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"-> $F2"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.E()
	sm.OnInt(115)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"-> $F1"}, x.tape)
}

/*  Test that a transition guarded by a nested conditional expression
triggers an early return from the handler, but this return doesn't
 apply to non-transitioned branches. */
func TestNestedGuardedTransition(t *testing.T) {
	sm := NewBranch()
	x := sm.(*branchStruct)
	sm.F()
	sm.OnInt(5)
	assert.Equal(t, BranchState_F3, x._compartment_.State)
	assert.Equal(t, []string{"-> $F3"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.F()
	sm.OnInt(15)
	assert.Equal(t, BranchState_F2, x._compartment_.State)
	assert.Equal(t, []string{"-> $F2"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.F()
	sm.OnInt(65)
	assert.Equal(t, BranchState_F3, x._compartment_.State)
	assert.Equal(t, []string{"-> $F3"}, x.tape)
	sm = NewBranch()
	x = sm.(*branchStruct)
	sm.F()
	sm.OnInt(115)
	assert.Equal(t, BranchState_F1, x._compartment_.State)
	assert.Equal(t, []string{"-> $F1"}, x.tape)
}
