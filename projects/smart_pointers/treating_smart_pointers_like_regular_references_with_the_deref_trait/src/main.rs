use std::ops::Deref;

fn main() {
   // implementing the Deref trait allows you to customize the behaviour of the dereference operator * 

   // it won't work, you can't compare a number to a reference 

   //Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to.

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // assert_eq!(5, y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If Rust didn’t implement deref coercion, we would have to write the code below instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.

    hello(&(*m)[..])

    //The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello. This code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.
}

//defining our own smart pointer 

// struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//Treating a type like a reference by implementing the Deref Trait 

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//When we entered *y now behind the scenes Rust actually ran this code:

// *(y.deref())

// rust substitutes the * operator with a call to the deref method and then a plain dereference 

//Deref Coercion
//Converts a reference to a type that implements the Deref trait into a reference to another type 

// eg, it can convert &String to &str , because String implements the Deref trait such that it returns &str

//We can call the hello function with a string slice as an argument, such as hello("Rust"); for example. Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String

fn hello(name: &str) {
    println!("Hello, {name}!");
}


// how Deref Coercion interacts with mutability

// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

// Rust does deref coercion when it finds types and trait implementations in three cases:

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

