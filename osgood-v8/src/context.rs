use super::binding;
use super::handle_scope::HandleScope;
use super::isolate;

use std::marker::PhantomData;

/// A sandboxed JavaScript execution context.
#[derive(Debug, Copy, Clone)]
pub struct Context<'a> {
    pub(crate) context_: binding::v8::Local<binding::v8::Context>,
    scope: PhantomData<&'a i8>,
}

impl<'a> Context<'a> {
    /// Creates a new execution context.
    pub fn new<P>(_scope: &'a HandleScope<P>) -> Self {
        let isolate = isolate::get_current();
        let context_ = unsafe { binding::context::new_context(isolate) };
        Context {
            context_,
            scope: PhantomData,
        }
    }

    /// Sets this isolate as the currently active one for this thread.
    pub fn enter(&mut self) {
        unsafe { (*self.context_.val_).Enter() };
    }

    /// Unsets this isolate as the currently active one for this thread. V8 keeps track of which
    /// isolate was previously entered and will set the most recent one as active.
    pub fn exit(&mut self) {
        unsafe { (*self.context_.val_).Exit() };
    }
}

#[cfg(test)]
mod test {
    use super::Context;
    use crate::handle_scope::HandleScope;
    use crate::isolate::Isolate;

    #[test]
    fn context_new() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(&mut isolate);
        let _context = Context::new(scope);
        isolate.exit();
    }

    #[test]
    fn context_enter_exit_early_drop() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(&mut isolate);
        let mut context = Context::new(scope);

        context.enter();
        context.exit();
        drop(scope);
        isolate.exit();
    }

    #[test]
    fn context_enter_exit() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(&mut isolate);
        let mut context = Context::new(scope);

        context.enter();
        context.exit();
        drop(scope);
        isolate.exit();
    }

    #[test]
    fn context_multiple() {
        let mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(&mut isolate);
        let mut context1 = Context::new(scope);
        let mut context2 = Context::new(scope);

        context1.enter();
        context2.enter();
        context2.exit();
        context1.exit();
        drop(scope);
        isolate.exit();
    }
}
