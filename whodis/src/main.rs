fn main() {
    let instrs = whodis::decode::decode(&whodis::sample::CODE, whodis::sample::EIP);

    for instr in &instrs {
        println!("{}", instr);
        let eff = whodis::effect::instr_effects(instr);
        for e in eff {
            println!("  {e:x?}");
        }
    }

    let mut mem = whodis::memory::ImageMemory::new();
    for &(addr, val) in whodis::sample::IAT_ENTRIES.iter() {
        mem.write_u32(addr, val);
    }

    let mut state = whodis::effect::State::new(&mem);
    for instr in instrs {
        println!("{}", instr);
        for eff in state.effects(&instr) {
            println!("  {eff:x?}");
        }
    }
}
