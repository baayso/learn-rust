fn main() {
    println!("Hello World!");

    let val = 99u8; // byte
    match val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("{}", val),
    }

    let val = Some(3u8);
    if let Some(3) = val {
        println!("three")
    } else {
        println!("others")
    }
}
