use brave_emulator_common::{
    instruction_sets::{Arm32, Thumb32},
    memory::{Memory, MemoryResult},
};

use crate::memory::{AccessWidth, GBAMemory};

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

    #[inline]
    pub fn is_init_state(&self) -> bool {
        self.fetcher.address_of_bytes == 0 && self.fetcher.bytes == [0, 0, 0, 0]
    }

    pub fn read_next_instruction(&mut self, memory: &mut GBAMemory) -> MemoryResult<usize> {
        // TODO Check the Thumb state from the registers
        self.fetcher.read_next_instruction(memory, self.registers.r15 as usize, false)?;
        let cycles = memory.get_cycles_for_address(self.registers.r15 as usize, AccessWidth::Bit32);
        self.registers.r15 += 4;
        Ok(cycles)
    }

    pub fn run_next_instruction(&mut self, memory: &mut GBAMemory) -> MemoryResult<usize> {
        let mut cycles = 0;

        // Run an instruction first if we have one
        match self.decoded {
            // TODO Run the instruction
            CpuInstruction::Arm(_) => {}
            CpuInstruction::Thumb(_) => {}
            CpuInstruction::None(_) => (),
        }

        self.decode_instruction();
        cycles += self.read_next_instruction(memory)?;
        Ok(cycles)
    }
}
impl Cpu {
    fn decode_instruction(&mut self) {
        self.decoded = self.fetcher.decode(self.registers.r15 as usize, false);
        if let CpuInstruction::None(e) = &self.decoded {
            panic!(e.clone());
        }
    }
}

#[derive(Default)]
struct RegisterSet {
    r0: u32, r1: u32, r2: u32, r3: u32,
    r4: u32, r5: u32, r6: u32, r7: u32,
    r8: u32, r9: u32, r10: u32, r11: u32, r12: u32,
    /// Register 13 can also be used as a Stack Pointer
    r13: u32,
    /// Register 14 is the Link Register
    r14: u32,
    /// Register 15 is the Program Counter
    r15: u32,

    // Fast Interrupt specific registers
    r8_fiq: u32, r9_fiq: u32,
    r10_fiq: u32, r11_fiq: u32,
    r12_fiq: u32, r13_fiq: u32,
    r14_fiq: u32,

    // Supervisor specific registers
    r13_scv: u32,
    r14_scv: u32,

    // Abort specific registers
    r13_abt: u32,
    r14_abt: u32,

    // Interrupt specific registers
    r13_irq: u32,
    r14_irq: u32,

    // Undefined specific registers
    r13_und: u32,
    r14_und: u32,

    /// The current Program Status Register
    cpsr: u32,

    // Program status registers specific to each mode
    spsr_fiq: u32,
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_irq: u32,
    spsr_und: u32,
}

#[derive(Default)]
struct InstructionFetcher {
    address_of_bytes: usize,
    bytes: [u8; 4],
}
impl InstructionFetcher {
    pub fn read_next_instruction(&mut self, memory: &Memory, address: usize, is_thumb: bool)
    -> MemoryResult<()> {
        if is_thumb &&
            (self.address_of_bytes..self.address_of_bytes + 4).contains(&address) {
            // Just use the back half if we haven't yet
            return Ok(());
        }

        match memory.read(address, &mut self.bytes) {
            Ok(_) => {
                self.address_of_bytes = address;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn decode(&self, address: usize, is_thumb: bool) -> CpuInstruction {
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
