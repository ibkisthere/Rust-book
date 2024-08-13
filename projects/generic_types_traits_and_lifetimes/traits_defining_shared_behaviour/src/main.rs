use std::{fmt::{Debug, Display}};



fn main() {
    //We can use traits to define functionality that a particular type has and can share with other types
    // we can use it to deine shared behavioour in an abstract way 
    // they are similar to a feature called interfaces in other languages , but with some differences 

  


    // let tweet = Tweet {
    //     username:String::from("horse_ebooks"),
    //     content:String::from(
    //         "of course,as you probably already know, people",
    //     ),
    //     reply:false,
    //     retweet:false,
    // };

    // println!("1 new tweet:{}", tweet.summarize());

        let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
     username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
};


// println!("1 new tweet: {}", tweet.summarize());
}


//Definng a trait 
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline:String,
    pub location :String,
    pub author :String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// to use a defualt implementation to sumarize instances of NewsArticle we specify an empty impl block 


// default implementation mized with methods that require implementation
pub trait Summary1 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


impl Summary1 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters - it accepts any parameter that implements the Summary trait 
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}" , item.summarize());
// }

//Trait bound Syntax 
// The impl Trait works for straightforward cases but is actually syntax sugar for a longer form known as trait bound 
//This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

pub fn notify<T:Summary>(item: &T) {
    println!("breaking news! {}", item.summarize());
}


// the impl trait syntax is convenient and makes for more concise , the fuller trait bound syntaz can express more complexity in other cases , for example , we can have two parameters that implement Summary Doing so with imple Trait syntaz will look like this 
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// }

//Using impl trait is appropriate if we want this fn to allow item1 and item2 to have different types , if we want both parameters to have the same type , we must have a trait bound, like this 

// pub fn notify<T:Summary>(item1: &T, item: &T) {

// }


//Specifying multiple trait bounds with the + Syntax 

//we can also specify more than one trait bound , Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:

// pub fn notify(item: &(impl Summary + Display)) {}

// the + Syntax is also valid with trait bounds on generic types: 

// pub fn notify<T:Summary + Display>(item: &T) {}

//clearer trait bounds with where clauses 


// each geenric has its own trait bounds so fns wiht multiple generic type parameters can contain lots of trait bound information between the fns name and parameter list , making the fn signature hard to read , for this reason, rust has an alt syntaz for specifying trait bounds inside a where clause after the fn signature

// so instead of writing this 
// fn some_function<T:Display + Clone, U:Clone + Debug>(t: &T, u : &U) -> i32 {}

// we can use a where clause like this :

// fn some_function<T,U>(t:&T, u :&U) -> i32 
// where 
//     T:Display + Clone, 
//     U:Clone + Debug,
// {

// }

// returning types that implement traits 

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// we specify that the returns_summarizable function returns some type that implements the SUmmary trait without naming the concrete type 

//This isn't allowed

//if and else have incomaptiable types 
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


//using trait bounds to conditionally implement methods 

//or example, the type Pair<T> in Listing 10-15 always implements the new function to return a new instance of Pair<T> (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type of the impl block, which in this case is Pair<T>). But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.


struct Pair<T> {
    x:T,
    y:T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y}
    }
}


impl <T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}