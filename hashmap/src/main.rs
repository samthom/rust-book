use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores { // iterate over hashmap
        println!("{}: {}", key, value);
    }

    // Hashmap and ownership
    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // println!("{}: {}", field_name, field_value) This is invalid, but this is valid if we passed
    // the references to the insert. But the values has to be valid till the end of hashmap's
    // lifetime

    // adding value using insert to the same key present will overwrite the existing value.
    
    // Adding a Key value Only if a Key isn't present
    let mut s = HashMap::new();
    s.insert(String::from("Blue"), 10);

    s.entry(String::from("yellow")).or_insert(50);
    s.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", s);

    let text = "hello world wonderful world";

    let mut mp = HashMap::new();

    for word in text.split_whitespace() {
        let count = mp.entry(word).or_insert(0);
        *count += 1; // .or_insert() returns a mutable references to the count, we have to use * to
        // dereference the value. Right after the increment the variable goes out of scope so the
        // changes are safe.
    }

    println!("{:?}", mp);
}
