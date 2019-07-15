use std::env::args;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex, Weak};

use super::binding;

lazy_static! {
    static ref GLOBAL_PLATFORM: Mutex<Weak<Platform>> = Mutex::new(Weak::new());
}

/// An initialized instance of the V8 platform.
///
/// This library will automatically, lazily initialize V8 and construct a platform instance if one
/// is not already active, and will tear down V8 once all references to the platform are dropped.
#[derive(Debug)]
pub struct Platform {
    phantom_: PhantomData<c_void>,
}

impl Platform {
    /// Obtains a reference to the V8 platform. As long as this reference is held, V8 will not be
    /// torn down.
    pub fn get() -> Arc<Platform> {
        let mut mutex = GLOBAL_PLATFORM.lock().unwrap();
        if let Some(platform) = mutex.upgrade() {
            platform
        } else {
            let args: Vec<String> = args().collect();
            let name = format!("{}\0", args[0]).as_ptr() as *const c_char;
            unsafe { binding::platform::init(name) };

            let platform = Arc::new(Platform {
                phantom_: PhantomData,
            });
            *mutex = Arc::downgrade(&platform);
            platform
        }
    }
}

#[cfg(test)]
mod test {
    use super::Platform;

    #[test]
    fn get_one_platform() {
        let platform = Platform::get().clone();

        drop(platform);
    }

    #[test]
    fn get_multiple_platforms() {
        let platform1 = Platform::get().clone();
        let platform2 = Platform::get().clone();

        drop(platform1);
        drop(platform2);
    }

    #[test]
    fn init_multiple_platforms() {
        {
            let platform1 = Platform::get().clone();
            drop(platform1);
        }

        {
            let platform2 = Platform::get().clone();
            drop(platform2);
        }
    }
}
