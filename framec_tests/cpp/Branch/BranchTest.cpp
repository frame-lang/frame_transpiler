#include <cassert>
#include <vector>
#include <string>
#include "branch.h"

class BranchController : public Branch {
public:
    BranchController() : Branch() {}

protected:
    void log_do(const std::string& msg) {
        tape.push_back(msg);
    }
};

void TestSimpleIfBool() {
    BranchController sm;
    sm.A();
    sm.OnBool(true);

    assert(sm.state_info() == "7");
    std::vector<std::string> expected = {"then 1", "then 2"};
    assert(sm.state_info() == "7");
    assert(sm.tape == expected);

    sm = BranchController();
    sm.A();
    sm.OnBool(false);

    assert(sm.state_info() == "8");
    std::vector<std::string> expected1 = {"else 1", "else 2"};
    assert(sm.state_info() == "8");
    assert(sm.tape == expected1);
}

void TestSimpleIfInt() {
    BranchController sm;
    sm.A();
    sm.OnInt(7);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected = {"> 5", "< 10", "== 7"};
    assert(sm.tape == expected);

    sm = BranchController();
    sm.A();
    sm.OnInt(-3);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected1 = {"<= 5", "< 10", "!= 7"};
    assert(sm.tape == expected1);

    sm = BranchController();
    sm.A();
    sm.OnInt(12);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected2 = {"> 5", ">= 10", "!= 7"};
    assert(sm.tape == expected2);
}

void TestNegatedIfBool() {
    BranchController sm;
    sm.B();
    sm.OnBool(true);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected = {"else 1", "else 2"};
    assert(sm.tape == expected);

    sm = BranchController();
    sm.B();
    sm.OnBool(false);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected1 = {"then 1", "then 2"};
    assert(sm.tape == expected1);
}

void TestNegatedIfInt() {
    BranchController sm;
    sm.B();
    sm.OnInt(7);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected = {">= 5", "<= 10", "== 7"};
    assert(sm.tape == expected);

    sm = BranchController();
    sm.B();
    sm.OnInt(5);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected1 = {">= 5", "<= 10", "!= 7"};
    assert(sm.tape == expected1);

    sm = BranchController();
    sm.B();
    sm.OnInt(10);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected2 = {">= 5", "<= 10", "!= 7"};
    assert(sm.tape == expected2);

    sm = BranchController();
    sm.B();
    sm.OnInt(0);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected3 = {"< 5", "<= 10", "!= 7"};
    assert(sm.tape == expected3);

    sm = BranchController();
    sm.B();
    sm.OnInt(100);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected4 = {">= 5", "> 10", "!= 7"};
    assert(sm.tape == expected4);
}

void OperatorPrecedenceTest() {
    BranchController sm;
    sm.C();
    sm.OnInt(0);
    std::vector<std::string> expected = {"then 1", "else 2", "then 3", "then 4"};
    assert(sm.tape == expected);
    sm.tape.clear();
    sm.OnInt(7);
    std::vector<std::string> expected1 = {"else 1", "then 2", "else 3", "then 4"};
    assert(sm.tape == expected1);
    sm.tape.clear();
    sm.OnInt(-3);
    std::vector<std::string> expected2 = {"then 1", "else 2", "else 3", "else 4"};
    assert(sm.tape == expected2);
    sm.tape.clear();
    sm.OnInt(12);
    std::vector<std::string> expected3 = {"else 1", "else 2", "then 3", "else 4"};
    assert(sm.tape == expected3);
    sm.tape.clear();
}

void NestedIfTest() {
    BranchController sm;
    sm.D();
    sm.OnInt(50);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected = {"> 0", "< 100"};
    assert(sm.tape == expected);

    sm = BranchController();
    sm.D();
    sm.OnInt(200);
    assert(sm.state_info() == "4");
    std::vector<std::string> expected1 = {"> 0", ">= 100"};
    assert(sm.tape == expected1);

    sm = BranchController();
    sm.D();
    sm.OnInt(-5);
    assert(sm.state_info() == "4");
    std::vector<std::string> expected2 = {"<= 0", "> -10"};
    assert(sm.tape == expected2);

    sm = BranchController();
    sm.D();
    sm.OnInt(-10);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected3 = {"<= 0", "<= -10"};
    assert(sm.tape == expected3);
}

void TestGuardedTransition() {
    BranchController sm;
    sm.E();
    sm.OnInt(5);
    assert(sm.state_info() == "9");
    std::vector<std::string> expected = {"-> $F3"};
    assert(sm.tape == expected);

    sm = BranchController();
    sm.E();
    sm.OnInt(15);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected1 = {"-> $F2"};
    assert(sm.tape == expected1);

    sm = BranchController();
    sm.E();
    sm.OnInt(115);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected2 = {"-> $F1"};
    assert(sm.tape == expected2);
}

void TestNestedGuardedTransition() {
    BranchController sm;
    sm.F();
    sm.OnInt(5);
    assert(sm.state_info() == "9");
    std::vector<std::string> expected = {"-> $F3"};
    assert(sm.tape == expected);
    sm = BranchController();
    sm.F();
    sm.OnInt(15);
    assert(sm.state_info() == "8");
    std::vector<std::string> expected1 = {"-> $F2"};
    assert(sm.tape == expected1);

    sm = BranchController();
    sm.F();
    sm.OnInt(65);
    assert(sm.state_info() == "9");
    std::vector<std::string> expected2 = {"-> $F3"};
    assert(sm.tape == expected2);

    sm = BranchController();
    sm.F();
    sm.OnInt(115);
    assert(sm.state_info() == "7");
    std::vector<std::string> expected3 = {"-> $F1"};
    assert(sm.tape == expected3);
}

int main() {
    TestSimpleIfBool();
    TestSimpleIfInt();
    TestNegatedIfBool();
    TestNegatedIfInt();
    OperatorPrecedenceTest();
    NestedIfTest();
    TestGuardedTransition();
    TestNestedGuardedTransition();

    return 0;
}


