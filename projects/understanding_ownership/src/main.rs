fn main() {
    // Ownership is a set of rules that govern how a rust program manages memory , some langauges have garbage collection that regularly looks ofr no longer usef memorty as the program runs , in other languagesthe programmer must explicitly allocate and free memeory  Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

    // the String type manages data allocated on the heap and is able to store amount of text that is unknown to us at compile time 

    // create a String from a string literal 
    // let s = String::from("hello");
    // // this kindof string can be mutated
    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);

    // so whats the difference ? why can String be mutated but Strng literals cannot? The diffferene is inhow thee two types deal with memory 

    //String 
    //

    //String literal
    //we know the content at compile time - so the text is hardcoded into the final executable , but this property only comes from the Strings immutability , unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size may change during runtime

    //String literal 
    // in most languages  the memory is automatically becomes free - that is it is cleanded up by the garbage collector when it is no longer in use 
    // in languages without a GC , it is our responsisbility to know when memory is not longer in use , then explicitly free it - to do this correctly is a problem , if you do it twice eg, calling free() in C you will end up getting a bug

    // in rust memeory is automtically returned once the variable that owns it goes out of scope 
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s 

        // this scope is now over, s is no longer valid 

    }

    // when a variable goes out of scope Rust calls a special function for us called drop() - this is where the author of th stirn can put the code to return the memory

    //variables and data interacting with move

    //let use this as an example 
    
    // let x = 5
    // let y = x

    //this means "bind the value 5 to the variable x , then copy the value of x to y"
    // because two of these integers are with a known fixed size and are pushed onto the stack

    //lets look at the String version 

    // let s1 = String::from("hello");

    // let s2 = s1;

    // this looks very similar so we might assume thta it might be the same , right? well no, let's explain 

    // let s go in depth 

    //String is made up of three parts
    // - the length the amount of memory that it occupies in bytes (currently using)
    // - the capacity - this is the amount of memory that is can hold in bytes (String has received from the allocator)
    // the pointer (ptr) 

    // when we assign s2 to s1 we are not assigning the data String holds, but we are copying the pointer , length and capacity that are on the stack , we do not copy the data on the heap that the pointer points(refers) to 

    //Rust does not copy the heap data as well , if it did this , the operations s2 = s1 could be very expensive 

    //earlir we said when a variable goes out of scope, Rust automatically calls the drop function and cleans the heap memory for that variable but s1 and s2 point to the same at location , so when s1 and s2 go out of scope , it leads to a double free error - they both try to free the same area of memory. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

    //This wouldn't work

    // let s1 = String::from("hello");
    // let s2 = s1;
    // to ensure memory safety , after the line let s2 = s1 rust doesn;t need to free anything as it considers s1 as no longer valid 
    //borrow of moved value: `s1` value borrowed here after move
    // 
    // println!("{}, world!", s1);
    // println!("Hello, world!");

    //if you've heard the terms shallow and deep copy in other programming languages , you'll probably think that copying the pointer , capacity and length is making a shallow copy , but no , since Rust invalidates the first variable , this is called a Move for example we would say s1 was moved into s2 

    // there a deisgn choice implied by this -  rust will never create automatic deep copise of your data 

    // variables and data interacting with clone

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 ={}", s1, s2);

    // the code is fine, the heap data , actually gets copied , when ou see a call to clone just know that some arbitrary code is being executed and the code may be expensive

    // STACK ONLY DATA
    // but when we write code like this , doesn't this contradict everything we've learnt so far?

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // the reason is that the sizes are known at compile time and the size is allocate on the stack
    // there is no differene between shallow cloning and deep cloning, so using clone doesn't do anything
    //Copy trait
    // Rust has a special annotation called Copy that we can place on typs that are stored on the stack , as integers are 
    // If a type implements the Copy trait , variables that use it do not move , but rather are trivially copied, making them still valid after assignment to another variable.

    //Here are some of the types that implement Copy:

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // Ownership and functions 
    // the way we pass a value to a function is similar to how we assign a value to a variable 
    //passing a variable to a function will move or copy , just as assignament does 
    
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // Returning values can also transfer ownership 
    let s3 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s4 = String::from("hello");     // s2 comes into scope

    let s5 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3


    //Rust does allow us to return multiple values from a function using a tuple

    let s6 = String::from("hello");

    let (s2 , len) = calculate_length(s1);
}

// Here, s5 goes out of scope and is dropped. s4 was moved, so nothing happens, s3 goes out of scope and is dropped

//the owndership of a variable follows the same pattern every time - asigning a value to another variable moves it, when the variable includes data on the heap goes out of scope , the value will be dropped (cleaned up by drop) unless the ownership of the data has been moved to another variable 



fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn calculate_length(s:String) -> (String,usize) {
    let length = s.len();

    (s, length)
}

// what if we want to transfer data to another variable without transferring ownership? that where we use references

