fn message(arg: bool) {
    if arg {
        println!("hello");
    }

    else if !arg {
        println!("goodbye")
    }
}

fn main() {
    let greet_signal = true;
    message(greet_signal);
    message(!greet_signal)

}