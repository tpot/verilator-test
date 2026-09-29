// Bridge between Rust and the verilated C++
#[cxx::bridge]
pub(crate) mod ffi {
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
