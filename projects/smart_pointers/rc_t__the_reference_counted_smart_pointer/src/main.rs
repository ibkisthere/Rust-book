mod List;

use crate::List::List::{Cons,Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // we get an error - use of moved value - a , the Cons variant owns the data they hold, so when we create the b list 
    // let c = Cons(4, Box::new(a));
    
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //  let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    //Rc::clone does not make deep copies of code , it only increments the reference count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// There are some times where ownership is clear we know which variable owns a given value, but there are cases where a single value cna have multiple owners , take for example in a graph where multiple edges point to the same node , nd that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

// You have to enable multiple ownership using the rust type Rc<T> an abbreviation for reference counting , keeps track of the number of refernces to the value to determine whether or not its still in use

//We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

// Only for use in single threaded scenarios 


//USING RC<T> TO SHARE DATA
// Retunr to the previous Lisp - linked List example we use in Box<T>, this time if we want to create a new list that has shared ownership of a third list , we can create a list a that 5, 10 and then points to Nil( signifying the end) , then we can have two other list b containing 3 pointing to list a , then list c that contains 4 that pints to a , in other words , both lists will share the first list containing 5 and 10 

// if we try to implement this using out knowledge of Box<T> it would not work 

// okay so we can change the defintion of Cons to hold references instead? but then we will have to specify lifetime parameters 

// - e will change our deifntion of list to use RC<T> in place of Box<T> 


//Cloning an Rc<T> increases the reference count 

