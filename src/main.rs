extern crate core;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::ffi::c_void;
use std::io::Bytes;
use std::os::linux::raw::stat;


fn main() {

}


fn control_flow(){

}

fn functions(){

    fn another_function(x: i32){
        println!("the value of x is {x}");
    }

    fn print_labeled_measurement(value : i32, unit_label: char){
        println!("the measurment is {value}{unit_label}");
    }

    fn statements_expressions(){
        //let x =(let y = 6); not allowed
        let y = {
            let x = 5;
            x + 1 //note: no semicolon as this would become a statement then with no return value
        };
    }

    fn five() -> i32{
        // we can also return using the "return" keyword
        5
    }

    fn plus_one(x: i32) -> i32{
        x + 1
    }



}

fn data_types(){
    //constant decleration

    //const name:datatype = value;
    const NUMBER: i32 = 20;
    const FLOAT: f32 = 21.2;

    //variable decleration
    let _ff:char = 's';
    let _float:f64 = 24.11;

    //if we add the 'mut' modifier a variable becomes mutable
    let mut _lm:i32 =23;

    // if the DataType is u-unsigned then the data cannot be negative
    let mut _uns:usize = 23;
    let mut _sin:isize =-23;

    // dynamic dataType allocation
    let _apples = 23;
    let _figgs = -23;
    let _semen = 23.23;
    let _pears = "pears";

    // rust scopes are a wonderful a crucial part of the language

    let x = 5;
    let x = x + 1;//mut modifier is not needed to increase the number because we are shadowing it

    // if we attempt to change the value of x in the form of x = x+1 then we will get a compile time error

    {// by adding a curly brackets we create a new scope for the program
        //anything inside will behave separately from the outside;

        let x = x * 2;
        println!("this is x:{}",x);

    }// once we leave the scope we return to all values outside
    println!("this is original x:{}",x);

    // this would not be possible to do with mut flag.
    let spaces = "   ";
    let spaces = spaces.len();

    let _decimal:i64 = 1_829_642;//underscore can be used to make things clearer
    let _heximal:i64 = 0xfba;
    let _octimal:i64 = 0o723;
    let _binary :i64 = 0b1101_0011;
    let _bytimal:u8  = b'A';

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let seven_eighths = '⅞';

    //tuples are fixed length mutli datatype list
    let tup: (i32, f64, u8) =(500, 6.4, 1);
    let tup2 = (22,64.3,'c',"ss");

    let (x,y,z,o) = tup2;

    println!("the values in tup 2 are {x},{y},{z},{o}");

    // we can also acess the values within a tuple like this:
    let twenty_two = tup2.0;
    let ss = tup2.3;


    let array = ["monday","tuesday","wednesday"];
    let arr: [i32; 5] = [1,2,3,4,5];// the five after the semicolon indicates the size

    let arrr = [3;5]; // fills the array with five, threes.

    //elements of an array
    let first = arr[0];
    let second = array[2];



}

fn guessing_game(){

    /*------------------------*/
    /*-rust docs walk through--*/
    /*------------------------*/


    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //^the random number generator uses the os threading system.

    loop {
        println!("please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)// calls readline method within the IO library
            .expect("Failed to read line");// this line expects an error from the previous command. 2 return values err or ok, if err the command will output the comment

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,

        };// the match expression will return err or ok and based on the output we can catch the error


        println!("you guessed: {guess}");
        //println!("this is  your {} but not your {} guess",guess,guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal =>{ println!("you win!"); break;}
        }
    }
}