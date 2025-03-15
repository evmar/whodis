#[derive(Debug, Clone)]
pub enum Expr {
    Reg(iced_x86::Register),
    Imm(u32),
    Math(ExprMath),
    Mem(Box<Expr>),
}

impl Expr {
    pub fn is_imm(&self) -> bool {
        matches!(self, Expr::Imm(_))
    }

    pub fn new_math(lhs: Expr, op: char, rhs: Expr) -> Expr {
        Expr::Math(ExprMath::new(lhs, op, rhs))
    }

    pub fn from_op(instr: &iced_x86::Instruction, op: u32) -> Expr {
        use iced_x86::OpKind::*;
        match instr.op_kind(op) {
            Register => Expr::Reg(instr.op_register(op)),
            Immediate8 => Expr::Imm(instr.immediate8() as u32),
            Immediate8to32 => Expr::Imm(instr.immediate8to32() as u32),
            Immediate16 => Expr::Imm(instr.immediate16() as u32),
            Immediate32 => Expr::Imm(instr.immediate32() as u32),
            Memory => {
                let mut expr = Expr::Imm(instr.memory_displacement32());

                if instr.memory_index() != iced_x86::Register::None {
                    let index = Expr::new_math(
                        Expr::Reg(instr.memory_index()),
                        '*',
                        Expr::Imm(instr.memory_index_scale()),
                    );
                    expr = Expr::new_math(index, '+', expr);
                }

                if instr.memory_base() != iced_x86::Register::None {
                    let base = Expr::Reg(instr.memory_base());
                    expr = Expr::new_math(base, '+', expr);
                }

                Expr::Mem(Box::new(expr))
            }

            NearBranch32 => Expr::Imm(instr.near_branch32() as u32),

            _ => todo!("op_expr for {:?}", instr.op_kind(op)),
        }
    }

    pub fn simplify(self) -> Expr {
        match self {
            Expr::Math(math) => {
                let (lhs, op, rhs) = (math.lhs.simplify(), math.op, math.rhs.simplify());

                match (&lhs, &rhs) {
                    (Expr::Imm(lhs), Expr::Imm(rhs)) => {
                        // constant fold
                        match op {
                            '+' => return Expr::Imm(lhs + rhs),
                            '-' => return Expr::Imm(lhs - rhs),
                            _ => {}
                        };
                    }
                    _ => {}
                }

                return Expr::new_math(lhs, op, rhs);
            }
            Expr::Mem(expr) => Expr::Mem(Box::new(expr.simplify())),
            expr => expr,
        }
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
    pub lhs: Box<Expr>,
    pub op: char,
    pub rhs: Box<Expr>,
}

impl ExprMath {
    pub fn new(lhs: Expr, op: char, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}
