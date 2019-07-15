use super::binding;
use super::context::Context;
use super::handle::Handle;
use super::handle_scope::HandleScope;
use super::isolate;
use super::value::Value;

use std::marker::PhantomData;

/// A JavaScript object value.
#[derive(Clone)]
pub struct Object<'a> {
    pub(crate) val: Handle<binding::v8::Object>,
    scope: PhantomData<&'a u8>,
}

impl<'a> Object<'a> {
    /// Creates a new object.
    pub fn new<'p, P>(_scope: &'a HandleScope<'a, 'p, P>) -> Self {
        let isolate = isolate::get_current();
        let obj = unsafe { binding::v8::Object::New(isolate) };

        Object {
            val: obj.into(),
            scope: PhantomData,
        }
    }

    /// Gets a property from this object.
    pub fn get<'k, 'c, 'p, P>(
        &self,
        scope: &'a HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
        key: impl Into<Value<'k>>,
    ) -> Option<Value<'a>> {
        let mut val = self.val.clone();
        let maybe_val = unsafe { val.Get(context.context_, *key.into().val.into_local(scope)) };
        maybe_val
            .to_option()
            .map(|val| Value::from(scope, val.into()))
    }

    /// Sets a property on this object.
    pub fn set<'k, 'v, 'c, 'p, P>(
        &mut self,
        scope: &'a HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
        key: impl Into<Value<'k>>,
        val: impl Into<Value<'v>>,
    ) -> bool {
        let res = unsafe {
            (*self.val).Set(
                context.context_,
                *key.into().val.into_local(scope),
                *val.into().val.into_local(scope),
            )
        };
        res.to_option().unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use crate::{Context, HandleScope, Isolate, Object, String};

    #[test]
    fn object_new() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let _obj = Object::new(scope);

        context.exit();
        isolate.exit();
    }

    #[test]
    fn object_set_get() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let mut obj = Object::new(scope);
        let key = String::new(scope, "foo");
        let val = String::new(scope, "bar");
        obj.set(scope, context, key, val);

        let val = obj.get(scope, context, key);
        assert!(val.is_some());
        let val = val.unwrap().into_rust_string(scope);
        assert_eq!(val, "bar");

        context.exit();
        isolate.exit();
    }
}
