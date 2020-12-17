use std::{
    fs,
    ops::{Deref, DerefMut},
    path::{Path},
};
use brave_emulator_common::{
    EmulatorCoreResult, EmulatorCoreError,
    memory::{Memory, MemoryRegion},
};

/// The BIOS file will always be 16Kb
const BIOS_FILE_SIZE: usize = 16 << 10;
/// The work RAM on the board is 256KB
const WRAM_ON_BOARD_SIZE: usize = 256 << 10;
/// The work RAM on the chip is 32KB
const WRAM_ON_CHIP_SIZE: usize = 32 << 10;
/// 1KB for the IO registers
const IO_REGISTERS_SIZE: usize = 1 << 10;
/// 1KB for the palette RAM
const PALETTE_RAM_SIZE: usize = 1 << 10;
/// 96KB for the Video RAM
const VRAM_SIZE: usize = 96 << 10;
/// 1KB for the Object Attribute Memory
const OAM_SIZE: usize = 1 << 10;
/// The gamepak can be a max of 32MB
const GAMEPAK_MAX_FILE_SIZE: usize = 32 << 20;
/// 32KB for the SRAM
const GAMEPAK_SRAM_SIZE: usize = 32 << 10;

const ADDRESS_START_BIOS: usize = 0x0000_0000;
const ADDRESS_END_BIOS: usize = ADDRESS_START_BIOS + BIOS_FILE_SIZE;
const ADDRESS_START_WRAM_BOARD: usize = 0x0200_0000;
const ADDRESS_END_WRAM_BOARD: usize = ADDRESS_START_WRAM_BOARD + WRAM_ON_BOARD_SIZE;
const ADDRESS_START_WRAM_CHIP: usize = 0x0300_0000;
const ADDRESS_END_WRAM_CHIP: usize = ADDRESS_START_WRAM_CHIP + WRAM_ON_CHIP_SIZE;
const ADDRESS_START_IO_REGISTERS: usize = 0x0400_0000;
const ADDRESS_END_IO_REGISTERS: usize = ADDRESS_START_IO_REGISTERS + IO_REGISTERS_SIZE;
const ADDRESS_START_PALETTE: usize = 0x0500_0000;
const ADDRESS_END_PALETTE: usize = ADDRESS_START_PALETTE + PALETTE_RAM_SIZE;
const ADDRESS_START_VRAM: usize = 0x0600_0000;
const ADDRESS_END_VRAM: usize = ADDRESS_START_VRAM + VRAM_SIZE;
const ADDRESS_START_OAM: usize = 0x0700_0000;
const ADDRESS_END_OAM: usize = ADDRESS_START_OAM + OAM_SIZE;
const ADDRESS_START_GAMEPAK_WAIT0: usize = 0x0800_0000;
const ADDRESS_END_GAMEPAK_WAIT0: usize = ADDRESS_START_GAMEPAK_WAIT0 + GAMEPAK_MAX_FILE_SIZE;
const ADDRESS_START_GAMEPAK_WAIT1: usize = 0x0A00_0000;
const ADDRESS_END_GAMEPAK_WAIT1: usize = ADDRESS_START_GAMEPAK_WAIT1 + GAMEPAK_MAX_FILE_SIZE;
const ADDRESS_START_GAMEPAK_WAIT2: usize = 0x0C00_0000;
const ADDRESS_END_GAMEPAK_WAIT2: usize = ADDRESS_START_GAMEPAK_WAIT2 + GAMEPAK_MAX_FILE_SIZE;
const ADDRESS_START_GAMEPAK_SRAM: usize = 0x0E00_0000;
const ADDRESS_END_GAMEPAK_SRAM: usize = ADDRESS_START_GAMEPAK_SRAM + GAMEPAK_SRAM_SIZE;

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

    pub fn get_cycles_for_address(&self, address: usize, access_width: AccessWidth) -> usize {
        match address {
            ADDRESS_START_BIOS..=ADDRESS_END_BIOS => 1,
            // TODO WRAM on the board needs to use waitstate settings
            ADDRESS_START_WRAM_BOARD..=ADDRESS_END_WRAM_BOARD => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 3,
                AccessWidth::Bit32 => 6,
            },
            ADDRESS_START_WRAM_CHIP..=ADDRESS_END_WRAM_CHIP => 1,
            ADDRESS_START_IO_REGISTERS..=ADDRESS_END_IO_REGISTERS => 1,
            // TODO Plus 1 cycle if video memory is being accessed at the same time
            ADDRESS_START_PALETTE..=ADDRESS_END_PALETTE => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 1,
                AccessWidth::Bit32 => 2,
            },
            ADDRESS_START_VRAM..=ADDRESS_END_VRAM => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 1,
                AccessWidth::Bit32 => 2,
            },
            ADDRESS_START_OAM..=ADDRESS_END_OAM => 1,
            // TODO All gamepak accesses need to use waitstate settings
            ADDRESS_START_GAMEPAK_WAIT0..=ADDRESS_END_GAMEPAK_WAIT0 => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 5,
                AccessWidth::Bit32 => 8,
            },
            ADDRESS_START_GAMEPAK_WAIT1..=ADDRESS_END_GAMEPAK_WAIT1 => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 5,
                AccessWidth::Bit32 => 8,
            },
            ADDRESS_START_GAMEPAK_WAIT2..=ADDRESS_END_GAMEPAK_WAIT2 => match access_width {
                AccessWidth::Bit8 | AccessWidth::Bit16 => 5,
                AccessWidth::Bit32 => 8,
            },
            ADDRESS_START_GAMEPAK_SRAM..=ADDRESS_END_GAMEPAK_SRAM => 5,
            _ => 0,
        }
    }
}
impl Deref for GBAMemory {
    type Target = Memory;
    fn deref(&self) -> &Memory { &self.0 }
}
impl DerefMut for GBAMemory {
    fn deref_mut(&mut self) -> &mut Memory { &mut self.0 }
}

#[derive(Copy, Clone)]
pub enum AccessWidth {
    Bit8,
    Bit16,
    Bit32,
}
