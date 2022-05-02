fn main() {
    // chapter 1.0
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // chapter 1.1
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // chapter 1.2
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!(
        "{} of {:b} people know binary, then other half doesn't",
        1, 2
    );
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` won't print...", Structure(3));
}
