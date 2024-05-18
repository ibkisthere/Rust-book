// we can use nested paths to bring in the same items, instead of taking multiple lines 


//instead of
// use std::cmp::Ordering;
// use std::io;

// we use 
// use std::{cmp::Ordering, io};

// we can use a nested path at any level 

//instead of
// use std::io;
// use std::io::Write;

// this line brings std::io and std::io::Write into scope 
use std::io::{self, Write};

// The Glob Operator 

//If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:


use std::collections::*;

// e careful when using the Glob Operator !


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use only creates a shortcut for the particular scope that the use occurs - if we move eat_at_restaurant into a new child module named customer, the function body won't compile
// use crate::front_of_house::hosting;

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }


pub fn eat_at_restaurant() {
     hosting::add_to_waitlist();
}

// Creating idiomatic use paths 

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist()
// }

// both ways are ways to bring the add_to_waitlist() function into scope , the way above is the idiomatic method 

// on the other hand when using structs , enums and other items with use , its idiomatic to specify the full path 

// Re exporting Names with pub use 

pub use crate::front_of_house::hosting;

// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(). Now that this pub use has re-exported the hosting module from the root module, external code can now use the path restaurant::hosting::add_to_waitlist() instead.

