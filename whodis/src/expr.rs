#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn take(&mut self) -> Expr {
        std::mem::replace(self, Expr::Imm(0))
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

    pub fn simplify(&mut self) {
        match self {
            Expr::Math(math) => {
                math.simplify();
                self.constant_fold();
            }
            Expr::Mem(expr) => expr.simplify(),
            _ => {}
        }
    }

    fn constant_fold(&mut self) {
        let Expr::Math(math) = self else {
            return;
        };

        /// take the immediate value from an expression, potentially simplifying it to None
        fn take_imm(expr: &mut Expr) -> (Option<Expr>, i32) {
            match expr {
                Expr::Imm(val) => return (None, *val as i32),
                Expr::Math(inner) => {
                    if let Expr::Imm(val) = &*inner.rhs {
                        let val = *val as i32;
                        match inner.op {
                            '+' => return (Some(inner.lhs.take()), val),
                            '-' => return (Some(inner.lhs.take()), -val),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            (Some(expr.take()), 0)
        }

        let (lhs, lc) = take_imm(&mut *math.lhs);
        let (rhs, rc) = take_imm(&mut *math.rhs);
        let cfold = match math.op {
            '+' => lc + rc,
            '-' => lc - rc,
            _ => return,
        };

        let mut new = ExprMath::combine(lhs, rhs);
        if cfold != 0 {
            new = ExprMath::combine(new, Some(Expr::Imm(cfold as u32)));
        }

        *self = match new {
            Some(expr) => expr,
            None => Expr::Imm(0),
        };
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn simplify(&mut self) {
        self.lhs.simplify();
        self.rhs.simplify();
        // ensure constant on the right
        if self.op == '+' && self.lhs.is_imm() {
            std::mem::swap(&mut self.lhs, &mut self.rhs);
        }
    }

    fn combine(lhs: Option<Expr>, rhs: Option<Expr>) -> Option<Expr> {
        match (lhs, rhs) {
            (Some(lhs), None) => Some(lhs),
            (None, Some(rhs)) => Some(rhs),
            (Some(lhs), Some(rhs)) => Some(Expr::Math(ExprMath::new(lhs, '+', rhs))),
            _ => None,
        }
    }
}

// simple expression parser, only used in tests
#[cfg(test)]
mod parser {
    use std::{iter::Peekable, str::Chars};

    use super::*;

    pub fn parse(text: &str) -> Option<Expr> {
        let mut p = text.chars().peekable();
        parse_expr(&mut p)
    }

    fn parse_expr(p: &mut Peekable<Chars>) -> Option<Expr> {
        let mut expr = None;
        loop {
            let Some(&c) = p.peek() else {
                break;
            };
            match c {
                c if c.is_whitespace() => {
                    p.next();
                    continue;
                }
                c if c.is_digit(10) => {
                    expr = Some(Expr::Imm(parse_imm(p)));
                }
                '+' | '-' | '*' => {
                    p.next();
                    let rhs = parse_expr(p).unwrap();
                    expr = Some(Expr::new_math(expr.unwrap(), c, rhs));
                }
                '(' => {
                    p.next();
                    let inner = parse_expr(p).unwrap();
                    assert_eq!(p.next(), Some(')'));
                    assert!(expr.is_none());
                    expr = Some(inner);
                }
                ')' => break,
                c if c.is_alphabetic() => {
                    let reg = parse_reg(p).unwrap();
                    assert!(expr.is_none());
                    expr = Some(Expr::Reg(reg));
                }
                c => unimplemented!("{:?}", c),
            }
        }
        expr
    }

    fn parse_imm(p: &mut Peekable<Chars>) -> u32 {
        let mut imm = 0;
        while let Some(c) = p.peek() {
            if c.is_digit(10) {
                imm = imm * 10 + c.to_digit(10).unwrap();
                p.next();
            } else {
                break;
            }
        }
        imm
    }

    fn parse_reg(p: &mut Peekable<Chars>) -> Option<iced_x86::Register> {
        let mut reg = String::new();
        while let Some(&c) = p.peek() {
            if c.is_alphabetic() {
                reg.push(c);
                p.next();
            } else {
                break;
            }
        }
        let reg = match reg.as_str() {
            "eax" => iced_x86::Register::EAX,
            "ecx" => iced_x86::Register::ECX,
            "edx" => iced_x86::Register::EDX,
            "ebx" => iced_x86::Register::EBX,
            "esp" => iced_x86::Register::ESP,
            _ => unimplemented!(),
        };
        Some(reg)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse() {
            let exp = parser::parse("0").unwrap();
            assert_eq!(exp.to_string(), "0x0");

            let exp = parser::parse("0 + 1").unwrap();
            assert_eq!(exp.to_string(), "0x0 + 0x1");

            let exp = parser::parse("eax").unwrap();
            assert_eq!(exp.to_string(), "eax");

            let exp = parser::parse("(esp - 5) - 4").unwrap();
            assert_eq!(exp.to_string(), "esp - 0x5 - 0x4");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        let mut exp = parser::parse("0").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "0x0");
        let mut exp = parser::parse("1").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "0x1");

        let mut exp = parser::parse("eax + 1").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "eax + 0x1");
    }

    #[test]
    fn test_cfold() {
        let mut exp = parser::parse("0 + 1").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "0x1");

        let mut exp = parser::parse("1 - 1").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "0x0");

        let mut exp = parser::parse("(eax + 1) + 2").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "eax + 0x3");

        let mut exp = parser::parse("(eax + 1) + (ebx + 2)").unwrap();
        exp.simplify();
        assert_eq!(exp.to_string(), "eax + ebx + 0x3");

        // let mut exp = parser::parse("(esp - 5) - 4").unwrap();
        // exp.simplify();
        // assert_eq!(exp.to_string(), "esp - 0x9");
    }
}
