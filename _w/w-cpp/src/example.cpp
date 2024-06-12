int add(const int a, const int b) { return a + b; }

#include <gtest/gtest.h>

TEST(CoreTest, Basics) {
    EXPECT_EQ(7, add(3, 4));
    EXPECT_EQ(-1, add(3, -4));
}
