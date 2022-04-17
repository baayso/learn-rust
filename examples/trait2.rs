fn largest_1<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];

    // item    &T
    // &item   T
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_2<T: PartialOrd + Clone>(list: &[T]) -> &T {
    // &list[0] 等于 &(list[0])
    // &list[0] 是 &T
    // list[0]  是 T
    let mut largest: &T = &list[0];

    // item  是 &T
    // &item 是 T
    for item in list.iter() {
        // &largest 是 &&T
        // largest  是 &&T
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let string_list = vec![String::from("hello"), String::from("world")];

    let result = largest_1(&number_list);
    println!("The largest number is {}", result);

    let result = largest_1(&char_list);
    println!("The largest char is {}", result);

    let result = largest_2(&string_list);
    println!("The largest string is {}", result);

    let result = largest_2(&number_list);
    println!("The largest number is {}", result);
}
