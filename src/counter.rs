#![allow(dead_code)]

// Bridge between Rust and the verilated C++
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("src/Counter.h");

        type Counter;

        fn new_counter() -> UniquePtr<Counter>;

        fn count(&self) -> u8;

        fn tick(self: Pin<&mut Counter>);
        fn reset(self: Pin<&mut Counter>);
        fn enable(self: Pin<&mut Counter>);
        fn disable(self: Pin<&mut Counter>);
    }
}

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
