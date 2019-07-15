//! A crate for working with V8.

#![warn(missing_docs)]
#![deny(clippy::all)]

#[macro_use]
extern crate lazy_static;

mod binding;
mod context;
mod handle;
mod handle_scope;
mod isolate;
mod module;
mod object;
mod platform;
mod script;
mod string;
mod value;

pub use context::Context;
pub use handle_scope::HandleScope;
pub use isolate::Isolate;
pub use module::Module;
pub use object::Object;
pub use platform::Platform;
pub use script::Script;
pub use string::String;
pub use value::Value;
