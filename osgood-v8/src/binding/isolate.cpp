#include <libplatform/libplatform.h>
#include <v8-platform.h>
#include <v8.h>
#include <iostream>

static std::unique_ptr<v8::Platform> g_platform;

namespace platform {
extern "C" void init(const char* argv0) {
  if (g_platform == nullptr) {
    v8::V8::InitializeICUDefaultLocation(argv0);
    v8::V8::InitializeExternalStartupData(argv0);
    g_platform = v8::platform::NewDefaultPlatform();
    v8::V8::InitializePlatform(g_platform.get());
    v8::V8::Initialize();
  }
}
}  // namespace platform

namespace isolate {
extern "C" v8::Isolate* new_isolate() {
  if (g_platform == nullptr) {
    return nullptr;
  }
  v8::Isolate::CreateParams create_params;
  create_params.array_buffer_allocator =
      v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  v8::Isolate* isolate = v8::Isolate::New(create_params);

  return isolate;
}

extern "C" v8::Isolate* get_current() { return v8::Isolate::GetCurrent(); }

extern "C" bool is_in_use(v8::Isolate* isolate) { return isolate->IsInUse(); }
}  // namespace isolate

