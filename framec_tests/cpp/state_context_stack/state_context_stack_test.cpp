#include <gtest/gtest.h>
#include <vector>
#include <string>
#include <algorithm>
#include "state_context_stack.cpp"

class StateContextStackController : public StateContextStack {
public:
    StateContextStackController() : StateContextStack() {}
};

class StateContextStackTest : public ::testing::Test {
protected:
    void SetUp() {
        sm = new StateContextStackController();
    }

    void TearDown() {
        delete sm;
    }

    StateContextStackController* sm;
};

TEST_F(StateContextStackTest, PushPopTest) {
    StateContextStackController* sm;
    ASSERT_EQ("0", sm->state_info());
    sm->push();
    sm->to_b();
    ASSERT_EQ("1", sm->state_info());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
}

TEST_F(StateContextStackTest, MultiplePushPopsTest) {
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

TEST_F(StateContextStackTest, PopTransitionEventsTest) {
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

    std::vector<std::string> expectedTape = { "A:<", "C:>" };
    ASSERT_EQ(expectedTape, sm->tape);

    sm->tape.clear();
    sm->pop();
    sm->pop();
    ASSERT_EQ("1", sm->state_info());

    expectedTape = { "C:<", "A:>", "A:<", "B:>" };
    ASSERT_EQ(expectedTape, sm->tape);
}

TEST_F(StateContextStackTest, PopChangesStateNoEvents) {
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
    ASSERT_EQ("1", sm->state_info());
    std::vector<std::string> expectedTape = { "C:<", "A:>" };
    ASSERT_EQ(expectedTape, sm->tape);
}

TEST_F(StateContextStackTest, PopRestoresStateVariables) {
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(2, sm->value());
    sm->to_b();
    sm->inc();
    sm->push();
    ASSERT_EQ("1", sm->state_info());
    ASSERT_EQ(5, sm->value());
    sm->to_c();
    sm->inc();
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(30, sm->value());
    sm->to_a();
    sm->inc();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(1, sm->value());
    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(30, sm->value());
    sm->pop();
    ASSERT_EQ("1", sm->state_info());
    ASSERT_EQ(5, sm->value());
    sm->to_a();
    sm->inc();
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(3, sm->value());
    sm->to_c();
    sm->inc();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(10, sm->value());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(3, sm->value());
    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(2, sm->value());
}

TEST_F(StateContextStackTest, PushStoresStateVariableSnapshot) {
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(2, sm->value());
    sm->inc();
    sm->inc();
    ASSERT_EQ(4, sm->value());

    sm->to_b();
    sm->inc();
    sm->push();
    ASSERT_EQ("1", sm->state_info());
    ASSERT_EQ(5, sm->value());
    sm->inc();
    sm->inc();
    ASSERT_EQ(15, sm->value()); // these changes should be forgotten

    sm->to_c();
    sm->inc();
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(30, sm->value());
    sm->inc();
    ASSERT_EQ(40, sm->value()); // forgotten

    sm->to_a();
    sm->inc();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(1, sm->value());

    sm->pop();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(30, sm->value());

    sm->pop();
    ASSERT_EQ("1", sm->state_info());
    ASSERT_EQ(5, sm->value());

    sm->to_a();
    sm->inc();
    sm->inc();
    sm->inc();
    sm->push();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(3, sm->value());
    sm->inc();
    ASSERT_EQ(4, sm->value()); // forgotten

    sm->to_c();
    sm->inc();
    ASSERT_EQ("2", sm->state_info());
    ASSERT_EQ(10, sm->value());

    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(3, sm->value());

    sm->pop();
    ASSERT_EQ("0", sm->state_info());
    ASSERT_EQ(2, sm->value());
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
