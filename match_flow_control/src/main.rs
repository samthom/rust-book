#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Washington,
    Nevada,
    Utah,
    Nebraska,
    Texas,
    Ohio,
    Florida,
    Hawai,
    NewJersey,
    Virginia,
    Colorado,
    Philadelphia,
    Pennsylvania,
    Oregon,
    Michigan,
    Arizona,
    NorthCarolina,
    Illinois,
    Indiana,
    NewYork,
    Tennessee,
    Minnesota,
    Montana,
    Maryland,
    Maine,
    Kentucky,
    Missouri,
    Wisconsin,
    Connecticut,
    Oklahoma,
    NewMexico,
    Mississippi,
    SouthCarolina,
    Louisiana,
    SouthDakota,
    Kansas,
    Arkansas,
    Idaho,
    Iowa,
    Wyoming,
    RhodeIsland,
    Vermont,
    WestVirginia,
    Delaware,
}
enum Coin {
    Penny,
    Dime,
    Nickel,
    Quarter(UsState),
}

fn main() {
    println!("value is {}", value_in_cents(Coin::Penny));
    println!("value is {}", value_in_cents(Coin::Dime));
    println!("value is {}", value_in_cents(Coin::Nickel));
    println!("value is {}", value_in_cents(Coin::Quarter(UsState::Utah)));
    // --------------------------------------------------------

    let five = Some(5);
    let six = plus_one(five);
    let none: Option<i32> = None; // Binding `None` to a variable needs to be type annotated
    let none_1 = None::<i32>;

    println!("five: {}, six: {}, none: {}", five.unwrap(), six.unwrap(), 0);

    let mut count = 0;

    if let Coin::Quarter(state) = Coin::Quarter(UsState::Utah) {
        println!("State quarter from {:?}", state)
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
        _ => 5 // default match (_other can also be used)
        // _ => () // We are telling Rust explicitly that we aren't going to use any other values
        // that doesn't match the pattern in an earlier arm, and we don't want to run any code in
        // this case.
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
