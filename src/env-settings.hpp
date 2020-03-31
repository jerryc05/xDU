#ifndef ENV_SETTINGS_HPP
#define ENV_SETTINGS_HPP

#include <cstring>
#include <filesystem>
#include <iostream>
#include <optional>
#include <vector>
#include "utils/logger.hpp"

using std::cout;

using InvalidArgument = std::invalid_argument;
using OutOfRange = std::out_of_range;
using RuntimeError = std::runtime_error;
using Exception = std::exception;

template<typename T> using Vec = std::vector<T>;
template<typename T> using Optional = std::optional<T>;
using Path = std::filesystem::path;

namespace Fs = std::filesystem;

constexpr auto ERR_STR_NO_ARG                                    =
                       "ERR_STR_NO_ARG";
constexpr auto ERR_STR_DUP_DEF_OF_D_FLAG                         =
                       "ERR_STR_DUP_DEF_OF_D_FLAG";
constexpr auto ERR_STR_NO_INPUT_FOR_DEPTH_AFTER_D_FLAG           =
                       "ERR_STR_NO_INPUT_FOR_DEPTH_AFTER_D_FLAG";
constexpr auto ERR_STR_INVALID_INPUT_FOR_DEPTH_AFTER_D_FLAG      =
                       "ERR_STR_INVALID_INPUT_FOR_DEPTH_AFTER_D_FLAG";
constexpr auto ERR_STR_INPUT_FOR_DEPTH_AFTER_D_FLAG_OUT_OF_RANGE =
                       "ERR_STR_INPUT_FOR_DEPTH_AFTER_D_FLAG_OUT_OF_RANGE";
constexpr auto ERR_STR_INPUT_PATH_IS_NOT_A_DIR_OR_FILE           =
                       "ERR_STR_INPUT_PATH_IS_NOT_A_DIR_OR_FILE";

// Windows
#if defined(_WIN32) || defined(_WIN64)
#if defined(_WIN64) && _WIN64
#define ENV_64_BIT
#elif defined(_WIN32) && _WIN32
#define ENV_32_BIT
#endif // _WIN64
#endif // defined(_WIN32) || defined(_WIN64)

// GCC
#if defined(__GNUC__) && __GNUC__
#if (defined(__x86_64__) && __x86_64__) || (defined(__ppc64__) && __ppc64__)
#define ENV_64_BIT
#else
#define ENV_32_BIT
#endif // (defined(__x86_64__) && __x86_64__) ||
// (defined(__ppc64__) &&__ppc64__)
#endif // defined(__GNUC__) && __GNUC__

#endif // ENV_SETTINGS_HPP
