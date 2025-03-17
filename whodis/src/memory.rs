use std::collections::HashMap;

// This implementation is all wrong, just trying to sketch out the API.
// Idea: some representation corresponding to mapped sections of the binary?

/// Static program memory.
#[derive(Default)]
pub struct ImageMemory {
    u32s: HashMap<u32, u32>,
}

impl ImageMemory {
    pub fn read_u32(&self, addr: u32) -> Option<u32> {
        self.u32s.get(&addr).copied()
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) {
        // XXX overlapping writes etc
        self.u32s.insert(addr, value);
    }
}
