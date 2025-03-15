use wasm_bindgen::prelude::*;
use whodis::effect::Effect;

#[derive(tsify_next::Tsify, serde::Serialize, serde::Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Instr {
    ip: u32,
    asm: String,
    effects: Vec<Effect>,
}

#[wasm_bindgen]
pub fn sample() -> Vec<Instr> {
    let instrs = whodis::decode::sample();
    instrs
        .iter()
        .map(|i| Instr {
            ip: i.ip() as u32,
            asm: i.to_string(),
            effects: whodis::effect::instr_effects(i),
        })
        .collect()
}
