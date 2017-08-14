// use std::thread;

fn ackermann(m: u32, n: u32) -> u32 {
    match(m, n) {
        (0, n) => n+1,
        (m, 0) => ackermann(m-1, 1),
        (m, n) => ackermann(m-1, ackermann(m, n-1))
    }
}

fn main() {
    println!("___ Ackermann Function ___");

    for m in 0..5 {
        for n in 0..(16-m) {
            // let handle = thread::spawn(move || {
            //     println!("ackermann({}, {}) = {}", m, n, ackermann(m, n))
            // });
            // handle.join();
            println!("ackermann({}, {}) = {}", m, n, ackermann(m, n));
        }
    }
}