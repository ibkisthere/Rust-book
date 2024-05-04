fn main() {
  
    //METHOD SYNTAX
    //Methods are similar to functions - we declare them with the fn keyword and aname, they can have parameters and a return value, but methods are defined within the context of a struct or a enum or a trait object and their first parameter is always self , which represents the instance of the struct the method is being called on 
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }

     let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}  


//DEFINING METHODS

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

//in the signature for area we use &self instead of rectangle: &Rectangle. the &self is actually short for self:&Self.

// Within an impl block,the type Self is an alias for the type that the impl block is for 
// Methods must have the type self as the first parameters, notive that we use the & in front of the self shorthand , to indicate that we are borrowing the Self instance , just as we did in rectangle : &Rectangle 

// Methods can take ownership of self, borrow self immutably , as we've done here , or borrow self mutably , just as any other parameter 

// we chose &self here for the same reason we chose &Rectangle we don't want to take ownership of self, wejust want to read data from the struct , not write to it,  now if we wanted to modify the instance that self was called on as part of what the method does we would have use &mut self as the first parameter 

// having a method that use the self instance as the first parameter is usually rare, but this can be used when part of the method involves transforming the self instance and we don't want to use the same instance after transformation 

// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other:Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size:u32) -> Self {
        Self {
            width:size,
            height:size
        }
    }
}

// when we give a method the same name we want it to only return the value in the field and do nothing else , methods like this are called getters rust does not implement them automatically for structs ike other languages do , getters are useful because they aollow is to make the filed provate nbut the method public 

//METHODS WITH MORE PARAMETERS 

// We want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle), other wise , it should return false 

//ASSOCIATED FUNCTIONS 

//All functions defined in an impl block are called associated functions 

//we can defined assocaited functions as functions that don;t have self as their first parameter and don't need an instance of the type to work with 

// The Self keywords in the return type and in the bidy of the function are aliases for the type that appears after the impl keyword , which in this case is Rectangle 

//To call this associated function we use the :: syntax with the struct name 
// eg let sq = Rectangle::square(3)


// Each struct is allowed to have multiple impl blocks 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax. We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.

// we can use structs to kepp associated pieces of data together 