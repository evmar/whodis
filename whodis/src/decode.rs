pub fn decode(code: &[u8], eip: u32) -> Vec<iced_x86::Instruction> {
    let decoder = iced_x86::Decoder::with_ip(32, code, eip as u64, iced_x86::DecoderOptions::NONE);
    decoder.into_iter().collect()
}

pub struct Block {
    pub instrs: std::ops::Range<usize>,
}

pub fn blocks(instrs: &[iced_x86::Instruction]) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut start = 0;
    for (i, instr) in instrs.iter().enumerate() {
        if instr.flow_control() != iced_x86::FlowControl::Next {
            blocks.push(Block {
                instrs: start..i + 1,
            });
            start = i + 1;
        }
    }
    if start < instrs.len() {
        blocks.push(Block {
            instrs: start..instrs.len(),
        });
    }
    blocks
}
