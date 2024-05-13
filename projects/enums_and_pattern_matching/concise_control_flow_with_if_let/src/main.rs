fn main() {
    // the if let syntax lets you combine if and let into a less verbose way to ahndle values that match one pattern whil e ignoring the rest 
// Here, if the value is Some, we print out the value in the Some variant by bindning to the variable max in the pattern 
// also, to satisfy the match expression we haeve to add _ => (), because we are not matchiing anything
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
// we could write this in a shorter way using if let syntax 
let config_max_1 = Some(3u8);
// the code in the if let isn;t run if the vlaue doesn;t match the pattern 
//In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }

    // we can use match with enums and the values the variants they hold like this 
    //  let coin = Coin::Penny;
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // or use it with an if let statement
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
