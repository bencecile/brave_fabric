pub mod instruction_sets;
pub mod memory;

use std::{
    time::{Duration},
};

pub trait EmulatorCore {
    // TODO We will want to pass in any UI input events that occured between ticks.
    /// Called with the amount of time passed since the last update (delta).
    /// Returns the amount of time to wait until the next update.
    fn on_update(&mut self, delta: Duration) -> Duration;

    fn on_pause(&mut self);
    fn on_resume(&mut self);
}

pub type EmulatorCoreResult<T> = Result<T, EmulatorCoreError>;
#[derive(Clone, Debug)]
pub enum EmulatorCoreError {
    IncompatibleRom,
    BiosFileNotFound(String),
}
