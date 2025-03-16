use wasm_bindgen::prelude::*;

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
pub struct Instr {
    ip: u32,
    asm: String,
    effects: Vec<String>,
}

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
pub struct Block {
    start: usize,
    end: usize,
    effects: Vec<String>,
}

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Analysis {
    instrs: Vec<Instr>,
    blocks: Vec<Block>,
}

#[wasm_bindgen]
pub fn sample() -> Analysis {
    let instrs = whodis::decode::decode(&whodis::sample::CODE, whodis::sample::EIP);
    let blocks = whodis::decode::blocks(&instrs);

    let mut s = whodis::effect::State::new();

    Analysis {
        instrs: instrs
            .iter()
            .map(|i| Instr {
                ip: i.ip() as u32,
                asm: i.to_string(),
                effects: s.effects(i).iter().map(|e| e.to_string()).collect(),
            })
            .collect(),
        blocks: blocks
            .iter()
            .map(|b| Block {
                start: b.instrs.start,
                end: b.instrs.end,
                effects: b.effects.iter().map(|e| e.to_string()).collect(),
            })
            .collect(),
    }
}
