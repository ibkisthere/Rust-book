use std::net::IpAddr;

fn main() {
    //in situations such as writing tests or prototype code its more appropriate to write code that pnaics instead of returning a result 
    // if a method call fails in a test, you'd want the whole test to fail, even if that method isn't the
    //functionality under the test , because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen 

    
    // Cases in which you have more information thatn the compiler 
    // it would be appropriate to call unwrap or expect when you have some other logic that ensures the Result will have an OK value,
    //You'll still have a Result value that you need to handle, whatever operation you;re calling still has the possibility of failing in general , even though it's logically impossible in your partciular situation.
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");

    //We creating an IpAddr instance by parsing a hardcoded String , we can see the 127.0.0.1 is a vlaid IP address here , so its acceptable to use expect here , but having a hardcoded String , doesn't change the the return type of the parse value of it being a Result value , and the compiler isn;t smart enough to know that the String is always a valid IP address , if the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure we would want to account for errors , using expect to better error handling code if in the future, we need 


    //Guidelines for Error Handling 
    // its advisable to have your code panci when you feel your prgram will end up in a bad state
    // SImilarly you can use panic! to call external code that is out of your control 
    // wehn you code performs operations that could put the user at risk   
    // havaing alot of error checks i your code wil make ti verbose and annoying , swo we can take advantage of Rusts's type system to solve it for example, Rust's u32 type will not allow us to have negative values (Parameter),if you have a type rather thatn Option your code expects your to have something rather than nothing , your code wouldn;t then have tohandle for Some and None value - it will only handle one value becaue it expects something.

    // Creating Custom types for validation 
    // remember our guessing game in chapter 2 ? we only checked if the values wetre too high too low 
    loop {
        // let guess:i32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // if guess < 1 || guess > 100 {
        //     println!("The secret number will be between 1 and 100");
        //      continue;
        // }

        // match guess.cmp(&secret_number) {

        // }
    }
}

pub struct Guess {
    value:i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}",value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
