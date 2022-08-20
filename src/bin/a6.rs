fn main() {
    let mut i = 0;

    while i < 10 {
        let modulo = i % 2;
        match modulo {
            0 => println!("is even!"),
            _ => println!("is odd!")
        }
        i = i + 1;
    }
    
}