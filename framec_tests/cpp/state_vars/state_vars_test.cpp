#include <gtest/gtest.h>
#include "state_vars.cpp"

class StateVarsController : public StateVars {
public:
    StateVarsController() : StateVars() {}
};

class StateVarsControllerTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new StateVarsController();
    }

    void TearDown() override {
        delete sm;
    }

    StateVarsController* sm;
};

TEST_F(StateVarsControllerTest, TestSingleVariable) {
    EXPECT_EQ("1", sm->state_info());
    sm->X(); // increment x
    sm->X(); // increment x
    EXPECT_EQ(2,  std::any_cast<int>(sm->_compartment_->stateVars["x"]));
}

TEST_F(StateVarsControllerTest, TestMultipleVariables) {
    sm->Y();
    EXPECT_EQ("2", sm->state_info());
    EXPECT_EQ(10, std::any_cast<int>(sm->_compartment_->stateVars["y"]));
    EXPECT_EQ(100, std::any_cast<int>(sm->_compartment_->stateVars["z"]));
    sm->Y();
    sm->Y();
    sm->Z();
    sm->Y();
    EXPECT_EQ(13, std::any_cast<int>(sm->_compartment_->stateVars["y"]));
    EXPECT_EQ(101, std::any_cast<int>(sm->_compartment_->stateVars["z"]));
}

TEST_F(StateVarsControllerTest, TestVariablesAreReset) {
    sm->X(); // increment x
    sm->X(); // increment x
    EXPECT_EQ(2, std::any_cast<int>(sm->_compartment_->stateVars["x"]));
    sm->Z(); // transition to B
    sm->Z(); // increment z
    sm->Y(); // increment y
    sm->Z(); // increment z
    EXPECT_EQ(11, std::any_cast<int>(sm->_compartment_->stateVars["y"]));
    EXPECT_EQ(102, std::any_cast<int>(sm->_compartment_->stateVars["z"]));
    sm->X(); // transition to A
    EXPECT_EQ(0, std::any_cast<int>(sm->_compartment_->stateVars["x"]));
    sm->Y(); // transition to B
    EXPECT_EQ(10, std::any_cast<int>(sm->_compartment_->stateVars["y"]));
    EXPECT_EQ(100, std::any_cast<int>(sm->_compartment_->stateVars["z"]));
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
