package var_scope

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *varScopeStruct) log(s string) {
	m.tape = append(m.tape, s)
}

func (m *varScopeStruct) do_nn() {
	m.Nn("|nn|[d]")
}
func (m *varScopeStruct) do_ny() {
	m.Ny("|ny|[d]")
}
func (m *varScopeStruct) do_yn() {
	m.Yn("|yn|[d]", "|yn|[x]")
}
func (m *varScopeStruct) do_yy() {
	m.Yy("|yy|[d]", "|yy|[x]")
}

func expected(state string, msg string, x string) []string {
	var result []string = []string{}

	result = append(result, "#.a")
	result = append(result, "$"+state+"[b]")
	result = append(result, "$"+state+".c")
	result = append(result, "|"+msg+"|"+"[d]")
	result = append(result, "|"+msg+"|"+".e")
	result = append(result, x)

	return result
}

func TestNoShadowing(t *testing.T) {
	sm := NewVarScope()
	x := sm.(*varScopeStruct)
	x.To_nn()
	x.do_nn()
	assert.Equal(t, expected("NN", "nn", "#.x"), x.tape)

}

func TestAllShadowingScenarios(t *testing.T) {
	sm := NewVarScope()
	x := sm.(*varScopeStruct)
	sm.To_nn()
	x.do_ny()
	assert.Equal(t, expected("NN", "ny", "|ny|.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_yn()
	assert.Equal(t, expected("NN", "yn", "|yn|[x]"), x.tape)
	x.tape = x.tape[:0]
	x.do_yy()
	assert.Equal(t, expected("NN", "yy", "|yy|.x"), x.tape)

	sm = NewVarScope()
	x = sm.(*varScopeStruct)
	sm.To_ny()
	x.do_nn()
	assert.Equal(t, expected("NY", "nn", "$NY.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_ny()
	assert.Equal(t, expected("NY", "ny", "|ny|.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_yn()
	assert.Equal(t, expected("NY", "yn", "|yn|[x]"), x.tape)
	x.tape = x.tape[:0]
	x.do_yy()
	assert.Equal(t, expected("NY", "yy", "|yy|.x"), x.tape)

	sm = NewVarScope()
	x = sm.(*varScopeStruct)
	sm.To_yn()
	x.do_nn()
	assert.Equal(t, expected("YN", "nn", "$YN[x]"), x.tape)
	x.tape = x.tape[:0]
	x.do_ny()
	assert.Equal(t, expected("YN", "ny", "|ny|.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_yn()
	assert.Equal(t, expected("YN", "yn", "|yn|[x]"), x.tape)
	x.tape = x.tape[:0]
	x.do_yy()
	assert.Equal(t, expected("YN", "yy", "|yy|.x"), x.tape)

	sm = NewVarScope()
	x = sm.(*varScopeStruct)
	sm.To_yy()
	x.do_nn()
	assert.Equal(t, expected("YY", "nn", "$YY.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_ny()
	assert.Equal(t, expected("YY", "ny", "|ny|.x"), x.tape)
	x.tape = x.tape[:0]
	x.do_yn()
	assert.Equal(t, expected("YY", "yn", "|yn|[x]"), x.tape)
	x.tape = x.tape[:0]
	x.do_yy()
	assert.Equal(t, expected("YY", "yy", "|yy|.x"), x.tape)

}

func TestDisAmbiguation(t *testing.T) {}
