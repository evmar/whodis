use std::collections::HashMap;

use crate::expr::ExprMath;

use super::expr::Expr;

#[derive(Debug)]
pub enum Effect {
    Write(EffectWrite),
    Jmp(Expr),
    TODO,
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Write(w) => write!(f, "{} = {}", w.dst, w.src),
            Effect::Jmp(dst) => write!(f, "→ {}", dst),
            Effect::TODO => write!(f, "TODO"),
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
            let val = Expr::Math(Box::new(ExprMath {
                op,
                lhs: dst.clone(),
                rhs: src,
            }));
            vec![Effect::Write(EffectWrite { dst, src: val })]
        }

        Push => {
            let src = Expr::from_op(instr, 0);
            vec![
                Effect::Write(EffectWrite {
                    dst: Expr::Reg(iced_x86::Register::ESP),
                    src: Expr::Math(Box::new(ExprMath {
                        op: '-',
                        lhs: Expr::Reg(iced_x86::Register::ESP),
                        rhs: Expr::Imm(4),
                    })),
                }),
                Effect::Write(EffectWrite {
                    dst: Expr::Mem(Box::new(Expr::Reg(iced_x86::Register::ESP))),
                    src,
                }),
            ]
        }

        Pop => {
            let dst = Expr::from_op(instr, 0);
            vec![
                Effect::Write(EffectWrite {
                    dst,
                    src: Expr::Mem(Box::new(Expr::Reg(iced_x86::Register::ESP))),
                }),
                Effect::Write(EffectWrite {
                    dst: Expr::Reg(iced_x86::Register::ESP),
                    src: Expr::Math(Box::new(ExprMath {
                        op: '+',
                        lhs: Expr::Reg(iced_x86::Register::ESP),
                        rhs: Expr::Imm(4),
                    })),
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

        Call | Test | Ret => {
            vec![Effect::TODO]
        }

        Nop => {
            vec![]
        }

        m => todo!("effects for {:?}", m),
    }
}

#[derive(Default)]
pub struct State {
    reg: HashMap<iced_x86::Register, Expr>,
}

impl State {
    fn update_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Reg(reg) => self.reg.get(&reg).cloned().unwrap_or(expr),
            Expr::Mem(expr) => Expr::Mem(Box::new(self.update_expr(*expr))),
            Expr::Math(math) => {
                let lhs = self.update_expr(math.lhs);
                let rhs = self.update_expr(math.rhs);
                Expr::Math(Box::new(ExprMath {
                    op: math.op,
                    lhs,
                    rhs,
                }))
            }
            _ => expr,
        }
    }

    fn update_effect(&self, eff: Effect) -> Effect {
        match eff {
            Effect::Write(w) => Effect::Write(match &w.dst {
                Expr::Reg(_) => EffectWrite {
                    dst: w.dst,
                    src: self.update_expr(w.src),
                },
                Expr::Mem(_) => EffectWrite {
                    dst: self.update_expr(w.dst),
                    src: self.update_expr(w.src),
                },
                _ => w,
            }),
            Effect::Jmp(dst) => Effect::Jmp(self.update_expr(dst)),
            Effect::TODO => eff,
        }
    }

    fn evolve(&mut self, eff: &Effect) {
        match eff {
            Effect::Write(w) => match w.dst {
                Expr::Reg(reg) => {
                    self.reg.insert(reg, w.src.clone());
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn effects(&mut self, instr: &iced_x86::Instruction) -> Vec<Effect> {
        let mut effects = Vec::new();
        for eff in instr_effects(instr) {
            let eff = self.update_effect(eff);
            self.evolve(&eff);
            effects.push(eff);
        }
        effects
    }
}
