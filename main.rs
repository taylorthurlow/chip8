mod cpu;

fn main() {
    let mut cpu: cpu::CPU = cpu::CPU { ..Default::default() };

    cpu.initialize();

    match cpu.load_program("test_opcode.ch8") {
        Ok(_) => println!("Loaded program successfully."),
        Err(e) => eprintln!("Program load failed: {}", e),
    }

    loop {
        match cpu.fetch_decode_execute() {
            Ok(reached_end) => {
                if reached_end {
                    break;
                }
            }
            Err(e) => eprintln!("Error in fetch/decode/execute: {}", e),
        }
    }
}
