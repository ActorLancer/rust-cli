use anyhow::Result;
// Parser: 为结构体自动生成命令行参数解析逻辑。
// ArgAction: 指定参数的行为（如果是否是布尔开关、计数器等）。
// ValueEnum: 让枚举类型可以直接作为参数值，并自动生成帮助信息。
// PossibleValue: 定义参数可能的取值（用于补全、校验等）。
use clap::{Parser, ArgAction, ValueEnum, builder::PossibleValue};
use regex::Regex;
// 递归遍历目录；WalkDir 可以递归遍历一个目录下的所有文件和子目录
// DirEntry 表示遍历到的每一个文件的或目录项
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `find`
struct Args {
    /// Search path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Names
    // 用正则表达式过滤文件名，可以指定多个
    #[arg(
        short('n'),
        long("name"),
        value_name("NAME"),
        // 用 Regex::new 解析参数值（即参数会被当作正则表达式）
        value_parser(Regex::new),
        // 没出现一次参数就追加到 names 向量中（允许多次出现）
        action(ArgAction::Append),
        // 参数数量为 0个或更多（可选，可多次）
        num_args(0..),
    )]
    names: Vec<Regex>,

    /// Entry types
    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        // 用 EntryType 枚举解析参数值
        // clap::value_parser! : 生成适合某种类型的解析器（必须实现 ValuEnum trait）
        // 自动将命令行参数（如d、f、l）转换成指定的枚举变量
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..)
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
// 用枚举可以让你的代码类型安全，参数只能是这三种之一，避免字符串拼写错误
/// 表示文件系统中的三种条目类型
enum EntryType {
    Dir,    // 目录（directory）
    File,   // 普通文件（file）
    Link,   // 符号链接（symlink）
}

impl ValueEnum for EntryType {
    // 返回所有可选的枚举值
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    // 把每个枚举值映射为命令行参数字符串
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    // 如果没有指定类型过滤，不过滤任何类型；否则只保留匹配的类型
    let type_filter = |entry: &DirEntry| {
    args.entry_types.is_empty()
        || args.entry_types.iter().any(|entry_type| match entry_type {
            EntryType::Link => entry.file_type().is_symlink(),
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
        })
    };

    // 如果没有指定名称过滤，不过滤任何名称，否则只保留文件名能被任一正则表达式匹配的项
    let name_filter = |entry: &DirEntry| {
        args.names.is_empty() || args.names.iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path_name in &args.paths {
        let entries = WalkDir::new(path_name)
            .into_iter()     // 返回一个迭代器，产生 Result<DirEntry, Error> 类型的值
            // 过滤和映射迭代器中的值，跳过 None,保留 Some
            .filter_map(|e| match e {
                Err(err) => {
                    eprintln!("{err}");
                    None
                },
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
