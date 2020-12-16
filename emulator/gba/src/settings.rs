use std::{
    path::{Path, PathBuf},
};
use brave_emulator_common::{EmulatorCoreResult, EmulatorCoreError};

#[derive(Default)]
pub struct GBASettingsBuilder {
    rom_path: Option<PathBuf>,
    bios_path: Option<PathBuf>,
    save_dir: Option<PathBuf>,
}
impl GBASettingsBuilder {
    pub fn new() -> GBASettingsBuilder { Self::default() }

    pub fn with_rom_path(mut self, rom_path: impl Into<PathBuf>) -> Self {
        self.rom_path = Some(rom_path.into());
        self
    }
    pub fn with_bios_path(mut self, bios_path: impl Into<PathBuf>) -> Self {
        self.bios_path = Some(bios_path.into());
        self
    }
    pub fn with_save_dir(mut self, save_dir: impl Into<PathBuf>) -> Self {
        self.save_dir = Some(save_dir.into());
        self
    }

    pub fn build(self) -> Result<GBASettings, String> {
        let rom_path = self.rom_path.ok_or_else(
            || "There must be a ROM path for GBA".to_string())?;
        let bios_path = self.bios_path.ok_or_else(
            || "There must be a BIOS path for GBA".to_string())?;
        let save_dir = if let Some(save_dir) = self.save_dir {
            save_dir
        } else {
            rom_path.parent()
                .ok_or_else(|| "The ROM path must be a file".to_string())?
                .into()
        };

        Ok(GBASettings {
            rom_path,
            bios_path,
            save_dir,
        })
    }
}

pub struct GBASettings {
    rom_path: PathBuf,
    bios_path: PathBuf,
    save_dir: PathBuf,
}

pub fn validate_rom_path(settings: &GBASettings) -> EmulatorCoreResult<&Path> {
    match settings.rom_path.extension() {
        Some(ext) => if ext == "gba" {
            Ok(settings.rom_path.as_path())
        } else {
            Err(EmulatorCoreError::IncompatibleRom)
        },
        _ => Err(EmulatorCoreError::IncompatibleRom),
    }
}
pub fn validate_bios_path(settings: &GBASettings) -> EmulatorCoreResult<PathBuf> {
    if settings.bios_path.is_file() {
        Ok(settings.bios_path.clone())
    } else {
        Err(EmulatorCoreError::InvalidBiosFile(settings.bios_path.display().to_string()))
    }
}
/// ROM validation must be performed before this one.
pub fn make_save_path(settings: &GBASettings) -> PathBuf {
    let rom_name = format!("{}.sav", settings.rom_path.file_stem().unwrap().to_str().unwrap());
    return settings.save_dir.join(rom_name);
}
