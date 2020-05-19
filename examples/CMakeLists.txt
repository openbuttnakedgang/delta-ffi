
cmake_minimum_required(VERSION 3.7)
project(example)

add_executable(example main.c)

target_include_directories(example PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/../lib")

if (WIN32)
    target_link_libraries(example "${CMAKE_CURRENT_SOURCE_DIR}/../lib/delta_ffi.dll.lib")
else ()
    target_link_libraries(example "${CMAKE_CURRENT_SOURCE_DIR}/../lib/libdelta_ffi.so")
endif ()