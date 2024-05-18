use std::collections::HashMap;
use std::fmt;
use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    // idomatic way to brint the standard library's HashMap struct into the scope of a binary crate

    let mut map = HashMap::new();
    map.insert(1, 2);
}

// th only exception to this idiom is when we have two types with the same name into the smae scope from different packages -> we have to use their parent modules. If instead we specified use std::fmt::Result and use std::io::Result, we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.


// fn function() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// providing new names with the as keyword 
//Another solution to this - bring two type with the same name into the same scope is by using an alias 

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }


