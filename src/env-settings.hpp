#ifndef ENV_SETTINGS_HPP
#define ENV_SETTINGS_HPP

#include <cstring>
#include <filesystem>
#include <iostream>
#include <optional>
#include <vector>

using
std::cout, std::cerr,
std::stoull, std::min, std::numeric_limits;

using InvalidArgument = std::invalid_argument;
using OutOfRange = std::out_of_range;
using RuntimeError = std::runtime_error;
using Exception = std::exception;

template<typename T> using Vec = std::vector<T>;
template<typename T> using Optional = std::optional<T>;

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

auto init_env() {
#ifndef NDEBUG
  // debug mode
#else
  // non-debug mode
  std::ios::sync_with_stdio(false);
#endif
}

// Windows
#if defined(_WIN32) || defined(_WIN64)
#if defined(_WIN64) && _WIN64
#define _ENV_64
#elif defined(_WIN32) && _WIN32
#define _ENV_32
#endif // _WIN64
#endif // defined(_WIN32) || defined(_WIN64)

// GCC
#if defined(__GNUC__) && __GNUC__
#if (defined(__x86_64__) && __x86_64__) || (defined(__ppc64__) && __ppc64__)
#define _ENV_64
#else
#define _ENV_32
#endif // (defined(__x86_64__) && __x86_64__) ||
// (defined(__ppc64__) &&__ppc64__)
#endif // defined(__GNUC__) && __GNUC__

#endif // ENV_SETTINGS_HPP
