#include <gtest/gtest.h>
#include <vector>
#include <algorithm>
#include "transition.cpp"

class TransitionSmController : public TransitionSm {
public:
    TransitionSmController() : TransitionSm() {}
};

class TransitionSmTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new TransitionSmController();
    }

    void TearDown() override {
        delete sm;
    }

    TransitionSmController* sm;
};

TEST_F(TransitionSmTest, TestTransitionEvents) {
    sm->enters.clear();
    sm->exits.clear();
    sm->transit();
    EXPECT_EQ("1", sm->state_info());
    EXPECT_EQ(std::vector<std::string>{"S0"}, sm->exits);
    EXPECT_EQ(std::vector<std::string>{"S1"}, sm->enters);
}

TEST_F(TransitionSmTest, TestChangeStateNoEvents) {
    sm->enters.clear();
    sm->exits.clear();
    sm->change();
    EXPECT_EQ("1", sm->state_info());
    sm->change();
    EXPECT_EQ("2", sm->state_info());
    sm->change();
    EXPECT_EQ("3", sm->state_info());
    sm->change();
    EXPECT_EQ("4", sm->state_info());
    EXPECT_TRUE(sm->exits.empty());
    EXPECT_TRUE(sm->enters.empty());
}

TEST_F(TransitionSmTest, TestCascadingTransition) {
    sm->change();
    sm->enters.clear();
    sm->exits.clear();
    EXPECT_EQ("1", sm->state_info());
    sm->transit();
    EXPECT_EQ("3", sm->state_info());
    EXPECT_EQ((std::vector<std::string>{"S1", "S2"}), sm->exits);
    EXPECT_EQ((std::vector<std::string>{"S2", "S3"}), sm->enters);
}

TEST_F(TransitionSmTest, TestCascadingChangeSheet) {
    sm->change();
    sm->change();
    sm->change();
    sm->enters.clear();
    sm->exits.clear();
    EXPECT_EQ("3", sm->state_info());
    sm->transit();
    EXPECT_EQ("0", sm->state_info());
    EXPECT_EQ(std::vector<std::string>{"S3"}, sm->exits);
    EXPECT_EQ(std::vector<std::string>{"S4"}, sm->enters);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
