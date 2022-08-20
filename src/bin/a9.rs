enum Color {
    Red,
    Yellow,
    Blue
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("This is BLUE!"),
        Color::Red => println!("This is RED!"),
        Color::Yellow => println!("This is YELLOW!"),
    }
}

fn main() {
    print_color(Color::Blue);
    print_color(Color::Red);
    print_color(Color::Yellow);
}