#[derive(Debug, serde::Serialize)]
enum Effect {
    Write(EffectWrite),
    TODO,
}

#[derive(Debug, serde::Serialize)]
struct EffectWrite {
    dst: Expr,
    src: Expr,
}

fn ser_reg<S: serde::Serializer>(reg: &iced_x86::Register, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!("{:?}", reg))
}

#[derive(Debug, Clone, serde::Serialize)]
enum Expr {
    #[serde(serialize_with = "ser_reg")]
    Reg(iced_x86::Register),
    Imm(u32),
    Math(Box<ExprMath>),
    Mem(),
    //BinOp(Box<Expr>, iced_x86::Code, Box<Expr>),
}

#[derive(Debug, Clone, serde::Serialize)]
struct ExprMath {
    op: char,
    lhs: Expr,
    rhs: Expr,
}

fn op_expr(instr: &iced_x86::Instruction, op: u32) -> Expr {
    match instr.op_kind(op) {
        iced_x86::OpKind::Register => Expr::Reg(instr.op_register(op)),
        iced_x86::OpKind::Immediate8 => Expr::Imm(instr.immediate8() as u32),
        iced_x86::OpKind::Immediate8to32 => Expr::Imm(instr.immediate8to32() as u32),
        iced_x86::OpKind::Immediate16 => Expr::Imm(instr.immediate16() as u32),
        iced_x86::OpKind::Immediate32 => Expr::Imm(instr.immediate32() as u32),
        iced_x86::OpKind::Memory => Expr::Mem(),
        _ => todo!("op_expr for {:?}", instr.op_kind(op)),
    }
}

fn effects(instr: &iced_x86::Instruction) -> Vec<Effect> {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Add | Sub | Xor => {
            let op = match instr.mnemonic() {
                Add => '+',
                Sub => '-',
                Xor => '^',
                _ => unreachable!(),
            };
            let src = op_expr(instr, 1);
            let dst = op_expr(instr, 0);
            let val = Expr::Math(Box::new(ExprMath {
                op,
                lhs: dst.clone(),
                rhs: src,
            }));
            vec![Effect::Write(EffectWrite { dst, src: val })]
        }

        Push => {
            let src = op_expr(instr, 0);
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
            let dst = op_expr(instr, 0);
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
            let src = op_expr(instr, 1);
            let dst = op_expr(instr, 0);
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

fn main() {
    let paint: [u8; 175] = [
        0x83, 0xec, 0x50, 0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x04, 0x8b, 0x40, 0x04, 0x6a, 0x00, 0x8d,
        0x4c, 0x24, 0x08, 0x51, 0x50, 0xff, 0x15, 0x64, 0x52, 0x44, 0x00, 0x85, 0xc0, 0x74, 0x57,
        0x8b, 0x56, 0x04, 0x8b, 0x42, 0x04, 0x57, 0x8d, 0x4c, 0x24, 0x18, 0x51, 0x50, 0xff, 0x15,
        0x68, 0x52, 0x44, 0x00, 0x8b, 0x15, 0x68, 0xbc, 0x45, 0x00, 0x8b, 0x4c, 0x24, 0x0c, 0x8b,
        0x7c, 0x24, 0x10, 0x52, 0x8b, 0x54, 0x24, 0x18, 0x2b, 0xd1, 0x52, 0x8b, 0x54, 0x24, 0x10,
        0x2b, 0xfa, 0x57, 0x51, 0x52, 0x50, 0x8d, 0x4e, 0x28, 0xe8, 0x77, 0x3e, 0xff, 0xff, 0x8b,
        0x4e, 0x04, 0x8b, 0x51, 0x04, 0x8d, 0x44, 0x24, 0x18, 0x50, 0x52, 0xff, 0x15, 0x08, 0x52,
        0x44, 0x00, 0x5f, 0x32, 0xc0, 0x5e, 0x83, 0xc4, 0x50, 0xc2, 0x04, 0x00, 0x32, 0xc0, 0x5e,
        0x83, 0xc4, 0x50, 0xc2, 0x04, 0x00, 0x90, 0x90, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10, 0x57,
        0x68, 0x9c, 0xf7, 0x44, 0x00, 0x8b, 0xf1, 0xe8, 0x5d, 0x70, 0x02, 0x00, 0x8b, 0x44, 0x24,
        0x14, 0x83, 0xc4, 0x08, 0x57, 0x50, 0x8b, 0xce, 0xe8, 0xbd, 0x65, 0xff, 0xff, 0x5f, 0x5e,
        0xc2, 0x08, 0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    ];
    let eip = 0x40d580u32;

    let decoder =
        iced_x86::Decoder::with_ip(32, &paint, eip as u64, iced_x86::DecoderOptions::NONE);
    for instr in decoder {
        println!("{}", instr);
        let eff = effects(&instr);
        for e in eff {
            let js = serde_json::to_string(&e).unwrap();
            println!("  {}", js);
        }
    }
}
