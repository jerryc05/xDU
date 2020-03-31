#include "src/xdu.hpp"

using std::stoul, std::min, std::numeric_limits;

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
            "`depth` is 2^32 on 64/32bit OS, and 2^8 otherwise. "
            "Default value is "
         << DEFAULT_DEPTH << ".\n";
    throw RuntimeError(ERR_STR_NO_ARG);

  } else {
    Vec<Fs::path> paths;
    DepthType     depth_plus_one = 0;

    for (int i = 0; i < argc; ++i) {
      if (strcmp("-d", argv[i]) == 0) {
        if (depth_plus_one != 0) {
          ErrLogger() << "Duplicate definition of \"-d\"";
          throw RuntimeError(ERR_STR_DUP_DEF_OF_D_FLAG);
        }
        if (i + 1 >= argc) {
          ErrLogger() << "No input for `depth` after \"-d\"";
          throw RuntimeError(ERR_STR_NO_INPUT_FOR_DEPTH_AFTER_D_FLAG);
        }
        try {
          depth_plus_one = stoul(argv[++i]) + 1;
        } catch (const InvalidArgument &e) {
          ErrLogger() << "Invalid input for `depth` after \"-d\"";
          throw RuntimeError(ERR_STR_INVALID_INPUT_FOR_DEPTH_AFTER_D_FLAG);
        } catch (const OutOfRange &e) {
          ErrLogger() << "Input for `depth` after \"-d\" is out of range";
          throw RuntimeError(ERR_STR_INPUT_FOR_DEPTH_AFTER_D_FLAG_OUT_OF_RANGE);
        }

      } else {
        auto path = Path(argv[i]);
        if (!Fs::is_directory(path) && !Fs::is_regular_file(path)) {
          ErrLogger() << "The input is not a directory or a regular file: "
                      << path << " (aka " << Fs::absolute(path).c_str() << " )";
          throw RuntimeError(ERR_STR_INPUT_PATH_IS_NOT_A_DIR_OR_FILE);
        }
        paths.push_back(path);
      }
    }
    return depth_plus_one == 0
           ? XDuConfig(paths)
           : XDuConfig{paths, depth_plus_one - 1};
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
} catch (const Exception &e) {
  ErrLogger() << e.what();
  return -1;
}