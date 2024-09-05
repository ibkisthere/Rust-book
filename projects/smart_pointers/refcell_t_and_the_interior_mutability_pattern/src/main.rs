#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

//interior mutability pattern

// design pattern that allows you to mutate data even when there are immmutable references to that data , - normlaly disallowed by borrowing rules,  To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us

// we can use the interior mutability pattern in our code when we can ensure theat our code will obey the borrowing rules at runtime , even thoughthe compiler cannot guarantee that , lets explore this concept by looking at RefCell<T> type that follows the interior mutability pattern 

//Enforcing borrowing rules at Runtime with RefCell<T> 

// RefCell<T> reprsnts signle ownership over the data it holds , what makes it differnet from a type like Box<T> ? 


// Well with referenes and Box<T> the borrowing rules' invariants are enforced at compile time , with RefCell<T> these invariantts are enforced at runtime 

// with references i you break these rules you willl get a compiler error , with RefCell<T> if you break these rules, your program will panic and exit 

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.



fn main() {
//    let x = 5;
//    let y = &mut x;

// / the code above wil give an error - but there are timmes whenit si good for code to be immutable but for it to appear immutable to other code , code outside the vlaues methods would not be able to mutate the value 

// a case for interiro mutability is Mock objects 

//we'll create a library that tracks a value against a maximu value and sends messages based on how close to the maximu value the current value is 


//Having nultiple owners of mutable data by COmbining Rc<T> and RefCell<T> 

// a common way to use RefCell<T> is with Rc<T> remember that Rc<T> allows you tohave multippe owners of the same data , but gives you immutable access to that data , now , if you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!

//By using RefCell<T>, we have an outwardly immutable List value. But we can use the methods on RefCell<T> that provide access to its interior mutability so we can modify our data when we need to. The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures. Note that RefCell<T> does not work for multithreaded code! Mutex<T> is the thread-safe version of RefCell<T> 
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

}


pub trait Messenger {
    fn send(&self, msg:&str);
}

pub struct LimitTracker<'a,T:Messenger> {
    messenger: &'a T,
    value:usize,
    max:usize,
}

impl<'a, T> LimitTracker<'a, T> where T : Messenger, {
    pub fn new (messenger : &'a T, max :usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max , }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

         if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages:RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages:RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        // fn send(&self, message:&str) {
        //     //We get an error here - cannot borrow self.sent_messages as mutable as it is behind a & reference 
        //     // - we can't modify sent messages because the send method takes an immutable reference,
        //     // we can't make it &mut , because then it wouldn't match the function signature in the Messenger trait definition 
        //     // self.sent_messages.push(String::from(message));

        //     // usign RefCell<T> can help 
        //     self.sent_messages.borrow_mut().push(String::from(message));
        // }
        //now you'll be choosing to catch borrowing errors at runtime rather than compile time , means you'll be finding errors in your code later in the development process
        //. However, using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of the messages it has seen     

        //causes borrow error
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

//keeping track of borrows at runtime with RefCell<T> 

// When creating immutable and mutable references, we use the & and &mut syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.


