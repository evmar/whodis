use wasm_bindgen::prelude::*;

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
pub struct Instr {
    ip: u32,
    asm: String,
    lit: bool,
    effects: Vec<String>,
}

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
pub struct Block {
    start: usize,
    end: usize,
}

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Analysis {
    instrs: Vec<Instr>,
    blocks: Vec<Block>,
    effects: Vec<String>,
}

#[wasm_bindgen]
pub struct Program {
    memory: whodis::memory::ImageMemory,
}

#[wasm_bindgen]
pub fn init() -> Program {
    let memory = whodis::sample::memory();
    Program { memory }
}

#[wasm_bindgen]
pub fn sample(program: &Program, mut count: usize) -> Analysis {
    let instrs = whodis::decode::decode(&whodis::sample::CODE, whodis::sample::EIP);
    let blocks = whodis::decode::blocks(&instrs);

    let mut s = whodis::effect::State::new(&program.memory);

    let instrs: Vec<_> = instrs
        .iter()
        .map(|i| {
            let effects = whodis::effect::instr_effects(i);
            let instr_effects = effects.iter().map(|e| e.to_string()).collect();
            let mut lit = false;
            if count > 0 {
                lit = true;
                s.run(effects);
                count -= 1;
            }
            Instr {
                ip: i.ip() as u32,
                asm: i.to_string(),
                lit,
                effects: instr_effects,
            }
        })
        .collect();

    Analysis {
        instrs,
        blocks: blocks
            .iter()
            .map(|b| Block {
                start: b.instrs.start,
                end: b.instrs.end,
            })
            .collect(),
        effects: s.effects().iter().map(|e| e.to_string()).collect(),
    }
}
