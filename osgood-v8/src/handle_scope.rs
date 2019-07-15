use super::binding;
use super::isolate;

use std::marker::PhantomData;

/// A Scope is a marker trait used to mark structs that operate on a stack-based V8 state.
pub trait Scope {}

/// A scope in which V8 heap values can be constructed.
#[derive(Debug)]
pub struct HandleScope<'a, 'p: 'a, P> {
    pub(crate) local: PhantomData<&'a u8>,
    parent: &'p mut P,
    scope: binding::v8::HandleScope,
}

impl<'a, 'p, P> HandleScope<'a, 'p, P>
where
    P: Scope,
{
    /// Constructs a new HandleScope.
    ///
    /// HandleScopes require mutable references in order to ensure that only one scope can be
    /// active at any given point in time.
    pub fn new(parent: &'p mut P) -> Self {
        let isolate = isolate::get_current();
        let scope = unsafe { binding::v8::HandleScope::new(isolate) };
        HandleScope {
            parent,
            scope,
            local: PhantomData,
        }
    }
}

impl<'a, 'p, P> Scope for HandleScope<'a, 'p, P> {}

#[cfg(test)]
mod test {
    use super::HandleScope;
    use crate::isolate::Isolate;

    #[test]
    fn handle_scope_empty() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let _scope = HandleScope::new(&mut isolate);
        isolate.exit();
    }

    #[test]
    fn handle_scope_nested() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let mut scope1 = HandleScope::new(&mut isolate);
        let scope2 = HandleScope::new(&mut scope1);
        drop(scope2);
        drop(scope1);
        isolate.exit();
    }

    #[test]
    #[should_panic]
    fn handle_scope_no_isolate_enter() {
        let mut isolate = Isolate::new();
        let _scope = HandleScope::new(&mut isolate);
    }
}
