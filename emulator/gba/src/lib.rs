use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use brave_emulator_common::{
    EmulatorCore,
    EmulatorCoreResult, EmulatorCoreError,
    memory::{Memory, MemoryRegion},
    instruction_sets::arm::{Arm32, Thumb32},
};
use brave_windowing::{Window};

pub struct GBACore {
    start_time: Instant,
}
impl GBACore {
    pub fn create(settings: GBASettings, window: &Window) -> EmulatorCoreResult<Self> {
        let _rom_path = settings.validate_rom()?;

        Ok(GBACore {
            start_time: Instant::now(),
        })
    }
}
impl EmulatorCore for GBACore {
    fn on_update(&mut self, delta: Duration) -> Duration {
        Duration::from_micros(13500)
    }

    fn on_pause(&mut self) {
    }

    fn on_resume(&mut self) {
    }
}

pub struct GBASettings {
    rom_path: PathBuf,
    bios_dir: PathBuf,
}
impl GBASettings {
    fn validate_rom(&self) -> EmulatorCoreResult<&Path> {
        match self.rom_path.extension() {
            Some(ext) => if ext == "gba" {
                Ok(self.rom_path.as_path())
            } else {
                Err(EmulatorCoreError::IncompatibleRom)
            },
            _ => Err(EmulatorCoreError::IncompatibleRom),
        }
    }
    fn validate_bios_file(&self) -> EmulatorCoreResult<PathBuf> {
        let bios_file = self.bios_dir.join("GBA_BIOS.bin");
        if !bios_file.is_file() {
            Err(EmulatorCoreError::BiosFileNotFound(bios_file.display().to_string()))
        } else {
            // TODO Check the length of the bios file since that is constant
            Ok(bios_file)
        }
    }
}

#[derive(Default)]
pub struct GBASettingsBuilder {
    rom_path: Option<PathBuf>,
    bios_dir: Option<PathBuf>,
}
impl GBASettingsBuilder {
    pub fn new() -> GBASettingsBuilder { Self::default() }

    pub fn with_rom_path(mut self, rom_path: impl Into<PathBuf>) -> Self {
        self.rom_path = Some(rom_path.into());
        self
    }
    pub fn with_bios_dir(mut self, bios_dir: impl Into<PathBuf>) -> Self {
        self.bios_dir = Some(bios_dir.into());
        self
    }

    pub fn build(self) -> Result<GBASettings, String> {
        let rom_path = self.rom_path.ok_or_else(|| "There must be a ROM path for GBA".to_string())?;
        let bios_dir = self.bios_dir.ok_or_else(|| "There must be a BIOS dir for GBA".to_string())?;

        Ok(GBASettings {
            rom_path,
            bios_dir,
        })
    }
}
