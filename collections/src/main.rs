fn main() {
    //  vectors
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];

    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);

    v1.push(4);

    let second: &i32 = &v[1];
    println!("The second element is {}", second);

    let fifth: Option<&i32> = v1.get(4);
    match fifth {
        Some(fifth) => println!("The fifth element is {}", fifth),
        None => println!("There is no fifth element."),
    }

    let arr = vec![0, 1, 2, 3, 4, 5];
    for i in &arr {
        println!("{}", i);
    }

    let mut arr1 = vec![0, 1, 2, 3, 4, 5];
    for i in &mut arr1 {
        *i += 1;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
