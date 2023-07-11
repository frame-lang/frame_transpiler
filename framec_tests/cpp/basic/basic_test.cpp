#include "../gtest/gtest.h"
#include <unordered_map>
#include <stdexcept>
#include <string>
#include <iostream>
#include <vector>
using namespace std;
#include "basic.cpp"

class BasicController;  // Forward declaration

class BasicControllerTest : public ::testing::Test {
protected:
    void SetUp() override;
    void TearDown() override;

    BasicController* sm;
};

class BasicController : public Basic {
public:
    BasicController() : Basic() {}
};

void BasicControllerTest::SetUp() {
    sm = new BasicController();
}

void BasicControllerTest::TearDown() {
    delete sm;
}


TEST_F(BasicControllerTest, TestInitialEnterEvent) {
    std::vector<std::string> expected = {"S0"};
    EXPECT_EQ(expected, sm->entry_log);
}

TEST_F(BasicControllerTest, TestTransitionEnterEvents) {
    BasicController* sm = new BasicController();
    sm->entry_log.clear();
    sm->A();
    sm->B();
    std::vector<std::string> expected = {"S1", "S0"};
    ASSERT_EQ(sm->entry_log, expected);
    delete sm;
}

TEST_F(BasicControllerTest, TestTransitionExitEvents) {
    sm->A();
    sm->B();
    std::vector<std::string> expected = {"S0", "S1"};
    EXPECT_EQ(expected, sm->exit_log);
}

TEST_F(BasicControllerTest, TestCurrentState) {
    EXPECT_EQ("0", sm->state_info());
    sm->A();
    EXPECT_EQ("1", sm->state_info());
    sm->B();
    EXPECT_EQ("0", sm->state_info());
}

int main(int argc, char* argv[]) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
