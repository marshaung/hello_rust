use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

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

    // 彩色問候
    for _ in 0..cli.count {
        println!("{}", format!("Hello, {}!", cli.name).green().bold());
    }

    // 計算結果用藍色顯示
    println!("{}", format!("2 + 3 = {}", add(2, 3)).blue());

    // 進度條示範
    println!("\n{}", "載入中...".yellow());
    let pb = ProgressBar::new(10);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:30.cyan/white} {pos}/{len} {msg}")
            .unwrap(),
    );

    for i in 0..10 {
        pb.inc(1);
        if i == 9 {
            pb.set_message("完成！");
        }
        thread::sleep(Duration::from_millis(80));
    }
    pb.finish();

    println!("{}", "✓ 全部完成".green());
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
