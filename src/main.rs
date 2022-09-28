extern crate core;

      //|\\
     //-|-\\
    //-/|\-\\
   //-/-|-\-\\
  //-/--|--\-\\
 //-/--/|\--\-\\
//-/--/-|-\--\-\\


use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::ffi::c_void;
use std::io:: Bytes;
use std::mem::take;
use std::ops::Index;
use std::os::linux::raw::stat;

fn main() {

}




fn slices(){

    fn first_word(s: &String) -> usize{
        let bytes = s.as_bytes();//This converts a string to an array of bytes

        for(index, &item) in bytes.iter().enumerate(){
            //the for loop defines i as the increasing num
            //&item is the value of each byte at said increasing num
            //&item should be each byte which is iter() and enumarate()
            //iter() iterates throuh given values
            //enumarate() returns a tuple of the index and the item
            if item == b' '{
                return index;
            }
        }
        s.lent()

        //the issue with this method is that we can save the value using the function
        // but we are able to modify the string after leaving the int num the original value
    }

    //the solution to this issue is to refrence certain parts of a string

    let s = String::from("hwllo world");

    let hello = &s[0..5];
    let world = &s[6..11];








}

fn refrences_borrowing(){

    let s1 = String::from("hello");

    let len = calculate_length(&s1);// & infront is how we show refrence

    println!("the length of {} is {}", s1, len);

    //the & opperator is a pointer, it points to someone else

    fn calculate_length(s: &String) ->usize{
        s.len()
    }// unlike normal case here s1 is not dropped as s is just a pointer
    // however we are not able to modify the data using this method we
    //can only borrow where it is
    /*
    let s = String::from("hello");

    change(&s);


    fn change(some_string: &String) {
        some_string.push_str(", world");
    }
    *///this code above will not work as we are only passing the refrence

    //----------soloution to this problem-----------\\


    let mut s2 = String::from("hello");

    change(&mut s2);

    fn  change( ss: &mut String){
        ss.push_str(", world");
    }
    //this solved the issue however
    // we can only have a single refrence if the variable is mutable

    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{},{}",r1,r2);
    */// the error here is that we borrow the value twice which isnt allowed

     //----------------------------\\
    // we cannot acess an immutable variable using a mutable one at the same time eg:
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    */
    let mut s = String::from("hello");

    let r1 = &s; //no problem
    let r2 = &s; //no problem

    println!("{} and {}",r1,r2);

    //this is fine as we are not mutating our variable!

    let r3 = &mut s;
    println!("{}", r3);



}

fn ownership(){

    let ss = "hello"; // this is a string literal which is on the stack with know size

    let mut s = String::from("hello"); // this is an example of a heap variable

    s.push_str(", world!"); // push_str() apends a literal to a string

    println!("{}", s);

    {
        // as i mentioned earlier execution of code here
        // is seperate from the rest of the program
        // memory is allocated and unallocated once it is over

        let sss = String::from("hello");
    }

    //--data_interaction--//

    //bind 5 to x the x to y
    let x = 5;
    let y = x;

    // unlike int case we only copy the meta data
    // meaning we dont make a copy of said data just the pointer
    let s1 = String::from( "hello");
    let s2 = s1;
    // when rust calls drop command it would try to delete both s1 and s2
    // even though they point to same memory location this is dangerous
    // so rust instead removes s1 out of scope.
    // if we try to acess s1 here well get an error

    let s3 = String::from("hells");
    let s4 = s3.clone();
    // this works the same as other prog langueages we make an exact copy
    // and therefore use the same amount of memory


    //in this case s5 looses ownership of the value of s5 and fn gains it
    let s5 = String::from("hello");
    take_ownership(s5);

    // however this is not the case for integer as it is a cloneable
    let x = 5;
    makes_copy(x);

    fn take_ownership(string: String){
        println!("{string}");
    }

    fn makes_copy(int: i32){
        println!("{int}");
    }


    let s6 = gives_ownership(); //gets ownership of string from fn

    let s7 = String::from("hello"); // gets ownership

    let s8 = take_and_gives_back(s7); //moves ownership from s7 to fn then back to s8




    fn gives_ownership() -> String{

        let some_string = String::from("yours");
        some_string
    }

    fn take_and_gives_back(a_string: String) -> String{
        a_string
    }

    let s9 = String::from("hello");
    let (s10, len) = calculate_length(s9);

    println!("length of {s10} is {len}",);


    fn calculate_length(s: String) ->(String,usize){
        let length = s.len();

        (s, length)
    }




}

fn control_flow(){
    let mut num = 3;
    if num < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    //as always breaks at first true statement
    let num = 6;
    if num % 4 == 0{
        println!("num is divisible by 4");

    }else if num % 3 == 0{
        println!("num is divisible by 3");

    }else if num % 2 == 0{
        println!("num is divisible by 2");

    }else{
        println!("number is not divisible");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; //return types of this statment have to match


    //---loops---//


    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    // loops within a variable
    let result = loop{
        counter +=1;

        if counter == 10{
            break counter *2;
        }
    };

    // loop names yay
    let mut count = 0;

    'counting_up: loop{

        println!("count = {count}");
        let mut remaining = 10;

        loop{

            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}!");



    let mut nub = 3;
    while nub != 0{
        println!("{nub}!");

        nub -= 1;
    }
    println!("Liftoff");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{//the loop automatically increment the index value
        println!("the value is: {}", a[index]);
    } // slow execution time, due to added code for out of bounds check

    for element in a{
        println!("the value is {element}");
    }

    // same shit as before but beter.
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("Liftoff!");








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