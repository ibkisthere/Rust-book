use std::collections::vec_deque;

fn main() {
    let v:Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // you can reference values in vectors by either using indexing or the get method
    let v = vec![1,2,3,4,5];

    // using &[] gives us a reference to the value
    let third: &i32 = &v[2];

    println!("The third element is {third}");

    // using get gives us an Option<T> that we can use 
    let third:Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),

        None => println!("There is no third element"),
    }

    // the reason wehy we have two methods to get the element from a vector using the index is that we can control how your program behaves 

    //100 is out of range, in this code here, the first line will panic, then the next one will return None 
    let v = vec![1,2,3,4,5];

    let does_not_exist = &v[100];

    let does_not_exist = v.get(100);

    // also rememeber that we can;t have mutable and immutable references in the same scope, When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid

    let mut v = vec![1,2,3,4,5];

    let first = &v[0];

    v.push(6);

    // we can use a for loop to get immutable references to each vector 
    let v = vec![100,32,57];

    for i in &v {
        println!("{i}");
    }

    // we can also iterate over mutable references to each element to each element in amutable vector in order to make changes to all the elements. 

    let mut v = vec![100,32,57];

    // to change the value that the mutable reference refers to we us the * dereference operator to get the value in i before we use the += operator 

    for i in &mut v {
        *i += 50;
    }

    // using an enum to store multiple types 
    // vectors can only store values that are the same type - which cna be incovnenient because there are usecases where we want to store a list of tiems of different types 

    // suppose we want to use a vector to store data from the rows of a spreadsheet where each row has different data types - one is a string, one is a num , on is text, we can use the enum data type to store it 

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    //Dropping a vector drops its elements 
    // Like any other struct , a vector is freedd when it goes out of scope 
    {
        let v = vec![1,2,3,4];
        //do stuff with v 
    }
    // v goes out of scope and is freed here 
    // the borrwo check ensure that the references to the vector are only used when its contents are still valid
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

