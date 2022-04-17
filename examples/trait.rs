use std::fmt::{Debug, Display};

// trait：定义共享的行为
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 带默认实现的方法
    // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    // 必须重写，因为Summary trait中该方法没默认实现
    fn summarize_author(&self) -> String {
        format!("NewsArticle@{}", self.author)
    }

    // 可选重写，因为Summary trait中该方法有默认实现
    fn summarize(&self) -> String {
        // todo!(); // 运行错误：not yet implemented
        format!(
            "{}, by {} ({} - {})",
            self.headline,
            self.author,
            self.location,
            self.summarize_author()
        )
    }
}

// Tweet 实现 Summary trait
impl Summary for Tweet {
    // 必须重写，因为Summary trait中该方法没默认实现
    fn summarize_author(&self) -> String {
        format!("Tweet@{}", self.username)
    }
}

impl Tweet {
    // 返回实现了 trait 的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("中国空间站"),
            content: String::from("这里是中国空间站！"),
            reply: false,
            retweet: false,
        }
    }
}

// trait 作为参数
pub fn notify_1(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}

// Trait Bound 语法
pub fn notify_2<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
}

// 通过 + 指定多个trait 作为参数
pub fn notify_3(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// 通过 + 指定多个 trait bound
pub fn notify_4<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，
// 所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息
fn some_function_1<T: Clone + Display, U: Clone + Debug>() -> i32 {
    0
}

// 通过 where从句 简化 trait bound
// 在函数签名之后的 where从句 中指定 trait bound 的语法
fn some_function_2<T, U>(t: T, u: U) -> i32
where
    T: Clone + Display,
    U: Clone + Debug,
{
    0
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let tweet2 = Tweet::returns_summarizable();

    println!("1 new tweet: {}", tweet2.summarize());
}
