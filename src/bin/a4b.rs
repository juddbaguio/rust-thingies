fn message(arg: i32) {
    match arg {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}

fn main() {
    let greet_signal = 2;
    message(greet_signal);
    message(!greet_signal)
}