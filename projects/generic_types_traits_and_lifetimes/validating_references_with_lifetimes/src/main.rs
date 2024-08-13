use std::fmt::Display;

fn main() {
    // lifetimes ensure that the references are as long as we want thm to be 
    // lifetimes are the scope for whcih the refernces are vlaid , thay can either be implicit or inferred 

    
    //preventing dangling references with lifetimes 
    // you will get an error that x did not live long enough , because we are trying to reference r that is out of scope , the reason why ris still valid is because it is alos in the outer scope - so we sya that r lives long enough 
    // how ddoe srust determine that the code is valid - we use a borrow checker 
    // let r;
    
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {r}");
    // The borrow checker 

    // the rust compiler has a borrow checker to compare scopes to determine whether all borrows are valid

    // we fix the code so we have no dangling references and the code compiles with no errors 

    let x = 5;

    let r = &x;

    println!("r: {r}");

    //Generic lifetime in functions
    // we'll write a fn that returns the longer of two string slices
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{
        part:first_sentence,
    };
}


// this fns return type contains a borrows value , but the signature does now say whether it is borrwed from x or y , consider introducing a lifetime specifier

// why do we have this error ?  we don't know the concrete values that will be passed into this fn so we don;t know whether it is the if case or else case will execute , we also don;t know the concrete lifetimes of the references that will be passed in , so we can;t look at the scopes to deterine whether the references we return wil always be valid The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis. 

// fn longest(x: &str, y : &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

//Lifetime Annotation Syntax 

// Lifetime annotations don;t cjhange how long any of the references live , rather they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes just as fns can acept any type when specified with a generic type parameter , functions can accept references of any lifetime by specifying a generic lifetime parameter 

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


//Lifetime annotations in function signatures

// We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add it to each reference,


//The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.

fn longest<'a>(x :&'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lets look at how the lifetime annotations restrict the longest function by passing in references that have diferent lifetimes 

// if we changed the longes fucntion to always return a refernce to the first parameter and not the second parameter then we wouldn't need a lifetime specifier for the second parameter 

// fn longest1<'a>(x:&'a str, y : & str) -> &'a str {

// }

// fn longest1<'a>(x:&'a str, y : & str) -> &'a str {
    // let result = String::from("really long string");
    // result.as_str()
// }

//Lifetime Annotations in struct definitions 

// we declare the name of the generic lifetime parameter inside angle brackets after the name in the struct 
struct ImportantExcerpt<'a> {
    part : &'a str 
}


//Lifetime elision 

// the reason this piece fo rsut code compiles wihtout lifteimes annoataions is actually historiacal -> in earlier version of rust , the rust team found out that the rust programmers were entering the same lifetime annotations over and over in particular situations and decided to code this into the compiler 
//These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

//these patterns coded into the compiler are called lifetime elision rules , these are not rules for prgrammers to follow , but are rules that the compiler can recognize , if your code fits these cases , you do not need to write the lifetimes explicitly 

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
         if item == b' ' {
            return &s[0..i];
         }
    }

    &s[..]
}

// lifetimes on fn or method parameters are called input parameters lifetimes on return values are called output lifetimes 

// the compiler uses three rules to figure out the lifetimes of the references when there aren't explicit annotations 

// first applies to input lifetimes , second amd third apply to output lifetimes , if it gets to the end of the three rules and it has references for which it can't figure out a lifetime then it will end in an error 

// the compiler assigns lifetimes to each parameter that is a reference 

// if there is one input parameter the lifetime is assigned to all output parameters 

//if there are multiple input lifetime parameters but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters 


//lifetime anotations in method definitions 

impl<'a> ImportantExcerpt<'a> {
    //we do not need to annotate with lifetimes because of rule number 1 - 1st lifetime elision rule 
    fn level(&self) -> i32 {
        3
    }

    // because of the first -> rust gives both &self and announcement their own lifetimes
    // becaue of the third rule, the return type gets the lifetime of &self 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    } 
}


// the static lifetime 

// 'static

//the affected reference can live for the entire duration of the program , all string literals have the static lifetime , which we can annotate as follows 

//Generic Type parameters Trait Bounds, and Lifetimes Together 

fn longest_with_an_announcement<'a,T>(
    x :&'a str,
    y : &'a str, 
    ann : T,
) -> &'a str where T: Display {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y 
    }
}
