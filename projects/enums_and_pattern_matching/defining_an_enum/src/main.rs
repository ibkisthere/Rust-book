use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    // enums are a way to define a type by enumerating its variant 
    println!("Hello, world!");
    // struc ts - give you a way to gorup related data together , enums - they give you a way to say that a type is one of possible variants 

    // Let us create instances of the Ip AddrKind enum like this 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //we've use a struct t undle the kind and address vvalues together , so now the variant is associated with the value 
    // let home = IpAddr {
    //     kind:IpAddrKind::V4,
    //     address:String::from("127.0.0.1")
    // }
    // let loopback = IpAddr {
    //     kind:IpAddrKind::V6,
    //     address:String::from("::1")
    // }
    // let home = IpAddr::V4(127,0,0,1);
    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number:Option<i32> = None;

    // because T and Option<T> are different types , the compiler won;t let us use an Option<T> value as if it were a definitely a valid value 
    // basically , Rust doesn't know how to add an i8 and Option<i8> together
    // you have to convert an Option<T> to a T before you can perform T operations with it
    //Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

    // let x:i8 = 5;
    // let y:Option<i8> = Some(5);

    // let sum = x + y;
}   

// enums even give us the advantage - we don't even need t0o store the data 
enum IpAddrKind {
    V4,
    V6
}

fn route(ip_kind:IpAddrKind) {

}

// struct IpAddr {
//     kind:IpAddrKind, 
//     address:String 
// }

// representing the data inside an enum more concise - instead of putting it inside a struct , also we can see another detail of h0ow enums work - the name of each enum is a function that returns an enum of its variant 
// for example let us see IpAddrKind::V4() it is a function that takes in a String type and returns IpAddr instance 

// enum IpAddr {
//     V4(String),
//     V6(String)
// }

//Anothr advantage of using enums rather than a struct is that each variant of an enum can have multiple associated types and amounts of associated data , imagine if we wanted to store an IPV4 address as four 8 byte numbers, but still express IPV6 address as a string, we wouldn't be able to do that with structs 


// enum IpAddr {
//     V4(u8,u8,u8,u8),
//     V6(String)
// }

// the standard library has definitions that support representing IP addresses because this is a common thing, it it embeds the variants in terms of two different structs 

// struct Ipv4Addr {
  
// }

// struct Ipv6Addr {

// }

// you can put any kind of data inside an enum variant 
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}
enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

// Defining an enum with variants such as the one above is similar to defining different kinds of struct definitions , bu the enum doesn;t use the struct keyword and all the variants are grouped together under the Message type 

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// just as we're able to deifn methods on structs , we are able to define methods on enums

impl Message {
    fn call(&self) {

    }
}


// another enum in the standard library that is very common and useful - Option 

//option over null values 
// Rust doesn't have the null feature like other languages 

// The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

// However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.


// The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. 

// The variants of the Option enum - Some and None 
enum Option<T> {
    None,
    Some(T)
}

// Rust can infer the different Option types because we specified a value inside the Some variant 

// When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense it means the same thing as null: we don’t have a valid value. So why is having Option<T> any better than having null?


// So how do we get the T value out of the Some variant when you have a value of type Option<T>? Well to use the value we get out of Option<T> , we can use a match statement to do this -> we want code that runs when we have a Some value, we want code to run when we have a None value, we want code that so that we can run differnt code based on differnt variants of the enum  