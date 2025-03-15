use crate::expr::{ExprMath, from_op};

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
            let src = from_op(instr, 1);
            let dst = from_op(instr, 0);
            let val = Expr::Math(Box::new(ExprMath {
                op,
                lhs: dst.clone(),
                rhs: src,
            }));
            vec![Effect::Write(EffectWrite { dst, src: val })]
        }

        Push => {
            let src = from_op(instr, 0);
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
            let dst = from_op(instr, 0);
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
            let src = from_op(instr, 1);
            let dst = from_op(instr, 0);
            vec![Effect::Write(EffectWrite { dst, src })]
        }

        Lea => {
            let dst = from_op(instr, 0);
            let src = match from_op(instr, 1) {
                Expr::Mem(expr) => *expr,
                src => panic!("lea {}", src),
            };
            vec![Effect::Write(EffectWrite { dst, src })]
        }

        Je => {
            let dst = from_op(instr, 0);
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
