use whodis_r::effect::instr_effects;

fn main() {
    let instrs = whodis_r::decode::sample();
    for instr in instrs {
        println!("{}", instr);
        let eff = instr_effects(&instr);
        for e in eff {
            let js = serde_json::to_string(&e).unwrap();
            println!("  {}", js);
        }
    }
}
