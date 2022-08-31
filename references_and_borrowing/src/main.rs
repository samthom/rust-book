fn main() {
    let mut s1 = String::from("hello");

    let l = calculate_length(&s1);
    change(&mut s1);

    println!("The length of {} is {}", s1, l);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// We call creating a reference borrowing. Just as variables are immutable by default, so are
// references. We can use mutable references for that.
// we can only have one mutable reference to a single value
// if we have one immutable reference to something, we cannot have a mutable reference after that

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_improved(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
