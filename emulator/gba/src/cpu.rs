use brave_emulator_common::{
    EmulatorCoreResult,
    memory::{Memory},
    instruction_sets::arm::{Arm32, Thumb32},
};

pub struct Cpu {
    // TODO Keep track of all the registers
    // fetched_instruction: u32,
    // decoded_instruction: CpuInstruction,
}
impl Cpu {
    pub fn new(memory: &Memory) -> EmulatorCoreResult<Cpu> {
        // TODO Fetch the first 2 instructions so that our fetched and encoded instructions can always be set
        Ok(Cpu {
        })
    }
}

// Depending on which mode is currently set
// TODO Check if the decoded instructions stay when switching modes
enum CpuInstruction {
    Arm(Arm32),
    Thumb(Thumb32),
}
