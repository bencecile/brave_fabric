pub mod instruction_sets;
pub mod memory;

use std::{
    io::{Error as IOError},
    time::{Duration},
};

pub trait EmulatorCore {
    // TODO We will want to pass in any UI input events that occured between ticks.
    fn on_update(&mut self, left_over: Duration) -> EmulatorCoreResult<Duration>;

    fn on_pause(&mut self) -> EmulatorCoreResult<()>;
    fn on_resume(&mut self);
}

pub type EmulatorCoreResult<T> = Result<T, EmulatorCoreError>;
#[derive(Debug)]
pub enum EmulatorCoreError {
    IOError(IOError),

    IncompatibleRom,
    InvalidBiosFile(String),
}
impl From<IOError> for EmulatorCoreError {
    fn from(error: IOError) -> Self { Self::IOError(error) }
}
