cmake_minimum_required(VERSION 3.21)

project(cxl_mimalloc)

set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

SET(MI_OVERRIDE OFF)
add_subdirectory(
  "../mi2"
  ${CMAKE_BINARY_DIR}/mi2
)

add_library(cxl_mimalloc SHARED
    ${CMAKE_CURRENT_SOURCE_DIR}/cxl_mimalloc.cpp
)
target_compile_options(cxl_mimalloc PRIVATE -O3 -ftls-model=initial-exec)
target_link_libraries(cxl_mimalloc PRIVATE mimalloc-static numa)
