#include "xdu.hpp"

XDuConfig::XDuConfig(Vec<Path> &paths_,
                     const DepthType &depth_)
        : paths(move(paths_)), depth(depth_) {
}
