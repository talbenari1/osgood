use super::binding;
use super::binding::v8;
use super::context::Context;
use super::handle::Handle;
use super::handle_scope::HandleScope;
use super::isolate;
use super::string::{IntoV8String, String};
use super::value::Value;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MODULE_TO_RESOLVER: Mutex<HashMap<i32, Option<ResolveCallback>>> =
        Mutex::new(HashMap::new());
}

pub type ResolveCallback = for<'a, 'c> fn(
    context: &'a Context<'c>,
    specifier: String<'a>,
    referrer: i32,
) -> Option<Module<'a, 'c>>;

/// A JavaScript script.
pub struct Module<'a, 'c> {
    pub(crate) val: Handle<binding::v8::Module>,
    context: &'a Context<'c>,
}

impl<'a, 'c> Module<'a, 'c> {
    /// Compiles a new module.
    pub fn new<'p, P>(
        scope: &'a HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
        src: impl IntoV8String<'a, 'p, P>,
        name: impl IntoV8String<'a, 'p, P>,
    ) -> Result<Self, Value<'a>> {
        let isolate = isolate::get_current();

        let name = name.to_v8_string(scope);
        let src = src.to_v8_string(scope);
        let origin = unsafe { binding::script::create_script_origin(*name.val.into_local(scope)) };
        let res = unsafe { binding::module::compile(isolate, origin, *src.val.into_local(scope)) };

        if res.is_ok {
            Ok(Module {
                val: res.ret_val.into(),
                context,
            })
        } else {
            Err(Value::from(scope, res.exception.into()))
        }
    }

    /// Instantiates this module.
    pub fn instantiate<'p, P>(
        &mut self,
        scope: &mut HandleScope<'a, 'p, P>,
        callback: Option<ResolveCallback>,
    ) -> bool {
        MODULE_TO_RESOLVER
            .lock()
            .unwrap()
            .insert(self.id(), callback);
        let _res = unsafe {
            binding::module::instantiate(
                self.context.context_,
                *self.val.into_local(scope),
                Some(resolve_callback),
            )
        };

        unimplemented!()
    }

    /// Gets this module's unique identifier.
    pub fn id(&self) -> i32 {
        unsafe { (*self.val).GetIdentityHash() }
    }
}

/// This is a trampoline function that also wraps each argument to/from the user-provided callback
/// in order to abstract away the V8 internals.
extern "C" fn resolve_callback(
    _context: v8::Local<v8::Context>,
    _specifier: v8::Local<v8::String>,
    _referrer: v8::Local<v8::Module>,
) -> v8::MaybeLocal<v8::Module> {
    unimplemented!()
}
