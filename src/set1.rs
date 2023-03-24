#![allow(unused)]

use std::{io, cmp::Ordering};
use rand::{Rng, thread_rng};

pub fn main() {
    z5();
}

fn z1(){
    let mut celc = String::new();
    println!("Prosze podac temperature w Celsjuszach");
    io::stdin().read_line(&mut celc);
    let fare: f32 = celc.trim_end().parse::<f32>().expect("Niepoprawna liczba") * 1.8 + 32.0;
    println!("{} Celsjusza to {} Farenheita", celc, fare);
}

fn z2() {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut var = String::new();
    let mut parsed:i32;
    println!("Prosze podac 3 liczby");
    for i in 0..3{
        var.clear();
        io::stdin().read_line(&mut var);
        parsed = var.trim_end().parse().expect("Bledny ciag");
        if parsed > max {
            max = parsed;
        }else if parsed < min {
            min = parsed;
        }
    }
    println!("Max to {}, Min to {}", max, min);
}

fn z3() {
    let mut var = String::new();
    println!("Prosze podac wage(kg): ");
    io::stdin().read_line(&mut var);
    let mut waga: f32 = var.trim_end().parse().expect("Bledny ciag");
    var.clear();
    io::stdin().read_line(&mut var);
    println!("Prosze podac wzrost(m)");
    let mut wzrost: f32 = var.trim_end().parse().expect("Bledny ciag");
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
    let mut var = String::new();
    io::stdin().read_line(&mut var);
    let mut dochod: f64 = var.trim_end().parse().expect("Bledny znak");
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
    let mut var = String::new();
    while x != y {
        println!("Prosze poddac liczbe");
        io::stdin().read_line(&mut var);
        y = var.trim_end().parse().expect("Zly znak");
        var.clear();
        if x > y {
            println!("Za mala liczba");
        }else if x < y {
            println!("Zbyt duza liczba");
        }
    }
    println!("Wygrales, poprawna liczba to {}", x)
}
