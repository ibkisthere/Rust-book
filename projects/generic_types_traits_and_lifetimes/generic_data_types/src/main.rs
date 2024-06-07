fn main() {
    //generics - a way to represent a type instead of using concrete typees such as i32

    // lifetimes - a variety of generics that give the compiler information about how references relate to each other, it allows us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations 
    
    // Removing Duplicates by Extracting a Function 
    // generics allow us to replae specific types with a placeholder that represents multiple types 
    // it allows us to remove duplication 
    // let number_list = vec![34,50,25,100,65];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // // we've now been tasked to find the largest number in twp different lists of numbers 
    // let number_list_1 = vec![102,34,6000,89,54,2,43,8];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // to eliminate this duplication, we will create an abstraction by defining a function that operates on any list of integers passed in a parameter

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // identify duplicate code - put them in functions 

    // update the instances of the duplicated code to call the function instead 

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // let integer = Point{x:5, y:10};
    // let float = Point{x:1.0, y:4.0};
    // let integer_and_float = Point { x: 5, y: 4.0 };


    let p = Point {x : 5, y:10};

    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    // should return p3.x = 5, p3.y = c
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    let integer = Some(5);
    let float = Some(5.0);

    // When rust compiles this code, it performs monomorphization , during that process the compiler reads the Option<T. type and identifies that there are two types - i32 and f64 , then replaces the generic definition with the specific ones 

    // The monomorphized code looks like this 

    enum Option_i32 {
        Some(i32),
        None,
    }
    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}


// fn largest (list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest 
    // we use generics to create deifnition sfor things like function signatures or structs , which can then be used with many different concrete data types. Let's first look at how to define functions, structs, enums, and methods using generics 

    // when defining a funciton that has generics , we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value, doing so makes our code flexible 

    // lets change out largest function into a parametrized function , by introducing a generic type parameter - we wil use T (you can use any identifier or type name)

    // To define the generic largest function , we place type name declarations inside angle brackets, between the name and the function parameter 

    // we will get an error because the T operator , cannot work for all possible types that T could be , as not all of the types have the trait std::cmp::PartialOrd implemented
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// in struct definitions (syntax for usng Generics in struct definitions)
// note that because we used only one generic type to define Point<T> , this deifnition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type 
// struct Point<T> {
//     x:T,
//     y:T,
// }

// to define a struct where x and y could be different types, we have to use it this way

struct Point1<T,U> {
    x:T,
    y:U,
}


// in Enum Definitions

enum Option<T> {
    Some(T),
    None,
}

// The Option<T> enum is generic over type T and has two variants : Some, which holds one value of type T, and a None variant that doesn;t hold any value. 

// By using this Option<T> enum, we can represent the abstract concept of an optional value and because it is generic , we can use this abstraction no matter what the type of the optional value is 

// Enums can use multiple generic types as well 

enum Result <T,E> {
    Ok(T),
    Err(E),
}

// if you find places in your code where you have multiple struct or enum definitions that differ by type, you can use generic types instead

// In method Definitions

// struct Point<T> {
//     x:T,
//     y:T,
// }

// a method named x on the Point<T> that returns a reference to the data in the field x, methods written with an impl that declares the generic type will be deifned on any instance of the type, no matter what concrete type ends up subtituting for the generic type

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// we can also specify constraints on generic types when defining methods on the type, we could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type 

// this means the thte type f32 will have the emthod distance_from_origin defined , other instances of Point<T> with type T that is not f32 will not have the method defined 
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

//Generic type parameters in a struct defintioon aren't always the same as those you use in that same struct's methods signatures 

struct Point<X1,Y1> {
    x : X1,
    y:Y1,
}

// This method creates a new Point instance from the x value from the self Point (of type X1) and the y value from the passed in Point (of type Y2)
impl<X1,Y1> Point<X1,Y1> {
    fn mixup<X2,Y2>(self , other : Point<X2,Y2>) -> Point<X1,Y2> {
        Point {
            x:self.x,
            y:other.y,
        }
    }
}

// Performace of cide using generics

// generic types wouldn;t make your code run any slower than using conrete types would 

// Rust does this by performing monomorphization of the code using generics at compile time, it basically does the opposite of all the steps that we went through to convert our code / methods to use generic types , it turns generic code into specific code by filling in the concrete types that are used when compiled
