#include "logger.hpp"
#include <iomanip>
#include <chrono>
#include <csignal>

/* Tweaking start */
using TimeUnit = std::chrono::microseconds;

#ifndef LOG_FILENAME_OVERRIDE
#define LOG_FILENAME_OVERRIDE
constexpr auto LOG_FILENAME = "log.log";
#endif
/* Tweaking end */

using OutFileStream = std::ofstream;
using RuntimeError = std::runtime_error;

constexpr auto TIME_FMT = "%Y-%m-%dT%H:%M:%S";

#ifdef unix
constexpr auto RED_FMT = "\033[1;91m";  // Bold High Intensity Red
constexpr auto GRN_FMT = "\033[1;92m";  // Bold High Intensity Green
constexpr auto LGR_FMT = "\033[0;37m";  // Light Grey
constexpr auto NOC_FMT = "\033[0m";  // No Colors
#else
constexpr auto RED_FMT = "";
constexpr auto GRN_FMT = "";
constexpr auto LGR_FMT = "";
constexpr auto NOC_FMT = "";
#endif

auto &write_time(OutStream &stream) {
  using namespace std::chrono;
  using Clock = system_clock;

  constexpr auto UNIT   = (seconds(1) - TimeUnit(0)).count();
  constexpr auto DIGITS = []() {
    switch (UNIT) {
      case 1:
        return 0;
      case 1000:
        return 3;
      case 1000000:
        return 6;
      case 1000000000:
        return 9;
      default:
        throw RuntimeError("log10(): Invalid UNIT number");
    }
  }();

  auto       now   = Clock::now();
  const auto time_ = Clock::to_time_t(now);
  const auto tm    = *std::localtime(&time_);

  const auto sub_s = duration_cast<TimeUnit>(now.time_since_epoch()) % UNIT;

  return stream << std::put_time(&tm, TIME_FMT)
                << '.' << std::setfill('0') << std::setw(DIGITS) << sub_s.count();
}

OutFileStream &file_logger() {
  static OutFileStream o_file;

  if (!o_file.is_open()) {
    o_file.open(
            LOG_FILENAME,
            std::ios_base::out | std::ios_base::binary | std::ios_base::app);
    if (!o_file.is_open())
      cerr << "Failed to open log file: " << LOG_FILENAME << '\n';

    o_file << '\n';

    /* Handle signals */ {
      constexpr auto sig_handler = [](auto sig) {
        write_time(o_file << '\n')
                << " | ERR  | Program terminated by signal: " << sig;
        o_file.flush();
      };
      signal(SIGABRT, sig_handler);
      signal(SIGFPE, sig_handler);
      signal(SIGILL, sig_handler);
      signal(SIGINT, sig_handler);
      signal(SIGSEGV, sig_handler);
      signal(SIGTERM, sig_handler);
    }
  }
  return o_file;
}

BaseLogger::BaseLogger(OutStream &stream,
                       const char *const type_str,
                       const char *const color)
        : stream_(stream), type_str_(type_str) {
  write_time(file_logger() << '\n') << " | " << type_str << " | ";
  write_time(stream << color << '\n') << " | " << type_str << " | ";
}

BaseLogger::~BaseLogger() {
  stream_ << NOC_FMT;
}

ErrLogger::ErrLogger() : BaseLogger(cerr, "ERR ", RED_FMT) {
}

ErrLogger::~ErrLogger() {
  stream_.flush();
  file_logger().flush();
}

InfoLogger::InfoLogger() : BaseLogger(cout, "INFO", GRN_FMT) {
}

DbgLogger::DbgLogger() : BaseLogger(cout, "DBG ", LGR_FMT) {
}
