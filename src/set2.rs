#![allow(unused)]

use std::{io, cmp::Ordering};
//zestaw 2

pub fn start() {
    println!("Which exercise to open?");
    let x = input_int();
    match x
    {
        1 => z1(),
        2 => z2(),
        3 => z3(),
        4 => z4(),
        _ => println!("Wrong number")
    
    };
}

//zadania

fn z1()
{
    println!("Input n:");
    let n = input_int();
    let mut x = 1;
    println!("Dividers of sqrt({}) are: ", n);
    let mut count = 0;
    while(x*x<=n){
        if(n%x==0){
        print!("{}, ",x);
        count+=1;
        }
        x+=1;
    }
    if n == 1 { println!("Not prime number");}
    else
    {match count.cmp(&1)
        {
            Ordering::Equal => println!("Prime number"),
            Ordering::Greater => println!("Not prime number"),
            Ordering::Less => println!("Wrong data")
        }
    }
}

fn z2()
{
    println!("Input chosen number please: ");
    let mut n = input_int();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut total = 1;
    print!("Values: ");
    while n != 1{
        total += n;
        print!("{}, ",n );
        if n%2==1 { n=3*n + 1; } 
        else { n=n/2; }
        if n< min { min = n; }
        else if n>max { max = n; }
    }
    println!("1");
    println!("Min = {}, Max = {}, total = {}", min, max, total);   
}

fn z3()
{
    println!("Hey, I am a calculator");
    let mut a= 0;
    let mut b = 0;
    let mut exit = false;
    while !exit {
        println!("Input your first number please");
        a = input_int();
        println!("Input your operator please: \n1. -, \n2. +, \n3. *,\n4. / ");
        let znak = input_str();
        println!("Input second number please");
        b = input_int();
        match znak.trim_end(){
            "-" => println!("{} - {} = {}", a, b, a - b),
            "+" => println!("{} + {} = {}", a, b, a + b),
            "*" => println!("{} * {} = {}", a, b, a * b),
            "/" => println!("{} / {} = {}", a, b, a / b),
            _ => println!("Wrong number")
        }
        let znak = input_str();
        if znak.trim_end() == "t" {exit = true;}
    }
    println!("Exited succesfully");
    
}

fn z4()
{
    println!("\n\n          Pine tree
    Please input the height of your pine tree: ");
    let mut height = input_int();
    let mut branches = 1;
    println!();
    while height != 0 {
        for i in 0 .. height-1 {
            print!(" ");
        }
        for i in 0.. branches {
            print!("*");
        }
        println!();
        height -= 1;
        branches += 2;
    }

    
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
