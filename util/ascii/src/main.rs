use transpose::transpose;   // 对切片中的元素进行矩阵转置

// 列优先显示
// --------------------------------------------------
fn main() {
    // 创建一个 u32 类型的向量，这些整数包括了 ASCII 字符的十进制值
    let range = (33..=127).collect::<Vec<u32>>();

    // 这个向量将用于存储转置后的 ASCII 码值
    let mut nums = vec![0; 95];

    // &range: 输入数据，即包含 ASCII 码值的 range 向量的引用
    // &mut nums: 输出数据，转置后的结果都会存储在这个可变向量中
    // 19: 输入矩阵的行数
    // 5: 输出矩阵的列数
    // 意味着 range 向量被视为一个 19 行 5 列的矩阵进行转置，然后存储在 nums 中
    transpose(&range, &mut nums, 19, 5);

    //
    let vals: Vec<String> = nums
        .iter()
        .map(|i| {
            format!(
                // 右对齐，宽度为 3
                "{i:3}: {}",
                if *i == 127 {
                    "DEL".to_string()
                } else {
                    std::char::from_u32(*i).unwrap().to_string()
                }
            )
        })
        .collect();

    // vals.chunks(5): 将 vals 向量分成多个切片，每个切片包含5个字符串
    let rows: Vec<&[String]> = vals.chunks(5).collect();
    for row in rows {
        // 将当前中的所有字符串用制表符 \t 连接起来
        println!("{}", row.join("\t"));
    }

}
