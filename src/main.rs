mod counter;
use counter::Counter;

fn main() {
    let mut counter = Counter::new();

    println!("initial count is {}", counter.count());

    counter.enable();
    counter.tick();

    println!("count after one tick is {}", counter.count());
}
