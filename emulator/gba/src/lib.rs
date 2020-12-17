mod cpu;
mod memory;
mod settings;
pub use self::{
    settings::{GBASettings, GBASettingsBuilder},
};

use std::{
    time::{Duration},
};
use brave_emulator_common::{
    EmulatorCore,
    EmulatorCoreResult,
};
use brave_windowing::{Window};
use crate::{
    cpu::Cpu,
    memory::GBAMemory,
};

// const SAVE_INTERVAL: Duration = Duration::from_secs(60);
/// The GBA's CPU clock speed in hertz
const CLOCK_SPEED: usize = 16_780_000;

pub struct GBACore {
    settings: GBASettings,
    memory: GBAMemory,
    cpu: Cpu,
}
impl GBACore {
    pub fn create(settings: GBASettings, window: &Window) -> EmulatorCoreResult<GBACore> {
        let rom_path = settings::validate_rom_path(&settings)?;
        let bios_path = settings::validate_bios_path(&settings)?;
        let _save_path = settings::make_save_path(&settings);

        let memory = GBAMemory::new(rom_path, &bios_path)?;
        let cpu = Cpu::new();

        Ok(GBACore {
            settings,
            memory,
            cpu,
        })
    }
}
impl GBACore {
}
impl EmulatorCore for GBACore {
    fn on_update(&mut self, left_over: Duration) -> EmulatorCoreResult<Duration> {
        let mut cycles = 0;

        if self.cpu.is_init_state() {
            match self.cpu.read_next_instruction(&mut self.memory) {
                Ok(ran_cycles) => cycles += ran_cycles,
                Err(e) => panic!(e),
            }
        }
        // TODO Figure out how many cycles we'll want to go
        while cycles < 100 {
            match self.cpu.run_next_instruction(&mut self.memory) {
                Ok(ran_cycles) => cycles += ran_cycles,
                Err(e) => panic!(e),
            }
        }
        Ok(Duration::from_micros(0))
    }

    fn on_pause(&mut self) -> EmulatorCoreResult<()> {
        Ok(())
    }

    fn on_resume(&mut self) {
    }
}
