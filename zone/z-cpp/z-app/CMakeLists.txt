project(z-app)

add_executable(${PROJECT_NAME} main.cpp)

install(
  TARGETS ${PROJECT_NAME}
  RUNTIME DESTINATION bin)
