#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("src/Counter.h");

        type Counter;

        fn new_counter() -> UniquePtr<Counter>;
        fn tick(self: Pin<&mut Counter>);
        fn reset(self: Pin<&mut Counter>);
        fn enable(self: Pin<&mut Counter>);
        fn disable(self: Pin<&mut Counter>);
        fn count(&self) -> u8;
    }
}

/// Owns the C++ counter while keeping pinning inside the FFI implementation.
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

fn main() {
    let mut counter = Counter::new();

    println!("initial count is {}", counter.count());

    counter.enable();
    counter.tick();

    println!("count after one tick is {}", counter.count());
}
