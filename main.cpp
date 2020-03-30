#include "src/xdu.hpp"

inline auto handle_arg_depth(int argc, char *argv[], int &i,
                             Optional<DepthType> &depth) {
  if (depth.has_value()) {
    cerr << "ERR!  Duplicate definition of \"-d\".\n";
    throw RuntimeError(ERR_STR_DUP_DEF_OF_D_FLAG);
  }
  if (i + 1 >= argc) {
    cerr << "ERR!  No input for `depth` after \"-d\".\n";
    throw RuntimeError(ERR_STR_NO_INPUT_FOR_DEPTH_AFTER_D_FLAG);
  }
  try {
#ifdef _ENV_64
    depth = stoull(argv[++i]);
#else
    auto parsed = stoull(argv[++i]);
    auto min_value =
        min(parsed, static_cast<uint64_t>(numeric_limits<DepthType>::max()));
    depth = static_cast<DepthType>(min_value);
#endif
  } catch (const InvalidArgument &e) {
    cerr << "ERR!  Invalid input for `depth` after \"-d\".\n";
    throw RuntimeError(ERR_STR_INVALID_INPUT_FOR_DEPTH_AFTER_D_FLAG);
  } catch (const OutOfRange &e) {
    cerr << "ERR!  Input for `depth` after \"-d\" is out of range.\n";
    throw RuntimeError(ERR_STR_INPUT_FOR_DEPTH_AFTER_D_FLAG_OUT_OF_RANGE);
  }
}

inline auto handle_arg_dir(char *path_c_str, Vec<Fs::path> &paths) {
  auto path = Fs::path(path_c_str);
  if (!Fs::is_directory(path) && !Fs::is_regular_file(path)) {
    cerr << "ERR!  The following input is not a directory or file:\n\t" << path
         << " (aka " << Fs::absolute(path) << " )\n";
    throw RuntimeError(ERR_STR_INPUT_PATH_IS_NOT_A_DIR_OR_FILE);
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
    throw RuntimeError(ERR_STR_NO_ARG);

  } else {
    Vec<Fs::path>       paths;
    Optional<DepthType> depth;

    for (int i = 0; i < argc; ++i) {
      if (strcmp("-d", argv[i]) == 0)
        handle_arg_depth(argc, argv, i, depth);
      else
        handle_arg_dir(argv[i], paths);
    }
    return depth.has_value() ? XDuConfig{paths, *depth} : XDuConfig{paths};
  }
}

int main(int argc, char *argv[]) try {
  init_env();

  auto config = parse_args(argc - 1, argv + 1);
  cout << +config.depth << '\n';
  for (const auto &path : config.paths) {
    cout << Fs::absolute(path) << '\n';
  }
  return 0;
} catch (Exception &e) {
  ErrLogger() << e.what();
  return -1;
}