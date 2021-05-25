/**
 * There are multiple trade-offs to consider in addition to the prevention of bugs.
 * For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances.
 * With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through,
 * so lower performance might be a worthwhile penalty for gaining that clarity.
 */

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
