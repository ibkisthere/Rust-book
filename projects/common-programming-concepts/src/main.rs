use std::io;

fn main() {
   let mut x = 5;
//    println!("The value of x is {x}");
   x = 6;
//    println!("The value of x is {x}");
//    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

   //shadowing
   let y = 5;

   let y = y + 1;
   {
     let y = y * 2;
    //  println!("The value of y in the inner scope is: {y}");
   }

//    println!("The value of y is {y}");

   // difference betwen shadowing and mut - we can change the type of value but reuse 
   // the same name 

   let spaces = "   ";
   let spaces = spaces.len();

   //shadowing spares us having go come up with different names eg spaces_str, or spaces_num
   
// if we do this , we'll get an error - we are not allowed to mutate a variables type
//    let mut spaces = "  "
//    spaces = spaces.len()


// Data types
//scalar types - represents a single value . Rust has four primary scalar types 
//floating-point numbers, Booleans, and characters. 


//floating point numbers - the default type is 64bits because on modern CPUs it roughtly the same speed as 
// f32 but is capable of more precision 



    let x = 2.0;

    let y:f32 = 3.0;

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; 

    let t = true;

    let f: bool = false; // with explicit type annotation

    // the character type 

    let c = 'z';
    let z :char = 'Z';
    let heart_eyed_cat = '😻';

    //Compound types 
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (a, b , c) = tup;

    println!("The value of b is: {b}");

    //we can also access the element in the tuple based on its index using a "." period

     let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // the Array type - they must have fixed lengths
    // arrays are useful when you want your data allocated on the stack rather than the heap 
    
    let a = [1,2,3,4,5];

    // They are more useful when you know the number of elements are not going to change - eg the months of the year 
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

// you ca write an arrays type using the square brackets with the type of each element , a semicolon, and then the number of elements in the array 

   let a: [i32; 5] = [1,2,3,4,5];

//i32 - type of each element , 5 - number of elements in the array 

//You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:


 let a = [3; 5];

 // an array is a singl chunk of memeory of a known fixed size that can be allocated on the stack , you can access aray elements using indexing 

 let a = [1,2,3,4,5];

 let first = a[0];
 let second = a[1];
 

 // simple  console application to take an index for the user as inout and retur the corresponding element from a pre existing array 

//  println!("Please enter an array index.");

//  let mut index = String::new();

//  io::stdin().read_line(&mut index).expect("Failed to read line");

//  let index:usize = index.trim().parse().expect("Index entered was not a mistake");

//  let element = a[index];

//  println!("The value of the element at index {index} is: {element}");

 another_function(34);

 print_labeled_measurement(32, 'o');


 let q = {
      // this expression evaluates to 4 
      let r = 3;
      // you an see that thre is no ; at the end of this line - its because if we do we make it 
      // a statement , and it will not return a value,  expression do not include ending semicolons 
      r + 1
 };

 // functions with return valus 
 let x = five();

 println!("The value of x is: {x}");

let s = plus_one(5);

println!("The value of x is: {x}");

//Control flow
let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
      println!("number was something other than zero")
    }

    if number % 4 == 0 {
      println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
   //  using if in a let statement 
    let condition = true;
    let number = if condition { 5 } else { 6 };

   //  Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type;
   // this would give us an error `if` and `else` have incompatible types expected integer, found `&str`
   // let number = if condition { 5 } else { "six" };
    // the experssion in the if block evaluates to an integer, and the expression in the else block evaluates to a string , this won;t work because variables must hae a single type , and rust needs to know at compile time what the type number is 
    println!("The value of the number is {number}");

    //Repitition with loops
    // tells rust to execute a block of code over and over agai until you explicitly tell it to stop
   //  loop {
   //    print!("again!")
   //  } 
   //  returning values from loops
    let mut counter: i32 = 0;

    let result = loop {
      counter += 1;

      if counter == 10 {
         break counter * 2;
      }
    };

    println!("The result is {result}");

    // loop labels 
    // if you hae loops within loops , break and continue, apply to the innermost loop at that point
    // you can optionally specify a loop label ona a loop that uou can use 
    let mut count = 0;
    'counting_up: loop {
      println!("count = {count}");
      let mut remaining = 10;
      loop {
         println!("remaining = {remaining}");
         if remaining == 9 {
            break;
         }
         if count == 2 {
            break 'counting_up;
         }
         remaining -= 1;
      }
      count += 1;
    }

    //conditional loops with while
    let mut number1 = 3;

    while number1 != 0 {
      println!("{number}!");

      number1 -= 1;

    }

    println!("LIFTOFF!!");
    // for loops 
    let a = [10,20,30,40,50];

    let mut index = 0;

    while index < 5 {
      println!("the value is {}", a[index]);

      index += 1;
    }

    let a1 = [60,70,80,90,100];

    for element in a1 {
      println!("the value is: {element}")
    }
    //Range - generates all numbers in sequence starting from one number and ending before another number
    // the countdown method using anther mehtod we have not talked about called rev 

    for number in (1..4).rev() {
      println!("{number}!")
    }
    println!("LIFTOFF!!!")

//     Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
}


fn another_function(x:i32) {
   println!("The value of x is :{x}")
}

// multiple parameters 
fn print_labeled_measurement(value:i32, unit_label:char) {
   println!("The measurement is: {value} {unit_label}")
}

// a new scope can be created with curly braces - this is still an expression

// functions returns values to the code that calls them , we don't name return values, but we must declare their type after an arrow in rust 

// There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust.
fn five() -> i32 {
   5
}

fn plus_one(x:i32) -> i32 {
   // if we place a semi colon at the ending of it turning it from an expression to a statement we will get an error 
   x + 1
}

