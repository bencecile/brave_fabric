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
    cpu::{Cpu},
    memory::{GBAMemory},
};

const SAVE_INTERVAL: Duration = Duration::from_secs(60);

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
        let cpu = Cpu::new(&memory)?;

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
        Ok(Duration::from_micros(13500))
    }

    fn on_pause(&mut self) -> EmulatorCoreResult<()> {
        Ok(())
    }

    fn on_resume(&mut self) {
    }
}
