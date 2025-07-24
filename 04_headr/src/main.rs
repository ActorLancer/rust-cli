use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};


#[derive(Parser)]
#[command(author, version, about, long_about(None))]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,

    /// Number of lines to print
    #[arg(
        short('l'),
        long("lines"),
        value_name("LINES"),
        default_value("10"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        short('c'),
        long("bytes"),
        value_name("BYTES"),
        conflicts_with("lines"),
        // 类型安全：自动把命令行输入的字符串转换为指定的类型
        // 自动校验：可以加上范围、枚举等校验，防止用户输入非法值
        // 错误提示：如果用户输入不合法，clap 会自动给出友好的错误提示
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    // 只要字段类型是字符串，且不需要特殊校验，就不需要 `value_parser`
    // 如果字段类型是数字、布尔、枚举等，或者你想加范围/格式校验，就需要 `value_parser`
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();

    for (file_index, file_name) in args.files.iter().enumerate() {
        match open(file_name) {
            Err(err) => eprintln!("{file_name}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {file_name} <==",
                        if file_index > 0 { "\n" } else { "" }
                    );
                }
                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 { break; }
                    print!("{line}");
                    line.clear();
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}



// use clap::Parser;
// use anyhow::Result;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// /// Rust version of `head`
// struct Args {
//     /// Input file(s)
//     #[arg(default_value = "-", value_name = "FILE")]
//     files: Vec<String>,

//     /// Number of lines to print
//     #[arg(
//         short('n'),
//         long("lines"),
//         default_value = "10",
//         value_name = "LINES",
//         value_parser = clap::value_parser!(u64).range(1..)
//     )]
//     lines: u64,

//     /// Numbers of bytes
//     #[arg(
//         short = 'c',
//         long,
//         value_name = "BYTES",
//         conflicts_with = "lines",
//         value_parser = clap::value_parser!(u64).range(1..)
//     )]
//     bytes: Option<u64>,
// }

// // --------------------------------------------------
// fn main() {
//     if let Err(e) = run(Args::parse()) {
//         eprintln!("{e}");
//         std::process::exit(1);
//     }
// }

// fn run(args: Args) -> Result<()> {
//     let num_files = args.files.len();

//     for (file_index, file_name) in args.files.iter().enumerate() {
//         match open(file_name) {
//             Err(err) => eprintln!("{file_name}: {err}"),
//             Ok(mut file) => {
//                 if num_files > 1 {
//                     println!(
//                         "{}==> {file_name} <==",
//                         if file_index > 0 { "\n" } else { "" }
//                     );
//                 }

//                 if let Some(num_bytes) = args.bytes {
//                     let mut buffer = vec![0; num_bytes as usize];
//                     let bytes_read = file.read(&mut buffer)?;
//                     print!(
//                         "{}",
//                         String::from_utf8_lossy(&buffer[..bytes_read])
//                     );
//                 } else {
//                     let mut line = String::new();
//                     for _ in 0..args.lines {
//                         let bytes = file.read_line(&mut line)?;
//                         if bytes == 0 {
//                             break;
//                         }
//                         print!("{line}");
//                         line.clear();
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// fn open(filename: &str) -> Result<Box<dyn BufRead>> {
//     match filename {
//         "-" => Ok(Box::new(BufReader::new(io::stdin()))),
//         _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
//     }
// }
