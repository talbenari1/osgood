use super::binding::v8;
use super::handle::{Handle, HandleKind};
use super::handle_scope::HandleScope;
use super::isolate;
use super::Value;

use std::convert::TryFrom;
use std::marker::PhantomData;

macro_rules! c_str {
    ($str:expr) => {
        format!("{}\0", $str).as_ptr() as *const ::std::os::raw::c_char
    };
}

/// A JavaScript string value.
#[derive(Debug)]
pub struct String<'a> {
    pub(crate) val: Handle<v8::String>,
    pub(crate) scope: PhantomData<&'a u8>,
}

impl<'a> Copy for String<'a> {}

impl<'a> Clone for String<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> String<'a> {
    /// Constructs a new String from a UTF-8-encoded string slice.
    pub fn new<'p, P>(_scope: &'a HandleScope<'a, 'p, P>, val: &str) -> Self {
        let val = val.to_owned();
        let isolate = isolate::get_current();
        let val = unsafe {
            v8::String::NewFromUtf8(isolate, c_str!(val), 0, i32::try_from(val.len()).unwrap())
        };

        let val = val.to_option().unwrap();

        String {
            val: val.into(),
            scope: PhantomData,
        }
    }

    /// Checks if this string value is equal to another string value.
    pub fn equals<'p, P>(&self, scope: &'a HandleScope<'a, 'p, P>, other: &String<'a>) -> bool {
        let mut val = self.val;
        unsafe { val.StringEquals(*other.val.into_local(scope)) }
    }
}

impl<'a> From<String<'a>> for Value<'a> {
    fn from(string: String<'a>) -> Self {
        match &string.val.base {
            HandleKind::Local(local) => {
                let val: v8::Local<v8::Value> = unsafe { local.clone().cast() };

                Value {
                    val: val.into(),
                    scope: string.scope,
                }
            }
        }
    }
}

pub trait IntoV8String<'a, 'p, P> {
    fn to_v8_string(&self, scope: &'a HandleScope<'a, 'p, P>) -> String<'a>;
}

impl<'a, 'p, P> IntoV8String<'a, 'p, P> for String<'a> {
    fn to_v8_string(&self, _scope: &'a HandleScope<'a, 'p, P>) -> Self {
        *self
    }
}

impl<'a, 'p, P> IntoV8String<'a, 'p, P> for &str {
    fn to_v8_string(&self, scope: &'a HandleScope<'a, 'p, P>) -> String<'a> {
        String::new(scope, self)
    }
}

impl<'a, 'p, P> IntoV8String<'a, 'p, P> for std::string::String {
    fn to_v8_string(&self, scope: &'a HandleScope<'a, 'p, P>) -> String<'a> {
        String::new(scope, self)
    }
}

#[cfg(test)]
mod test {
    use crate::{Context, HandleScope, Isolate, String};

    #[test]
    fn string_new() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let _str = String::new(scope, "foo");

        context.exit();
        isolate.exit();
    }

    #[test]
    fn string_eq() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let str1 = String::new(scope, "foo");
        let str2 = String::new(scope, "foo");
        let str3 = String::new(scope, "bar");

        assert!(str1.equals(scope, &str2));
        assert!(!str1.equals(scope, &str3));

        context.exit();
        isolate.exit();
    }
}
