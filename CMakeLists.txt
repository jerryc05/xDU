cmake_minimum_required(VERSION 2.4)
project(xDU)
message("")

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    message("Adding GCC Flags...")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} \
-fstrict-enums \
-ftrapv \
-funroll-loops \
-std=gnu++2a \
-Wall \
-Wcast-align \
-Wconversion \
-Wctor-dtor-privacy \
-Wdelete-non-virtual-dtor \
-Wdeprecated-copy \
-Wdouble-promotion \
-Wduplicated-branches \
-Wduplicated-cond \
-Wextra \
-Wfloat-equal \
-Winit-self \
-Wlogical-op \
-Wmisleading-indentation \
-Wnon-virtual-dtor \
-Wnull-dereference \
-Wold-style-cast \
-Woverloaded-virtual \
-Wpedantic \
-Wpointer-arith \
-Wshadow \
-Wsign-conversion \
-Wswitch-default \
-Wswitch-enum \
-Wundef \
-Wunused \
-Wuseless-cast \
-Wwrite-strings \
")
endif ()
message("")

if (CMAKE_BUILD_TYPE MATCHES "Debug")
    message("CMAKE IN DEBUG MODE")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Og")
elseif (CMAKE_BUILD_TYPE MATCHES "Release")
    message("CMAKE IN RELEASE MODE")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Ofast -march=native")
endif ()
message("")

message("[${CMAKE_CXX_FLAGS}]")
message("")

add_executable(xDU main.cpp xdu/core.cpp xdu/core.hpp xdu/env-settings.hpp)
