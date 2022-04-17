use std::collections::HashSet;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Pair<T> 总是实现了 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 使用 trait bound 有条件地实现方法
// 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法
// 只有那些为 T 类型实现了 PartialOrd trait（来允许比较）和 Display trait（来启用打印）的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 也可以对任何实现了特定 trait 的类型有条件地实现 trait。
// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，
// 他们被广泛的用于 Rust 标准库中。
// 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样：
// impl<T: Display> ToString for T {
//     // --snip--
// }

// 因为标准库有了这些 blanket implementation，
// 我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法。
// 例如，可以将整型转换为对应的 String 值，因为整型实现了 Display：
// let s = 3.to_string();

fn main() {
    let pair = Pair::new(HashSet::from([2, 3, 4]), HashSet::from([5, 6, 7]));
    // 编译错误：method cannot be called on `Pair<HashSet<{integer}>>` due to unsatisfied trait bounds
    // pair.cmp_display();

    let pair = Pair::new(String::from("a"), String::from("A"));
    pair.cmp_display(); // 输出：The largest member is x = a
}
