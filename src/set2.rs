#![allow(unused)]

use std::{io, cmp::Ordering};

//zestaw 2

pub fn main() {
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
    println!("Podaj n:");
    let n = input_int();
    let mut x = 1;
    println!("Dzielniki sqrt({}) to:", n);
    let mut count = 0;
    while(x*x<=n){
        if(n%x==0){
        println!("{}",x);
        count+=1;
        }
        x+=1;
    }
    if n == 1 { println!("Nie liczba pierwsza");}
    else
    {match count.cmp(&1)
        {
            Ordering::Equal => println!("Liczba pierwsza"),
            Ordering::Greater => println!("Nie liczba pierwsza"),
            Ordering::Less => println!("Bledne dane")
        }
    }
}

fn z2()
{
    println!("Prosze podac liczbe");
    let mut n = input_int();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut sum = 1;
    print!("Kolejne wart: ");
    while n != 1{
        sum += n;
        print!("{}, ",n );
        if n%2==1 { n=3*n + 1; } 
        else { n=n/2; }
        if n< min { min = n; }
        else if n>max { max = n; }
    }
    println!("1");
    println!("Min = {}, Max = {}, Suma = {}", min, max, sum);   
}

fn z3()
{
    println!("Witam, jestem kalkulatorem");
    let mut a= 0;
    let mut b = 0;
    let mut znak;
    let mut exit = false;
    while !exit {
        println!("Prosze wpisac pierwsza liczbe");
        a = input_int();
        println!("Prosze wybrac dzialnie: \n1. -, \n2. +, \n3. *,\n4. / ");
        znak = input_int();
        println!("Prosze wpisac druga liczbe");
        b = input_int();
        match znak{
            1 => println!("{} - {} = {}", a, b, a - b),
            2 => println!("{} + {} = {}", a, b, a + b),
            3 => println!("{} * {} = {}", a, b, a * b),
            4 => println!("{} / {} = {}", a, b, a / b),
            _ => println!("Bledna liczba")
        }
        
        println!("Wyjsc z programu t/n");
        znak = input_int();
        if znak == 0 {exit = true;}
    }
    println!("Poprawne zakonczenie programu");
    
}

pub fn z4()
{
    println!("\n\n          Choinka
    Prosze podac wysokosc choinki:");
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

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
