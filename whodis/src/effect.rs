use std::collections::HashMap;

use crate::memory::ImageMemory;

use super::expr::Expr;

#[derive(Debug)]
pub enum Effect {
    Write(EffectWrite),
    Jmp(Expr),
    TODO(String),
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Write(w) => write!(f, "{} = {}", w.dst, w.src),
            Effect::Jmp(dst) => write!(f, "→ {}", dst),
            Effect::TODO(msg) => write!(f, "TODO({})", msg),
        }
    }
}

#[derive(Debug)]
pub struct EffectWrite {
    pub dst: Expr,
    pub src: Expr,
}

pub fn instr_effects(instr: &iced_x86::Instruction) -> Vec<Effect> {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Add | Sub | Xor => {
            let op = match instr.mnemonic() {
                Add => '+',
                Sub => '-',
                Xor => '^',
                _ => unreachable!(),
            };
            let src = Expr::from_op(instr, 1);
            let dst = Expr::from_op(instr, 0);
            let val = Expr::new_math(dst.clone(), op, src);
            vec![Effect::Write(EffectWrite { dst, src: val })]
        }

        Push => {
            let src = Expr::from_op(instr, 0);
            vec![
                Effect::Write(EffectWrite {
                    dst: Expr::new_reg(iced_x86::Register::ESP),
                    src: Expr::new_math(Expr::new_reg(iced_x86::Register::ESP), '-', Expr::Imm(4)),
                }),
                Effect::Write(EffectWrite {
                    dst: Expr::Mem(Box::new(Expr::new_reg(iced_x86::Register::ESP))),
                    src,
                }),
            ]
        }

        Pop => {
            let dst = Expr::from_op(instr, 0);
            vec![
                Effect::Write(EffectWrite {
                    dst,
                    src: Expr::Mem(Box::new(Expr::new_reg(iced_x86::Register::ESP))),
                }),
                Effect::Write(EffectWrite {
                    dst: Expr::new_reg(iced_x86::Register::ESP),
                    src: Expr::new_math(Expr::new_reg(iced_x86::Register::ESP), '+', Expr::Imm(4)),
                }),
            ]
        }

        Mov => {
            let src = Expr::from_op(instr, 1);
            let dst = Expr::from_op(instr, 0);
            vec![Effect::Write(EffectWrite { dst, src })]
        }

        Lea => {
            let dst = Expr::from_op(instr, 0);
            let src = match Expr::from_op(instr, 1) {
                Expr::Mem(expr) => *expr,
                src => panic!("lea {}", src),
            };
            vec![Effect::Write(EffectWrite { dst, src })]
        }

        Je => {
            let dst = Expr::from_op(instr, 0);
            vec![Effect::Jmp(dst)]
        }

        Call => {
            let dst = Expr::from_op(instr, 0);
            vec![
                // push return address
                Effect::Write(EffectWrite {
                    dst: Expr::new_reg(iced_x86::Register::ESP),
                    src: Expr::new_math(Expr::new_reg(iced_x86::Register::ESP), '-', Expr::Imm(4)),
                }),
                Effect::Write(EffectWrite {
                    dst: Expr::Mem(Box::new(Expr::new_reg(iced_x86::Register::ESP))),
                    src: Expr::Imm(instr.next_ip() as u32),
                }),
                // jmp target
                Effect::Jmp(dst),
            ]
        }

        Ret => {
            let pop = match instr.op0_kind() {
                iced_x86::OpKind::Immediate16 => instr.immediate16(),
                _ => todo!(),
            };
            let pop = pop + 4; // return addr

            let return_address = Expr::Mem(Box::new(Expr::new_reg(iced_x86::Register::ESP)));
            vec![
                Effect::Write(EffectWrite {
                    dst: Expr::new_reg(iced_x86::Register::ESP),
                    src: Expr::new_math(
                        Expr::new_reg(iced_x86::Register::ESP),
                        '+',
                        Expr::Imm(pop as u32),
                    ),
                }),
                Effect::Jmp(return_address),
            ]
        }

        Test => vec![Effect::TODO("test".into())],

        Nop => vec![],

        m => todo!("effects for {:?}", m),
    }
}

pub struct State<'a> {
    memory: &'a ImageMemory,
    var: HashMap<String, Expr>,
}

impl<'a> State<'a> {
    pub fn new(memory: &'a ImageMemory) -> Self {
        let mut s = Self {
            memory,
            var: HashMap::new(),
        };
        s.initial_regs();
        s
    }

    fn initial_regs(&mut self) {
        for var in "eax ecx edx ebx esp ebp esi edi".split(' ') {
            self.var
                .insert(var.to_string(), Expr::Var(format!("${var}")));
        }
    }

    fn update_expr(&self, expr: &mut Expr) {
        match expr {
            Expr::Var(var) => {
                if let Some(e) = self.var.get(var) {
                    *expr = e.clone();
                }
            }
            Expr::Mem(addr) => {
                let addr = &mut **addr;
                self.update_expr(addr);
                match addr {
                    Expr::Imm(addr) => {
                        if let Some(val) = self.memory.read_u32(*addr) {
                            *expr = Expr::Imm(val);
                        }
                    }
                    _ => {}
                }
            }
            Expr::Math(math) => {
                self.update_expr(&mut math.lhs);
                self.update_expr(&mut math.rhs);
            }
            _ => {}
        }
        expr.simplify();
    }

    fn update_effect(&self, eff: &mut Effect) {
        match eff {
            Effect::Write(w) => match &w.dst {
                Expr::Var(_) => self.update_expr(&mut w.src),
                Expr::Mem(_) => {
                    self.update_expr(&mut w.dst);
                    self.update_expr(&mut w.src);
                }
                _ => {}
            },
            Effect::Jmp(dst) => self.update_expr(dst),
            _ => {}
        }
    }

    fn evolve(&mut self, eff: &Effect) {
        match eff {
            Effect::Write(w) => match &w.dst {
                Expr::Var(var) => {
                    self.var.insert(var.clone(), w.src.clone());
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn effects(&mut self, instr: &iced_x86::Instruction) -> Vec<Effect> {
        let mut effects = Vec::new();
        for mut eff in instr_effects(instr) {
            self.update_effect(&mut eff);
            self.evolve(&eff);
            effects.push(eff);
        }
        effects
    }
}

pub fn accumulate(memory: &ImageMemory, instrs: &[iced_x86::Instruction]) -> Vec<Effect> {
    let mut vars = Vec::new();
    let mut state = State::new(memory);
    let mut effects = Vec::new();
    for instr in instrs {
        for eff in state.effects(instr) {
            match &eff {
                Effect::Write(w) => match &w.dst {
                    Expr::Var(var) => {
                        if !vars.contains(var) {
                            vars.push(var.clone());
                        }
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            effects.push(eff);
        }
    }
    vars.sort();
    for var in vars {
        let src = state.var.get(&var).unwrap().clone();
        effects.push(Effect::Write(EffectWrite {
            dst: Expr::Var(var),
            src,
        }));
    }
    effects
}
