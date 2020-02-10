use std::{
    fs,
    ops::{Deref, DerefMut},
    path::{Path},
};
use brave_emulator_common::{
    EmulatorCoreResult, EmulatorCoreError,
    memory::{Memory, MemoryRegion},
};

pub const ADDRESS_START_BIOS: usize = 0x0000_0000;
pub const ADDRESS_START_WRAM_BOARD: usize = 0x0200_0000;
pub const ADDRESS_START_WRAM_CHIP: usize = 0x0300_0000;
pub const ADDRESS_START_IO_REGISTERS: usize = 0x0400_0000;
pub const ADDRESS_START_PALETTE: usize = 0x0500_0000;
pub const ADDRESS_START_VRAM: usize = 0x0600_0000;
pub const ADDRESS_START_OAM: usize = 0x0700_0000;
pub const ADDRESS_START_GAMEPAK_WAIT0: usize = 0x0800_0000;
pub const ADDRESS_START_GAMEPAK_WAIT1: usize = 0x0A00_0000;
pub const ADDRESS_START_GAMEPAK_WAIT2: usize = 0x0C00_0000;
pub const ADDRESS_START_GAMEPAK_SRAM: usize = 0x0E00_0000;

const BIOS_FILE_SIZE: usize = 16 << 10; // THe BIOS file will always be 16Kb
const GAMEPAK_MAX_FILE_SIZE: usize = 32 << 20; // The gampak can be a max of 32MB
const WRAM_ON_BOARD_SIZE: usize = 256 << 10; // The work RAM on the board is 256KB
const WRAM_ON_CHIP_SIZE: usize = 32 << 10; // The work RAM on the chip is 32KB
const IO_REGISTERS_SIZE: usize = 1 << 10; // 1KB for the IO registers
const PALETTE_RAM_SIZE: usize = 1 << 10; // 1KB for the palette RAM
const VRAM_SIZE: usize = 96 << 10; // 96KB for the Video RAM
const OAM_SIZE: usize = 1 << 10; // 1KB for the Object Attribute Memory
const GAMEPAK_SRAM_SIZE: usize = 32 << 10; //32KB for the SRAM

pub struct GBAMemory(Memory);
impl GBAMemory {
    pub fn new(rom_path: &Path, bios_path: &Path) -> EmulatorCoreResult<GBAMemory> {
        let bios_bytes = fs::read(bios_path)?;
        let rom_bytes = fs::read(rom_path)?;

        if bios_bytes.len() != BIOS_FILE_SIZE {
            return Err(EmulatorCoreError::InvalidBiosFile(bios_path.display().to_string()));
        }
        if rom_bytes.len() > GAMEPAK_MAX_FILE_SIZE {
            return Err(EmulatorCoreError::IncompatibleRom);
        }

        let memory = Memory::new(vec![
            MemoryRegion::new(ADDRESS_START_BIOS, bios_bytes),
            MemoryRegion::new(ADDRESS_START_WRAM_BOARD, vec![0; WRAM_ON_BOARD_SIZE]),
            MemoryRegion::new(ADDRESS_START_WRAM_CHIP, vec![0; WRAM_ON_CHIP_SIZE]),
            MemoryRegion::new(ADDRESS_START_IO_REGISTERS, vec![0; IO_REGISTERS_SIZE]),
            MemoryRegion::new(ADDRESS_START_PALETTE, vec![0; PALETTE_RAM_SIZE]),
            MemoryRegion::new(ADDRESS_START_VRAM, vec![0; VRAM_SIZE]),
            MemoryRegion::new(ADDRESS_START_OAM, vec![0; OAM_SIZE]),
            MemoryRegion::new(ADDRESS_START_GAMEPAK_WAIT0, rom_bytes.clone()),
            MemoryRegion::new(ADDRESS_START_GAMEPAK_WAIT1, rom_bytes.clone()),
            MemoryRegion::new(ADDRESS_START_GAMEPAK_WAIT2, rom_bytes),
            // TODO May need to do a save file for the SRAM
            MemoryRegion::new(ADDRESS_START_GAMEPAK_SRAM, vec![0; GAMEPAK_SRAM_SIZE]),
        ]);

        Ok(GBAMemory(memory))
    }
}
impl Deref for GBAMemory {
    type Target = Memory;
    fn deref(&self) -> &Memory { &self.0 }
}
impl DerefMut for GBAMemory {
    fn deref_mut(&mut self) -> &mut Memory { &mut self.0 }
}
