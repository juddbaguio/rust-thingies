fn main() {
    let mut i = 1;  
    loop {
        if i > 4 {
            break;
        }
        println!("{:?}",i);
        i = i + 1;
    }
}