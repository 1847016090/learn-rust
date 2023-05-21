// ! 字符类型
// 1. 所有的 Unicode 值都可以作为 Rust 字符，由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
// 2. Rust 的字符只能用 '' 来表示， "" 是留给字符串的
fn define_char() {
    let x = "中";
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x)); // 4

    let y = "中";
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&y)); // 16
}

// !布尔(bool)
//两个值： true 和 false，且都是1个字节
fn define_bool() {
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }
}

// ! 单元类型
// 单元类型就是 ()
// 没有返回值的函数被定义为：发散函数( diverge function )，顾名思义，无法收敛的函数
// main，println!()等 函数就是发散函数
