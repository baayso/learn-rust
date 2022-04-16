use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let team = vec![blue, yellow];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = team.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);

    // 编译错误：blue和yellow所有权已经移动到Vec中了
    // println!("{} {}", blue, yellow);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    map.insert("demo", 1);

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!("{:#?}", map);

    let val = match map.get("demo") {
        Some(value) => *value,
        None => -1,
    };

    if val >= 0 {
        println!("{}", val);
    } else {
        println!("在Map中未找到！")
    }
}
