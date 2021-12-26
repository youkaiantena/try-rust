fn main() {
    // let number = 3;
    let mut number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    number = 0;
    if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("number was 0");
    }

    let condition = true;
    let number2: String = if condition {
        "five".to_string()
    } else {
        "six".to_string()
    };

    println!("The value of number is: {}", number2);
}
