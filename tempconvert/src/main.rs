use std::io;

fn main() {
    let mode = input("Enter F to convert to Fahrenheit and C to convert to Celsius: ");
    let temp = input("Enter the number: ");
    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num, // Assigns the parsed number to `temp`
        Err(_) => {
            println!("invalid input, defaulting to 0");
            0
        },
    };
    match &mode as &str {
        "F" => println!("Temperature in Fahrenheit is: {}", ((temp * 9) / 5 + 32)),
        "C" => println!("Temperature in Celsius is: {}", (((temp - 32) * 5) / 9)),
        _ => println!("invalid input!"),
    };
}

fn input(msg: &str) -> String {
    println!("{}", msg);
    let mut v = String::new();
    io::stdin().read_line(&mut v).expect("Failed to read line");
    v.trim().to_string()
}
