use std::collections::HashMap;

fn main() {
    // the type HashMap<K,V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
     scores.insert(String::from("Yellow"), 50);

     //hashmapos stores data on the heap 
     // all values must have the same type, all keys must have the same type 

     //accessing values in a HashMap

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    // we would try to get the value associated with the teamname, it returns an Option<132> if theres no value for that key, it will return None, the program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32> as from the definition of Copied -> Maps an Option<&T> to an Option<T> by copying the contents of the option, then unwrap_or to set score to zero if scores doesn't have an entry for the key 

    for (key,value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmaps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map= HashMap::new();

    map.insert(field_name, field_value);

    // we cannot use the variables field_name and field_value again they have been moved into the hash map with the call to insert 

    //Updating a hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

// This code will print {"Blue": 25}. The original value of 10 has been overwritten.

//Adding a Key and Valeu only if a key is present

//Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

//The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. 

//Updating a value based on the old value 

// another common use case for hash maps is to look up a key’s value and then update it based on the old value. 

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);

    *count += 1;
}

// Exercises 

}
