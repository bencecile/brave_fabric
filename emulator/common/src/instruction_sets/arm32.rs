pub enum Arm32 {
    Branch(Branch),
}
impl Arm32 {
    pub fn find_instruction(instruction: u32) -> Option<Arm32> {
        if let Some(branch) = Branch::from_instruction(instruction) {
            Some(Arm32::Branch(branch))
        } else {
            None
        }
    }
}

pub struct Branch {
    pub condition: Condition,
    /// If the condition is full, it's a BLX (switch to Thumb and this is the half-word offset).
    /// If not, false is branch and true is branch with link
    pub opcode: bool,
    /// The offset to the new instruction in bytes
    pub offset: i32,
}
impl Branch {
    fn from_instruction(instruction: u32) -> Option<Branch> {
        const IDENTIFIER: u32 = 0b00001010_00000000_00000000_00000000;
        const OPCODE_BIT: u32 = 0b00000001_00000000_00000000_00000000;
        const OFFSET_BYTES: u32 = 0b00000000_11111111_11111111_11111111;
        const OFFSET_SIGNED_BIT: u32 = 0b00000000_10000000_00000000_00000000;
        const OFFSET_SIGNED_EXTRA: u32 = 0b11111111_00000000_00000000_00000000;
        if instruction & IDENTIFIER == IDENTIFIER {
            let condition = Condition::from_instruction(instruction);
            let opcode = instruction & OPCODE_BIT == OPCODE_BIT;
            let mut offset = instruction & OFFSET_BYTES;
            if offset & OFFSET_SIGNED_BIT == OFFSET_SIGNED_BIT {
                // The offset is supposed to be negative so fill them out before converting
                offset |= OFFSET_SIGNED_EXTRA;
            }
            let offset = offset as i32 * 4;
            Some(Branch { condition, opcode, offset })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Condition {
    /// Flags: Z=1, equal (zero) (same)
    Equal,
    /// Flags: Z=0, not equal (nonzero) (not same)
    NEqual,
    /// Flags: C=1, unsigned higher or same (carry set)
    CarrySet,
    /// Flags: C=0, unsigned lower (carry cleared)
    CarryClear,
    /// Flags: N=1, signed negative (minus)
    Minus,
    /// Flags: N=0, signed positive or zero (plus)
    Positive,
    /// Flags: V=1, signed overflow (V set)
    VSet,
    /// Flags: V=0, signed no overflow (V cleared)
    VClear,
    /// Flags: C=1 and Z=0, unsigned higher
    UHigh,
    /// Flags: C=0 or Z=1, unsigned lower or same
    ULow,
    /// Flags: N=V, signed greater or equal
    GreaterEqual,
    /// Flags: N<>V, signed less than
    Less,
    /// Flags: Z=0 and N=V, signed greater than
    Greater,
    /// Flags: Z=1 or N<>V, signed less or equal
    LessEqual,
    /// Flags: None, always (the "AL" suffix can be omitted)
    Always,
    /// Flags: None, never (ARMv1,v2 only) (Reserved ARMv3 and up)
    AllSet,
}
impl Condition {
    fn from_instruction(instruction: u32) -> Condition {
        // We only want to match against the first 4 bits
        match instruction & 0xF000_0000 {
            0x0000_0000 => Condition::Equal,
            0x1000_0000 => Condition::NEqual,
            0x2000_0000 => Condition::CarrySet,
            0x3000_0000 => Condition::CarryClear,
            0x4000_0000 => Condition::Minus,
            0x5000_0000 => Condition::Positive,
            0x6000_0000 => Condition::VSet,
            0x7000_0000 => Condition::VClear,
            0x8000_0000 => Condition::UHigh,
            0x9000_0000 => Condition::ULow,
            0xA000_0000 => Condition::GreaterEqual,
            0xB000_0000 => Condition::Less,
            0xC000_0000 => Condition::Greater,
            0xD000_0000 => Condition::LessEqual,
            0xE000_0000 => Condition::Always,
            0xF000_0000 => Condition::AllSet,
            _ => panic!("Programming error"),
        }
    }
}
