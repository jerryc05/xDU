template<typename T>
BaseLogger &BaseLogger::operator<<(const T &msg) {
  file_logger() << msg;
  stream_ << msg;
  return *this;
}