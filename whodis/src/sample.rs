//! Sample function while we develop.

use crate::{function::Function, memory::ImageMemory};

pub const CODE: &[u8] = &[
    0x83, 0xec, 0x50, 0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x04, 0x8b, 0x40, 0x04, 0x6a, 0x00, 0x8d, 0x4c,
    0x24, 0x08, 0x51, 0x50, 0xff, 0x15, 0x64, 0x52, 0x44, 0x00, 0x85, 0xc0, 0x74, 0x57, 0x8b, 0x56,
    0x04, 0x8b, 0x42, 0x04, 0x57, 0x8d, 0x4c, 0x24, 0x18, 0x51, 0x50, 0xff, 0x15, 0x68, 0x52, 0x44,
    0x00, 0x8b, 0x15, 0x68, 0xbc, 0x45, 0x00, 0x8b, 0x4c, 0x24, 0x0c, 0x8b, 0x7c, 0x24, 0x10, 0x52,
    0x8b, 0x54, 0x24, 0x18, 0x2b, 0xd1, 0x52, 0x8b, 0x54, 0x24, 0x10, 0x2b, 0xfa, 0x57, 0x51, 0x52,
    0x50, 0x8d, 0x4e, 0x28, 0xe8, 0x77, 0x3e, 0xff, 0xff, 0x8b, 0x4e, 0x04, 0x8b, 0x51, 0x04, 0x8d,
    0x44, 0x24, 0x18, 0x50, 0x52, 0xff, 0x15, 0x08, 0x52, 0x44, 0x00, 0x5f, 0x32, 0xc0, 0x5e, 0x83,
    0xc4, 0x50, 0xc2, 0x04, 0x00, 0x32, 0xc0, 0x5e, 0x83, 0xc4, 0x50, 0xc2, 0x04, 0x00, 0x90, 0x90,
    0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10, 0x57, 0x68, 0x9c, 0xf7, 0x44, 0x00, 0x8b, 0xf1, 0xe8, 0x5d,
    0x70, 0x02, 0x00, 0x8b, 0x44, 0x24, 0x14, 0x83, 0xc4, 0x08, 0x57, 0x50, 0x8b, 0xce, 0xe8, 0xbd,
    0x65, 0xff, 0xff, 0x5f, 0x5e, 0xc2, 0x08, 0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
];
pub const EIP: u32 = 0x40d580;

#[allow(non_snake_case)]
pub fn functions() -> [Function; 3] {
    let mut GetUpdateRect = Function {
        addr: 0x448814,
        name: "GetUpdateRect".to_string(),
        arg_count: 3,
        effects: vec![],
    };
    GetUpdateRect.stdcall_effects();
    let mut BeginPaint = Function {
        addr: 0x448806,
        name: "BeginPaint".to_string(),
        arg_count: 2,
        effects: vec![],
    };
    BeginPaint.stdcall_effects();
    let mut EndPaint = Function {
        addr: 0x4487fa,
        name: "EndPaint".to_string(),
        arg_count: 2,
        effects: vec![],
    };
    EndPaint.stdcall_effects();
    [GetUpdateRect, BeginPaint, EndPaint]
}

/// Memory address => value, after IAT is loaded.
const IAT_ENTRIES: &[(u32, u32)] = &[
    (0x445264, 0x448814), // GetUpdateRect
    (0x445268, 0x448806), // BeginPaint
    (0x445208, 0x4487fa), // EndPaint
];

pub fn memory() -> ImageMemory {
    let mut mem = ImageMemory::default();
    for &(addr, val) in IAT_ENTRIES.iter() {
        mem.write_u32(addr, val);
    }
    mem
}
