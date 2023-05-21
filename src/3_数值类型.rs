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

