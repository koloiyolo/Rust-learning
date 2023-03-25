#![allow(unused)]
use std::{io};
mod set1;
mod set2;
mod set3;
mod basic;

fn main() 
{
    println!("Which set to open?");
    let x = basic::input_int();
    match x
    {
        1 => set1::start(),
        2 => set2::start(),
        3 => set3::start(),
        _ => println!("Wrong number")
    
    };
}

