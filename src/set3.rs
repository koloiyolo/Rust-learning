#![allow(unused)]

use std::{io, vec, cmp::Ordering, };

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
    let mut arr = [4.2, 5.7, 3.3, 7.7, -1.0, 1.8, 6.6, 1.5, 2.2, 7.9, -6.1, 4.3, 6.3, 3.3,
    3.2, 7.0, 2.9, 4.2, 4.8, 4.7];
    let mut total = 0.0;
    let mut min = f64::MAX;
    let mut standard_deviation = 0.0;
    let mut count_of_places = 0;

    for i in 0..arr.len() {
        total += arr[i];
        if arr[i] < min {min = arr[i];}
    }
    
    
    let med = total/(arr.len() as f64);
    println!("Avereage temperature is: {:.1} *C", med);
    
    for i in 0..arr.len() {
        if arr[i]>= med {count_of_places +=1;}
    }

    println!("In {} places temperature was higher than avereage.", count_of_places);

    for i in 0..arr.len() {
        standard_deviation += (arr[i]-med)*(arr[i]-med);
    }
    standard_deviation /= arr.len() as f64;

    println!("Standard deviation = {:.5}", standard_deviation.sqrt());
    
    //float sort xD
    let mut arr2: [i32; 20]= [0; 20];
    for i in 0.. arr.len() {
        arr2[i]= (arr[i]*10.0) as i32;
    }
    arr2.sort();
    let mut j =0;
    for i in 0..arr.len(){
        arr[i]=arr2[19-j] as f64;
        arr[i] /= 10.0;
        j+=1;
    }

    println!("Sorted array: ");
    for i in 0..arr.len(){
        print!(" {}*C, ", arr[i])
    }
}

// fn z2()
// {
    
// }




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
