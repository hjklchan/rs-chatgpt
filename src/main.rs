
/// ## Rust 构建的价值一个亿的 ChatGPT 应用  
/// 
/// 如何运行?
/// 当前目录 `cargo run`
fn main() {
    loop {
        let mut s = String::new();
        match std::io::stdin().read_line(&mut s) {
            Ok(_) => println!("{}", s.replace("吗", "").replace("？", "！").replace("?", "!")),
            Err(err) => eprintln!("读取问题时发生错误, err: {}", err.to_string())
        }
    }
}
