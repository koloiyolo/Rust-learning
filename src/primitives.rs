use std::io;

pub fn primitives_start() {
    let border:i32;
    //starting point
    let number = 2;
    println!("Please input number");
    
    //set your border here
    //border = 50;
    
    //this line under should be used instead of line above but online compilator doesnt support input from user
    border = input_int();
    
    
    primitives(border, number);
}
//our function
pub fn primitives(border:i32, number:i32)
{
    let mut is_divided = false;
    //dont divide by zero you inbecille
    let mut i = 2;

    while i*i <= number{
        //if remainder from any number lesser than sqrt(our number) is zero, our number is not primitive
        if number % i == 0
        {
            is_divided = true;
        }
        i+=1;
    }

    if !is_divided {
        println!("{}", number);
    }
    //reccurency if border hasn't been reached
    if number<border
    {
        primitives(border, number + 1)
    }
    
}

//getting the input from the user

pub fn input_int() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).err();
    let x = str.trim_end().parse::<i32>().expect("Wrong number");
    x
}