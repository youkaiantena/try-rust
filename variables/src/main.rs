fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of const is: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
