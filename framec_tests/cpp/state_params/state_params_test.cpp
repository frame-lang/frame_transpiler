#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "state_params.cpp"

class StateParamsController : public StateParams {
public:
    StateParamsController() : StateParams() {}
};

class StateParamsControllerTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new StateParamsController();
    }

    void TearDown() override {
        delete sm;
    }

    StateParamsController* sm;
};

TEST_F(StateParamsControllerTest, TestSingleParameter) {
    sm->Next();
    sm->Log();
    ASSERT_EQ(std::vector<std::string>{"val=1"}, sm->param_log);
}

TEST_F(StateParamsControllerTest, TestMultipleParameters) {
    sm->Next();
    sm->Next();
    sm->Log();
    ASSERT_EQ((std::vector<std::string>{"left=1", "right=2"}), sm->param_log);
}

TEST_F(StateParamsControllerTest, TestSeveralPasses) {
    sm->Next(); // val=1
    sm->Next(); // left=1 right=2
    sm->Next(); // val=3
    sm->Log();
    sm->Prev(); // left=4 right=3
    sm->Log();
    sm->Prev(); // val=12
    sm->Log();
    ASSERT_EQ((std::vector<std::string>{"val=3", "left=4", "right=3", "val=12"}), sm->param_log);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
