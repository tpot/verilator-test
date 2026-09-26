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

fn main() {
    let mut counter = ffi::new_counter();

    println!("initial count is {}", counter.count());

    counter.pin_mut().enable();
    counter.pin_mut().tick();

    println!("count after one tick is {}", counter.count());
}
