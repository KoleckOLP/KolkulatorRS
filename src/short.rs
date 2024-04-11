
use std::io::{stdin, stdout, Write};

fn number_input(text: &str) -> f32 {
    loop {
        print!("{}", text);
        stdout().flush().unwrap();

        let mut number = String::new();

        stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() {
            Ok(num) => return num,
            Err(err) => {
                println!("Please input only numbers: {}", err); // this should not be panic :D
                continue;
            }
        };
    }
}

pub fn addition() { // 1
    println!("This is addition!");
    let num1 = number_input("input first number:");
    let num2 = number_input("input second number:");
    let result = num1 + num2;
    println!("{} + {} = {}", num1, num2, result);
}

pub fn subtraction() { // 2
    println!("This is subtraction!");
    let num1 = number_input("input first number:");
    let num2 = number_input("input second number:");
    let result = num1 - num2;
    println!("{} - {} = {}", num1, num2, result);
}

pub fn multiplicaion() { // 3
    println!("This is multiplicaion!");
    let num1 = number_input("input first number:");
    let num2 = number_input("input second number:");
    let result = num1 * num2;
    println!("{} * {} = {}", num1, num2, result);
}

pub fn division() { // 4
    println!("This is division!");
    let num1 = number_input("input first number:");
    let num2 = number_input("input second number:");
    let result = num1 / num2;
    println!("{} / {} = {}", num1, num2, result);
}

pub fn power() { // 5
    println!("This is power!");
    let num1 = number_input("input first number:");
    let num2 = number_input("input second number:");
    let result = num1.powf(num2);
    println!("{} to the power of {} is {}", num1, num2, result);
}

pub fn root() { // 6
    println!("This is root!");
    let num1 = number_input("input first number:");
    let result = num1.sqrt();
    println!("the square root of {} is {}", num1, result);
}

pub fn quadratic() { // 7
    println!("This is quadratic!");
}

pub fn storage() { // 8
    println!("This is storage!");
    let num1 = number_input("input first number:");
    let result = num1 * 1000.0 * 1000.0 * 1000.0 / 1024.0 / 1024.0 / 1024.0;
    println!("Manufacturer say {} GB but it's actually {} GB", num1, result)
}
