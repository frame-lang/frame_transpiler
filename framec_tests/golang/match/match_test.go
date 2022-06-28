package match

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func (m *matchStruct) log(msg string) {
	m.tape = append(m.tape, msg)
}

/// Test matching the empty string.
/// TODO: Matching the empty string currently only works in multi-string
/// patterns. The pattern `//`, which should match only the empty string,
/// instead produces a parse error.

func TestEmptyString(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Empty()
	sm.Onstring("")
	assert.Equal(t, []string{"empty"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("hi")
	assert.Equal(t, []string{"?"}, x.tape)
}

func TestIntegerMatch(t *testing.T) {

	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Simple()
	sm.OnInt(0)
	assert.Equal(t, []string{"0"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(42)
	assert.Equal(t, []string{"42"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(-200)
	assert.Equal(t, []string{"-200"}, x.tape)
}

/* Test simple string matching.
TODO: Testing revealed some limitations:
 * Frame does not support UTF-8 graphemes larger than 1 byte, so we're
   restricted to ASCII.
 * Frame does not have a way to match the '/' or '|' characters,
   which are part of the matching syntax.
*/
func TestStringMatch(t *testing.T) {

	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Simple()
	sm.Onstring("hello")
	assert.Equal(t, []string{"hello"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("goodbye")
	assert.Equal(t, []string{"goodbye"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("Testing 1, 2, 3...")
	assert.Equal(t, []string{"testing"}, x.tape)
	assert.Equal(t, []string{"testing"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("$10!")
	assert.Equal(t, []string{"money"}, x.tape)
	assert.Equal(t, []string{"money"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("missing")
	assert.Equal(t, []string{"?"}, x.tape)
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("Testing")
	assert.Equal(t, []string{"?"}, x.tape)
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("")
	assert.Equal(t, []string{"?"}, x.tape)
	assert.Equal(t, []string{"?"}, x.tape)
}

/* Test the multiple match syntax for integers. */

func TestIntegerMultiMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Multi()
	sm.OnInt(3)
	assert.Equal(t, []string{"3|-7"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(-7)
	assert.Equal(t, []string{"3|-7"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(-4)
	assert.Equal(t, []string{"-4|5|6"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(5)
	assert.Equal(t, []string{"-4|5|6"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(6)
	assert.Equal(t, []string{"-4|5|6"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(4)
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(0)
	assert.Equal(t, []string{"?"}, x.tape)

}

// Test the multiple match syntax for integers. Also tests matching
// whitespace-only strings.
func TestStringMultiMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Multi()
	sm.Onstring("$10")
	assert.Equal(t, []string{"symbols"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("12.5%")
	assert.Equal(t, []string{"symbols"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("@#*!")
	assert.Equal(t, []string{"symbols"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring(" ")
	assert.Equal(t, []string{"whitespace"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("  ")
	assert.Equal(t, []string{"whitespace"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("\t")
	assert.Equal(t, []string{"whitespace"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("\n")
	assert.Equal(t, []string{"whitespace"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("10")
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("#")
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("   ")
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("")
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
}

/* Test nested integer matching. */
func TestIntegerNestedMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Nested()
	sm.OnInt(1)
	assert.Equal(t, []string{"1-3", "1"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(2)
	assert.Equal(t, []string{"1-3", "2"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(3)
	assert.Equal(t, []string{"1-3", "3"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(4)
	assert.Equal(t, []string{"4-5", "4"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(5)
	assert.Equal(t, []string{"4-5", "5"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(10)
	assert.Equal(t, []string{"too big"}, x.tape)
	x.tape = x.tape[:0]
	sm.OnInt(0)
	assert.Equal(t, []string{"too small"}, x.tape)

}

/* Test nested string matching. */
func TestStringNestedMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Nested()
	sm.Onstring("hello")
	assert.Equal(t, []string{"greeting", "English"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("hola")
	assert.Equal(t, []string{"greeting", "Spanish"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("bonjour")
	assert.Equal(t, []string{"greeting", "French"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("goodbye")
	assert.Equal(t, []string{"farewell", "English"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("adios")
	assert.Equal(t, []string{"farewell", "Spanish"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("au revoir")
	assert.Equal(t, []string{"farewell", "French"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("hallo")
	assert.Equal(t, []string{"?"}, x.tape)
	x.tape = x.tape[:0]
	sm.Onstring("ciao")
	assert.Equal(t, []string{"?"}, x.tape)
}

/* Test hierarchical integer matching. */
func TestIntegerHierarchicalMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Child()
	sm.OnInt(0)
	assert.Equal(t, MatchState_Final, x._compartment_.State)
	assert.Empty(t, x.tape)

	sm = NewMatch()
	x = sm.(*matchStruct)
	sm.Child()
	sm.OnInt(4)
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"4"}, x.tape)

	x.tape = x.tape[:0]
	sm.OnInt(5)
	assert.Equal(t, MatchState_Final, x._compartment_.State)
	assert.Equal(t, []string{"5"}, x.tape)

	sm = NewMatch()
	x = sm.(*matchStruct)
	sm.Child()
	sm.OnInt(5)
	assert.Equal(t, MatchState_Final, x._compartment_.State)
	assert.Equal(t, []string{"5"}, x.tape)

	sm = NewMatch()
	x = sm.(*matchStruct)
	sm.Child()
	sm.OnInt(3)
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"3", "?"}, x.tape)

	x.tape = x.tape[:0]
	sm.OnInt(42)
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"42 in child", "42"}, x.tape)

	x.tape = x.tape[:0]
	sm.OnInt(-200)
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"no match in child", "-200"}, x.tape)

	x.tape = x.tape[:0]
	sm.OnInt(100)
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"no match in child", "?"}, x.tape)
}

/* Test hierarchical string matching. */
func TestStringHierarchicalMatch(t *testing.T) {
	sm := NewMatch()
	x := sm.(*matchStruct)
	sm.Child()
	sm.Onstring("goodbye")
	assert.Equal(t, MatchState_Final, x._compartment_.State)
	assert.Empty(t, x.tape)

	sm = NewMatch()
	x = sm.(*matchStruct)
	sm.Child()
	sm.Onstring("hello")
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"hello in child", "hello"}, x.tape)

	x.tape = x.tape[:0]
	sm.Onstring("Testing 1, 2, 3...")
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"testing in child"}, x.tape)

	x.tape = x.tape[:0]
	sm.Onstring("$10!")
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"no match in child", "money"}, x.tape)

	x.tape = x.tape[:0]
	sm.Onstring("testing 1, 2, 3...")
	assert.Equal(t, MatchState_ChildMatch, x._compartment_.State)
	assert.Equal(t, []string{"no match in child", "?"}, x.tape)
}
