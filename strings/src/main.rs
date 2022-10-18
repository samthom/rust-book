
fn main() {
    // String is a wrapper arround Vec<T> with some extra restrictions and capabilites.
    let mut _s = String::new();

    let data = "Hello World.";
    let _s = data.to_string();

    let _s1 = String::from(data);

    // Remember strings are UTF-8 encoded.

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let d1 = String::from("Hello, ");
    let d2 = String::from("World!");
    let _d3 = d1 + &d2; // note d2 has been moved here and can no longer be used

    let r1 = String::from("tic");
    let r2 = String::from("tac");
    let r3 = String::from("toe");
    let _r = format!("{}-{}-{}", r1, r2, r3);

    // Rust avoid indexing strings to avoid confusion as string is treated as Vec<u8> as in go 
    // So using index to fetch a single char will return uninteded value that can confuse the user.
    // Hence the concept indexing string is avoided altogther.

    let hello = "Здравствуйте";
    let _s = &hello[0..4]; // This is valid, but have to be careful because [0..1] will return an
    // error since first byte might not create an entire char sometimes

    for c in hello.chars() { // returns each char
        println!("{}", c);
    }

    for b in hello.bytes() { // returns each byte value
        println!("{}", b);
    }
}
