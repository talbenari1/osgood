#include <v8.h>

namespace script {

struct CompileResult {
  v8::Local<v8::Script> ret_val;
  v8::Local<v8::Value> exception;
  bool is_ok;
};

struct RunResult {
  v8::Local<v8::Value> ret_val;
  bool is_ok;
};

CompileResult compile(v8::Isolate *const isolate,
                      const v8::Local<v8::Context> &ctx,
                      const v8::Local<v8::String> &src) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Script> maybe_script = v8::Script::Compile(ctx, src);
  CompileResult result;
  if (try_catch.HasCaught()) {
    result.exception = try_catch.Exception();
    result.is_ok = false;
  } else {
    // assumption: `maybe_script` is only empty when an exception was thrown
    result.ret_val = maybe_script.ToLocalChecked();
    result.is_ok = true;
  }
  return result;
}

RunResult run(v8::Isolate *const isolate, const v8::Local<v8::Context> &ctx,
              const v8::Local<v8::Script> &script) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Value> ret = script->Run(ctx);
  if (try_catch.HasCaught()) {
    return RunResult{try_catch.Exception(), false};
  } else {
    return RunResult{ret.ToLocalChecked(), true};
  }
}

v8::ScriptOrigin create_script_origin(v8::Local<v8::String> name) {
  return v8::ScriptOrigin(name);
}
}  // namespace script
