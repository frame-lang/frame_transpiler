#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "match.cpp"

class MatchController : public Match {
public:
    MatchController() : Match() {}
};

class MatchTest : public ::testing::Test {
protected:
    void SetUp() override {
        sm = new MatchController();
    }

    void TearDown() override {
        delete sm;
    }

    MatchController* sm;
};

TEST_F(MatchTest, TestEmptyString) {
    sm->Empty();
    sm->OnString("");
    std::vector<std::string> expected;
    expected.push_back("empty");
    ASSERT_EQ(sm->tape, expected);
    
    sm->tape.clear();
    sm->OnString("hi");
    std::vector<std::string> expected1;
    expected1.push_back("?");
    ASSERT_EQ(sm->tape, expected1);
}

TEST_F(MatchTest, TestIntegerMatch) {
    sm->Simple();
    sm->OnInt(0);
    std::vector<std::string> expected;
    expected.push_back("0");
    ASSERT_EQ(sm->tape, expected);
    
    sm->tape.clear();
    sm->OnInt(42);
    std::vector<std::string> expected1;
    expected1.push_back("42");
    ASSERT_EQ(sm->tape, expected1);
    
    sm->tape.clear();
    sm->OnInt(-200);
    std::vector<std::string> expected2;
    expected2.push_back("-200");
    ASSERT_EQ(sm->tape, expected2);
}

TEST_F(MatchTest, TestStringMatch) {
    sm->Simple();
    sm->OnString("hello");
    std::vector<std::string> expected;
    expected.push_back("hello");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnString("goodbye");
    std::vector<std::string> expected1;
    expected1.push_back("goodbye");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnString("Testing 1, 2, 3...");
    std::vector<std::string> expected2;
    expected2.push_back("testing");
    ASSERT_EQ(sm->tape, expected2);
    std::vector<std::string> expected3;
    expected3.push_back("testing");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnString("$10!");
    std::vector<std::string> expected4;
    expected4.push_back("money");
    ASSERT_EQ(sm->tape, expected4);
    std::vector<std::string> expected5;
    expected5.push_back("money");
    ASSERT_EQ(sm->tape, expected5);

    sm->tape.clear();
    sm->OnString("missing");
    std::vector<std::string> expected6;
    expected6.push_back("?");
    ASSERT_EQ(sm->tape, expected6);
    std::vector<std::string> expected7;
    expected7.push_back("?");
    ASSERT_EQ(sm->tape, expected7);

    sm->tape.clear();
    sm->OnString("Testing");
    std::vector<std::string> expected8;
    expected8.push_back("?");
    ASSERT_EQ(sm->tape, expected8);
    std::vector<std::string> expected9;
    expected9.push_back("?");
    ASSERT_EQ(sm->tape, expected9);

    sm->tape.clear();
    sm->OnString("");
    std::vector<std::string> expected10;
    expected10.push_back("?");
    ASSERT_EQ(sm->tape, expected10);
    std::vector<std::string> expected11;
    expected11.push_back("?");
    ASSERT_EQ(sm->tape, expected11);
}

TEST_F(MatchTest, TestIntegerMultiMatch) {
    sm->Multi();
    sm->OnInt(3);
    std::vector<std::string> expected;
    expected.push_back("3|-7");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnInt(-7);
    std::vector<std::string> expected1;
    expected1.push_back("3|-7");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnInt(-4);
    std::vector<std::string> expected2;
    expected2.push_back("-4|5|6");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnInt(5);
    std::vector<std::string> expected3;
    expected3.push_back("-4|5|6");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnInt(6);
    std::vector<std::string> expected4;
    expected4.push_back("-4|5|6");
    ASSERT_EQ(sm->tape, expected4);

    sm->tape.clear();
    sm->OnInt(4);
    std::vector<std::string> expected5;
    expected5.push_back("?");
    ASSERT_EQ(sm->tape, expected5);

    sm->tape.clear();
    sm->OnInt(0);
    std::vector<std::string> expected6;
    expected6.push_back("?");
    ASSERT_EQ(sm->tape, expected6);
}

TEST_F(MatchTest, TestStringMultiMatch) {
    sm->Multi();
    sm->OnString("$10");
    std::vector<std::string> expected;
    expected.push_back("symbols");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnString("12.5%");
    std::vector<std::string> expected1;
    expected1.push_back("symbols");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnString("@#*!");
    std::vector<std::string> expected2;
    expected2.push_back("symbols");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnString(" ");
    std::vector<std::string> expected3;
    expected3.push_back("whitespace");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnString(" ");
    std::vector<std::string> expected4;
    expected4.push_back("whitespace");
    ASSERT_EQ(sm->tape, expected4);

    sm->tape.clear();
    sm->OnString("\t");
    std::vector<std::string> expected5;
    expected5.push_back("whitespace");
    ASSERT_EQ(sm->tape, expected5);

    sm->tape.clear();
    sm->OnString("\n");
    std::vector<std::string> expected6;
    expected6.push_back("whitespace");
    ASSERT_EQ(sm->tape, expected6);

    sm->tape.clear();
    sm->OnString("10");
    std::vector<std::string> expected7;
    expected7.push_back("?");
    ASSERT_EQ(sm->tape, expected7);

    sm->tape.clear();
    sm->OnString("#");
    std::vector<std::string> expected8;
    expected8.push_back("?");
    ASSERT_EQ(sm->tape, expected8);

    sm->tape.clear();
    sm->OnString("   ");
    std::vector<std::string> expected9;
    expected9.push_back("?");
    ASSERT_EQ(sm->tape, expected9);

    sm->tape.clear();
    sm->OnString("");
    std::vector<std::string> expected10;
    expected10.push_back("?");
    ASSERT_EQ(sm->tape, expected10);
}

TEST_F(MatchTest, TestIntegerNestedMatch) {
    sm->Nested();
    sm->OnInt(1);
    std::vector<std::string> expected;
    expected.push_back("1-3");
    expected.push_back("1");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnInt(2);
    std::vector<std::string> expected1;
    expected1.push_back("1-3");
    expected1.push_back("2");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnInt(3);
    std::vector<std::string> expected2;
    expected2.push_back("1-3");
    expected2.push_back("3");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnInt(4);
    std::vector<std::string> expected3;
    expected3.push_back("4-5");
    expected3.push_back("4");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnInt(5);
    std::vector<std::string> expected4;
    expected4.push_back("4-5");
    expected4.push_back("5");
    ASSERT_EQ(sm->tape, expected4);

    sm->tape.clear();
    sm->OnInt(10);
    std::vector<std::string> expected5;
    expected5.push_back("too big");
    ASSERT_EQ(sm->tape, expected5);

    sm->tape.clear();
    sm->OnInt(0);
    std::vector<std::string> expected6;
    expected6.push_back("too small");
    ASSERT_EQ(sm->tape, expected6);
}

TEST_F(MatchTest, TestStringNestedMatch) {
    sm->Nested();
    sm->OnString("hello");
    std::vector<std::string> expected;
    expected.push_back("greeting");
    expected.push_back("English");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnString("hola");
    std::vector<std::string> expected1;
    expected1.push_back("greeting");
    expected1.push_back("Spanish");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnString("bonjour");
    std::vector<std::string> expected2;
    expected2.push_back("greeting");
    expected2.push_back("French");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnString("goodbye");
    std::vector<std::string> expected3;
    expected3.push_back("farewell");
    expected3.push_back("English");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnString("adios");
    std::vector<std::string> expected4;
    expected4.push_back("farewell");
    expected4.push_back("Spanish");
    ASSERT_EQ(sm->tape, expected4);

    sm->tape.clear();
    sm->OnString("au revoir");
    std::vector<std::string> expected5;
    expected5.push_back("farewell");
    expected5.push_back("French");
    ASSERT_EQ(sm->tape, expected5);

    sm->tape.clear();
    sm->OnString("hallo");
    std::vector<std::string> expected6;
    expected6.push_back("?");
    ASSERT_EQ(sm->tape, expected6);

    sm->tape.clear();
    sm->OnString("ciao");
    std::vector<std::string> expected7;
    expected7.push_back("?");
    ASSERT_EQ(sm->tape, expected7);
}

TEST_F(MatchTest, TestIntegerHierarchicalMatch) {
    sm->Child();
    sm->OnInt(0);
    ASSERT_EQ(sm->state_info(), "6");
    sm->tape.clear();

    sm = new MatchController();
    sm->Child();
    sm->OnInt(4);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected;
    expected.push_back("4");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnInt(5);
    ASSERT_EQ(sm->state_info(), "6");
    std::vector<std::string> expected1;
    expected1.push_back("5");
    ASSERT_EQ(sm->tape, expected1);
    
    sm = new MatchController();
    sm->tape.clear();
    sm->Child();
    sm->OnInt(3);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected2;
    expected2.push_back("3");
    expected2.push_back("?");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnInt(42);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected3;
    expected3.push_back("42 in child");
    expected3.push_back("42");
    ASSERT_EQ(sm->tape, expected3);

    sm->tape.clear();
    sm->OnInt(-200);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected4;
    expected4.push_back("no match in child");
    expected4.push_back("-200");
    ASSERT_EQ(sm->tape, expected4);

    sm->tape.clear();
    sm->OnInt(100);
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected5;
    expected5.push_back("no match in child");
    expected5.push_back("?");
    ASSERT_EQ(sm->tape, expected5);
}

TEST_F(MatchTest, TestStringHierarchicalMatch) {
    sm->Child();
    sm->OnString("goodbye");
    ASSERT_EQ(sm->state_info(), "6");
    sm->tape.clear();

    sm = new MatchController();
    sm->Child();
    sm->OnString("hello");
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected;
    expected.push_back("hello in child");
    expected.push_back("hello");
    ASSERT_EQ(sm->tape, expected);

    sm->tape.clear();
    sm->OnString("Testing 1, 2, 3...");
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected1;
    expected1.push_back("testing in child");
    ASSERT_EQ(sm->tape, expected1);

    sm->tape.clear();
    sm->OnString("$10!");
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected2;
    expected2.push_back("no match in child");
    expected2.push_back("money");
    ASSERT_EQ(sm->tape, expected2);

    sm->tape.clear();
    sm->OnString("testing 1, 2, 3...");
    ASSERT_EQ(sm->state_info(), "5");
    std::vector<std::string> expected3;
    expected3.push_back("no match in child");
    expected3.push_back("?");
    ASSERT_EQ(sm->tape, expected3);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
