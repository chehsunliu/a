cmake_minimum_required(VERSION 3.18)

project(w-cpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# --- main ---

set(WCPP_EXECUTABLE_NAME ${PROJECT_NAME})
add_executable(${WCPP_EXECUTABLE_NAME} src/main.cpp)
