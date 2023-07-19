#include <gtest/gtest.h>
#include "simple_handler_calls.cpp"

class SimpleHandlerCallsController : public SimpleHandlerCalls {
public:
    SimpleHandlerCallsController() : SimpleHandlerCalls() {}
};

TEST(SimpleHandlerCallsTest, TestSimpleCall) {
    SimpleHandlerCallsController sm;
    sm.C();
    ASSERT_EQ(sm.state_info(), "1");
}

TEST(SimpleHandlerCallsTest, TestCallsTerminateHandler) {
    SimpleHandlerCallsController sm;
    sm.D();
    ASSERT_EQ(sm.state_info(), "2");

    sm = SimpleHandlerCallsController();
    sm.E();
    ASSERT_EQ(sm.state_info(), "2");
}

int main(int argc, char** argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
