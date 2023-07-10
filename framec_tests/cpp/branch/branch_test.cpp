#include "branch.cpp"
#include <iostream>
#include <vector>
#include <cassert>

using namespace std;

class BranchController : public Branch {
public:
    BranchController() : Branch() {}

    void log_do(string msg) {
        tape.push_back(msg);
    }

    vector<string> tape;
};

void TestSimpleIfBool() {
    BranchController sm;
    sm.A();
    sm.OnBool(true);

    assert(sm.state_info() == "7");
    vector<string> expected = {"then 1", "then 2"};

    assert(sm.state_info() == "7");
    assert(sm.tape == expected);

    sm = BranchController();
    sm.A();
    sm.OnBool(false);

    assert(sm.state_info() == "8");
    vector<string> expected1 = {"else 1", "else 2"};

    assert(sm.state_info() == "8");
    assert(sm.tape == expected1);
}

void TestSimpleIfInit() {
    BranchController sm;
    sm.A();
    sm.OnInt(7);

    assert(sm.state_info() == "7");
    vector<string> arr = {"> 5", "< 10", "== 7"};

    assert(sm.tape == arr);

    sm = BranchController();
    sm.A();
    sm.OnInt(-3);

    assert(sm.state_info() == "8");
    vector<string> arr1 = {"<= 5", "< 10", "!= 7"};

    assert(sm.tape == arr1);

    sm = BranchController();
    sm.A();
    sm.OnInt(12);

    assert(sm.state_info() == "8");
    vector<string> arr2 = {"> 5", ">= 10", "!= 7"};

    assert(sm.tape == arr2);
}

void TestNegatedIfBool() {
    BranchController sm;
    sm.B();
    sm.OnBool(true);

    assert(sm.state_info() == "8");
    vector<string> expected = {"else 1", "else 2"};

    assert(sm.tape == expected);

    sm = BranchController();
    sm.B();
    sm.OnBool(false);

    assert(sm.state_info() == "7");
    vector<string> expected1 = {"then 1", "then 2"};

    assert(sm.tape == expected1);
}

void TestNegatedIfInt() {
    BranchController sm;
    sm.B();
    sm.OnInt(7);

    assert(sm.state_info() == "7");
    vector<string> expected = {">= 5", "<= 10", "== 7"};

    assert(sm.tape == expected);

    sm = BranchController();
    sm.B();
    sm.OnInt(5);

    assert(sm.state_info() == "8");
    vector<string> expected1 = {">= 5", "<= 10", "!= 7"};

    assert(sm.tape == expected1);

    sm = BranchController();
    sm.B();
    sm.OnInt(10);

    assert(sm.state_info() == "8");
    vector<string> expected2 = {">= 5", "<= 10", "!= 7"};

    assert(sm.tape == expected2);

    sm = BranchController();
    sm.B();
    sm.OnInt(0);

    assert(sm.state_info() == "8");
    vector<string> expected3 = {"< 5", "<= 10", "!= 7"};

    assert(sm.tape == expected3);

    sm = BranchController();
    sm.B();
    sm.OnInt(100);

    assert(sm.state_info() == "8");
    vector<string> expected4 = {">= 5", "> 10", "!= 7"};

    assert(sm.tape == expected4);
}



