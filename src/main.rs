extern crate core;

use std::io;


fn main() {
    //this is a comment


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
    let _plums = 'p';


    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)// calls readline method within the IO library
        .expect("Failed to read line");// this line expects an error from the previouse command. 2 return values err or ok, if err the command will output the comment










}

fn this(){




}