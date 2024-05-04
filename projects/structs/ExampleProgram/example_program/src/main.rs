fn main() {
    //REFACTORING WITH TUPLES 
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(rect1)
    )

    let rect1 = Rectangle {
        width:30,
        height:50
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    //ADDING USEFUL FUNCTIONALITY WITH DERIVED TRAITS 
    // if we try to print the rectnagel value we get an error - `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rect1 is {}", rect1);

    //The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. The primitive types we’ve seen so far implement Display by default because there’s only one way you’d want to show a 1 or any other primitive type to a use

    // the println1() macro is more straightforward for primitive traits , for structs however , we hae to specify how we want it, eg do we want commas or not , brackets or not  so because of that ambiguity we get an error 

    // println!("rect1 is {:?}", rect1);
    // let scale = 2;
    // let rect1 = Rectangle {
    //     width:dbg!(30 * scale),
    //     height:50
    // };
    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. 
    // dbg!(&rect1);

}



// We'll add the rectangles program here 

// it would be more readable and more manageable to group width and height together 
// fn area(width:u32, height:u32) -> u32 {
//     width * height 
// }

//versions with tuple - tuples help us add a bit of structure, but this version is less clear - tuples don;t allow us to name our params , so we have to index into the parts of the tuple , we would have to keep in mind that width is tuple index 0 and height is tuple index 1
// fn area(dimensions:(u32,u32)) -> u32 {
//     dimensions.0 * dimensions.1 
// }



//REFACTORING WTH STRUCTS
// We use structs to add meaning by labeling the data - lets transform the tuple into a struct 

// Rust includes the functionality to include debugging information , but we have to explicitly opt in to make the functionality available for the struct , to do this we add the attribute below 
// #[derive(Debug)]
// struct Rectangle {
//     width:u32,
//     height:u32,
// }

//The area function accesses the width and height fields of the Rectangle instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs).
fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
