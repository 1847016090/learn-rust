// !定义变量
// 定义变量规则 https://course.rs/practice/naming.html
// 具体命名规则：
// 1. 模块，变量，方法，函数使用蛇形命名法，例如： snake_name
// 2. 静态类型，常量使用纯大写的蛇形命名法，例如： SNAKE_NAME
// 3. 其余命名均使用大驼峰命名法, 例如：SnakeName
fn define_variables() {
    // let x = 5;
    // x = 6; // error
    // 正确的定义可变变量的方式
    let mut x: i32 = 5;
    x = 6;
    println!("{}", x)
}

// !使用下划线开头忽略未使用的变量
// 当我们有变量未使用时，我们需要给他加一个下划线避免报错
fn define_unused_variables() {
    let _x: i32 = 5;
}

// !进行变量解构
fn define_var_deconstruction() {
    let (a, mut b): (bool, bool) = (true, false);
    b = true;
    // a = true,不可变; b = false，可变，使用assert_eq进行断言
    assert_eq!(a, b);
    println!("a = {}, b = {}", a, b);
}

// todo 解构式赋值，学习完Struct之后再回来补充

// !怎么定义一个常量
// 1. 必须使用const
// 2. 不能使用mut
// 3. 值的类型必须标注
fn define_constant_variable() {
    const MY_NAME: u32 = "stephen";
    println!("{}", MY_NAME)
}

// !变量遮蔽
// 定义两个变量会涉及一次内存对象的再分配
// 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好
fn define_shadow_variable() {
    let x = 5;
    let x = 6;
    println!("{}", x)
}
