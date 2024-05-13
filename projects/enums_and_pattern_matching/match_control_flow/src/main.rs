fn main() {
    //the match control flow construct 
    // compare a value against a series of patterns and execute code based on which patterns matched

    println!("Hello, world!");
    value_in_cents(Coin::Quarter(UsState::Alabama));

    // matching with Options
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // println!("{:?}", six)

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other)
        // _  => reroll(),
        _ => ()
    }

    // rust also has a patter we can ise when wewant a catch all but don;t want to use the value in the catch all pattern , _ is a specoal pattern 
}

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn value_in_cents(coin:Coin) -> u8 {
//     match coin {
//         Coin::Penny =>  {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// using match statements can feel like using if values - but the differnce here is that if statments have to evalute to a boolean value , match statments can be any type 

//Patterns that bind to values 
// they can bind to the parts of the values that match the pattern , this is how we can extract values out of enum variants 

// let's change one of our enum variants to hold data inside it 

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//if we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska) , when we compare the value wth each of the match arms , none of them match till we reach Coin::Quarter(state) , we can use the binding , and get the inner state valie out of coin enum variant for Quarter 

//Matching with Option<T>

// we can also handle the Option<T> valueusing match the way we handled enum instead of caompring Coins, we compre the variants of Option<T> - the way match expression works remains the same 

//let write a function that takes and Option<i32> and if theres a value inside returns the value + 1 , but if theres no value, it returns the None value and not attempt to perform any operations

// fn plus_one(x:Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// combining match and enums is very useful - you will see this pattern inside a lot of Rust code , match against an enum, bind a variable to the data inside, and then execute code based on it

//Matches are Exhaustive
// the arms patterns must cover all possibilities

// we have to cover the None case - our ide already tells us that we didn't do that 
// if preents us from assuming we have a value when we might have null 
// fn plus_one(x:Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


//Catch all Patterns and the _Placeholder 
// Using enums, we can take special actions for a particular values , but for all other values take one default action

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}