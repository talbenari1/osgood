use super::binding;
use super::context::Context;
use super::handle::Handle;
use super::handle_scope::HandleScope;
use super::isolate;
use super::string::IntoV8String;
use super::value::Value;

/// A JavaScript script.
pub struct Script<'a, 'c: 'a> {
    pub(crate) val: Handle<binding::v8::Script>,
    context: &'a Context<'c>,
}

impl<'a, 'c> Script<'a, 'c> {
    /// Compiles a new script.
    pub fn new<'p, P>(
        scope: &'a HandleScope<'a, 'p, P>,
        context: &'a Context<'c>,
        source: impl IntoV8String<'a, 'p, P>,
    ) -> Result<Self, Value<'a>> {
        let isolate = isolate::get_current();

        let source = source.to_v8_string(scope);
        let res = unsafe {
            binding::script::compile(isolate, &context.context_, source.val.into_local(scope))
        };

        if res.is_ok {
            Ok(Script {
                val: res.ret_val.into(),
                context,
            })
        } else {
            Err(Value::from(scope, res.exception.into()))
        }
    }

    /// Executes this script.
    pub fn run<'p, P>(
        &mut self,
        scope: &'a HandleScope<'a, 'p, P>,
    ) -> Result<Value<'a>, Value<'a>> {
        let isolate = isolate::get_current();
        let res = unsafe {
            binding::script::run(isolate, &self.context.context_, self.val.into_local(scope))
        };
        if res.is_ok {
            Ok(Value::from(scope, res.ret_val.into()))
        } else {
            Err(Value::from(scope, res.ret_val.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::Script;
    use crate::{Context, HandleScope, Isolate};

    static SCRIPT_OK: &str = "let foo = 1;";
    static SCRIPT_ERR_SYNTAX: &str = "let foo bar;";
    static SCRIPT_ERR_THROW: &str = "throw new Error();";

    #[test]
    // Compiles successfully, runs successfully.
    fn script_new_ok() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let script = Script::new(scope, context, SCRIPT_OK);
        assert!(script.is_ok());

        let mut script = script.unwrap();
        let res = script.run(scope);
        assert!(res.is_ok());

        context.exit();
        isolate.exit();
    }

    #[test]
    // Compiles with an error.
    fn script_new_compile_err() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let script = Script::new(scope, context, SCRIPT_ERR_SYNTAX);
        assert!(script.is_err());

        context.exit();
        isolate.exit();
    }

    #[test]
    // Compiles successfully, runs with an error.
    fn script_new_runtime_err() {
        let ref mut isolate = Isolate::new();
        isolate.enter();
        let ref mut scope = HandleScope::new(isolate);
        let ref mut context = Context::new(scope);
        context.enter();

        let script = Script::new(scope, context, SCRIPT_ERR_THROW);
        assert!(script.is_ok());

        let mut script = script.unwrap();
        let res = script.run(scope);
        assert!(res.is_err());

        context.exit();
        isolate.exit();
    }
}
