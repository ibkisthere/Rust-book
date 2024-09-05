fn main() {
    //Drop lets you customize what happens when a value is about to go out of scope , you can provide an implementation of the Drop trait 
    //We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.   
    // other languages - the programmer must call code to free memory or resources every time they finish using an instance of a type, eg , file handles , sockets , locks
    // in rust alue goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources, you do this by implementing the Drop trait 
    //Rust automatically called drop for us when our instances went out of scope, calling the code we specified. Variables are dropped in the reverse order of their creation, so d was dropped before c.

    // let c = CustomSmartPointer {
    //     data:String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");
    // println!("Hello, world!");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

//DROPPING A VALUE EARLY with std::mem::drop

// use the drop mehtod to call the drop manually to force a value to be dropped before the end of its scope 

// rust does not let us call the drop explicitly like c.drop() because it will still do that when the value goes out of scope (at the end of main) this would cause a double free error 

