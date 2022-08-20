fn message(arg: bool) {
    match arg {
        true => println!("it's true!"),
        false => println!("it's false!"),
    }
}

fn main() {
    let greet_signal = true;
    message(greet_signal);
    message(!greet_signal)

}