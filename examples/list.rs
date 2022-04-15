enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    let mut list = vec![1, 2, 3];
    list.push(4);

    let x1 = &list[2];
    println!("{}", x1);

    match list.get(100) {
        Some(item) => println!("{}", item),
        None => println!("not found"),
    }

    let mut list2 = Vec::new();
    list2.push(1);
}
