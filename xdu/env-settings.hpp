#ifndef XDU_ENV_SETTINGS_HPP
#define XDU_ENV_SETTINGS_HPP

#include <cstring>
#include <filesystem>
#include <iostream>
#include <optional>
#include <vector>
using std::cout, std::cerr, std::stoull;
using InvalidArgument = std::invalid_argument;
using OutOfRange = std::out_of_range;

namespace fs = std::filesystem;
template <typename T> using Vec = std::vector<T>;

template <typename T> using Optional = std::optional<T>;

// Windows
#if defined(_WIN32) || defined(_WIN64)
#if defined(_WIN64) && _WIN64
#define _ENV_64
#elif defined(_WIN43) && _WIN32
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

#endif // XDU_ENV_SETTINGS_HPP
