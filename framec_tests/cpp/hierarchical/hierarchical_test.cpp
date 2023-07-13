#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "hierarchical.cpp"

class HierarchicalController;

class HierarchicalController : public Hierarchical {
public:
    HierarchicalController() : Hierarchical() {}
};


class HierarchicalTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new HierarchicalController();
    }

    void TearDown() override {
        delete sm;
    }

    HierarchicalController* sm;
};

TEST_F(HierarchicalTest, TestEnterContinue) {
    HierarchicalController sm;
    sm.enters.clear();
    sm.A();

    std::vector<std::string> expected1{"S0", "S"};
    EXPECT_EQ(sm.enters, expected1);

    sm.enters.clear();
    sm.C();

    std::vector<std::string> expected2{"S2", "S0", "S"};
    EXPECT_EQ(sm.enters, expected2);
}

TEST_F(HierarchicalTest, TestExitContinue) {
    sm->A();
    sm->exits.clear();
    sm->C();
    std::vector<std::string> expected = {"S0", "S"};
    ASSERT_EQ(sm->exits, expected);

    sm->exits.clear();
    sm->A();
    std::vector<std::string> expected1 = {"S2", "S0", "S"};
    ASSERT_EQ(sm->exits, expected1);
}

TEST_F(HierarchicalTest, TestEnterReturn) {
    HierarchicalController sm;
    sm.enters.clear();
    sm.B();

    std::vector<std::string> expected1{"S1"};
    EXPECT_EQ(sm.enters, expected1);

    sm = HierarchicalController();
    sm.A();
    sm.A();
    EXPECT_EQ(sm.state_info(), "6");

    sm.enters.clear();
    sm.C();

    std::vector<std::string> expected2{"S3", "S1"};
    EXPECT_EQ(sm.enters, expected2);
}

TEST_F(HierarchicalTest, TestExitReturn) {
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    sm->exits.clear();
    sm->A();
    std::vector<std::string> expected = {"S1"};
    ASSERT_EQ(sm->exits, expected);

    sm = new HierarchicalController();
    sm->A();
    sm->A();
    sm->C();
    ASSERT_EQ(sm->state_info(), "5");
    sm->exits.clear();
    sm->B();
    std::vector<std::string> expected1 = {"S3", "S1"};
    ASSERT_EQ(sm->exits, expected1);
}

TEST_F(HierarchicalTest, TestCurrentStateSimple) {
    ASSERT_EQ(sm->state_info(), "1");
    sm->A();
    ASSERT_EQ(sm->state_info(), "2");
    sm->A();
    ASSERT_EQ(sm->state_info(), "6");
    sm->C();
    ASSERT_EQ(sm->state_info(), "5");
    sm->B();
    ASSERT_EQ(sm->state_info(), "4");
}

TEST_F(HierarchicalTest, TestCurrentStateWithPropagation) {
    ASSERT_EQ(sm->state_info(), "1");
    sm->A();
    ASSERT_EQ(sm->state_info(), "2");
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    sm->C();
    ASSERT_EQ(sm->state_info(), "3");
    sm->A();
    ASSERT_EQ(sm->state_info(), "2");
    sm->C();
    ASSERT_EQ(sm->state_info(), "4");
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
}

TEST_F(HierarchicalTest, TestOverrideParentHandler) {
    sm->A();
    sm->tape.clear();
    sm->A();
    ASSERT_EQ(sm->state_info(), "6");
    std::vector<std::string> expected = {"S0.A"};
    ASSERT_EQ(sm->tape, expected);

    sm->C();
    sm->tape.clear();
    sm->B();
    ASSERT_EQ(sm->state_info(), "4");
    std::vector<std::string> expected1 = {"S3.B"};
    ASSERT_EQ(sm->tape, expected1);
}

TEST_F(HierarchicalTest, TestBeforeParentHandle) {
    sm->A();
    sm->tape.clear();
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    std::vector<std::string> expected = {"S0.B", "S.B"};
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->exits.clear();
    sm->enters.clear();
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    std::vector<std::string> expected1 = {"S1.B", "S.B"};
    ASSERT_EQ(sm->tape, expected1);
    std::vector<std::string> expected2 = {"S1"};
    ASSERT_EQ(sm->exits, expected2);
    std::vector<std::string> expected3 = {"S1"};
    ASSERT_EQ(sm->enters, expected3);

    sm = new HierarchicalController();
    sm->A();
    sm->C();
    ASSERT_EQ(sm->state_info(), "4");
    sm->tape.clear();
    sm->exits.clear();
    sm->enters.clear();
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    std::vector<std::string> expected4 = {"S2.B", "S0.B", "S.B"};
    ASSERT_EQ(sm->tape, expected4);
    std::vector<std::string> expected5 = {"S2", "S0", "S"};
    ASSERT_EQ(sm->exits, expected5);
    std::vector<std::string> expected6 = {"S1"};
    ASSERT_EQ(sm->enters, expected6);
}

TEST_F(HierarchicalTest, TestDeferToParentHandler) {
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    sm->tape.clear();
    sm->A();
    ASSERT_EQ(sm->state_info(), "2");
    std::vector<std::string> expected = {"S.A"};
    ASSERT_EQ(sm->tape, expected);

    sm->A();
    sm->C();
    ASSERT_EQ(sm->state_info(), "5");
    sm->tape.clear();
    sm->A();
    ASSERT_EQ(sm->state_info(), "2");
    std::vector<std::string> expected1 = {"S.A"};
    ASSERT_EQ(sm->tape, expected1);
}

TEST_F(HierarchicalTest, TestBeforeMissingHandler) {
    sm->B();
    ASSERT_EQ(sm->state_info(), "3");
    sm->tape.clear();
    sm->exits.clear();
    sm->enters.clear();
    sm->C();
    ASSERT_EQ(sm->state_info(), "3");
    std::vector<std::string> expected = {"S1.C"};
    ASSERT_EQ(sm->tape, expected);
    ASSERT_EQ(sm->exits.size(), 0);
    ASSERT_EQ(sm->enters.size(), 0);
}


TEST(HierarchicalControllerTest, ContinueAfterTransitionIgnored) {
    HierarchicalController sm;
    sm.A();
    sm.C();
    EXPECT_EQ(sm.state_info(), "4");
    sm.tape.clear();
    sm.enters.clear();
    sm.C();
    EXPECT_EQ(sm.state_info(), "6");
    
    std::vector<std::string> expectedEnters{"T"};
    EXPECT_EQ(sm.enters, expectedEnters);
    
    std::vector<std::string> expectedTape{"S2.C"};
    EXPECT_EQ(sm.tape, expectedTape);
}

int main() {
    ::testing::InitGoogleTest();
    return RUN_ALL_TESTS();
}

