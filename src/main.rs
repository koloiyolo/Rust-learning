#![allow(unused)]
use std::{io};

use crate::basic::input_str;
mod set1;
mod set2;
mod set3;
mod set4;
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
        4 => set4::start(),
        10 => println!("X{}X", input_str().trim_end()),
        _ => println!("Wrong number")
    
    };
}

