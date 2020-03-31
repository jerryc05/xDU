#ifndef XDU_HPP
#define XDU_HPP

#include "env-settings.hpp"

using DepthType = uint32_t; // 1*4 bytes


const uint64_t DEFAULT_DEPTH = 2;

struct XDuConfig {
  explicit XDuConfig(Vec<Path> &paths,
                     const DepthType &depth = DEFAULT_DEPTH);

  const Vec<Path> paths; // 3*8 bytes
  const DepthType depth = DEFAULT_DEPTH;
};

#endif // XDU_HPP
