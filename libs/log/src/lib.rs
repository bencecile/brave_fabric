use std::{
    ffi::{CStr},
};

#![feature(const_fn)]

/// The logger object.
/// This should be created before any log messages:
/// ```
/// const LOGGER: Logger = Logger::new_main(b"SomeMainLogger");
/// ...
/// log_debug!("Some debug message");
/// ```
pub struct Logger {
    log_buffer: liblog::log_id_t,
    tag: &'static CStr,
}
impl Logger {
    pub const fn new_system(tag: &'static [u8]) -> Logger {
        Self::new(liblog::log_id_t::LOG_ID_SYSTEM, tag)
    }
    pub const fn new_main(tag: &'static [u8]) -> Logger {
        Self::new(liblog::log_id_t::LOG_ID_MAIN, tag)
    }
}
impl Logger {
    const fn new(log_buffer: liblog::log_id_t, tag: &'static [u8]) -> Logger {
        Logger {
            log_buffer,
            // As long as any calling code isn't insane (nuls in the middle), this will be fine
            // We will see a performance gain instead of making a checked CStr for every log
            tag: unsafe { CStr::from_bytes_with_nul_unchecked(tag) },
        }
    }
}

/// All of the logging methods
impl Logger {
    #[cfg(debug_assertions)]
    #[inline]
    pub fn log_debug(&self, message: &str) {
    }
}
