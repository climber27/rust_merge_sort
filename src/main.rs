fn main() {
    println!("Hello, world!");
    let x: i32 = 3;
    zero(x);
}

fn zero(mut x: i32) -> bool {
    println!("x equals {}", x);
    if x < 0 {
        x = 0;
    }
    if x == 0 {
        return true;
    }

    return zero(x-1);

}