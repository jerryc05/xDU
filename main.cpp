#include "xdu/core.h"
#include <iostream>
using std::cout;

XDuConfig parse_args(int, char *[]);

int main(int argc, char *argv[]) {
#ifndef NDEBUG
  // debug mode
#else
  // non-debug mode
  std::ios::sync_with_stdio(false);
#endif

  XDuConfig config = parse_args(argc, argv);
  return 0;
}

XDuConfig parse_args(int argc, char *argv[]) {
  if (argc == 1) {
    cout <<

        "NAME\n"
        "\txdu -- Display Disk Usage - eXperimental version by @jerryc05\n"
        "\n"
        "SYNOPSIS\n"
        "\txdu [-d depth] directory [dir2 dir3...]\n"
        "\n"
        "DESCRIPTION\n"
        "\tThis xdu tool is a simplified version of *ix's `du` command. "
        "Fore more info, please refer to `man du`.\n"
        "\n"
        "\tThe options are as follows:\n"
        "\n"
        "\t-d depth\n"
        "\tPrint only `depth` level of directory. Default to "
         << DEFAULT_DEPTH << '\n';
    exit(0);
  } else {
    Vec<fs::path> paths;
    paths.push_back(fs::path(argv[0]).parent_path());
    //
    return XDuConfig{paths};
  }
}