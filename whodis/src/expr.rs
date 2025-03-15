#[derive(Debug, Clone)]
pub enum Expr {
    Reg(iced_x86::Register),
    Imm(u32),
    Math(Box<ExprMath>),
    Mem(Box<Expr>),
}

pub fn from_op(instr: &iced_x86::Instruction, op: u32) -> Expr {
    match instr.op_kind(op) {
        iced_x86::OpKind::Register => Expr::Reg(instr.op_register(op)),
        iced_x86::OpKind::Immediate8 => Expr::Imm(instr.immediate8() as u32),
        iced_x86::OpKind::Immediate8to32 => Expr::Imm(instr.immediate8to32() as u32),
        iced_x86::OpKind::Immediate16 => Expr::Imm(instr.immediate16() as u32),
        iced_x86::OpKind::Immediate32 => Expr::Imm(instr.immediate32() as u32),
        iced_x86::OpKind::Memory => {
            let mut expr = Expr::Imm(instr.memory_displacement32());

            if instr.memory_index() != iced_x86::Register::None {
                let index = Expr::Math(Box::new(ExprMath {
                    op: '*',
                    lhs: Expr::Reg(instr.memory_index()),
                    rhs: Expr::Imm(instr.memory_index_scale()),
                }));
                expr = Expr::Math(Box::new(ExprMath {
                    op: '+',
                    lhs: index,
                    rhs: expr,
                }));
            }

            if instr.memory_base() != iced_x86::Register::None {
                let base = Expr::Reg(instr.memory_base());
                expr = Expr::Math(Box::new(ExprMath {
                    op: '+',
                    lhs: base,
                    rhs: expr,
                }));
            }

            Expr::Mem(Box::new(expr))
        }
        _ => todo!("op_expr for {:?}", instr.op_kind(op)),
    }
}

fn reg_to_string(reg: &iced_x86::Register) -> String {
    format!("{:?}", reg).to_lowercase()
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Reg(reg) => write!(f, "{}", reg_to_string(reg)),
            Expr::Imm(imm) => write!(f, "{:#x}", imm),
            Expr::Math(math) => write!(f, "{} {} {}", math.lhs, math.op, math.rhs),
            Expr::Mem(expr) => write!(f, "[{}]", expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprMath {
    pub op: char,
    pub lhs: Expr,
    pub rhs: Expr,
}
