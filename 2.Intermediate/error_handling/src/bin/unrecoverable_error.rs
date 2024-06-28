// To run app go to rust-playground/2.Intermediate/error_handling folder
// cargo run  --bin unrecoverable_error

// To increase debug level run as follows
// RUST_BACKTRACE=full cargo run  --bin unrecoverable_error

// 0: This is the default setting. No backtrace is displayed when a panic occurs.
// 1: A backtrace is displayed when a panic occurs. This is useful for debugging to see where the panic originated.
// full: This provides a more detailed backtrace, including all frames. This can be particularly useful for in-depth debugging of complex issues.

fn main() {
    let count = vec![1, 2, 3];
    println!("{}",count[2]);

    // will panic above statement
    // println!("{}",count[5]);

    match count.get(5) {
        Some(val) => println!("Value is {}", val),
        None => println!("Index not found")
    }

    if let Some(val) = count.get(2) {
        println!("Value is {}", val);
    }

    // or_else method will return the original Option if it is Some(T).
    // If the original Option is None, it will call the provided closure f and return its result.
    println!("Index 1: {}", count.get(1).or_else(|| Some(&10)).unwrap());
    println!("Default value : {}", count.get(5).or_else(|| Some(&10)).unwrap());

    // default value -  diff between or and or_else
    println!("{:?}", count.get(6).or(Some(&7)));



}

