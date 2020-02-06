mod event;
mod platform;
pub use crate::{
    event::{Event},
};

use crate::{
    platform::{WindowImpl},
};

pub type WindowingResult<T> = Result<T, WindowingError>;
#[derive(Debug)]
pub enum WindowingError {
    BadCreation(String),
}

pub struct Window {
    window_impl: WindowImpl,
}
impl Window {
    pub fn new() -> WindowingResult<Window> {
        Ok(Window {
            window_impl: unsafe { WindowImpl::new()? },
        })
    }

    pub fn fetch_current_events(&mut self) -> impl Iterator<Item = Event> {
        unsafe { self.window_impl.fetch_current_events() }
    }
}
