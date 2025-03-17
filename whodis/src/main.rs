fn main() {
    let instrs = whodis::decode::decode(&whodis::sample::CODE, whodis::sample::EIP);

    for instr in &instrs {
        println!("{}", instr);
        let eff = whodis::effect::instr_effects(instr);
        for e in eff {
            println!("  {e:x?}");
        }
    }

    let memory = whodis::sample::memory();

    let mut state = whodis::effect::State::new(&memory);
    for instr in instrs {
        println!("{}", instr);
        for eff in state.effects(&instr) {
            println!("  {eff:x?}");
        }
    }
}
