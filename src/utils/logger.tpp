template<typename T>
BaseLogger &BaseLogger::operator<<(T msg) {
  file_logger() << msg;
  stream_ << msg;
  return *this;
}