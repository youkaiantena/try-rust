fn main() {
    another_function(5, 6);

    let x = plus_one(five());
    let y = {
        let x = 3;
        x + 1 // 式を返したい場合は;をつけてはいけない
    };

    println!("The main value of y is: {} and x is: {}", y, x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {} and y is: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}