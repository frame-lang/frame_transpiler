#include <gtest/gtest.h>
#include <vector>
#include <algorithm>
#include <string>
#include "transition_params.cpp"

class TransitParamsController : public TransitParams {
public:
    TransitParamsController() : TransitParams() {}
};

class TransitParamsControllerTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new TransitParamsController();
    }

    void TearDown() override {
        delete sm;
    }

    TransitParamsController* sm;
};

TEST_F(TransitParamsControllerTest, TestEnter) {
    sm->Next();
    EXPECT_EQ(std::vector<std::string>{"hi A"}, sm->tape);
}

TEST_F(TransitParamsControllerTest, TestEnterAndExit) {
    sm->Next();
    sm->tape.clear();
    sm->Next();
    EXPECT_EQ((std::vector<std::string>{"bye A", "hi B", "42"}), sm->tape);
    sm->tape.clear();
    sm->Next();
    EXPECT_EQ((std::vector<std::string>{"true", "bye B", "hi again A"}), sm->tape);
}

TEST_F(TransitParamsControllerTest, TestChangeState) {
    EXPECT_EQ("0", sm->state_info());
    sm->Change();
    EXPECT_EQ("1", sm->state_info());
    sm->Change();
    EXPECT_EQ("2", sm->state_info());
    sm->Change();
    EXPECT_EQ("1", sm->state_info());
    EXPECT_TRUE(sm->tape.empty());
}

TEST_F(TransitParamsControllerTest, TestChangeAndTransition) {
    sm->Change();
    EXPECT_EQ("1", sm->state_info());
    EXPECT_TRUE(sm->tape.empty());
    sm->Next();
    EXPECT_EQ("2", sm->state_info());
    EXPECT_EQ((std::vector<std::string>{"bye A", "hi B", "42"}), sm->tape);
    sm->tape.clear();
    sm->Change();
    EXPECT_EQ("1", sm->state_info());
    EXPECT_TRUE(sm->tape.empty());
    sm->Change();
    sm->Next();
    EXPECT_EQ("1", sm->state_info());
    EXPECT_EQ((std::vector<std::string>{"true", "bye B", "hi again A"}), sm->tape);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
