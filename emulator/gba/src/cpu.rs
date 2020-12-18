mod register;

use brave_emulator_common::{
    EmulatorCoreResult,
    instruction_sets::{Arm32, Thumb32, Condition},
    memory::{Memory, MemoryResult}
};

use crate::memory::{AccessWidth, GBAMemory};

use self::register::RegisterSet;

pub struct Cpu {
    registers: RegisterSet,
    fetcher: InstructionFetcher,
    decoded: CpuInstruction,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: RegisterSet::default(),
            fetcher: InstructionFetcher::default(),
            decoded: CpuInstruction::None("Init".to_string()),
        }
    }

    pub fn read_next_instruction(&mut self, memory: &mut GBAMemory) -> EmulatorCoreResult<usize> {
        self.fetcher.read_next_instruction(memory, self.registers.r15 as usize,
            self.registers.get_thumb_state())?;
        let cycles = memory.get_cycles_for_address(self.registers.r15 as usize, AccessWidth::Bit32);
        self.registers.r15 += 4;
        Ok(cycles)
    }

    pub fn run_next_instruction(&mut self, memory: &mut GBAMemory) -> EmulatorCoreResult<usize> {
        let mut cycles = 0;

        // Run an instruction first if we have one
        match self.decoded.take() {
            // TODO Run the instruction
            CpuInstruction::Arm(arm) => cycles += self.run_arm_instruction(memory, arm)?,
            CpuInstruction::Thumb(thumb) => cycles += self.run_thumb_instruction(memory, thumb)?,
            // Do nothing if none here since it could be init
            CpuInstruction::None(e) => println!("Empty instruction, {:?}", e),
        }

        self.decode_instruction();
        cycles += self.read_next_instruction(memory)?;
        Ok(cycles)
    }
}
impl Cpu {
    fn decode_instruction(&mut self) {
        self.decoded = self.fetcher.decode(self.registers.r15 as usize,
            self.registers.get_thumb_state());
        if let CpuInstruction::None(e) = &self.decoded {
            panic!("At address {:#X}, {}", self.registers.r15, e.clone());
        }
    }

    fn run_arm_instruction(&mut self, memory: &mut GBAMemory, arm: Arm32) ->
    EmulatorCoreResult<usize> {
        match arm {
            Arm32::Branch(branch) => {
                if branch.condition == Condition::AllSet {
                    // Do the special BLX here
                    self.registers.set_thumb_state(true);
                    self.registers.r14 = self.registers.r15;
                    self.registers.r15 = ((self.registers.r15 as i32) + branch.offset) as u32;
                    if branch.opcode {
                        // This wanted the other halfword
                        self.registers.r15 += 2;
                    }
                    Ok(1 + self.read_next_instruction(memory)?)
                } else if self.registers.does_condition_pass(branch.condition) {
                    if branch.opcode {
                        self.registers.r14 = self.registers.r15;
                    }
                    self.registers.r15 = ((self.registers.r15 as i32) + branch.offset) as u32;
                    Ok(1 + self.read_next_instruction(memory)?)
                } else {
                    Ok(1)
                }
            }
        }
    }

    fn run_thumb_instruction(&mut self, memory: &mut GBAMemory, thumb: Thumb32) ->
    EmulatorCoreResult<usize> {
        Ok(0)
    }
}

#[derive(Default)]
struct InstructionFetcher {
    address_of_bytes: usize,
    bytes: [u8; 4],
}
impl InstructionFetcher {
    pub fn read_next_instruction(&mut self, memory: &Memory, mut address: usize, is_thumb: bool)
    -> MemoryResult<()> {
        if is_thumb &&
            (self.address_of_bytes..self.address_of_bytes + 4).contains(&address) {
            // Just use the back half if we haven't yet
            return Ok(());
        }

        // Align the address to the word that's at or behind the address
        address -= address % 4;
        match memory.read(address, &mut self.bytes) {
            Ok(_) => {
                self.address_of_bytes = address;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn decode(&self, address: usize, is_thumb: bool) -> CpuInstruction {
        // TODO We need to raise an exception when the address is not aligned or inside our fetched instruction
        if is_thumb {
            let thumb_bytes: u16 = if address == self.address_of_bytes {
                u16::from_le_bytes([self.bytes[0], self.bytes[1]])
            } else {
                u16::from_le_bytes([self.bytes[2], self.bytes[3]])
            };
            match Thumb32::find_instruction(thumb_bytes) {
                Some(thumb_instruction) => CpuInstruction::Thumb(thumb_instruction),
                None => CpuInstruction::None(
                    format!("Failed to convert {:#X} ({:#b}) into Thumb32",
                        thumb_bytes, thumb_bytes)
                )
            }
        } else {
            let arm_bytes= u32::from_le_bytes(self.bytes);
            match Arm32::find_instruction(arm_bytes) {
                Some(arm_instruction) => CpuInstruction::Arm(arm_instruction),
                None => CpuInstruction::None(
                    format!("Failed to convert {:#X} ({:#b}) into ARM32", arm_bytes, arm_bytes)
                )
            }
        }
    }
}

// Depending on which mode is currently set
enum CpuInstruction {
    Arm(Arm32),
    Thumb(Thumb32),
    None(String),
}
impl CpuInstruction {
    pub fn take(&mut self) -> CpuInstruction {
        std::mem::replace(self, CpuInstruction::None("Taken".to_string()))
    }
}
