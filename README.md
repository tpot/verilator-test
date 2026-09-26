# verilator-test

A demonstration of using Verilator to compile a simple SystemVerilog
counter module into C++ and call it from Rust.

Usage:
```
$ cargo build
[lots of build stuff...]

$ cargo run
[some more build stuff which could probably be cleaned up...]
initial count is 0
count after one tick is 1
```

Calling C++ from Rust is interesting, as nearly always the `this` pointer for an object is mutable since we like to mutate state a lot in C++. This leads to messy code like the below as we need to tell the Rust compiler we are modifying memory.

```rust
fn main() {
    let mut counter = ffi::new_counter();

    println!("initial count is {}", counter.count());

    counter.pin_mut().enable();
    counter.pin_mut().tick();

    println!("count after one tick is {}", counter.count());
}
```

Interestingly, the C++ compiler catches this during compile-time with a mismatched `const` qualifier error:
```
warning: verilator-test@0.1.0: src/main.rs.cc:62:21: error: cannot initialize a variable of type 'void (Counter::*)() const' with an rvalue of type 'void (Counter::*)()': different qualifiers ('const' vs unqualified)
```
