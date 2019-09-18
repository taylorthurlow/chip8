use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

mod instruction;

pub struct CPU {
    // General-purpose registers
    // 16 8-bit data registers named V0 to VF. The VF register doubles as a flag
    // for some instructions; this, it should be avoided. In an addition
    // operation, VF is the carry flag, while in subtraction, it is the "no
    // borrow" flag. In the draw instruction, VF is set upon pixel collision.
    pub v: [u8; 16],
    pub i: u16, // index register
    pub pc: u16, // program counter

    // Opcodes
    // There are 35 opcodes in total. Opcodes are all 2 bytes long and stored
    // in big-endian. List: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
    pub opcode: u16,

    // System memory map (4KB memory)
    // 0x000 -> 0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050 -> 0x0A0 - Used for the built-in 4x5 pixel font set (0->F)
    // 0x200 -> 0xFFF - Program ROM and work RAM
    pub memory: [u8; 4096],

    // Graphics system
    // The chip 8 has one instruction that draws a sprite to the screen. Drawing
    // is done in XOR mode and if a pixel is turned off as a result of drawing,
    // the VF register is set. This is used for collision detection. The graphics
    // of the Chip 8 are black and white, and the screen has a total of 2048
    // pixels (64x32).
    pub gfx: [u8; 64 * 32],

    // The Chip 8 has no interrupts or hardware registers, but there are two
    // timer registers that count at 60Hz. When set above zero they will count
    // down to zero. The system's buzzer sounds whenever the sound timer reaches
    // zero.
    pub delay_timer: u16,
    pub sound_timer: u16,

    // The stack
    pub stack: [u16; 16],
    pub sp: u16,

    // HEX-based keypad (0x0 -> 0xF)
    // This array stores the current state of each key in the keypad.
    pub key: [u16; 16],
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            v: [0; 16],
            i: 0,
            pc: 0x200, // Program data starts at 0x200 or 512
            opcode: 0,
            memory: [0; 4096],
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }
}

impl CPU {
    pub fn initialize(&mut self) {
        // Clear display
        // Load fontset
    }

    pub fn load_program(&mut self, filename: &str) -> std::result::Result<(), std::io::Error> {
        let mut file = File::open(filename)?;
        // let mut buffer = [0; 4096];
        let mut buffer = vec![0u8];

        file.read_to_end(&mut buffer)?;

        for index in 0..buffer.len() - 1 {
            self.memory[index + 512] = buffer[index];
        }

        Ok(())
    }

    pub fn fetch_decode_execute(&mut self) -> Result<bool, &'static str> {
        // Fetch
        if self.pc < 4096 {
            self.opcode = read_word(self.memory, self.pc);
            self.pc += 16;

            // Decode
            match instruction::lookup(self.opcode) {
                Ok(i) => {
                    // Execute
                    let instruction: instruction::Instruction = i;
                    if instruction.opcode != 0x0000 {
                        println!("Executing opcode: {:#06X}", self.opcode);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            };

            Ok(false)
        } else {
            Ok(true)
        }
    }
}

// Fetch a single word from memory by fetching a byte at an index, fetching the
// next byte, and ORing the results after a bit shift.
fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | (memory[index as usize + 1] as u16)
}
