#include <v8.h>

namespace module {

struct CompileResult {
  v8::Local<v8::Module> ret_val;
  v8::Local<v8::Value> exception;
  bool is_ok;
};

struct EvaluateResult {
  v8::Local<v8::Value> ret_val;
  bool is_ok;
};

CompileResult compile(v8::Isolate *isolate, v8::ScriptOrigin origin,
                                 v8::Local<v8::String> code) {
  v8::TryCatch try_catch(isolate);
  v8::ScriptCompiler::Source source(code, origin);
  v8::MaybeLocal<v8::Module> ret =
      v8::ScriptCompiler::CompileModule(isolate, &source);
  CompileResult result;
  if (try_catch.HasCaught()) {
    result.exception = try_catch.Exception();
    result.is_ok = false;
  } else {
    result.ret_val = ret.ToLocalChecked();
    result.is_ok = true;
  }
  return result;
}

bool instantiate(v8::Local<v8::Context> context,
                        v8::Local<v8::Module> module,
                        v8::Module::ResolveCallback callback) {
  v8::Maybe<bool> res = module->InstantiateModule(context, callback);
  if (res.IsJust()) {
    return res.FromJust();
  }

  return false;
}

EvaluateResult evaluate(v8::Isolate *isolate,
                            v8::Local<v8::Context> context,
                            v8::Local<v8::Module> module) {
  v8::TryCatch try_catch(isolate);
  v8::MaybeLocal<v8::Value> ret = module->Evaluate(context);
  if (try_catch.HasCaught()) {
    return EvaluateResult{try_catch.Exception(), false};
  } else {
    return EvaluateResult{ret.ToLocalChecked(), true};
  }
}
}  // namespace module
