#include <v8.h>

namespace value {
bool is_undefined(const v8::Local<v8::Value>& val) {
  return val->IsUndefined();
}

v8::MaybeLocal<v8::String> to_string(const v8::Local<v8::Value> &val, const v8::Local<v8::Context> &context) {
  return val->ToString(context);
}
}
