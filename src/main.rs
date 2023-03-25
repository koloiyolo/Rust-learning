#![allow(unused)]
use std::{io};
mod set1;
mod set2;

fn main() 
{
    println!("Which set to open?");
    let x = input_int();
    match x
    {
        1 => set1::start(),
        2 => set2::start(),
        _ => println!("Wrong number")
    
    };
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
