use substring::Substring;

fn main() {
    let str_1: &str = "Hello";

    // let str_2: String = String::from("NiHao");
    let mut str_3: String = "NiHao".to_string();
    str_3.push_str(" World");
    str_3.push(';');
    println!("{}", str_3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    let length = "Hello".to_string().len();
    println!("String \"Hello\".len(): {}", length);
    let length = "你好".to_string().len();
    println!("String \"你好\".len(): {}", length);

    let w = "你好".to_string();
    println!("String \"你好\".chars().count(): {}", w.chars().count());

    for ch in w.chars() {
        println!("{}", ch);
    }

    for b in w.bytes() {
        println!("{}", b);
    }

    // 判断指定位置是不是一个合法的UTF-8边界
    let index: usize = 3;
    if w.is_char_boundary(index) {
        let x: &str = &w[..index];
        println!("{}", x)
    }

    let w: String = "中国China".to_string();

    let str = match w.get(0..=5) {
        Some(str) => str,
        None => {
            println!("不是有效的字符");
            ""
        }
    };

    println!("{}", str);

    // 第三方库 https://crates.io/crates/substring
    let str: &str = w.substring(0, 1);
    println!("{}", str);
    let str: &str = w.substring(1, 2);
    println!("{}", str);
    let str: &str = w.substring(2, w.chars().count());
    println!("{}", str);
}
