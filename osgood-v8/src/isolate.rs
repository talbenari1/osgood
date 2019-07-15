use std::sync::Arc;

use super::binding;
use super::handle_scope::Scope;
use super::platform::Platform;

/// An isolated instance of the V8 engine.
///
/// Currently, isolates are not considered thread-safe.
#[derive(Debug)]
pub struct Isolate {
    _platform: Arc<Platform>,
    isolate_: *mut binding::v8::Isolate,
}

impl Isolate {
    /// Creates a new isolate.
    pub fn new() -> Self {
        let platform = Platform::get().clone();
        let isolate = unsafe { binding::isolate::new_isolate() };
        assert!(!isolate.is_null(), "Failed to initialize the V8 platform");
        Isolate {
            _platform: platform,
            isolate_: isolate,
        }
    }

    /// Sets this isolate as the currently active one for this thread.
    pub fn enter(&mut self) {
        unsafe { (*self.isolate_).Enter() };
    }

    /// Unsets this isolate as the currently active one for this thread. V8 keeps track of which
    /// isolate was previously entered and will set the most recent one as active.
    pub fn exit(&mut self) {
        let current_isolate = get_current();
        assert!(
            self.isolate_ == current_isolate,
            "Cannot exit isolate because another isolate is currently active"
        );
        unsafe { (*self.isolate_).Exit() };
    }
}

impl Drop for Isolate {
    fn drop(&mut self) {
        let is_in_use = unsafe { (*self.isolate_).IsInUse() };
        // We're only able to safely dispose the isolate when we're not panicking
        if !std::thread::panicking() {
            assert!(!is_in_use, "Cannot drop isolate that is currently in use");
            unsafe { (*self.isolate_).Dispose() };
        }
    }
}

impl Scope for Isolate {}

#[inline]
pub fn get_current() -> *mut binding::v8::Isolate {
    let isolate = unsafe { binding::v8::Isolate::GetCurrent() };
    assert!(
        !isolate.is_null(),
        "No isolate is currently active (did you forget to `enter()`?)"
    );
    isolate
}

#[cfg(test)]
mod test {
    use super::Isolate;

    #[test]
    fn isolate_new() {
        let _isolate = Isolate::new();
    }

    #[test]
    fn isolate_enter_exit() {
        let mut isolate = Isolate::new();

        isolate.enter();
        isolate.exit();
    }

    #[test]
    #[should_panic]
    fn isolate_enter_no_exit() {
        let mut isolate = Isolate::new();

        isolate.enter();
    }
}
