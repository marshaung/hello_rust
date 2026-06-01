use anyhow::{Context, Result};
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

fn greet(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("名字不能是空的");
    }
    println!("{}", format!("Hello, {}!", name).green().bold());
    Ok(())
}

fn run_progress() -> Result<()> {
    println!("\n{}", "載入中...".yellow());

    let pb = ProgressBar::new(10);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:30.cyan/white} {pos}/{len} {msg}")
            .context("進度條樣式設定失敗")?,
    );

    for i in 0..10 {
        pb.inc(1);
        if i == 9 {
            pb.set_message("完成！");
        }
        thread::sleep(Duration::from_millis(80));
    }
    pb.finish();
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        greet(&cli.name)?;
    }

    println!("{}", format!("2 + 3 = {}", add(2, 3)).blue());

    run_progress()?;

    println!("{}", "✓ 全部完成".green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn greet_empty_name_fails() {
        assert!(greet("").is_err());
    }

    #[test]
    fn greet_valid_name_ok() {
        assert!(greet("Mars").is_ok());
    }
}
