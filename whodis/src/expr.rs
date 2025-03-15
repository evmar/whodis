fn ser_reg<S: serde::Serializer>(reg: &iced_x86::Register, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!("{:?}", reg))
}

fn deser_reg<'de, D: serde::Deserializer<'de>>(_d: D) -> Result<iced_x86::Register, D::Error> {
    unimplemented!()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    #[serde(serialize_with = "ser_reg")]
    #[serde(deserialize_with = "deser_reg")]
    Reg(iced_x86::Register),
    Imm(u32),
    Math(Box<ExprMath>),
    Mem(),
    //BinOp(Box<Expr>, iced_x86::Code, Box<Expr>),
}

pub fn from_op(instr: &iced_x86::Instruction, op: u32) -> Expr {
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Reg(reg) => write!(f, "{:?}", reg),
            Expr::Imm(imm) => write!(f, "{:#x}", imm),
            Expr::Math(math) => write!(f, "{} {} {}", math.lhs, math.op, math.rhs),
            Expr::Mem() => write!(f, "[TODO]"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExprMath {
    pub op: char,
    pub lhs: Expr,
    pub rhs: Expr,
}
