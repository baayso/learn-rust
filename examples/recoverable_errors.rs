use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};

fn main() {
    let path = "hello.txt";
    let f = File::open(path);

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // let f = File::open(path).unwrap();
    let f = File::open(path).expect(&format!("Failed to open {}", path));
}

fn read_username_from_file() -> Result<String, io::Error> {
    let path = "hello.txt";

    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let path = "hello.txt";

    let mut f = File::open(path)?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let path = "hello.txt";

    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;

    Ok(s)
}

// 将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，
// 它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
// 当然，这样做就没有展示所有这些错误处理的机会了
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
