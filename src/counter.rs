use crate::ffi;


pub struct Counter {
    inner: cxx::UniquePtr<ffi::Counter>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_counter(),
        }
    }

    pub fn tick(&mut self) {
        self.inner.pin_mut().tick();
    }

    pub fn reset(&mut self) {
        self.inner.pin_mut().reset();
    }

    pub fn enable(&mut self) {
        self.inner.pin_mut().enable();
    }

    pub fn disable(&mut self) {
        self.inner.pin_mut().disable();
    }

    pub fn count(&self) -> u8 {
        self.inner.count()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
