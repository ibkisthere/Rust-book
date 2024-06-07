use core::panic;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self,Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    // we will add an inner match expression to mathc different errors 
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file : {:?}", error),
    // };
    // let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //      ErrorKind::NotFound  => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {:?}", e),
    //        },
    //        other_error => {
    //         panic!("Problem opening the file: {:?}", other_error);
    //        }
    //    },
    // };

    // Alternatives to using match with Result<T,E> 
    // although thei code has no mathc statements, it has many nested if - else statments which may be hard to read 
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             File::create("hello.txt").unwrap_or_else(|error| {
    //                 panic!("Problem creating the file: {:?}", error);
    //             })
    //         })
    //     } else {
    //         panic!("Problem opening the file :{:?}", error);
    //     }
    // });

    // Shortcuts for panic on error : unwrap and expect 
    // Using match works well , but it can be verbose 
    // the Result<T,E> type has a lot of methods , the unwrap() method is a shortcut method implemented just like the match method 
    // let greeting_file = File::open("hello.txt").unwrap();
    // using the expect! method allows us to choose the error message to use -> unwrap will just panic , using expect instead instead of unwrap and providing error messages can cnvey intent and make tracking down the source of a panic easier
    let greeting_file = File::open("hello.text").expect("hello.txt should be included in this project");
    // in production wuality code most rustaceans choose to use expect rather than unwrap 
    
    //Propagating Errors
    // when a function;s implementation calls something that might fail, instead of handling the error within the function istelf you return the error to tthe calling code so that it can decide what to do this is known as propagating the erorr ad gives more control to the calling code , where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code 
}

// thi9s is a functio that returns errors to the calling code using match 
fn read_username_from_file()-> Result<String,io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // we don;t need to explictly say :return because this is the last experession in the function 
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// the code that calls this code will then handle getting either an OK value that contains a username or an Err value that contains an io:Error , its up to teh calling code to decide what to do with those value. if the calling code gets an Err value it could call panic! and crash the program ,
//  use a default username, or look up the username from somewhere other than a file, for example. We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.

// This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

// A shortcut for propagating errors the ? Operator 

/// a function that returns error to the calling code using the ? Operator 
fn read_user_name_from_file() -> Result<String,io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


// the ? placed after the lines of code that have a return value of Result will work the same way that it would work if we had used the match expression , if the value of the Result is an Ok , the value the Ok result is returned to the expression and the program wil continue , if the value is an Err, thE Err will be returned from the whole function as if we had the return keyword so the error value gets propagated to the calling code 

// The difference between the ? operator and the match expression error values that have ? called on them go through the from function , defined in the From trait in the std library which is used to convert values from one type to another , so when an error is returned its return type will be the return type of the function - the calling code that it is in - the error type received is converted from one type into the error type defined in the return type of the current function .
// This is useful when a function returns one error type to represent all the ways a function might fail , even if parts might fail for many different reasons

// The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ?

fn read_user_name_from_file_shortened() -> Result<String,io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// we can even make this shorter using read_to_string 

//Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.

fn read_user_name_from_file_shortest() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt")
}

//The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined in Listing 9-6. In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that it’s compatible with this return.


// function that finds the last character of the first line in the given text 
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last();
// }

// the main function is special because is the entry and exit points of executable prgrams and it has restrictions on what its return type can be for the prgrams to behave as expected 


// Box<dyn Error> is a trait object - for now , read it to mean "any kind of error", main function with the error type Box<dyn Error> is allowed, because it allows any Err value to be returned early. Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }