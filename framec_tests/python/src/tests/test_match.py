from match.match import Match

class MatchController(Match):

    def log_do(self,  msg: str):
        self.tape.append(msg)

def return_state_name(state: str) -> str:

    return f'__match_state_{state}'


class TestMatch:
    """
    Test integer matching and string matching constructs. We're not testing the
    floating point number matching for now since checking floats for equality
    is usually not a good idea.
    """

    def test_empty_string(self):
        """
        Test matching the empty string.
        TODO: Matching the empty string currently only works in multi-string
        patterns. The pattern `//` == which should match only the empty string,
        instead produces a parse error.
        """

        sm = MatchController()
        sm.Empty()
        sm.Onstring("")
        assert sm.tape == ["empty"]
        sm.tape.clear()
        sm.Onstring("hi")
        assert sm.tape == ["?"]

    def test_integer_match(self):
        """
        Test simple integer matching.
        """
        sm = MatchController()
        sm.Simple()
        sm.OnInt(0)
        assert sm.tape == ["0"]
        sm.tape.clear()
        sm.OnInt(42)
        assert sm.tape == ["42"]
        sm.tape.clear()
        sm.OnInt(-200)
        assert sm.tape == ["-200"]
        sm.tape.clear()
    
    def test_string_match(self):
        """
        Test simple string matching.

        TODO: Testing revealed some limitations:
        * Frame does not support UTF-8 graphemes larger than 1 byte == so we're
            restricted to ASCII.
        * Frame does not have a way to match the '/' or '|' characters,
            which are part of the matching syntax.
        """

        sm = MatchController()
        sm.Simple()
        sm.Onstring("hello")
        assert sm.tape == ["hello"]
        sm.tape.clear()
        sm.Onstring("$10!")
        assert sm.tape == ["money"]
        sm.tape.clear()
        sm.Onstring("missing")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("Testing")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("")
        assert sm.tape == ["?"]

    def test_integer_multi_match(self):
        """
        Test the multiple match syntax for integers.
        """

        sm = MatchController()
        sm.Multi()
        sm.OnInt(3)
        assert sm.tape == ["3|-7"]
        sm.tape.clear()
        sm.OnInt(-7)
        assert sm.tape == ["3|-7"]
        sm.tape.clear()
        sm.OnInt(-4)
        assert sm.tape == ["-4|5|6"]
        sm.tape.clear()
        sm.OnInt(5)
        assert sm.tape == ["-4|5|6"]
        sm.tape.clear()
        sm.OnInt(6)
        assert sm.tape == ["-4|5|6"]
        sm.tape.clear()
        sm.OnInt(4)
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.OnInt(0)
        assert sm.tape == ["?"]

    def test_string_multi_match(self):
        """
        Test the multiple match syntax for integers. Also tests matching
        whitespace-only strings.
        """

        sm = MatchController()
        sm.Multi()
        sm.Onstring("$10")
        assert sm.tape == ["symbols"]
        sm.tape.clear()
        sm.Onstring("12.5%")
        assert sm.tape == ["symbols"]
        sm.tape.clear()
        sm.Onstring("@#*!")
        assert sm.tape == ["symbols"]
        sm.tape.clear()
        sm.Onstring(" ")
        assert sm.tape == ["whitespace"]
        sm.tape.clear()
        sm.Onstring("  ")
        assert sm.tape == ["whitespace"]
        sm.tape.clear()
        sm.Onstring("\t")
        assert sm.tape == ["whitespace"]
        sm.tape.clear()
        sm.Onstring("\n")
        assert sm.tape == ["whitespace"]
        sm.tape.clear()
        sm.Onstring("10")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("#")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("   ")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("")
        assert sm.tape == ["?"]
        sm.tape.clear()
    
    def test_integer_nested_match(self):
        """
        Test nested integer matching.
        """
        sm = MatchController()
        sm.Nested()
        sm.OnInt(1)
        assert sm.tape == ["1-3", "1"]
        sm.tape.clear()
        sm.OnInt(2)
        assert sm.tape == ["1-3", "2"]
        sm.tape.clear()
        sm.OnInt(3)
        assert sm.tape == ["1-3", "3"]
        sm.tape.clear()
        sm.OnInt(4)
        assert sm.tape == ["4-5", "4"]
        sm.tape.clear()
        sm.OnInt(5)
        assert sm.tape == ["4-5", "5"]
        sm.tape.clear()
        sm.OnInt(10)
        assert sm.tape == ["too big"]
        sm.tape.clear()
        sm.OnInt(0)
        assert sm.tape == ["too small"]

    def test_string_nested_match(self):
        """Test nested string matching."""
        sm = MatchController()
        sm.Nested()
        sm.Onstring("hello")
        assert sm.tape == ["greeting", "English"]
        sm.tape.clear()
        sm.Onstring("hola")
        assert sm.tape == ["greeting", "Spanish"]
        sm.tape.clear()
        sm.Onstring("bonjour")
        assert sm.tape == ["greeting", "French"]
        sm.tape.clear()
        sm.Onstring("goodbye")
        assert sm.tape == ["farewell", "English"]
        sm.tape.clear()
        sm.Onstring("adios")
        assert sm.tape == ["farewell", "Spanish"]
        sm.tape.clear()
        sm.Onstring("au revoir")
        assert sm.tape == ["farewell", "French"]
        sm.tape.clear()
        sm.Onstring("hallo")
        assert sm.tape == ["?"]
        sm.tape.clear()
        sm.Onstring("ciao")
        assert sm.tape == ["?"]
    
    def test_integer_hierarchical_match(self):
        """Test hierarchical integer matching."""
        sm = MatchController()
        sm.Child()
        sm.OnInt(0)
        assert sm.state_info() == return_state_name("Final")
        assert len(sm.tape) == 0

        sm = MatchController()
        sm.Child()
        sm.OnInt(4)
        assert sm.state_info() ==  return_state_name("ChildMatch")
        assert sm.tape ==  ["4"]

        sm.tape.clear()
        sm.OnInt(5)
        assert sm.state_info() ==  return_state_name("Final")
        assert sm.tape ==  ["5"]

        sm = MatchController()
        sm.Child()
        sm.OnInt(5)
        assert sm.state_info() ==  return_state_name("Final")
        assert sm.tape ==  ["5"]

        sm = MatchController()
        sm.Child()
        sm.OnInt(3)
        assert sm.state_info() ==  return_state_name("ChildMatch")
        assert sm.tape ==  ["3", "?"]

        sm.tape.clear()
        sm.OnInt(42)
        assert sm.state_info() ==  return_state_name("ChildMatch")
        assert sm.tape ==  ["42 in child", "42"]

        sm.tape.clear()
        sm.OnInt(-200)
        assert sm.state_info() ==  return_state_name("ChildMatch")
        assert sm.tape ==  ["no match in child", "-200"]

        sm.tape.clear()
        sm.OnInt(100)
        assert sm.state_info() ==  return_state_name("ChildMatch")
        assert sm.tape ==  ["no match in child", "?"]
    

    def test_string_hierarchical_match(self):
        """Test hierarchical string matching."""     
        sm = MatchController()
        sm.Child()
        sm.Onstring("goodbye")
        assert sm.state_info() == return_state_name("Final")
        assert len(sm.tape) == 0

        sm = MatchController()
        sm.Child()
        sm.Onstring("hello")
        assert sm.state_info() == return_state_name("ChildMatch")
        assert sm.tape == ["hello in child", "hello"]

        sm.tape.clear()
        sm.Onstring("Testing 1, 2, 3...")
        assert sm.state_info() == return_state_name("ChildMatch")
        assert sm.tape == ["testing in child"]

        sm.tape.clear()
        sm.Onstring("$10!")
        assert sm.state_info() == return_state_name("ChildMatch")
        assert sm.tape == ["no match in child", "money"]

        sm.tape.clear()
        sm.Onstring("testing 1, 2, 3...")
        assert sm.state_info() == return_state_name("ChildMatch")
        assert sm.tape == ["no match in child", "?"]
    
