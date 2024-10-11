cmake_minimum_required(VERSION 3.21)

project(cxl_mimalloc)

set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

SET(MI_OVERRIDE OFF)
SET(MI_BUILD_OBJECT OFF)
SET(MI_BUILD_SHARED OFF)
SET(MI_BUILD_TESTS OFF)
add_subdirectory(
  "../mi2"
  ${CMAKE_BINARY_DIR}/mi2
)

add_library(cxl_mimalloc_dynamic SHARED
    ${CMAKE_CURRENT_SOURCE_DIR}/cxl_mimalloc_dynamic.cpp
)
target_compile_options(cxl_mimalloc_dynamic PRIVATE -O3 -ftls-model=initial-exec)
target_link_libraries(cxl_mimalloc_dynamic PRIVATE mimalloc-static numa)

add_library(cxl_mimalloc_static STATIC
    ${CMAKE_CURRENT_SOURCE_DIR}/cxl_mimalloc_static.cpp
)
target_include_directories(cxl_mimalloc_static INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_compile_options(cxl_mimalloc_static PRIVATE -O3)
target_link_libraries(cxl_mimalloc_static PRIVATE mimalloc-static numa)
