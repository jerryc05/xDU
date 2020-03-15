#ifndef XDU_CORE_H
#define XDU_CORE_H

#include "env-settings.h"

using DepthType =
#ifdef _ENV_64
    uint64_t // 1*8 bytes
#elif defined(_ENV_32)
    uint32_t // 1*4 bytes
#else
    uint8_t // 1*1 bytes
#endif
    ;

const uint64_t DEFAULT_DEPTH = 2;

struct XDuConfig {
  Vec<fs::path> paths; // 3*8 bytes
  DepthType depth = DEFAULT_DEPTH;
};

#endif // XDU_CORE_H
