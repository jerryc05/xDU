#ifndef LOGGER_HPP
#define LOGGER_HPP

#include <iostream>
#include <fstream>

using std::cout, std::cerr;
using OutStream = std::ostream;
using OutFileStream = std::ofstream;

OutFileStream &file_logger();

class BaseLogger {
public:
  template<typename T>
  BaseLogger &operator<<(const T &msg);

protected:
  OutStream &stream_;
  const char *const type_str_;

  BaseLogger(OutStream &stream, const char *type_str, const char *color);

  ~BaseLogger();
};

class [[maybe_unused]] ErrLogger : public BaseLogger {
public:
  ErrLogger();

  ~ErrLogger();
};

class [[maybe_unused]] InfoLogger : public BaseLogger {
public:
  InfoLogger();
};

#ifndef NDEBUG

class [[maybe_unused]] DbgLogger : public BaseLogger {
public:
  DbgLogger();
};

#endif

#include "logger.tpp"

#endif //LOGGER_HPP
