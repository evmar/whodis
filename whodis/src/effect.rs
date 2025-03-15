use crate::expr::{ExprMath, from_op};

use super::expr::Expr;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Effect {
    Write(EffectWrite),
    TODO,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
                    dst: Expr::Mem(),
                    src,
                }),
            ]
        }

        Pop => {
            let dst = from_op(instr, 0);
            vec![
                Effect::Write(EffectWrite {
                    dst,
                    src: Expr::Mem(),
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

        Lea | Call | Test | Je | Ret => {
            vec![Effect::TODO]
        }

        Nop => {
            vec![]
        }

        m => todo!("effects for {:?}", m),
    }
}
