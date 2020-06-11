use std::string::String;

pub struct Instruction {
    pub opcode: u16,
    pub category: String,
    pub description: String,
}

pub fn lookup(opcode: u16) -> Result<Instruction, String> {
    match opcode {
        0x0000 => Ok(Instruction {
            opcode,
            category: String::from("NOOP"),
            description: String::from("Perform no operation."),
        }),
        0x00E0 => Ok(Instruction {
            opcode,
            category: String::from("Display"),
            description: String::from("Clear the screen."),
        }),
        0x00EE => Ok(Instruction {
            opcode,
            category: String::from("Flow"),
            description: String::from("Return from a subroutine."),
        }),
        0x1000..=0x1FFF => Ok(Instruction {
            opcode,
            category: String::from("Flow"),
            description: String::from("Jump to address."),
        }),
        0x2000..=0x2FFF => Ok(Instruction {
            opcode,
            category: String::from("Flow"),
            description: String::from("Call subroutine."),
        }),
        0x3000..=0x3FFF => Ok(Instruction {
            opcode,
            category: String::from("Conditional"),
            description: String::from("Skip next instruction if VX equals NN."),
        }),
        0x4000..=0x4FFF => Ok(Instruction {
            opcode,
            category: String::from("Conditional"),
            description: String::from("Skip next instruction if VX not equal to NN."),
        }),
        0x5000..=0x5FFF => Ok(Instruction {
            opcode,
            category: String::from("Conditional"),
            description: String::from("Skip next instruction if VX equals VY."),
        }),
        0x6000..=0x6FFF => Ok(Instruction {
            opcode,
            category: String::from("Constant"),
            description: String::from("Set VX to NN."),
        }),
        0x7000..=0x7FFF => Ok(Instruction {
            opcode,
            category: String::from("Constant"),
            description: String::from("Add NN to VX. Carry flag is not changed."),
        }),
        0x8000..=0x8FFF => {
            // Match against rightmost hex digit
            match opcode & 0x000F {
                0x0 => Ok(Instruction {
                    opcode,
                    category: String::from("Assignment"),
                    description: String::from("Set VX to the value of VY."),
                }),
                0x1 => Ok(Instruction {
                    opcode,
                    category: String::from("Bitwise operation"),
                    description: String::from("Set VX to VX OR VY."),
                }),
                0x2 => Ok(Instruction {
                    opcode,
                    category: String::from("Bitwise operation"),
                    description: String::from("Sets VX to VX AND VY."),
                }),
                0x3 => Ok(Instruction {
                    opcode,
                    category: String::from("Bitwise operation"),
                    description: String::from("Set VX to VX XOR VY."),
                }),
                0x4 => Ok(Instruction {
                    opcode,
                    category: String::from("Math"),
                    description: String::from("Add VY to VX. VF is set to 1 when there's a carry, and 0 otherwise."),
                }),
                0x5 => Ok(Instruction {
                    opcode,
                    category: String::from("Math"),
                    description: String::from("Subtract VY from VX. VF is set to 0 when there's a borrow, and 1 otherwise."),
                }),
                0x6 => Ok(Instruction {
                    opcode,
                    category: String::from("Bitwise operation"),
                    description: String::from("Store the least significant bit of VX in VF an shifts VX right by 1."),
                }),
                0x7 => Ok(Instruction {
                    opcode,
                    category: String::from("Math"),
                    description: String::from("Set VX to VY minus VX. VF is set to 0 when there's a bottow, and 1 otherwise."),
                }),
                0xE => Ok(Instruction {
                    opcode,
                    category: String::from("Bitwise operation"),
                    description: String::from("Store the most significant bit of VX in VF an shifts VX left by 1."),
                }),
                _ => Err(format!("Opcode {} not found", format!("{:0>4X}", opcode))),
            }
        }
        0x9000..=0x9FFF => Ok(Instruction {
            opcode,
            category: String::from("Conditional"),
            description: String::from("Skip the next instruction if VX does not equal VY."),
        }),
        0xA000..=0xAFFF => Ok(Instruction {
            opcode,
            category: String::from("Memory"),
            description: String::from("Set I to the address NNN."),
        }),
        0xB000..=0xBFFF => Ok(Instruction {
            opcode,
            category: String::from("Flow"),
            description: String::from("Jump to the address NNN plus V0."),
        }),
        0xC000..=0xCFFF => Ok(Instruction {
            opcode,
            category: String::from("Random"),
            description: String::from(
                "Set VX to the result of a bitwise AND on a random number and NN.",
            ),
        }),
        0xD000..=0xDFFF => Ok(Instruction {
            opcode,
            category: String::from("Display"),
            description: String::from("Draw a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I's value doesn't change after the execution of this instruction. VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and 0 otherwise."),
        }),
        0xE09E | 0xE19E | 0xE29E | 0xE39E | 0xE49E | 0xE59E | 0xE69E | 0xE79E | 0xE89E | 0xE99E => Ok(Instruction {
            opcode,
            category: String::from("Key operation"),
            description: String::from("Skip the next instruction if the key stored in VX is pressed."),
        }),
        0xE0A1 | 0xE1A1 | 0xE2A1 | 0xE3A1 | 0xE4A1 | 0xE5A1 | 0xE6A1 | 0xE7A1 | 0xE8A1 | 0xE9A1 => Ok(Instruction {
            opcode,
            category: String::from("Key operation"),
            description: String::from("Skip the next instruction if the key stored in VX is not pressed."),
        }),
        0xF000..=0xFFFF => {
            match opcode & 0x0FFF {
                0x007 | 0x107 | 0x207 | 0x307 | 0x407 | 0x507 | 0x607 | 0x707 | 0x807 | 0x907 | 0xA07 | 0xB07 | 0xC07 | 0xD07 | 0xE07 | 0xF07 => Ok(Instruction {
                    opcode,
                    category: String::from("Timer"),
                    description: String::from("Set VX to the value of the delay timer."),
                }),
                0x00A | 0x10A | 0x20A | 0x30A | 0x40A | 0x50A | 0x60A | 0x70A | 0x80A | 0x90A | 0xA0A | 0xB0A | 0xC0A | 0xD0A | 0xE0A | 0xF0A => Ok(Instruction {
                    opcode,
                    category: String::from("Key operation"),
                    description: String::from("Await a key press, then store in VX (blocking operation)."),
                }),
                0x015 | 0x115 | 0x215 | 0x315 | 0x415 | 0x515 | 0x615 | 0x715 | 0x815 | 0x915 | 0xA15 | 0xB15 | 0xC15 | 0xD15 | 0xE15 | 0xF15 => Ok(Instruction {
                    opcode,
                    category: String::from("Timer"),
                    description: String::from("Set the delay timer to VX."),
                }),
                0x018 | 0x118 | 0x218 | 0x318 | 0x418 | 0x518 | 0x618 | 0x718 | 0x818 | 0x918 | 0xA18 | 0xB18 | 0xC18 | 0xD18 | 0xE18 | 0xF18 => Ok(Instruction {
                    opcode,
                    category: String::from("Sound"),
                    description: String::from("Set the sound timer to VX."),
                }),
                0x01E | 0x11E | 0x21E | 0x31E | 0x41E | 0x51E | 0x61E | 0x71E | 0x81E | 0x91E | 0xA1E | 0xB1E | 0xC1E | 0xD1E | 0xE1E | 0xF1E => Ok(Instruction {
                    opcode,
                    category: String::from("Memory"),
                    description: String::from("Add VX to I. VF is set to 1 when there is a range overflow (I + VX > 0xFFF), 0 otherwise."),
                }),
                0x029 | 0x129 | 0x229 | 0x329 | 0x429 | 0x529 | 0x629 | 0x729 | 0x829 | 0x929 | 0xA29 | 0xB29 | 0xC29 | 0xD29 | 0xE29 | 0xF29 => Ok(Instruction {
                    opcode,
                    category: String::from("Memory"),
                    description: String::from("Set I to the location of the sprite for the character in VX. Characters 0-F (in hex) are represented by a 4x5 font."),
                }),
                0x033 | 0x133 | 0x233 | 0x333 | 0x433 | 0x533 | 0x633 | 0x733 | 0x833 | 0x933 | 0xA33 | 0xB33 | 0xC33 | 0xD33 | 0xE33 | 0xF33 => Ok(Instruction {
                    opcode,
                    category: String::from("Binary-coded decimal"),
                    description: String::from("Stores the binary-coded decimal representation of VX, with the most significant 3 digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2."),
                }),
                0x055 | 0x155 | 0x255 | 0x553 | 0x455 | 0x555 | 0x655 | 0x755 | 0x855 | 0x955 | 0xA55 | 0xB55 | 0xC55 | 0xD55 | 0xE55 | 0xF55 => Ok(Instruction {
                    opcode,
                    category: String::from("Memory"),
                    description: String::from("Store V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I is left unmodified."),
                }),
                0x065 | 0x165 | 0x265 | 0x365 | 0x465 | 0x565 | 0x665 | 0x765 | 0x865 | 0x965 | 0xA65 | 0xB65 | 0xC65 | 0xD65 | 0xE65 | 0xF65 => Ok(Instruction {
                    opcode,
                    category: String::from("Memory"),
                    description: String::from("Fill V0 into VX (including VX) with values from memory starting address I. The offset from I is increased by 1 for each value written, but I is left unmodified."),
                }),
                _ => Err(format!("Opcode {} not found", format!("{:0>4X}", opcode))),
            }
        }
        _ => Err(format!("Opcode {} not found", format!("{:0>4X}", opcode))),
    }
}
