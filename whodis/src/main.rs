fn main() {
    let instrs = whodis::decode::decode(&whodis::sample::CODE, whodis::sample::EIP);
    let memory = whodis::sample::memory();

    let mut state = whodis::effect::State::new(&memory);

    for instr in &instrs {
        println!("{}", instr);
        let eff = whodis::effect::instr_effects(instr);
        for e in &eff {
            println!("  {e:x?}");
        }
        state.run(eff);
    }

    for eff in state.effects() {
        println!("  {eff:x?}");
    }
}
