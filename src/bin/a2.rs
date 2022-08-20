fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display(arg: i32) {
    println!("{:?}", arg)
}
fn main() {
    let a = 1;
    let b = 2;

    let sum = add(a,b);
    display(sum)
}