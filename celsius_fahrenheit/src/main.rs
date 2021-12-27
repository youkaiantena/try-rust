use std::io;

fn main() {
    println!("摂氏を華氏に、華氏を摂氏に変換します");

    loop {
        // 摂氏か華氏かを入力してもらう
        println!("摂氏を華氏にする場合は1、華氏を摂氏にする場合は2と入力してください");

        let mut _type = String::new();
        io::stdin().read_line(&mut _type)
            .expect("Failed to read line");
        
        let _type: u32 = match _type.trim().parse() {
            Ok(c_or_f) => c_or_f,
            Err(_) => continue
        };

        if _type != 1 && _type != 2{
            continue;
        }

        println!("温度を入力してください");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue
        };

        if _type == 1 {
            println!("摂氏{}度 => 華氏{}度", temperature, convert_celsius_to_fahrenheit(temperature));
        } else if _type == 2 {
            println!("華氏{}度 => 摂氏{}度", temperature, convert_fahrenheit_to_celsius(temperature));
        }

        break;
    }
}

fn convert_celsius_to_fahrenheit(temperature: f32) -> f32 {
    (temperature * 1.8) + 32.0
}

fn convert_fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) / 1.8
}