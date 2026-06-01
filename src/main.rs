use clap::Parser;

#[derive(Parser)]
#[command(name = "hello_rust", about = "一個簡單的 Hello World CLI 程式", version)]
struct Cli {
    /// 要打招呼的名字
    #[arg(short, long, default_value = "Mars Huang")]
    name: String,

    /// 重複幾次
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn add(left: u32, right: u32) -> u32 {
    left + right
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        println!("Hello, {}!", cli.name);
    }

    println!("2 + 3 = {}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
