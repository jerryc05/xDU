#include "xdu/core.hpp"

inline auto handle_arg_depth(int argc, char *argv[], int &i,
                             Optional<DepthType> &depth) {
  if (depth.has_value()) {
    cout << "ERR!  Duplicate definition of \"-d\".\n";
    exit(1);
  }
  if (i + 1 >= argc) {
    cout << "ERR!  No input for `depth` after \"-d\".\n";
    exit(2);
  }
  try {
    depth = stoull(argv[++i]);
  } catch (const InvalidArgument &e) {
    cout << "ERR!  Invalid input for `depth` after \"-d\".\n";
    exit(3);
  } catch (const OutOfRange &e) {
    cout << "ERR!  Input for `depth` after \"-d\" is out of range.\n";
    exit(4);
  }
}

inline auto handle_arg_dir(char *path_c_str, Vec<fs::path> &paths) {
  auto path = fs::path(path_c_str);
  if (!fs::is_directory(path) && !fs::is_regular_file(path)) {
    cout << "ERR!  The following input is not a directory or file:\n\t" << path
         << " (aka " << fs::absolute(path) << " )\n";
    exit(5);
  }
  paths.push_back(path);
}

inline auto parse_args(int argc, char *argv[]) {
  if (argc == 0) {
    cout << "\n"
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
            "\t\t\tPrint only `depth` level of directory. Maximum value for "
            "`depth` is 2^64 on 64bit OS, 2^32 on 32bit OS, and 2^8 otherwise. "
            "Default value is "
         << DEFAULT_DEPTH << ".\n";
    exit(0);

  } else {
    Vec<fs::path> paths;
    Optional<DepthType> depth = {};

    for (int i = 0; i < argc; ++i) {
      if (strcmp("-d", argv[i]) == 0)
        handle_arg_depth(argc, argv, i, depth);
      else
        handle_arg_dir(argv[i], paths);
    }
    return depth.has_value() ? XDuConfig{paths, *depth} : XDuConfig{paths};
  }
}

int main(int argc, char *argv[]) {
#ifndef NDEBUG
  // debug mode
#else
  // non-debug mode
  std::ios::sync_with_stdio(false);
#endif

  auto config = parse_args(argc - 1, argv + 1);
  cout << config.depth << '\n';
  for (const auto &path : config.paths) {
    cout << path << '\n';
  }
  return 0;
}