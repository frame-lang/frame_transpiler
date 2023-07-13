#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "handler_calls.cpp" 

class HandlerCallsController ;

class HandlerCallsController : public HandlerCalls {
public:
    HandlerCallsController() : HandlerCalls() {}
};

class HandlerCallsTest : public ::testing::Test {
protected:
    HandlerCallsController controller;
};

TEST_F(HandlerCallsTest, TestCallTerminateHandler) {
    controller.NonRec();
    controller.Foo(10);
    ASSERT_NE("Unreachable(0)", controller.tape.back());
}

TEST_F(HandlerCallsTest, TestNonRecursive) {
    controller.NonRec();
    controller.Foo(10);
    std::vector<std::string> expected = {"Foo(10)", "Bar(20)", "Final(30)"};
    EXPECT_EQ(expected, controller.tape);
}

TEST_F(HandlerCallsTest, TestSelfRecursive) {
    controller.SelfRec();
    controller.Foo(10);
    std::vector<std::string> expected = {"Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)"};
    EXPECT_EQ(expected, controller.tape);
}

TEST_F(HandlerCallsTest, TestMutuallyRecursive) {
    controller.MutRec();
    controller.Foo(2);
    std::vector<std::string> expected = {
        "Foo(2)", "Bar(4)", "Foo(4)", "Bar(8)",
        "Foo(16)", "Bar(32)", "Foo(96)", "Final(162)"
    };
    EXPECT_EQ(expected, controller.tape);
}

TEST_F(HandlerCallsTest, TestStringMatchCall) {
    controller.NonRec();
    controller.Call("Foo", 5);
    std::vector<std::string> expected = {"Foo(5)", "Bar(10)", "Final(15)"};
    EXPECT_EQ(expected, controller.tape);
    controller.tape.clear();

    controller.NonRec();
    controller.Call("Bar", 20);
    std::vector<std::string> expected1 = {"Bar(20)", "Final(20)"};
    EXPECT_EQ(expected1, controller.tape);
    controller.tape.clear();

    controller.NonRec();
    controller.Call("Qux", 37);
    std::vector<std::string> expected2 = {"Foo(1000)", "Bar(2000)", "Final(3000)"};
    EXPECT_EQ(expected2, controller.tape);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
