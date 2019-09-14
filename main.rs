// Fetch a single word from memory by fetching a byte at an index, fetching the
// next byte, and ORing the results after a bit shift.
fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | (memory[index as usize + 1] as u16)
}

fn main() {
    let mut cpu: CPU = CPU { ..Default::default() };

    cpu.fetch_decode_execute();
}
