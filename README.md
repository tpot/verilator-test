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

CXX requires mutable methods on opaque C++ types to take `Pin<&mut T>`
in the bridge. Pinning prevents Rust from moving the underlying C++ object
through a mutable reference; it is more than a marker for mutation. See
the [CXX documentation on opaque C++ types](https://cxx.rs/extern-c++.html#opaque-c-types).

The Rust `Counter` wrapper owns a private `UniquePtr<ffi::Counter>` and
exposes ordinary `&mut self` methods. Each method calls `pin_mut()` internally,
so callers can write:

```rust
fn main() {
    let mut counter = Counter::new();

    println!("initial count is {}", counter.count());

    counter.enable();
    counter.tick();

    println!("count after one tick is {}", counter.count());
}
```
