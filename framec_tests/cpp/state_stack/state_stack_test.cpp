#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "state_stack.cpp"

class StateStackController : public StateStack {
public:
    StateStackController() : StateStack() {}
};

class StateStackControllerTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new StateStackController();
    }

    void TearDown() override {
        delete sm;
    }

    StateStackController* sm;
};

TEST_F(StateStackControllerTest, TestPushPop) {
    ASSERT_EQ("0", sm->state_info());
    sm->push();
    sm->to_b();
    ASSERT_EQ("1", sm->state_info());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
}

TEST_F(StateStackControllerTest, TestMultiplePushPops) {
    ASSERT_EQ("0", sm->state_info());
    sm->push();
    sm->to_c();
    sm->push();
    sm->to_a();
    sm->push();
    sm->push();
    sm->to_c(); // no push
    sm->to_b();
    sm->push();
    sm->to_c();
    sm->push(); // stack top-to-bottom: C, B, A, A, C, A
    sm->to_a();
    ASSERT_EQ("0", sm->state_info());
    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    sm->to_a();
    ASSERT_EQ("0", sm->state_info());
    sm->pop();
    ASSERT_EQ("1", sm->state_info());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    sm->to_b();
    sm->push();
    sm->to_c();
    sm->push(); // stack top-to-bottom: C, B, A
    sm->to_a();
    sm->to_b();
    ASSERT_EQ("1", sm->state_info());
    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    sm->pop();
    ASSERT_EQ("1", sm->state_info());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
}

TEST_F(StateStackControllerTest, TestPopTransitionEvents) {
    sm->to_b();
    sm->push();
    sm->to_a();
    sm->push();
    sm->to_c();
    sm->push(); // stack top-to-bottom: C, A, B
    sm->to_a();
    sm->tape.clear();
    ASSERT_EQ("0", sm->state_info());
    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    std::vector<std::string> expected_tape = {"A:<", "C:>"};
    ASSERT_EQ(expected_tape, sm->tape);
    sm->tape.clear();
    sm->pop();
    sm->pop();
    ASSERT_EQ("1", sm->state_info());
    expected_tape = {"C:<", "A:>", "A:<", "B:>"};
    ASSERT_EQ(expected_tape, sm->tape);
}

TEST_F(StateStackControllerTest, TestPopChangeStateNoEvents) {
    sm->to_b();
    sm->push();
    sm->to_a();
    sm->push();
    sm->to_c();
    sm->push(); // stack top-to-bottom: C, A, B
    sm->to_a();
    sm->tape.clear();
    ASSERT_EQ("0", sm->state_info());
    sm->pop_change();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(0, sm->tape.size());
    sm->pop();
    sm->pop_change();
    std::vector<std::string> expected_tape = {"C:<", "A:>"};
    ASSERT_EQ(expected_tape, sm->tape);
}   


int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
