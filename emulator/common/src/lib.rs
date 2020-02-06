pub mod instruction_sets;
pub mod memory;

use std::{
    path::{Path},
    time::{Duration},
};
use brave_windowing::{Window};

pub trait EmulatorCore: Sized {
    /// The name of the BIOS file should it need one.
    fn bios_file_name() -> Option<&'static str>;

    /// Creates the Core with the path to the rom or an Error if the rom isn't compatible.
    /// Just the path is passed because the rom (like a PS2 or PSP ISO) may be too much to load.
    ///
    /// Only the bare minimum initialization should be done here.
    fn create(rom_path: &Path, bios_path: Option<&Path>, window: &Window)
    -> EmulatorCoreResult<Self>;

    // TODO We will want to pass in any UI input events that occured between ticks.
    /// Called with the amount of time passed since the last update (delta).
    /// Returns the amount of time to wait until the next update.
    fn on_update(&mut self, delta: Duration) -> Duration;

    fn on_pause(&mut self);
    fn on_resume(&mut self);
}

pub type EmulatorCoreResult<T> = Result<T, EmulatorCoreError>;
#[derive(Copy, Clone, Debug)]
pub enum EmulatorCoreError {
    IncompatibleRom,
}
