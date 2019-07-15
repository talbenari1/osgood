use super::binding::{v8, value};
use super::context::Context;
use super::handle::Handle;
use super::handle_scope::HandleScope;
use super::isolate;
use super::string::String;

use std::marker::PhantomData;

/// A JavaScript value.
#[derive(Debug, Copy, Clone)]
pub struct Value<'a> {
    pub(crate) val: Handle<v8::Value>,
    pub(crate) scope: PhantomData<&'a u8>,
}

// TODO(tal): This impl is woefully incomplete. It would be awesome if we could codegen this,
// but for now we'll add methods ad-hoc.
impl<'a> Value<'a> {
    pub(crate) fn from<'p, P>(_scope: &'a HandleScope<'a, 'p, P>, val: Handle<v8::Value>) -> Self {
        Value {
            scope: PhantomData,
            val,
        }
    }

    /// Returns true if this value is the undefined value.
    pub fn is_undefined<'p, P>(&self, scope: &'a mut HandleScope<'a, 'p, P>) -> bool {
        unsafe { value::is_undefined(self.val.into_local(scope)) }
    }

    /// Returns true if this value is true.
    pub fn is_true(&self) -> bool {
        unsafe { (*self.val).IsTrue() }
    }

    /// Returns true if this value is false.
    pub fn is_false(&self) -> bool {
        unsafe { (*self.val).IsFalse() }
    }

    /// Returns true if this value is a symbol or a string.
    pub fn is_name(&self) -> bool {
        unsafe { (*self.val).IsName() }
    }

    /// Returns true if this value is a function.
    pub fn is_function(&self) -> bool {
        unsafe { (*self.val).IsFunction() }
    }

    /// Converts this value to a string.
    pub fn to_string<'c: 'a, 'p, P>(
        &self,
        scope: &'a mut HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
    ) -> Option<String<'a>> {
        let maybe_string =
            unsafe { value::to_string(self.val.into_local(scope), &context.context_) };
        let maybe_string: Option<v8::Local<v8::String>> = maybe_string.into();
        maybe_string.map(|val| String {
            val: val.into(),
            scope: self.scope,
        })
    }

    /// Checks if this value is non-strictly equal to another value.
    pub fn equals<'c: 'a, 'p, P>(
        &self,
        scope: &'a mut HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
        other: &Self,
    ) -> bool {
        let res = unsafe { (*self.val).Equals(context.context_, *other.val.into_local(scope)) };

        res.to_option().unwrap()
    }

    // TODO: Is there any way we can take advantage of PartialEq here?
    /// Checks if this value is strictly equal to another value.
    pub fn strict_equals<'p, P>(
        &self,
        scope: &'a mut HandleScope<'a, 'p, P>,
        other: &Self,
    ) -> bool {
        unsafe { (*self.val).StrictEquals(*other.val.into_local(scope)) }
    }

    /// Converts this value to a rust string.
    pub fn into_rust_string<'p, P>(
        &self,
        scope: &'a HandleScope<'a, 'p, P>,
    ) -> std::string::String {
        let isolate = isolate::get_current();
        let utf8value = unsafe { v8::String_Utf8Value::new(isolate, *self.val.into_local(scope)) };
        unsafe { std::ffi::CString::from_raw(utf8value.str_) }
            .into_string()
            .unwrap()
    }
}
