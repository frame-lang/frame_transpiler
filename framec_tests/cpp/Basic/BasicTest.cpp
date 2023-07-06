#include <iostream>
#include <vector>
#include <cassert>
#include "Basic.h"
using namespace std;


class BasicController : public Basic {
public:
    BasicController() : Basic() {}

    void entered_do(std::string msg) {
        entry_log.push_back(msg);
    }

    void left_do(std::string msg) {
        exit_log.push_back(msg);
    }
    
};

class Basic_test {
public:
    void TestIntialEnterEvent() {
        BasicController sm;
        std::vector<std::string> expected;
        expected.push_back("S0");
        assert(expected == sm.entry_log);
    }

    void TestTransitionEnterEvents() {
        BasicController sm;
        sm.entry_log.clear();
        sm.A();
        sm.B();
        std::vector<std::string> expected;
        expected.push_back("S1");
        expected.push_back("S0");
        assert(expected == sm.entry_log);
    }

    void TestTransitionExitEvents() {
        BasicController sm;
        sm.A();
        sm.B();
        std::vector<std::string> expected;
        expected.push_back("S0");
        expected.push_back("S1");
        assert(expected == sm.exit_log);
    }

    void TestCurrentState() {
        BasicController sm;
        assert(sm.state_info() == "0");
        sm.A();
        assert(sm.state_info() == "1");
        sm.B();
        assert(sm.state_info() == "0");
    }
};

int main() {
    Basic_test basicTest;
    basicTest.TestIntialEnterEvent();
    basicTest.TestTransitionEnterEvents();
    basicTest.TestTransitionExitEvents();
    basicTest.TestCurrentState();

    return 0;
}
