#![allow(unused)]

use std::{io, cmp::Ordering};

//zestaw 2

pub fn start() {
    println!("Which exercise to open?");
    let x = input_int();
    match x
    {
        1 => z1(),
        // 2 => z2(),
        // 3 => z3(),
        // 4 => z4(),
        _ => println!("Wrong number")
    
    };
}

fn z1()
{
    let arr = [4.2, 5.7, 3.3, 7.7, -1.0, 1.8, 6.6, 1.5, 2.2, 7.9, -6.1, 4.3, 6.3, 3.3,
    3.2, 7.0, 2.9, 4.2, 4.8, 4.7];
    let mut total = 0.0;
    for i in 0..arr.len() {
        total += arr[i];
    }
    println!("Avereage temperature is: {:.1} *C", total/(arr.len() as f64));

}





//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
