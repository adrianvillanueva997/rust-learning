use std::{thread, time};

fn main() {
    let start = time::Instant::now();
    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
    });
    handler.join().unwrap();
    let finish = time::Instant::now();
    println!("Elapsed: {:?}", finish - start);
}
