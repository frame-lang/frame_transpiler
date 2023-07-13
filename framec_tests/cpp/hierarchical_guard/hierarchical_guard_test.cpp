#include <gtest/gtest.h>
#include <vector>
#include "hierarchical_guard.cpp" 


class HierarchicalGuardController;

class HierarchicalGuardController : public HierarchicalGuard {
public:
    HierarchicalGuardController() : HierarchicalGuard() {}
};


class HierarchicalGuardTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new HierarchicalGuardController();
    }

    void TearDown() override {
        delete sm;
    }
    HierarchicalGuardController* sm;
};

TEST_F(HierarchicalGuardTest, TestPropogateToParent) {
    HierarchicalGuardController sm;
    sm.A(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "2");
    sm.A(20);
    ASSERT_EQ(sm.state_info(), "4");
    std::vector<std::string> expected = {"S0.A"};
    ASSERT_EQ(sm.tape, expected);

    sm = HierarchicalGuardController();
    sm.A(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "2");
    sm.A(-5);
    ASSERT_EQ(sm.state_info(), "2");
    std::vector<std::string> expected1 = {"S0.A", "S.A"};
    ASSERT_EQ(sm.tape, expected1);

    sm = HierarchicalGuardController();
    sm.A(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "2");
    sm.B(-5);
    ASSERT_EQ(sm.state_info(), "3");
    std::vector<std::string> expected2 = {"S0.B"};
    ASSERT_EQ(sm.tape, expected2);

    sm = HierarchicalGuardController();
    sm.A(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "2");
    sm.B(5);
    ASSERT_EQ(sm.state_info(), "4");
}

TEST_F(HierarchicalGuardTest, TestPropogateMultipleLevels) {
    HierarchicalGuardController sm;
    sm.B(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "4");
    sm.A(7);
    ASSERT_EQ(sm.state_info(), "5");
    std::vector<std::string> expected = {"S2.A", "S1.A"};
    ASSERT_EQ(sm.tape, expected);

    sm = HierarchicalGuardController();
    sm.B(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "4");
    sm.A(-5);
    ASSERT_EQ(sm.state_info(), "2");
    std::vector<std::string> expected1 = {"S2.A", "S1.A", "S0.A", "S.A"};
    ASSERT_EQ(sm.tape, expected1);
}

TEST_F(HierarchicalGuardTest, TestPropogateSkipsLevels) {
    HierarchicalGuardController sm;
    sm.B(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "4");
    sm.B(-5);
    ASSERT_EQ(sm.state_info(), "3");
    std::vector<std::string> expected = {"S2.B", "S0.B"};
    ASSERT_EQ(sm.tape, expected);

    sm = HierarchicalGuardController();
    sm.B(0);
    sm.tape.clear();
    ASSERT_EQ(sm.state_info(), "4");
    sm.B(5);
    ASSERT_EQ(sm.state_info(), "4");
    std::vector<std::string> expected1 = {"S2.B", "S0.B", "S.B"};
    ASSERT_EQ(sm.tape, expected1);
}

TEST_F(HierarchicalGuardTest, TestConditionalReturns) {
    sm->B(20);
    sm->tape.clear();
    ASSERT_EQ(sm->state_info(), "5");
    sm->A(5);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected = {"S3.A", "stop"};
    ASSERT_EQ(sm->tape, expected);

    *sm = HierarchicalGuardController();
    sm->B(20);
    sm->tape.clear();
    ASSERT_EQ(sm->state_info(), "5");
    sm->A(-5);
    ASSERT_EQ(sm->state_info(), "2");
    std::vector<std::string> expected1 = {"S3.A", "continue", "S.A"};
    ASSERT_EQ(sm->tape, expected1);

    *sm = HierarchicalGuardController();
    sm->B(20);
    sm->tape.clear();
    ASSERT_EQ(sm->state_info(), "5");
    sm->B(-5);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected2 = {"S3.B", "stop"};
    ASSERT_EQ(sm->tape, expected2);

    *sm = HierarchicalGuardController();
    sm->B(20);
    sm->tape.clear();
    ASSERT_EQ(sm->state_info(), "5");
    sm->B(5);
    ASSERT_EQ(sm->state_info(), "4");
    std::vector<std::string> expected3 = {"S3.B", "continue", "S.B"};
    ASSERT_EQ(sm->tape, expected3);
}

int main() {
    ::testing::InitGoogleTest();
    return RUN_ALL_TESTS();
}