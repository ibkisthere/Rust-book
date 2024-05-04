fn main() {
    // - custom data type that lets you package togetehr and name multiple related values that make up a meaningful group 

    let user1 = User {
        active:true,
        username:String::from("some username 123"),
        email:String::from("someone@example.com"),
        sign_in_count:12,
    };

    // GET A VALUE FROM A STRUCT 

    // user1.email = String::from("anotheremail@example.com"); wouldn't work 
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
    // let user2 = User {
    //     active:user1.active,
    //     username:user1.username,
    //     email:String::from("another@example.com"),
    //     sign_in_count:user1.sign_in_count
    // };
    

    //CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX 

    // Using struct update syntax , we can achieve the same effect with less code 

    // the syntax (..) specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance 

    let user2 = User {
        email:String::from("another@example.com"),
        ..user1 
    };

    //USING TUPLES STRUCTS WITHOUT NAME FIELDS TO CREATE DIFFERENT TYPES

    // Rust supports structs that look similar to tuples fcalled tuple structs ,tuple strycts adds meaning the struct name provides but don't have names associated with their fields rather, they just have he types of fields.

    //Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples and when naming each field as a regular struct will be verbose or redundant 

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // INSTANTIATING A UNIT STRUCT 
    let subject = AlwaysEqual;

    //OWNERSHIP OF STRUCT DATA 
    // in the examples above for the structs , used the owned 'String' type for the string data type because we wanted the structs to have ownership of the data , and for that data to be valid for as long as the struct is valid, now we want to use &str the string slice type - it is possible for structs to store references to data owned by something else  , but to do so requirees the use of lifetimes , lifetimes ensure the data referenceed by a struct is valid for as long as the struct is valid 

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.", area(width1, height1)
    // );
}


struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn build_user(email:String, username:String) -> User {
    User {
        active:true,
        username:username,
        email:email,
        sign_in_count:1
    }
}

// USING INIT SHORTHAND
//Here, we’re creating a new instance of the User struct, which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function. Because the email field and the email parameter have the same name, we only need to write email rather than email: email.
fn build_user(email:String, username:String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count:1
    }
}

//TUPLE STRUCTS 

// Note that the black and origin values are different types because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//UNIT LIKE STRUCTS 
// you can also define structs that don't have any fields they are called unit like structs because they behave similarly to (), the unit type , that was mentioned in the tuple type section 
// Unit like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself 

struct AlwaysEqual;

//STRUCTS WITH SLICES
//we get an error - missing lifetime specifier
// struct User {
//     active:bool,
//     username:&str,
//     email:&str,
//     sign_in_count:u64
// }

