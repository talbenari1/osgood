#include <v8.h>

namespace context {
v8::Local<v8::Context> new_context(v8::Isolate *isolate) {
  return v8::Context::New(isolate);
}
}  // namespace context
