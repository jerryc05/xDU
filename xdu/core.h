#ifndef XDU_H
#define XDU_H

#include <filesystem>
#include <vector>
using std::vector;
namespace fs = std::filesystem;
template <typename T> using Vec = vector<T>;

const uint64_t DEFAULT_DEPTH = 2;

struct XDuConfig {
  Vec<fs::path> paths; // 3*8 bytes
  uint64_t depth = DEFAULT_DEPTH;  // 1*8 bytes
};

#endif // XDU_H
