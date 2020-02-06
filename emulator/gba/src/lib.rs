use std::{
    path::{Path},
    time::{Duration},
};
use brave_emulator_common::{
    EmulatorCore, EmulatorCoreResult, EmulatorCoreError,
    memory::{Memory, MemoryRegion},
    instruction_sets::arm::{Arm32, Thumb32},
};
use brave_windowing::{Window};

pub struct GBACore {
}
impl EmulatorCore for GBACore {
    fn bios_file_name() -> Option<&'static str> { Some("GBA_BIOS.bin") }

    fn create(rom_path: &Path, bios_path: Option<&Path>, window: &Window) ->
    EmulatorCoreResult<Self> {
        Err(EmulatorCoreError::IncompatibleRom)
    }

    fn on_update(&mut self, delta: Duration) -> Duration {
        delta
    }

    fn on_pause(&mut self) {
    }

    fn on_resume(&mut self) {
    }
}
