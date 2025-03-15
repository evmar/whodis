fn main() {
    let instrs = whodis::decode::sample();
    for instr in instrs {
        println!("{}", instr);
        let eff = whodis::effect::instr_effects(&instr);
        for e in eff {
            println!("  {e:?}");
        }
    }
}
