use clap::Parser;   // 用于命令行参数解析


#[derive(Parser)]   // 自动实现命令行参数解析功能
#[command(author, version, about)]      // 自动从 Cargo.toml 获取作者、版本和描述信息，用于 --help 输出
/// Rust version of `echo`
struct Args {
    /// Input text
    #[arg(required(true))]  // 指定该命令行参数是必须的，也就是说，用户在运行时必须提供这个参数，否则程序会报错并提示缺少参数
    text: Vec<String>,      // 用于接受文本，可以有多个单词

    /// Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,     // 模仿 Unix echo -n（不换行）
}

fn main() {
    // 生成 Args 实例
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),    // 把所有输入文本用空格拼接起来
        if args.omit_newline { "" } else { "\n" }
    );
}
