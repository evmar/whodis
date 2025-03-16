use crate::{
    effect::{Effect, EffectWrite},
    expr::Expr,
};

pub struct Function {
    pub addr: u32,
    pub name: String,
    pub arg_count: usize,
    pub effects: Vec<Effect>,
}

impl Function {
    pub fn stdcall_effects(&mut self) {
        // Stack arguments and return value consumed.
        let esp = Expr::new_reg(iced_x86::Register::ESP);
        self.effects.push(Effect::Write(EffectWrite {
            dst: esp.clone(),
            src: Expr::new_math(esp, '-', Expr::Imm((1 + self.arg_count) as u32 * 4)),
        }));

        // Register clobbers.
        // TODO: return value.
        self.effects.push(Effect::Write(EffectWrite {
            dst: Expr::new_reg(iced_x86::Register::EAX),
            src: Expr::Imm(0),
        }));
        self.effects.push(Effect::Write(EffectWrite {
            dst: Expr::new_reg(iced_x86::Register::ECX),
            src: Expr::Imm(0),
        }));
        self.effects.push(Effect::Write(EffectWrite {
            dst: Expr::new_reg(iced_x86::Register::EDX),
            src: Expr::Imm(0),
        }));
        // ebx, esdi, edi, ebp are preserved
    }
}
