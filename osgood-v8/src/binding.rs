#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

use std::marker::PhantomData;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl<T> v8::Local<T> {
    /// Convert this local into a local of a different type. Highly unsafe if you don't know what
    /// you're doing (it will literally recast a `mut *-ptr`), but very useful for helping Rust
    /// deal with the inheritance model used in v8.
    pub unsafe fn cast<U>(&mut self) -> v8::Local<U> {
        let val_: *mut U = std::mem::transmute(self.val_);
        v8::Local {
            val_,
            _phantom_0: PhantomData,
        }
    }
}

impl<T> v8::MaybeLocal<T> {
    pub fn to_option(self) -> Option<v8::Local<T>> {
        self.into()
    }
}

impl<T> Into<Option<v8::Local<T>>> for v8::MaybeLocal<T> {
    fn into(self) -> Option<v8::Local<T>> {
        if self.val_.is_null() {
            None
        } else {
            let mut local: v8::Local<T> = unsafe { std::mem::uninitialized() };
            local.val_ = self.val_;
            Some(local)
        }
    }
}

impl<T> v8::Maybe<T> {
    pub fn to_option(self) -> Option<T> {
        self.into()
    }
}

impl<T> Into<Option<T>> for v8::Maybe<T> {
    fn into(self) -> Option<T> {
        if self.has_value_ {
            Some(self.value_)
        } else {
            None
        }
    }
}

pub use root::context;
pub use root::isolate;
pub use root::module;
pub use root::platform;
pub use root::script;
pub use root::v8;
pub use root::value;
