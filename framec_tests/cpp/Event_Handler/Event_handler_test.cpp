#include "../gtest/gtest.h"
#include "Event_Handler.cpp"
#include <iostream>
#include <vector>
#include <string>


class EventHandlerController;

class EventHandlerController : public EventHandler {
public:
    EventHandlerController() : EventHandler() {}
};

class EventHandlerControllerTest : public ::testing::Test {
 protected:
  EventHandlerController controller;

  void SetUp() override {
    sm = new EventHandlerController();
  }

  void TearDown() override {
    delete sm;
  }

  EventHandlerController* sm;
};

TEST_F(EventHandlerControllerTest, TestSingleParameter) {
  controller.LogIt(2);
  std::vector<std::string> expected = {"x=2"};
  EXPECT_EQ(controller.tape, expected);
}

TEST_F(EventHandlerControllerTest, TestComputeTwoParameter) {
  controller.LogAdd(-3, 10);
  std::vector<std::string> expected = {"a=-3", "b=10", "a+b=7"};
  EXPECT_EQ(controller.tape, expected);
}

TEST_F(EventHandlerControllerTest, TestReturnLocalVariable) {
  int ret = controller.LogReturn(13, 21);
  std::vector<std::string> expected = {"a=13", "b=21", "r=34"};
  EXPECT_EQ(controller.tape, expected);
  EXPECT_EQ(ret, 34);
}

TEST_F(EventHandlerControllerTest, TestPassResult) {
  controller.PassAdd(5, -12);
  std::vector<std::string> expected = {"p=-7"};
  EXPECT_EQ(controller.tape, expected);
}

TEST_F(EventHandlerControllerTest, TestPassAndReturnResult) {
  int ret = controller.PassReturn(101, -59);
  std::vector<std::string> expected = {"r=42", "p=42"};
  EXPECT_EQ(controller.tape, expected);
  EXPECT_EQ(ret, 42);
}

int main(int argc, char** argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
