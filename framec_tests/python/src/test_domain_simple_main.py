import sys
sys.path.append('/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src')
from test_domain_simple import DomainTest

def main():
    test = DomainTest()
    test.run_test()
    return

if __name__ == '__main__':
    main()