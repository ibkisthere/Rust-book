
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // We can construct relative paths that begin in the parent module, rather than the current module or crate, by using super at the start of the path - equivalent to using ".." in a filesystem
        super::deliver_order();
    }
    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit:String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // to access the fields fo a struct , you have to make the struct and the field public
    //In contrast, if we make an enum public, all of its variants are then public. 
    // We only need the pub before the enum keyword,
     pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn cook_order() {}
}

mod front_of_house {
    pub mod hosting {
        // making the module public does not make the contents public , the pub keyword only lets code in its ancestor refer to it, 
        // and not its inner code , its not much to make the module public , but to make the contents public 
       pub fn add_to_waitlist() {}

        fn seat_at_table(){}
    }
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    //Absolute Path - start with crate , to root of our crate module's tree
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path 
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);


    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    

    // using the enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

