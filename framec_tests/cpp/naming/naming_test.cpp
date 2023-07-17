#include <gtest/gtest.h>
#include <vector>
#include "naming.cpp"

class NamingController : public Naming {
public:
    NamingController() : Naming() {}

};

class NamingTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new NamingController();
    }

    void TearDown() override {
        delete sm;
    }

    NamingController* sm;
};

TEST_F(NamingTest, TestFollowingNamingWorks) {
    sm->snake_event(1);
    EXPECT_EQ(sm->state_info(), "1");
    sm->snake_event(2);
    EXPECT_EQ(sm->state_info(), "0");
    sm->snake_event(1);
    EXPECT_EQ(sm->state_info(), "1");
    sm->CamelEvent(3);
    EXPECT_EQ(sm->state_info(), "0");
    sm->snake_event(1);
    EXPECT_EQ(sm->state_info(), "1");
    sm->event123(4);
    EXPECT_EQ(sm->state_info(), "0");

    std::vector<int> expected = {1103, 1104, 1105};
    EXPECT_EQ(sm->finalLog, expected);
    sm->finalLog.clear();

    sm->CamelEvent(11);
    EXPECT_EQ(sm->state_info(), "2");
    sm->snake_event(2);
    EXPECT_EQ(sm->state_info(), "0");
    sm->CamelEvent(11);
    EXPECT_EQ(sm->state_info(), "2");
    sm->CamelEvent(3);
    EXPECT_EQ(sm->state_info(), "0");
    sm->CamelEvent(11);
    EXPECT_EQ(sm->state_info(), "2");
    sm->event123(4);
    EXPECT_EQ(sm->state_info(), "0");

    std::vector<int> expected1 = {1213, 1214, 1215};
    EXPECT_EQ(sm->finalLog, expected1);
    sm->finalLog.clear();

    sm->event123(21);
    EXPECT_EQ(sm->state_info(), "3");
    sm->snake_event(2);
    EXPECT_EQ(sm->state_info(), "0");
    sm->event123(21);
    EXPECT_EQ(sm->state_info(), "3");
    sm->CamelEvent(3);
    EXPECT_EQ(sm->state_info(), "0");
    sm->event123(21);
    EXPECT_EQ(sm->state_info(), "3");
    sm->event123(4);
    EXPECT_EQ(sm->state_info(), "0");

    std::vector<int> expected2 = {1323, 1324, 1325};
    EXPECT_EQ(sm->finalLog, expected2);
    std::vector<int> expected3 = {1103, 1213, 1323};
    EXPECT_EQ(sm->snake_log, expected3);
    std::vector<int> expected4 = {1104, 1214, 1324};
    EXPECT_EQ(sm->CamelLog, expected4);
    std::vector<int> expected5 = {1105, 1215, 1325};
    EXPECT_EQ(sm->log123, expected5);
}

TEST_F(NamingTest, TestInterfaceCalls) {
    sm->call("snake_event", 1);
    sm->call("CamelEvent", 2);
    sm->call("event123", 3);
    sm->call("snake_event", 4);
    sm->call("CamelEvent", 5);
    sm->call("event123", 6);

    std::vector<int> expected = {1103, 1307, 1211};
    EXPECT_EQ(sm->finalLog, expected);
    std::vector<int> expected1 = {1307};
    EXPECT_EQ(sm->snake_log, expected1);
    std::vector<int> expected2 = {1103};
    EXPECT_EQ(sm->CamelLog, expected2);
    std::vector<int> expected3 = {1211};
    EXPECT_EQ(sm->log123, expected3);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
