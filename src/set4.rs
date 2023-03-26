use std::io;

pub fn start()
{



    let string = "ala ma kota";
    
    println!("a appears {} times", count_occurrences(string, 'a'));
    // println!("Which exercise to open?");
    // let x = input_int();
    // match x
    // {
    //     1 => z1(),
    //     2 => z2(),
    //     // 3 => z3(),
    //     // 4 => z4(),
    //     _ => println!("Wrong number")
    
    // };
}


fn count_occurrences(text :&str, target: char) -> u32
{
    let mut count = 0;
    

    for chr in text.chars() {
        if chr == target {
            count +=1;
        }
        
    }
    return count;
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
