fn main() {
    println!("Hello, world!");
    another_function(5);
    let value = five();
    println!("Returned {}", value);
    let result = plus_one(4);
    println!("Addition result {}", result)
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}