project(test)

set(SOURCE "${CMAKE_CURRENT_SOURCE_DIR}/lib.c")
message(${SOURCE})
add_library(test ${SOURCE})
