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

 println!("Please enter an array index.");

 let mut index = String::new();

 io::stdin().read_line(&mut index).expect("Failed to read line");

 let index:usize = index.trim().parse().expect("Index entered was not a mistake");

 let element = a[index];

 println!("The value of the element at index {index} is: {element}");
}