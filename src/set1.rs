#![allow(unused)]

use std::{io, cmp::Ordering};
use rand::{Rng, thread_rng};

pub fn main() {
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
    println!("Prosze podac temperature w Celsjuszach");
    let cels = input_float();
    println!("{} Celsjusza to {} Farenheita", cels, cels*1.8 +32.0);
}

fn z2() {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut var:i32;
    println!("Prosze podac 3 liczby");
    for i in 0..3{
        var = input_int();
        if var > max {
            max = var;
        }else if var < min {
            min = var;
        }
    }
    println!("Max to {}, Min to {}", max, min);
}

fn z3() {
    println!("Prosze podac wage(kg): ");
    let waga: f32 = input_float();
    println!("Prosze podac wzrost(m)");
    let mut wzrost: f32 = input_float();
    let mut bmi = waga/(wzrost*wzrost);
    if bmi < 18.5 {
        println!("Bmi wynosi {}, masz niedowage", bmi);
    } else if bmi < 25.0 {
        println!("Bmi wynosi {}", bmi);
    } else {
        println!("Bmi wynosi {}, masz nadwage", bmi);
    }
    
}

fn z4() {
    println!("Prosze podac dochod");
    let mut dochod: f32 = input_float();
    match dochod {
    0.0..=30_000.0 => println!("Twoj dochod wynosi {}zl, a podatek 0zl", dochod),
    30_000.0..=120_000.0 => println!("Twoj dochod wynosi {}zl, a podatek {}zl", dochod, (dochod*17.0) / 100.0 - 5100.0 ),
    120_000.0.. => println!("Twoj dochod wynosi {}zl, a podatek {}zl", dochod, ((dochod-120_000.0)*32.0) / 100.0 + 15300.0 ),
    _ => println!("Bledne dane")
    }
}

fn z5() {
    let x = rand::thread_rng().gen_range(1..101);
    let mut y = 0; 
    while x != y {
        println!("Prosze poddac liczbe");
        y = input_int();
        if x > y {
            println!("Za mala liczba");
        }else if x < y {
            println!("Zbyt duza liczba");
        }
    }
    println!("Wygrales, poprawna liczba to {}", x)
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn input_int() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<i32>().expect("Niepoprawna liczba");
    return x;
}

fn input_str() -> String
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Wystapil blad");
    return str;
}

fn input_float() -> f32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<f32>().expect("Niepoprawna liczba");
    return x;
}
