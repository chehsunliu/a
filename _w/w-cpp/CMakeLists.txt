cmake_minimum_required(VERSION 3.18)

project(w-cpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# --- main ---

set(WCPP_EXECUTABLE_NAME ${PROJECT_NAME})
add_executable(${WCPP_EXECUTABLE_NAME} src/main.cpp)

# --- snippet ---

set(WCPP_TEST_NAME ${PROJECT_NAME}-test)
add_executable(${WCPP_TEST_NAME} src/example.cpp src/leetcode.cpp)
add_test(${WCPP_TEST_NAME} ${WCPP_TEST_NAME})

add_subdirectory(3rd-party-libs/googletest EXCLUDE_FROM_ALL)

target_link_libraries(${WCPP_TEST_NAME} gtest_main)
