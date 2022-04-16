use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(22);
    set.insert(33);
    set.insert(22);

    println!("{:?}", set);

    match set.get(&33) {
        Some(val) => println!("{}", val),
        None => println!("not found"),
    }

    println!();

    for x in set {
        println!("{}", x);
    }
}
