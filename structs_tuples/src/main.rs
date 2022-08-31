#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rectl = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rectl,
        area(&rectl)
    );
}

fn area(rectl: &Rectangle) -> u32 {
    rectl.width * rectl.height
}
