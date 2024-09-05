// When we use RfCell<T> and Rc<T> WE CAN LEAK MEMORY 

// it is possible to create references wheer elements refr to each othr in a cycle 

// this wil create memory leaks because the reference count of each item in the cycle will never reach 0

use crate::List::{Cons,Nil};
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

// this is an example of how a reference 

#[derive(Debug)]

// instead of wanitn go tmodify the i32 value , now we want to modify the List value a Cons variant is pointing to 

// tbe tail method is for accessing the second item if we have a Cons variant 
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value:i32,
    parent:RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}
fn main() {
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}",a.tail());
    
    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));

    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // // Uncomment the next line to see that we have a cycle;
    // // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // //Preventing Reference Cycles: Turning an Rc<T> into a Weak<T> 
    // // Rc::cline increases the string count of an Rc<T> instance 

    // // when you call Rc::downgrade you get a smart pointer of type Weak<T>. Instead of increasing the strong_count , calling Rc::downgrade increases the weak_count by 1. The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

    // //Creating a Tree Data Structure: a Node with Child Nodes 
    // // looking at the struct defintion from above, in the tree, we want the Node to own its children and we want to share ownership with variables so we can share 
    // // access each Node in the tree directly . To do this , we define the Vec<T> items to be of type Rc<Node> , we also want to modify ywhich nodes are achildrn of another node so we use RefCell<T> in children around the Vec<Rc<Node>>

    // let leaf = Rc::new(Node {
    //     value:3,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![]),
    // });

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    

    // let branch = Rc::new(Node {
    //     value:5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)])
    // });
    
    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // //adding a reference from child to parent 

    // //parent should never own children , parent dropped , child drops too so use weak references 
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade())

    //visualises changes to strong count and weak count


     let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
