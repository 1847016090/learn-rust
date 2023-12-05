// 通用编程概念

// !数值类型
// 整数是没有小数部分的数字。
// i32 表示有符号的 32 位整数（ i= integer , u，= unsigned 无符号）
// 以下是所有整数类型
// i8，i16，i32，i64，i128，isize
// u8，u16，u32，u64，u128，usize
// 有符号的数据范围在-(2n - 1) ~ 2n - 1 - 1（n表示n次方）
// 无符号的数据范围在0 ~ 2n - 1 - 1（n表示n次方）
// (isize 和 usize 类型取决于程序运行的计算机 CPU 类型： 若 CPU 是 32 位的，则这两个类型是 32 位的，同理，若 CPU 是 64 位，那么它们则是 64 位)

// todo 整型溢出没看懂，需要再看看

// !浮点类型
// 分为两种: f32 和 f64；默认浮点类型是 f64
fn define_float() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// !NaN
// 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较
// 使用函数is_nan可以判断是否是NaN
fn define_judge_nan() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

// !数字运算
//包含了 +，-，*，/，%
fn define_calculate() {
    // 加法
    let sum = 1 + 1;
    // 减法
    let difference = 2 - 1;
    // 乘法
    let product = 2 * 4;
    // 除法
    let quotient = 12 / 3;
    // 求余
    let remainder = 3 % 2;
    println!(
        "sum={},difference={},product={},quotient={},remainder={}",
        sum, difference, product, quotient, remainder
    )
}

//!位运算
// & 位与   相同位置均为1时则为1，否则为0
// | 位或   相同位置只要有1时则为1，否则为0
// ^ 异或	相同位置不相同则为1，相同则为0
// ! 位非	把位中的0和1相互取反，即0置为1，1置为0
// << 左移	所有位向左移动指定位数，右位补0
// >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
fn define_bitwise_operation(){
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;
}

// !序列(Range)
// 常用于循环里面
// 使用1..5 ,生成从 1 到 4 的连续数字
// 使用1..=5，生成从 1 到 5 的连续数字，包含 5
fn define_range(){
    // 输出1-4
    for i in 1..5 {
        println!("{}", i)
    }
    // 输出a-z
    for i in 'a'..='z' {
        println!("{}", i)
    }
}

// !有理数和复数
// 使用社区库:num
// 使用步骤很简单
// 主要目录下面使用: cargo add num等待安装完毕即可

use num::complex::Complex;
fn define_complex(){
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;
   println!("{} + {}i", result.re, result.im)
}


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

// !语句
// 语句指的是完成了一个具体的操作，但是并没有返回值。
// 例如:
fn define_statement() {
    let x = 1;
    let (a, mut b) = (true, false);
}

// !表达式
// 表达式会进行求值，然后返回一个值
// 调用一个函数是表达式，因为会返回一个值，
// 调用宏也是表达式
// 用花括号包裹最终返回一个值的语句块也是表达式
// 总之，能返回值，它就是表达式
fn define_object() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

// ! 表达式不能包含分号，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值
// ! 表达式如果不返回任何值，会隐式地返回一个 ()



// !函数

// !要点
// 1. 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
// 2. 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
// 3. 每个函数参数都需要标注类型

// !函数参数
// 必须为每一个函数参数都标识出它的具体类型，否则就会报错
fn add(i: i32, j: i32) -> i32 {
    i + j
}

// !函数返回
// 函数的返回值就是函数体最后一条表达式的返回值
// 当然我们也可以使用 return 提前返回:
fn plus_five(x: i32) -> i32 {
    x + 5
}

// !Rust 中的特殊返回类型
// 无返回值()
// 1. 函数没有返回值，那么返回一个 ()
// 2. 结尾的表达式返回一个 ()

// !永不返回的发散函数 !
// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

pub mod homework {
    // 摄氏温度与华氏温度的相互转换。 1摄氏度 = 33.8 华氏温度
    pub fn transfer_temperature_to_hua(tem: f32) {
        let transferred_value = tem * 33.8;
        println!(
            "摄氏温度{a}度转为华氏摄氏度为{b}",
            a = tem,
            b = transferred_value
        )
    }
    pub fn transfer_temperature_to_wen(tem: f32) {
        let transferred_value = tem / 33.8;
        println!(
            "华氏摄氏度{a}度转为摄氏温度为{b}",
            a = tem,
            b = transferred_value
        )
    }
    // 生成一个n阶的斐波那契数列。
    pub fn fib(n: i8) -> i8 {
        if n == 1 || n == 2 {
            return 1;
        }
        return fib(n - 1) + fib(n - 2);
    }
    // 打印圣诞颂歌The Twelve Days of Christmas的歌词，并利用循环处理其中重复的内容。
    pub fn deal_song() -> Result<String, std::io::Error> {
        use std::fs;
        let result: Result<String, std::io::Error> = fs::read_to_string("./static/song.txt");
        let content = match result {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        return Ok(content);
    }
}