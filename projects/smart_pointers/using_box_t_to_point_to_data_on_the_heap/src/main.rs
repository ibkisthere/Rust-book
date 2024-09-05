mod List;

use crate::List::List::{Cons, Nil};

fn main() {
    // USING BOX<T> TO STORE DATA ON THE HEAP
    let b = Box::new(5);
    println!("b = {b}");

    //ENABLE RECURSIVE TYPES WITH BOXES 
    // At compile time , rust needs to know that size of each type, recursive types that theoretically consume infinite memory,, so we can use Box<> to enable recursive type by inserting it into the definition 

    //Cons List 
    // Lisp version of a linked list 
    //(1, (2, (3, Nil))) - pseudo code 
    // we are getting an "infinite size error" from the rust compiler , because of the way rust computes the sizes of types, looking at the List enum , its first looks at the size of List enum, then goes to look at the Cons() variant , it holds a values of type i32 which has a fixed size, and also holds the List type, so Cons needs an amount of space equal to the size of an 132 plus the size of a List, ok, then goes to value of type List, and this process continues infinitely 

    //continues infinitely 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
    
    //Using Box<T> to get a recursive Type with a known size
    //Because Box<T> is a pointer , rust always knows how much space a Box<T> needs : a pointer's size doesn't change based on the amount of data it's pointing to .
    //This means we can put a Box<T> inside the Cons variant instead of another List value directly. The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant

    // now the Cons variant in List needs the size of an i32 plus the space to store the box's pointer data 

    // The Box<T> Pointer is a smart pointer because it implements the Deref Trait which allows Box<t> values to be treated like references , when a Box<T> value goes out of scope , because of the drop Trait , the heap data that the Box is pointing to is cleaned up 
}




// INTRODUCTION

// Smart pointers - data structures that act like a pointer but also have additional metadata and capabilities

//Box<T> allows you to store data on the heap rather than the stack , what remains on the stac is the pointer tothe hea data 

// You'll mostly see them in situations 

// When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size 

// when you want a large amount of data you want to transfer ownership but you ant to ensure that data wont be copied also as you do 

// when you when you want to own a value and care only that it's a type that implements a particular trait rahter that being a specific type 


