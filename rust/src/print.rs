pub fn run() {
    // basic print statment
    println!("hellow from print.rs");
    // println!(1) WQRONG
    println!("number: {}", 1);
    // having multiple placeholders
    println!("{} word and {} word", "first", "second");
    //positional arguments
    println!("{0}{1}{1}{2}", "first", "middle", "end");
    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "baseball"
    );
    // placeholder traits
    println!("binary{:b} hex {:x} octal {:o}", 10, 10, 10);

    // placeholder for dbg trait
    println!("{:?}", (10, true, "hello"));
    //basic math
    println!("10 + 10 = {}", 10 + 10);
}
