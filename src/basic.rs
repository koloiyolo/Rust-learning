use std::{io};

// file containing some basic functions to use across whole repository

pub fn input_int() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<i32>().expect("Wrong number");
    return x;
}

pub fn input_str() -> String
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("There was an error");
    return str;
}

pub fn input_float() -> f32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let x = str.trim_end().parse::<f32>().expect("Niepoprawna liczba");
    return x;
}