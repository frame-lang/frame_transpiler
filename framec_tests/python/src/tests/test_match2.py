from match2.match2 import MatchTests


def test_match2():
    sm = MatchTests()
    assert sm.tape == ['Matched PEACH',
                       'Matched pear',
                       'Matched Banana or Watermelon',
                       'no enum match',
                       'matched !@#$%^&*()',
                       'matched a|b',
                       'matched empty string',
                       'matched null',
                       'matched a|b',
                       'no string match',
                       'Matched 1001.5 or 0.12',
                       'Matched 1001.5 or 0.12',
                       'Matched .5',
                       'Matched .111',
                       'no number match']
