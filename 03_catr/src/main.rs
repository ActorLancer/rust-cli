use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
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

// use anyhow::Result;
// use clap::Parser;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]   // 不使用长描述
// /// Rust version of `cat`
// struct Args {
//     /// Input file(s)
//     #[arg(value_name = "FILE", default_value = "-")]    // 在帮助信息中显示参数名为："FILE"；
//                                                         // 如果没有指定文件名，从标准输入读取
//     files: Vec<String>,

//     /// Number lines
//     #[arg(
//         short('n'),
//         long("number"),
//         conflicts_with("number_nonblank_lines") // 与 number_nonblank_lines 互斥，不能同时指定
//     )]
//     number_lines: bool,     // 是否为所有输出行编号

//     /// Number non-blank lines
//     #[arg(short('b'), long("number-nonblank"))]
//     number_nonblank_lines: bool,    // 是否只为非空行编号
// }

// // --------------------------------------------------
// fn main() {
//     if let Err(e) = run(Args::parse()) {
//         eprintln!("{e}");
//         std::process::exit(1);
//     }
// }
// // --------------------------------------------------
// fn run(args: Args) -> Result<()> {
//     for filename in args.files {
//         match open(&filename) {
//             Err(e) => eprintln!("{filename}: {e}"),
//             Ok(file) => {
//                 let mut prev_num = 0;
//                 for (line_num, line_result) in file.lines().enumerate() {
//                     let line = line_result?;
//                     if args.number_lines {
//                         println!("{:6}\t{line}", line_num + 1);
//                     } else if args.number_nonblank_lines {
//                         if line.is_empty() {
//                             println!();
//                         } else {
//                             prev_num += 1;
//                             println!("{prev_num:6}\t{line}");
//                         }
//                     } else {
//                         println!("{line}");
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }
// // --------------------------------------------------
// fn open(filename: &str) -> Result<Box<dyn BufRead>> {
//     match filename {
//         "-" => Ok(Box::new(BufReader::new(io::stdin()))),
//         _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
//     }
// }
