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
            description: String::from("Performs no operation."),
        }),
        0x00E0 => Ok(Instruction {
            opcode,
            category: String::from("Display"),
            description: String::from("Clears the screen."),
        }),
        _ => Err("Opcode not found"),
    }
}
