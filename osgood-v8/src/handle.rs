use super::binding::v8;
use super::handle_scope::HandleScope;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone)]
pub enum HandleKind<T> {
    Local(v8::Local<T>),
}

#[derive(Debug, Copy, Clone)]
pub struct Handle<T> {
    pub base: HandleKind<T>,
}

impl<T> Handle<T> {
    pub unsafe fn val(&self) -> &T {
        match &self.base {
            HandleKind::Local(local) => &*local.val_,
        }
    }

    pub unsafe fn val_mut(&mut self) -> &mut T {
        match &self.base {
            HandleKind::Local(local) => &mut *local.val_,
        }
    }

    /// Converts this Handle to a v8 Local if it isn't already.
    pub fn into_local<'a, 'p, P>(&self, _scope: &'a HandleScope<'a, 'p, P>) -> &v8::Local<T> {
        match &self.base {
            HandleKind::Local(local) => local,
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO: deref should return an Option
        unsafe { self.val() }
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // TODO: deref_mut should return an Option
        unsafe { self.val_mut() }
    }
}

impl<T> From<v8::Local<T>> for Handle<T> {
    fn from(local: v8::Local<T>) -> Self {
        Handle {
            base: HandleKind::Local(local),
        }
    }
}
