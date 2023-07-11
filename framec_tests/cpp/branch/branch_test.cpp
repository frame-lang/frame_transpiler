#include "../gtest/gtest.h"
#include "branch.cpp"
#include <iostream>
#include <vector>
#include <string>


class BranchController; 

class BranchController : public Branch {
public:
    BranchController() : Branch() {}
};

class BranchControllerTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new BranchController();
    }

    void TearDown() override {
        delete sm;
    }

    BranchController* sm;
};

TEST_F(BranchControllerTest, TestSimpleIfBool) {
    BranchController sm;
    sm.A();
    sm.OnBool(true);

    ASSERT_EQ("7", sm.state_info());
    std::vector<std::string> expected = {"then 1", "then 2"};

    ASSERT_EQ(sm.state_info(), "7");
    ASSERT_EQ(sm.tape, expected);

    sm = BranchController();
    sm.A();
    sm.OnBool(false);

    ASSERT_EQ("8" ,sm.state_info());
    std::vector<std::string> expected1 = {"else 1", "else 2"};
    ASSERT_EQ(sm.state_info(), "8");
    ASSERT_EQ(expected1, sm.tape);
}

TEST_F(BranchControllerTest, TestSimpleIfInit) {
    sm->A();
    sm->OnInt(7);
    ASSERT_EQ(sm->state_info(), "7");
    std::vector<std::string> arr = {"> 5", "< 10", "== 7"};
    ASSERT_EQ(sm->tape, arr);

    *sm = BranchController();
    sm->A();
    sm->OnInt(-3);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> arr1 = {"<= 5", "< 10", "!= 7"};
    ASSERT_EQ(sm->tape, arr1);

    *sm = BranchController();
    sm->A();
    sm->OnInt(12);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> arr2 = {"> 5", ">= 10", "!= 7"};
    ASSERT_EQ(sm->tape, arr2);
}

TEST_F(BranchControllerTest, TestNegatedIfBool) {
    sm->B();
    sm->OnBool(true);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected = {"else 1", "else 2"};
    ASSERT_EQ(sm->tape, expected);

    *sm = BranchController();
    sm->B();
    sm->OnBool(false);
    ASSERT_EQ(sm->state_info(), "7");
    std::vector<std::string> expected1 = {"then 1", "then 2"};
    ASSERT_EQ(sm->tape, expected1);
}

TEST_F(BranchControllerTest, TestNegatedIfInt) {
    sm->B();
    sm->OnInt(7);
    ASSERT_EQ(sm->state_info(), "7");
    std::vector<std::string> expected = {">= 5", "<= 10", "== 7"};
    ASSERT_EQ(sm->tape, expected);

    *sm = BranchController();
    sm->B();
    sm->OnInt(5);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected1 = {">= 5", "<= 10", "!= 7"};
    ASSERT_EQ(sm->tape, expected1);

    *sm = BranchController();
    sm->B();
    sm->OnInt(10);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected2 = {">= 5", "<= 10", "!= 7"};
    ASSERT_EQ(sm->tape, expected2);

    *sm = BranchController();
    sm->B();
    sm->OnInt(0);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected3 = {"< 5", "<= 10", "!= 7"};
    ASSERT_EQ(sm->tape, expected3);

    *sm = BranchController();
    sm->B();
    sm->OnInt(100);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected4 = {">= 5", "> 10", "!= 7"};
    ASSERT_EQ(sm->tape, expected4);
}

TEST_F(BranchControllerTest, TestOperatorPrecedence) {
    sm->C();
    sm->OnInt(0);
    std::vector<std::string> expected = {"then 1", "else 2", "then 3", "then 4"};
    ASSERT_EQ(expected, sm->tape);
    sm->tape.clear();

    sm->OnInt(7);
    std::vector<std::string> expected1 = {"else 1", "then 2", "else 3", "then 4"};
    ASSERT_EQ(expected1, sm->tape);
    sm->tape.clear();

    sm->OnInt(-3);
    std::vector<std::string> expected2 = {"then 1", "else 2", "else 3", "else 4"};
    ASSERT_EQ(expected2, sm->tape);
    sm->tape.clear();

    sm->OnInt(12);
    std::vector<std::string> expected3 = {"else 1", "else 2", "then 3", "else 4"};
    ASSERT_EQ(expected3, sm->tape);
    sm->tape.clear();
}

TEST_F(BranchControllerTest, TestNestedIf) {
    sm->D();
    sm->OnInt(50);
    ASSERT_EQ(sm->state_info(), "7");
    std::vector<std::string> expected = {"> 0", "< 100"};
    ASSERT_EQ(sm->tape, expected);

    *sm = BranchController();
    sm->D();
    sm->OnInt(200);
    ASSERT_EQ(sm->state_info(), "4");
    std::vector<std::string> expected1 = {"> 0", ">= 100"};
    ASSERT_EQ(sm->tape, expected1);

    *sm = BranchController();
    sm->D();
    sm->OnInt(-5);
    ASSERT_EQ(sm->state_info(), "4");
    std::vector<std::string> expected2 = {"<= 0", "> -10"};
    ASSERT_EQ(sm->tape, expected2);

    *sm = BranchController();
    sm->D();
    sm->OnInt(-10);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected3 = {"<= 0", "<= -10"};
    ASSERT_EQ(sm->tape, expected3);

    *sm = BranchController();
}

TEST_F(BranchControllerTest, TestGuardedTransition) {
    sm->E();
    sm->OnInt(5);
    EXPECT_EQ(sm->state_info(), "9");
    std::vector<std::string> expected = {"-> $F3"};
    EXPECT_EQ(sm->tape, expected);

    *sm = BranchController();
    sm->E();
    sm->OnInt(15);
    EXPECT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected1 = {"-> $F2"};
    EXPECT_EQ(sm->tape, expected1);

    *sm = BranchController();
    sm->E();
    sm->OnInt(115);
    EXPECT_EQ(sm->state_info(), "7");
    std::vector<std::string> expected2 = {"-> $F1"};
    EXPECT_EQ(sm->tape, expected2);
}

TEST_F(BranchControllerTest, TestNestedGuardedTransition) {
    sm->F();
    sm->OnInt(5);
    ASSERT_EQ(sm->state_info(), "9");
    std::vector<std::string> expected = {"-> $F3"};
    ASSERT_EQ(sm->tape, expected);

    *sm = BranchController();
    sm->tape.clear();
    sm->F();
    sm->OnInt(15);
    ASSERT_EQ(sm->state_info(), "8");
    std::vector<std::string> expected1 = {"-> $F2"};
    ASSERT_EQ(sm->tape, expected1);

    *sm = BranchController();
    sm->F();
    sm->OnInt(65);
    ASSERT_EQ(sm->state_info(), "9");
    std::vector<std::string> expected2 = {"-> $F3"};
    ASSERT_EQ(sm->tape, expected2);

    *sm = BranchController();
    sm->F();
    sm->OnInt(115);
    ASSERT_EQ(sm->state_info(), "7");
    std::vector<std::string> expected3 = {"-> $F1"};
    ASSERT_EQ(sm->tape, expected3);
}


int main(int argc, char** argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}



