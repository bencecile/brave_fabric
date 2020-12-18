use brave_emulator_common::instruction_sets::Condition;

const THUMB_STATE_BIT: u32 = 0b00000000_00000000_00000000_00100000;

#[derive(Default)]
pub struct RegisterSet {
    pub r0: u32, pub r1: u32, pub r2: u32, pub r3: u32,
    pub r4: u32, pub r5: u32, pub r6: u32, pub r7: u32,
    pub r8: u32, pub r9: u32, pub r10: u32, pub r11: u32, pub r12: u32,
    /// Register 13 can also be used as a Stack Pointer
    pub r13: u32,
    /// Register 14 is the Link Register
    pub r14: u32,
    /// Register 15 is the Program Counter
    pub r15: u32,

    // Fast Interrupt specific registers
    pub r8_fiq: u32, pub r9_fiq: u32,
    pub r10_fiq: u32, pub r11_fiq: u32,
    pub r12_fiq: u32, pub r13_fiq: u32,
    pub r14_fiq: u32,

    // Supervisor specific registers
    pub r13_scv: u32, pub r14_scv: u32,

    // Abort specific registers
    pub r13_abt: u32, pub r14_abt: u32,

    // Interrupt specific registers
    pub r13_irq: u32, pub r14_irq: u32,

    // Undefined specific registers
    pub r13_und: u32, pub r14_und: u32,

    /// The current Program Status Register
    cpsr: u32,

    // Program status registers specific to each mode
    spsr_fiq: u32,
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_irq: u32,
    spsr_und: u32,
}
impl RegisterSet {
    pub fn does_condition_pass(&self, condition: Condition) -> bool {
        const NEGATIVE_BIT: u32 = 0b10000000_00000000_00000000_00000000;
        const ZERO_BIT: u32 = 0b01000000_00000000_00000000_00000000;
        const CARRY_BIT: u32 = 0b00100000_00000000_00000000_00000000;
        const OVERFLOW_BIT: u32 = 0b00010000_00000000_00000000_00000000;
        match condition {
            Condition::Equal => self.cpsr & ZERO_BIT == ZERO_BIT,
            Condition::NEqual => self.cpsr & ZERO_BIT == 0,
            Condition::CarrySet => self.cpsr & CARRY_BIT == CARRY_BIT,
            Condition::CarryClear => self.cpsr & CARRY_BIT == 0,
            Condition::Minus => self.cpsr & NEGATIVE_BIT == NEGATIVE_BIT,
            Condition::Positive => self.cpsr & NEGATIVE_BIT == 0,
            Condition::VSet => self.cpsr & OVERFLOW_BIT == OVERFLOW_BIT,
            Condition::VClear => self.cpsr & OVERFLOW_BIT == 0,
            Condition::UHigh => self.cpsr & CARRY_BIT == CARRY_BIT && self.cpsr & ZERO_BIT == 0,
            Condition::ULow => self.cpsr & CARRY_BIT == 0 || self.cpsr & ZERO_BIT == ZERO_BIT,
            Condition::GreaterEqual =>
                // The N and V bit must be equal, so shift the N bit to the same place as the V bit
                (self.cpsr & NEGATIVE_BIT >> 3) == (self.cpsr & OVERFLOW_BIT),
            Condition::Less =>
                (self.cpsr & NEGATIVE_BIT >> 3) != (self.cpsr & OVERFLOW_BIT),
            Condition::Greater => self.cpsr & ZERO_BIT == 0 &&
                (self.cpsr & NEGATIVE_BIT >> 3) == (self.cpsr & OVERFLOW_BIT),
            Condition::LessEqual => self.cpsr & ZERO_BIT == ZERO_BIT ||
                (self.cpsr & NEGATIVE_BIT >> 3) != (self.cpsr & OVERFLOW_BIT),
            Condition::Always => true,
            Condition::AllSet => false,
        }
    }

    pub fn get_thumb_state(&self) -> bool { self.cpsr & THUMB_STATE_BIT == THUMB_STATE_BIT }
    pub fn set_thumb_state(&mut self, state: bool) {
        if state {
            self.cpsr |= THUMB_STATE_BIT;
        } else {
            self.cpsr &= !THUMB_STATE_BIT;
        }
    }
}
