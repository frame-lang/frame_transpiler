#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "state_context.cpp"

class StateContextSmController : public StateContextSm {
public:
    StateContextSmController() : StateContextSm() {}
};

class StateContextTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new StateContextSmController();
    }

    void TearDown() override {
        delete sm;
    }

    StateContextSmController* sm;
};

TEST_F(StateContextTest, TestInitialState) {
    int r = sm->Inc();
    EXPECT_EQ(4, r);
    sm->LogState();

    std::vector<std::string> expected = {"w=3", "w=4", "w=4"};
    EXPECT_EQ(expected, sm->tape);
}

TEST_F(StateContextTest, TestTransition) {
    sm->Inc();
    sm->Inc();
    sm->tape.clear();

    sm->Start();
    std::vector<std::string> expected = {"a=3", "b=5", "x=15"};
    EXPECT_EQ(expected, sm->tape);
    sm->tape.clear();

    sm->Inc();
    int r = sm->Inc();
    EXPECT_EQ(17, r);
    std::vector<std::string> expected1 = {"x=16", "x=17"};
    EXPECT_EQ(expected1, sm->tape);
    sm->tape.clear();

    sm->Next(3);
    std::vector<std::string> expected2 = {"c=10", "x=27", "a=30", "y=17", "z=47"};
    EXPECT_EQ(expected2, sm->tape);
    sm->tape.clear();

    sm->Inc();
    sm->Inc();
    r = sm->Inc();
    EXPECT_EQ(50, r);
    EXPECT_EQ((std::vector<std::string>{"z=48", "z=49", "z=50"}), sm->tape);
}

TEST_F(StateContextTest, TestChangeState) {
    sm->Inc();
    sm->Inc();
    sm->Start();
    sm->tape.clear();

    sm->Inc();
    EXPECT_EQ(std::vector<std::string>{"x=16"}, sm->tape);
    sm->tape.clear();

    sm->Change(10);
    sm->LogState();
    EXPECT_EQ((std::vector<std::string>{"y=26", "z=0"}), sm->tape);
    sm->tape.clear();

    sm->Inc();
    sm->Change(100);
    sm->LogState();
    EXPECT_EQ(sm->state_info(), "0");
    EXPECT_EQ((std::vector<std::string>{"z=1", "tmp=127", "w=0"}), sm->tape);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
