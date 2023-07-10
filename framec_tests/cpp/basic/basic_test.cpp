#include "../gtest/gtest.h"
#include <vector>
#include <string>

class Basic {
public:
    std::vector<std::string> entry_log;
    std::vector<std::string> exit_log;

public:
    Basic() {}
    virtual ~Basic() {}

    virtual void entered_do(const std::string& msg) {
        entry_log.push_back(msg);
    }

    virtual void left_do(const std::string& msg) {
        exit_log.push_back(msg);
    }
};

class BasicController : public Basic {
public:
    BasicController() : Basic() {}

    void entered_do(const std::string& msg) override {
        Basic::entered_do(msg);
    }

    void left_do(const std::string& msg) override {
        Basic::left_do(msg);
    }

    void A() {
        entered_do("S1");
    }

    void B() {
        entered_do("S0");
    }

    std::string state_info() const {
        if (entry_log.empty())
            return "0";
        return entry_log.back().substr(1);
    }
};

class BasicTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new BasicController();
    }

    void TearDown() override {
        delete sm;
    }

    BasicController* sm;
};

TEST_F(BasicTest, TestInitialEnterEvent) {
    std::vector<std::string> expected = {"S0"};
    EXPECT_EQ(expected, sm->entry_log);
}

TEST_F(BasicTest, TestTransitionEnterEvents) {
    sm->A();
    sm->B();
    std::vector<std::string> expected = {"S1", "S0"};
    EXPECT_EQ(expected, sm->entry_log);
}

TEST_F(BasicTest, TestTransitionExitEvents) {
    sm->A();
    sm->B();
    std::vector<std::string> expected = {"S0", "S1"};
    EXPECT_EQ(expected, sm->exit_log);
}

TEST_F(BasicTest, TestCurrentState) {
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
