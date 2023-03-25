#![allow(unused)]

use std::{io, cmp::Ordering};
use rand::{Rng, thread_rng};


pub fn start() {
    println!("Which exercise to open?");
    let x = input_int();
    match x
    {
        1 => z1(),
        2 => z2(),
        3 => z3(),
        4 => z4(),
        5 => z5(),
        _ => println!("Wrong number")
    
    };
}

fn z1(){
    println!("Please enter the temperature in Celsius");
    let cels = input_float();
    println!("{} Celsius is {} Farenheit", cels, cels*1.8 +32.0);
}

fn z2() {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut var:i32;
    println!("Please input 3 numbers");
    for i in 0..3{
        var = input_int();
        if var > max {
            max = var;
        }else if var < min {
            min = var;
        }
    }
    println!("Max is {}, Min is {}", max, min);
}

fn z3() {
    println!("Please input your weight(kg): ");
    let waga: f32 = input_float();
    println!("Please input your height(m): ");
    let mut wzrost: f32 = input_float();
    let mut bmi = waga/(wzrost*wzrost);
    if bmi < 18.5 {
        println!("Your BMI is {}, you are underweight", bmi);
    } else if bmi < 25.0 {
        println!("Your BMI is {}", bmi);
    } else {
        println!("Your BMI is {}, you are overweight", bmi);
    }
    
}

fn z4() {
    println!("Please input your yearly salary: ");
    let mut income: f32 = input_float();
    match income {
    0.0..=30_000.0 => println!("Your income is {} PLN, and tax is 0 PLN", income),
    30_000.0..=120_000.0 => println!("Your income is {} PLN, and tax is {} PLN", income, (income*17.0) / 100.0 - 5100.0 ),
    120_000.0.. => println!("Your income is {} PLN, and tax is {} PLN", income, ((income-120_000.0)*32.0) / 100.0 + 15300.0 ),
    _ => println!("Wrong data")
    }
}

fn z5() {
    let x = rand::thread_rng().gen_range(1..101);
    let mut y = 0; 
    while x != y {
        println!("Input your chosen number please: ");
        y = input_int();
        if x > y {
            println!("Number too low");
        }else if x < y {
            println!("Number too high");
        }
    }
    println!("You won, that's the correct answer {}", x)
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn input_int() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<i32>().expect("Wrong number");
    return x;
}

fn input_str() -> String
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("There was an error");
    return str;
}

fn input_float() -> f32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<f32>().expect("Niepoprawna liczba");
    return x;
}
