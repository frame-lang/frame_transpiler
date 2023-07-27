#include <gtest/gtest.h>
#include <vector>
#include <string>
#include "var_scope.cpp"

class VarScopeController : public VarScope {
public:
    VarScopeController() : VarScope() {}

    void do_nn() {
        nn("|nn|[d]");
    }

    void do_ny() {
        ny("|ny|[d]");
    }

    void do_yn() {
        yn("|yn|[d]", "|yn|[x]");
    }

    void do_yy() {
        yy("|yy|[d]", "|yy|[x]");
    }
};

class VarScopeTest : public ::testing::Test {
protected:
    std::vector<std::string> expected(const std::string& state, const std::string& msg, const std::string& x) {
        std::vector<std::string> result;
        result.push_back("#.a");
        result.push_back("$" + state + "[b]");
        result.push_back("$" + state + ".c");
        result.push_back("|" + msg + "|" + "[d]");
        result.push_back("|" + msg + "|" + ".e");
        result.push_back(x);

        return result;
    }
};

TEST_F(VarScopeTest, TestNoShadowing) {
    VarScopeController sm;
    sm.to_nn();
    sm.do_nn();
    EXPECT_EQ(expected("NN", "nn", "#.x"), sm.tape);
}

TEST_F(VarScopeTest, TestAllShadowingScenarios) {
    VarScopeController sm;
    sm.to_nn();
    sm.do_ny();
    EXPECT_EQ(expected("NN", "ny", "|ny|.x"), sm.tape);

    sm.tape.clear();
    sm.do_yn();
    EXPECT_EQ(expected("NN", "yn", "|yn|[x]"), sm.tape);

    sm.tape.clear();
    sm.do_yy();
    EXPECT_EQ(expected("NN", "yy", "|yy|.x"), sm.tape);

    sm = VarScopeController();
    sm.to_ny();
    sm.do_nn();
    EXPECT_EQ(expected("NY", "nn", "$NY.x"), sm.tape);

    sm.tape.clear();
    sm.do_ny();
    EXPECT_EQ(expected("NY", "ny", "|ny|.x"), sm.tape);

    sm.tape.clear();
    sm.do_yn();
    EXPECT_EQ(expected("NY", "yn", "|yn|[x]"), sm.tape);

    sm.tape.clear();
    sm.do_yy();
    EXPECT_EQ(expected("NY", "yy", "|yy|.x"), sm.tape);

    sm = VarScopeController();
    sm.to_yn();
    sm.do_nn();
    EXPECT_EQ(expected("YN", "nn", "$YN[x]"), sm.tape);

    sm.tape.clear();
    sm.do_ny();
    EXPECT_EQ(expected("YN", "ny", "|ny|.x"), sm.tape);

    sm.tape.clear();
    sm.do_yn();
    EXPECT_EQ(expected("YN", "yn", "|yn|[x]"), sm.tape);

    sm.tape.clear();
    sm.do_yy();
    EXPECT_EQ(expected("YN", "yy", "|yy|.x"), sm.tape);

    sm = VarScopeController();
    sm.to_yy();
    sm.do_nn();
    EXPECT_EQ(expected("YY", "nn", "$YY.x"), sm.tape);

    sm.tape.clear();
    sm.do_ny();
    EXPECT_EQ(expected("YY", "ny", "|ny|.x"), sm.tape);

    sm.tape.clear();
    sm.do_yn();
    EXPECT_EQ(expected("YY", "yn", "|yn|[x]"), sm.tape);

    sm.tape.clear();
    sm.do_yy();
    EXPECT_EQ(expected("YY", "yy", "|yy|.x"), sm.tape);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}

