pub struct HartMask {
    pub mask: usize,
    pub base: usize,
}

impl HartMask {
    pub fn new(base: usize) -> Self {
        Self { mask: 0, base }
    }

    pub fn all() -> Self {
        Self { mask: 0, base: usize::MAX }
    }

    pub fn with_hart(mut self, hart_id: usize) -> Self {
        self.mask |= 1 << (hart_id - self.base);
        self
    }

    pub fn as_ptr(&self) -> *const usize {
        &self.mask
    }
}
