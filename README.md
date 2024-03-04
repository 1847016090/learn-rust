# 笔记+总结

## 1 [入门](./src/enter_01.rs)

## 2 [猜字谜游戏](./src/guest_game_02.rs)

## 3 通用编程概念

### 3.1 变量

在rust中我们声明一个变量使用`let`,例如：

```rust
let x: i8 = 1;
```

**rust变量命名规范**使用`蛇形命名法`, 如`snake_name`,例如：

```rust
let my_name: &str = "stephen"
```

当我们声明了一个字符串变量，但是我想不改变变量名的情况下，重新声明一个数字类型，这时候我们可以使用rust变量`变量遮蔽`的特性，例如：

```rust
let x: i8 = 1;
let x: &str = "stephen"
```

当我们不需要更换类型，我们只需要在`let`后面加上一个可变`mut`符修复，表明当前的变量可以修改，例如：

```rust
let mut x: i8 = 1;
```

### 3.2 常量

在rust中，我们声明一个常量使用`const`关键字，并且命名使用`大写蛇形命名法`，例如：

```rust
const MY_NAME: &str = "stephen"
```

我们使用`const`声明的变量不能再此修改，否则会编译错误。

### 3.3 数据类型

#### 3.3.1 标量数据类型

标量分为：

1. 整数
2. 浮点数
3. 布尔值
4. 字符串

##### 3.3.1.1 整数

整数分为`带符号`和`不带符号`的整数，我们使用`i`开头表示带`正负`号，`u`开头表示不带符号(表示正数)，例如：

```rust
let with_symbol: i32 = -1; // rust默认推导出类型为i32
let without_symbol: u32 = 1;
```

`带符号`和`不带符号`的整数的长度类型各有6种，如下：
|带符号|不带符号|
|----|----|
|i8|u8|
|i16|i16|
|i32|u32|
|i64|u64|
|i128|u128|
|isize|usize|

其中`i8`和`u8`分别表示的含义(下面的n,表示n次方)，如下：

- `i8` 表示该数字的范围在 -(2n - 1) ~ (2n - 1)之间，也就是-255 ~ 255之间
- `u8` 表示该数字的范围在 0 ~ (2n - 1)之间，也就是 0 ~ 255之间

`isize`和`usize` 表示，如果系统的CPU是32位，则表示`i32``u32`，如果是64位，则表示`i64``i64`

##### 3.3.1.2 浮点数

在rust中，我们用`f`表示声明的是一个浮点数变量，例如：

```rust
let float_number: f64 = 1.0;
```

浮点数有两种长度类型，`f32`和`f64`，长度范围和整数一致。

**补充：**

- 尽量不要长度越界
举个例子，当我们使用`i8`的长度类型，但是我实际的长度为256，这个时候会出现长度越界的情况，实际编码中避免出现这种问题。

##### 3.3.1.3 布尔值

在rust中，我们声明一个布尔值如下：

```rust
let booleans: bool = false;
```

布尔值类型有两种值，`true`或者`false`

##### 3.3.1.4 字符&字符串

在rust，我们使用`单引号 ''`声明一个字符：

```rust
let character: &str = 'a';
```

使用`双引号 ""`声明一个字符串字面量：

```rust
let strings: &str = "stephen"
```

当我们想声明一个动态长度的字符串怎么生成呢？
这时候我们需要使用`String`的`new`方法去生成(后面会详细讲到)，使用`String`生成的字符串，会存储到堆中，并在栈中存储指向堆中的**指针**、**长度**、**容量**，例如：

```rust
let mut str: String = String::new();
str.push_str("hello")
str.push_str(", world")
```

#### 3.3.2 复合数据类型

复合类型分为：

1. 元组
2. 数组

##### 3.3.2.1 元祖

在rust中，我们创建一个元祖，使用`()`，元祖可以包含任意的数据类型，例如：

```rust
let tuple: (i32, &str) = (1, "stephen")
```

当我们需要元祖中某个数据项时，我们可以使用索引的方式，例如：

```rust
let tuple: (i32, &str) = (1, "stephen");
let first: i32 = tuple.0; // 1
let second: &str = tuple.1; // "stephen"
```

元祖的类型、长度是固定不变的，长度根据生成时决定，如果尝试修改会编译错误。但是可以使用变量的`变量遮蔽`去覆盖则不会报错。例如：

```rust
let tuple: (i32, String) = (1, String::new());
let tuple: (i32, i32) = (1, 1); // 正确编译

let mut tuple: (i32, String) = (1, String::new());
tuple: (i32, i32) = (1, 1); // 编译失败
```

当元祖没有数据时，也叫`单元类型`，当我们的函数没有返回时，其实返回的就是单元类型.

```rust
let unit_type:() = ();
```

##### 3.3.2.2 数组

在rust中，我们创建一个数组，使用中括号`[]`，**数组只能存储相同类型**，例如：

```rust
let mut arr: [i8; 2] = [1,2];
```

其中`[i8;2]`表示的含义是，第一个是你的数组每一项的类型，第二个表示数据的个数也就是数组的长度；

数组的长度是固定的，不能对其修改，否则会编译错误，例如：

```rust
let mut arr: [i8; 2] = [1, 2];
arr.push(2) // 编译出错
```

那如果我们想创建一个动态长度的数组呢？我们我们可以使用`Vec`结构上面的`new`方法来生成(后面会详细讲到)，或者使用简单的方式，`vec![]`，例如：

```rust
let mut arr: Vec<i32> = Vec::new(); // 不能初始化数据
arr.push(2) // 编译出错

let arr1: Vec<i32> = vec![1，2，3]; // 可以初始化数据
```

### 3.4 注释

在rust中，当我们需要注释的时候我们一般使用`//`后续会详细讲到

### 3.5 if 表达式

在rust中我们一般使用`if`来作为条件判断的一个表达式，例如：

```rust
let right: bool = true;
if right {
    print!("正确")
} else {
    print!("错误")
}
```

**注意**
我们放到`if`表达式中的`right`值必须保证是一个`布尔值`，否则会编译错误，因为它不会自动转化类型，例如：

```rust
let right: i32 = 1;
if right { // 编译错误
    print!("正确")
} else {
    print!("错误")
}
```

在rust中，`if`会有一个返回值，我们可以使用`if`进行条件判断，获取返回值进行后续操作，但是需要保证`if`，`else if`，`else`返回的数据类型相同，，例如：

```rust
let return_str: &str = if true { "right" } else { "wrong" };
print("{}", return_str)
```

并且这里我们可以使用`"right"`直接返回该值，非常方便。

这里强调一下，**语句和表达式**，在一个模块中，当我们没有使用`;`时，其实可以理解它就是一个表达式，它会返回该值。

### 3.6 循环

rust中的循环分为3种

- loop
- while
- for

对于`loop`的使用，我们可以使用`continue`跳过当前循环，使用`break`打断当前循环，例如：

```rust
 let mut index = 0;
    loop {
        index = index + 1;
        if index == 3 {
            continue; //跳过
        }
        println!("{}", index);
        if index == 5 {
            break; // 打断
        }
    }
```

但是每次写`break`显得很多余，于是我们可以使用`while`循环来优化，例如：

```rust
let mut index = 0;
    while index < 5 {
        index += 1;
        if index == 3 {
            continue;
        }
        println!("{}", index);
    }
```

但是当我们循环遍历数组的时候，每次都要去处理这个`index`值，很麻烦也不好维护，于是我们使用第三种最常见的循环遍历方法，`for`循环，例如：

```rust
let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in arr {
        if i == 3 {
            continue;
        }
        println!("{}", i)
    }
    println!("========:========");
    for i in 1..6 {
        if i == 3 {
            continue;
        }
        println!("{}", i)
    }
```

**[课后作业](src/general_program_concept_03.rs)**

- 摄氏温度与华氏温度的相互转换。
- 生成一个n阶的斐波那契数列。
- 打印圣诞颂歌The Twelve Days of Christmas的歌词，并利用循环处理其中重复的内容。

## 4 认识所有权

### 4.1 所有权

当我们对一个没有实现可`Copy`特征的数据进行移动时，此时就发生了所有权的转移，例如：

```rust
let x = String::new();
let y = x; // 所有权发生了移动，此时我们再去打印x的值，会发生错误
println!("{}", x); // 报错

let basic: &str = "hello world";
let copy_basic: &str = basic;
println!("{}", basic); // 可以正常打印出来
```

还有什么情况会发生所有权的移动呢？当我们向函数传参时，也会发生，传参的过程就相当于声明了一个隐式的变量。例如：

```rust
let mut str: String = String::new();
str.push_str("hello");
fn calc_len(x: String) -> usize {
    return x.len();
}
let len: usize = calc_len(str); 
println!("{}", len);
println!("{}", str); // 报错，所有权转移到了函数内部，并且在函数执行完毕之后就被销毁
```

那么哪些数据是可`Copy`的呢？现在我们把所有默认具有该特征的数据类型列出来：

- 所有的标量数据类型
  - 整数
  - 浮点数
  - 字符串字面量
  - 布尔值
- （只包含标量数据类型的）元祖

### 4.2 引用和借用

那么当我想要传入函数(所有权力的例子)后，不失去所有权呢？
此时，我们就需要引入`引用`这个概念了。

我们想要引用一个数据时，需要在变量名前面加上`&`，这个时候我们可以调用它内部的一些函数，例如：

```rust
let mut str: String = String::from("hello");
fn calc_len(x: &String) -> usize {
    return x.len();
}
let len = calc_len(&str);
println!("{}", len);
println!("{}", str); // 报错，所有权转移到了函数内部，并且在函数执行完毕之后就被销毁
```

那如果我想对该变量进行修改呢？很简单，我么可以直接在`&`加上`mut`属性。例如：

```rust
let mut str: String = String::from("hello");
fn add_string(x: &mut String) {
    x.push_str(", world")
}

add_string(&mut str);
println!("{}", str);
```

### 4.3 切片

当我们想获取字符串(数组)的其中某一段的时候，这个时候我们就需要使用到`切片`的功能，例如：

```rust
let str: String = String::from("hello");
let x = &str[1..3];
println!("{}", x) // "el"
```

其中对于`[1..3]`我们可以理解为

- 首位数为起始索引(包含)，末位数为结束索引(不包含)
- 当首位数没有的时候，默认为0，例如: `[..3]`
- 当末位数没有的时候，默认为最后一个索引，例如: `[1..]`
- 首位和末位都没有的时候，默认为整个字符串(数组)

## 5 结构体

### 5.1 声明一个结构体

在rust中，当我们需要声明一个结构体，需要在使用`struct`这个关键字，例如：

```rust
struct User {
    name: String,
    age: i8,
    height: i8,
}
let user = User {
    name: String::from("stephen"),
    age: 18,
    height: 176,
};
```

结构题还可以这样来简化一个初始化的传参，例如：

```rust
fn build_user(name: String, age: i8) -> User {
    User { name, age, height: 176 }
}
```

当我们想接受另外一个实例的数据，如果我们一个一个传进去，参数多了就会非常麻烦，我们可以使用`..`来将重复的参数传入进去，例如：

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: i8,
    height: i32,
}
let user = User {
    name: String::from("stephen"),
    age: 18,
    height: 176,
};

let user1 = User {
    name: String::from("james"),
    ..user
};

```

### 5.2 打印结构体

当我们要打印一个结构体实例的时候，如果使用`println("{}", user1)`是完全不够的，因为打印结构体是需要实现`Display`特征的，但是我们可以直接使用下面的方法来打印，例如：

```rust
println!("{:?}", user1); // 不会格式化
println!("{:#?}", user1) // 会格式化
```

### 5.3 元祖结构体

我们还可以定义一种特殊的结构体，叫做元祖结构体，例如：

```rust
struct Position(i16, i16, i16);
let pos = Position(1, 1, 1);
println!("{}", pos.0)
```

我们可以直接和元祖一样去结构，或者使用索引`pos.0`去访问具体的值。

### 5.3 空结构体

当我们需要为一个结构体加上`特征trait`时，并且不需要存储任何的数据时，这时候我们可以声明一个空结构体,例如：

```rust
struct User;
```

### 5.4 方法

我们可以为结构体声明方法，这个方法的第一个参数，永远返回的地是结构体的实例，例如：

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: i8,
    height: i32,
}
impl User {
    fn is_adult(&self) -> bool { // 为User定义一个判断是否是成人的方法
        self.age >= 18
    }
}
let user = User {
    name: String::from("stephen"),
    age: 18,
    height: 176,
};
println!("{:#?}", user.is_adult())
```

我们也可以给方法传入更多的参数，例如：

```rust
impl User {
    fn compare_age(&self, other: &User) -> bool {
        // 为User定义一个判断是否是成人的方法
        self.age >= other.age
    }
}
let user = User {
    name: String::from("james"),
    age: 17,
    height: 203,
};
let user1 = User {
    name: String::from("stephen"),
    age: 18,
    height: 176,
};
println!("我的年纪更大:{}", user1.compare_age(&user))
```

### 5.5 关联函数

我们也可以声明一个类似于`String::from`的一个关联函数，我们的参数可以自定义，例如：

```rust
impl User {
    fn relative_fn(name: String, age: i8) -> User {
        User {
            name,
            age,
            height: 176,
        }
    }
}

let user = User::relative_fn(String::from("stephen"), 18);
println!("{:#?}", user)
```

## 6. 枚举

### 6.1 定义枚举

在rust中我们使用`enum`定义一个枚举，并且枚举使用`大驼峰`的命名方式，例如：

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

那我们应该如何使用枚举呢？我们可以直接在定义好的枚举名后面使用`::`就可以访问到枚举内的数据，如下：

```rust
Color::Blue
```

我们还可以使用`enum`定义`不同变体`的枚举，例如：

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u16, u16, u16), // 元祖结构体
    Hex { input: String }, // 结构体
}
println!("{:#?}", Color::Hex { input: String::from("#000") });
println!("{:#?}", Color::RGB(255, 255, 255));
```

rust还为我们内置了`Option`枚举，用于处理一个不可靠的值，假如该值可能不存在，那我们可以使用该枚举来定义值；当我们完全能确认我们定义的值存在，那就没有必要使用，后面我们会结合着`match`使用。例如：

```rust
let _not_confirmed_value: Option<i8> = Some(1);
let _not_confirmed_value: Option<i8> = None;
```

### 6.2 为枚举定义方法

枚举和结构体一样，我们使用`impl`为其定义方法,如果第一个参数是`&self`表示需要生成实例后调用；否则我们只需要使用`::`调用即可。例如：

```rust
 #[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn instance_fn(&self) {
        println!("{:#?}", self);
    }
    fn static_fn() {
        println!("{:#?}", Color::Red)
    }
}

Color::static_fn(); // Red
let color = Color::Blue;
color.instance_fn();  //Blue
```

### 6.3 `match`匹配

当我们使用枚举类型，并且想要使用一个变量值去判断是否和其中的某一项相等，如果我们直接使用`if`去判断，会报错，这个时候就引出`match`，例如：

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn match_color(color: Color) {
    match color {
        Color::Blue => {
            println!("{}", "蓝色")
        }
        Color::Green => {
            println!("{}", "绿色")
        }
        Color::Red => {
            println!("{}", "红色")
        }
    }
}
match_color(Color::Blue);
```

由于rust里面的`match`是穷尽匹配，所有我们需要把所有的每项列出来，这样会很麻烦，如果数据过多，这时我们可以使用通用匹配符号`_`，例如：

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
fn match_color(color: Color) {
    match color {
        Color::Blue => {
            println!("{}", "蓝色")
        }
        _ => println!("其余颜色"), // 通配符
    }
}

match_color(Color::Red);
```

我们刚刚提到的内置的`Option`枚举类型，我们可以使用`match`来对其进行匹配，例如：

```rust
fn add_one(value: Option<i8>) -> Option<i8> {
    match value {
        Option::Some(i) => Some(i + 1),
        Option::None => None,
    }
}
let num: Option<i8> = Some(5);
let num: Option<i8> = add_one(num);
println!("{:#?}", num) // Some(6)
```

上面我们使用`Some(5)`去匹配`Some(i)`依然能匹配上，这里因为i是`i8`的数字类型，和我们定义的变量相同，所以能够匹配上，否则就会匹配到`None`

### 6.4 `if let`匹配

如果我们只想匹配枚举中的一个项，而不想写通配符，这个时候我们就可以使用`if let`来匹配枚举，例如：

```rust
enum Color {
    Red,
    Green,
    Blue,
}
let color: Color = Color::Blue;
if let Color::Blue = color {
    println!("蓝色")
}
```

## 7 包和模块

### 7.1 二进制单元包(项目)和库单元包

#### 7.1.1 什么是二进制单元包

二进制单元包也就是我们通过`cargo new project`生成的项目，最外层存在一个`Cargo.toml`的文件作为项目安装的文件目录，`src/main.rs`为该项目的根文件

#### 7.1.2 什么是库单元包

而库单元包则是使用`cargo new library --lib`命令生成，`src/lib.rs`是该项目的根文件

#### 7.1.3 单元包可以拆解成不同的模块

在Rust，库单元包可以由很多单元包组成，而单元包则可以由很多的模块组成

### 7.2 定义模块

那在Rust中我们应该怎么定义模块呢？我们需要使用关键字`mod`，例如：

```rust
mod my_module {}
```

我们可以在模块里面，定义函数，结构体，枚举，模块，例如：

```rust
mod my_module {
    fn test() {}
    mod other_module {}
    struct User {}
    enum Color {}
}
```

### 7.3 共有模块和私有模块

在模块中，Rust默认所有的属性都是私有属性，所以当我们需要在一个模块、文件中使用另外一个模块的内容，我们需要使用`pub`关键字声明它是一个公有属性，例如：

```rust
pub mod my_module {
    pub fn test() {}
    pub mod other_module {}
    pub struct User {}
    pub enum Color {}
}
```

其中结构体和枚举的共有声明有点差别，其中结构体不仅需要给自己添加`pub`关键字，自身的属性也需要添加才能对外暴露使用，而枚举之需要给自己添加`pub`即可，例如：

```rust
pub mod my_module {
    pub struct User {
        pub age: i8, // 公有
        name: String, // 私有
        email: String, // 私有
    }
    pub enum Color {
        Red, // 公有
        Green, // 公有
        Blue, // 公有
    }
}
```

### 7.4 引用模块

首先我们先声明一个模块，并且里面还包含了其他模块和方法，例如：

```rust
mod restaurant {
    pub mod free {
       pub fn add_rice() {}
    }
    mod money {
        fn add_vegetable(){}
        fn add_fruits() {}
    }
   
}
```

这里我们要讲到三个关键字，`self`，`super`，`crate`，我们讲讲他们的作用。
其中`self`，表示它可以引用自身模块的项，例如：

```rust
mod restaurant {
    pub mod free {
       pub fn add_rice() {}
    }
    mod money {
        fn add_vegetable(){}
        fn add_fruits() {
            // 其中add_vegetable、add_fruits 是最近一个模块money的所属方法
            self::add_vegetable(); 
            self::add_fruits();
        }
    }
}

self::restaurant::free // 但是这样显得多次一举，我们可以直接 restaurant::free
```

对于`super`， 它代表的是以父模块开始的引用，可以引用父模块同层的变体，例如：

```rust
mod restaurant {
    pub mod free {
       pub fn add_rice() {}
    }
    mod money {
        fn add_vegetable(){}
        fn add_fruits() {
            super::free;  // 引用父模块的同层
            super::money; // 引用父模块的同层
        }
    }
    fn here(){
        super::user // 可以调用 模块 restaurant 同层的模块
    }
}

mod user {}
```

对于`crate`，表示的是从`src/main.rs`或者`src/lib.rs`模块开始的引用，表示为根目录，我们可以在根目录的任何层级中使用`crate`调用该文件中的任何共有的变体，例如：

```rust
mod restaurant {
    mod free {
        pub fn add_rice() {}
        mod soup {
            fn add_soup() {
                // 我们在这里如果想要使用money中的方法，只有使用crate才能拿到该方法
                crate::restaurant::money::add_vegetable()
            }
        }
    }
    mod money {
        pub fn add_vegetable() {}
        fn add_fruits() {}
    }
}
```

### 7.5 拆分文件

在日常开发中，我们肯定不会将所有的方法之类的放到一个文件夹中，所以我们可以将这些变体单独提取到一个文件中，我们新建一个文件和`main.rs`同层级，例如：

```rust
// 我们将上面的restaurant模块单独拆分到restaurant文件中
// restaurant.rs 
mod restaurant {
    pub mod free {
       pub fn add_rice() {}
    }
    mod money {
        fn add_vegetable(){}
        fn add_fruits() {}
    }
   
}
```

这个时候，我们需要在`main.rs`来引用这个文件中的模块，此时，我们需要使用`use`关键字，例如：

```rust
mod restaurant; // 等同于直接将文件引入进来
use restaurant::free; // 此时就可以可以用到该模块
```

当我们需要在当前的文件中，把引入的模块再此导出去供其他文件使用，我们可以在`use`前面加一个`pub`关键字，例如：

```rust
mod restaurant; 
pub use restaurant::free; // 此时外部可以使用free模块
```

#### 7.4.3 全部引用

当我们想将一个模块中的所有变体都引入到当前的作用域中，我们可以使用`*`，例如：

```rust
use restaurant::*;
```

#### 7.4.4 解构引用

我们也可以将引入的模块解构来使用，例如：

```rust
use restaurant::{free, free::add_rice};
```

#### 7.4.5 重命名

我们在上面的引用，很有可能导致名字的重复，这个时候我们可以对引入的变体进行重命名，例如：

```rust
use restaurant::free as new_free;
```

## 8 通用集合类型

通用集合类型分为三种：动态数组，动态字符串，哈希映射，他们都会被存储在内存堆上。我们首先来看看动态数组。

### 8.1 动态数组

对于动态数组而言，我们只能存储相同数据类型的值。
首先我们先来创建一个动态数组，我们可以使用`Vec::new()`，例如：

```rust
let arr: Vec<i8> = Vec::new();
```

如果我们想往里面存储值，我们可以调用该实例的`push`方法，但是我们得先将变量名声明为可变`mut`，例如：

```rust
let mut arr: Vec<i8> = Vec::new();
arr.push(1);
arr.push(2);
println!("{:?}", arr)
```

我们使用`Vec::new()`只能声明一个空的动态数组，如果我们想直接生成一个具有初始值的动态数组呢？
我们可以使用`vec!`宏，例如：

```rust
let arr: Vec<i8> = vec![1,2,3];
```

我们在最开始提到，动态数组只能存储相同的数据类型，但是如果我们存储的值有字符串，数字等等呢？这个时候我们可以结合着枚举来使用，存储不同数据类型的值，例如：

```rust
#[derive(Debug)]
enum StoreData {
    Int(i32),
    Float(f64),
    Str(String),
    Boolean(bool),
}

let mut arr: Vec<StoreData> = Vec::new();

arr.push(StoreData::Float(1.0)); // 存储浮点数
arr.push(StoreData::Int(2)); // 存储整数
arr.push(StoreData::Str(String::from("这是一串字符"))); // 存储字符串
arr.push(StoreData::Boolean(true)); // 存储布尔值
println!("{:?}", arr);
```

讲了创建动态数组，接下来我们讲讲如何获取动态数组的值。
一般有以下两种方式，一种是使用索引，另一种是使用`get`方法。我们先使用索引的方式，例如：

```rust
let arr = vec![1, 2, 3];
println!("{}", &arr[0]); // 正常输出1
```

但是如果我们访问一个，超过数组长度的索引，会报越界访问的错误，例如：

```rust
println!("{}", &arr[5]); // index out of bounds: the len is 3 but the index is 5
// 所以我们需要要避免越界访问的出现
```

但是如果我们使用`get`的形式，去获取动态数组的值，它会返回一个`Option`的枚举，如果出现**访问越界**的情况，我们可以使用`Option::None`去处理，例如：

```rust
let arr = vec![1, 2, 3];
match &arr.get(5) {
    Option::Some(i) => {
        println!("{}", i);
    }
    Option::None => {
        println!("出现数组越界的情况"); // match匹配会达到这里，因为出现了数组越界的情况
    }
}
```

当然，如果我们需要批量读取数据的时候，这个时候我们就需要使用循环了，例如：

```rust
let arr = vec![1, 2, 3];
for i in &arr {
    println!("{}", i);
}
```

那如果我们要修改里面的数据，比如，我需要将数组里面的数据都乘以2， 例如：

```rust
let mut arr = vec![1, 2, 3];
for i in &mut arr {
    *i *= 2; // 这里我们需要对元素解引用才能修改元素
    println!("{}", i);
}
```

### 8.2 动态字符串

接下来我们讲讲，如何创建一个动态字符串，它和动态数组有点类似，我们可以直接使用`String`上面的`new`方法，例如：

```rust
let str: String = String::new();
```

这个实例创建好了之后，是没有初始值的，我们可以使用`push`或者`pus_str`方法手动为他赋值，例如：

```rust
let mut str = String::new();
str.push('h'); // push 方法只能传一个字符进去
str.push('e');
str.push('l');
str.push('l');
str.push('o');
str.push_str(", world"); // push_str可以传入一个字符串字面量
println!("{}", str);
```

当然我们可以直接将字符串相加来得到一个最终的字符串，例如：

```rust
let mut str1 = String::new();
str1.push_str("hello");
let mut str2 = String::new();
str2.push_str("world");
let str3 = str1 + &str2; // 实质是调用了内置的一个add(self, &str) -> String
println!("{}", str3);
println!("{}", str1); // 报错，这里str1的所有权转移到了str3;
```

但是这样显得非常得复杂，所以我们使用`format!`宏来快速拼接字符串，例如：

```rust
let mut str1 = String::new();
str1.push_str("hello");
let mut str2 = String::new();
str2.push_str("world");
let str3 = format!("{}{}", str1, str2);
println!("{}", str3);
```

当我们我们也可以使用`String::from`快速地创建并且初始化值，例如：

```rust
let str: String = String::from("hello, world");
```

如果我们有一个字符串字面量，我们想把它转成`String`类型， 我们可以使用`to_string`去转化，例如：

```rust
let new_str: String = "hello, world".to_string();
println!("{:?}", new_str);
```

字符串是不支持使用索引获取的，那我们应该如何获取字符串的某一段的值呢，这个时候我们可以使用之前提到过的字符串切片，例如：

```rust
let str_value = String::from("hello, world");
let new_str = &str_value[0..1];
println!("{}", new_str);
```

但是这里有个问题，因为rust是使用UTF-8作为编码格式，每一个字符占一个字节，但是如果是Unicode标量的话，它会占用两个字节，如果我们使用字符串切片不当就会报错，例如：

```rust
let str_value = String::from("дравствуйте");
let str_slice = &str_value[0..1]; // 因为д占两个字节，这里我只取一个字节，就会发生panic，所以对字符串使用切片需要小心谨慎
println!("{}", str_slice)
```

我们还可以遍历每个标量值，例如：

```rust
let str = String::from("Здравствуйте");
for i in str.chars() {
    println!("{}", i);
}
```

也可以把每个字节都遍历出来，例如：

```rust
let str = String::from("Здравствуйте");
for i in str.bytes() {
    println!("{}", i);
}
```

### 8.3 哈希映射

在rust中，我们创建`HashMap`同样使用其中的`new`方法，因为该集合类型用的比较少，所以不在rust的预导入模块中，我们需要使用`use std::collections::HashMap;`手动引入，例如：

```rust
use std::collections::HashMap;
let hm = HashMap::new();
```

如果我们想往里面添加值的话，我们使用`insert`方法，例如：

```rust
use std::collections::HashMap;
let mut hm = HashMap::new();
hm.insert(String::from("stephen"), 20);
hm.insert(String::from("james"), 30);
println!("{:?}", hm);
```

我们也可以使用`collect`方法把动态数组转化为哈希映射，其中`zip`的作用是创建一个元祖的数组，例如：

```rust
use std::collections::HashMap;

let arr1 = vec![String::from("stephen"), String::from("james")];
let arr2 = vec![20, 30];
let ages: HashMap<_, _> = arr1.iter().zip(arr2.iter()).collect();
println!("{:?}", ages)
```

我们同样也可以使用`get`方法，去获哈希映射里面的值，它返回的也是`Option`枚举，例如：

```rust
hm.get(&String::from("stephen")) // Some(20)，这里我们需要使用值的引用
```

遍历哈希映射也是使用`for`， 例如：

```rust
for (key, value) in &hm {
    println!("{}-{}", key, value);
}
// james-30
// stephen-20
```

那我们应该怎么去覆盖之前的值呢？ 为了保险，我们得先判断哈希映射里面有没有当前这个字段，这时候我们需要使用`entry`来判断，然后如果不存在的话，可以使用`or_insert`来插入值，有得话就会直接更新，例如：

```rust
use std::collections::HashMap;
let mut hm = HashMap::new();
hm.insert(String::from("stephen"), 20);
hm.insert(String::from("james"), 30);
hm.entry(String::from("kyrie")).or_insert(20);
println!("{:?}", hm)
```

## 9 错误处理

在Rust中，我们的错误处理有两种，分为可恢复处理和不可恢复处理。

### 9.1 不可恢复错误处理

#### 9.1.1 panic

我们首先举一个例子，这个例子会引发panic的报错，例如：

```rust
let arr = vec![1, 2, 3];
println!("{}", arr[100]);
```

当我们出现越界访问的时候，这个时候会触发panic报错，导致程序崩溃，我们在开发的时候应该尽量注意。这是个不可恢复的处理。

#### 9.1.2 错误回溯

当我们需要查看错误回溯的时候，我们可以使用`RUST_BACKTRACE=1 cargo run`命令，例如我们手动触发一个panic!报错：

```rust
fn main(){ // 在根文件中使用 panic! 抛出错误
    panic!("Error...")
}

// 控制台执行命令  RUST_BACKTRACE=1 cargo run
```

### 9.2 可恢复错误处理

我们在日常的开发中，更多的情况是，针对不同的错误进行不同的处理。

#### 9.2.1 Result 枚举

Rust在预模块中替我们引入`Result`枚举，里面包含了`OK`和`Err`两个变体的，`OK`表示正确变体，`Err`表示错误变体。接下来让我们来手动处理错误。

#### 9.2.2 手动处理错误

我们尝试写一段代码来打开一个不存在的文件，例如：

```rust
use std::fs::read;
let f = read("./hello-world.txt");
println!("{:?}", f)
```

因为我们这个文件是不存在的，所以这个时候会返回一个`Err`变体，这个时候我们可以使用`match`去处理，当我们的文件不同时，我们就创建这个文件，例如：

```rust
use std::fs;
use std::io::ErrorKind;

let f = fs::File::open("./hello-world.txt");
match f {
    Ok(data) => {
        println!("{:?}", data);
    }
    Err(error) => {
        return match error.kind() {
            ErrorKind::NotFound => {
                let create_data = fs::File::create("./hello-world.txt");
                match create_data {
                    Ok(file_data) => {
                        println!("{:?}", file_data);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            _ => {
                println!("{}", error);
            }
        };
    }
}
```

上面我们手动去处理读取文件失败的错误，然后创建文件，但是我们也要对创建文件去手动做错误处理，显得太麻烦了，于是我们可以用以下的方法去简化

#### 9.2.3 unwrap和expect快速处理错误

对于有返回`Result`枚举的方法、函数等，我们都可以使用`unwrap`和`expect`去处理。
对于`unwrap`，就相当于我们使用`match`去处理错误，只不过它会返回一个Rust默认的错误，例如：

```rust
use std::fs;

fs::File::open("./hello-world.txt").unwrap()
```

而`expect`，我们则可以穿入一个字符串，提示我们想提示的内容，例如：

```rust
fs::File::open("./hello-world.txt").expect("创建文件失败");
```

#### 9.2.4 向上返回结果和错误体

对于刚刚嵌套的问题，我们可以将其拆分成为多个函数，然后把我们读取或者创建文件后的`Result`变体想外部抛出，然后上一层去获取并且处理。我们现封装一个读取文件的函数，例如：

```rust
use std::io::ErrorKind;
use std::{fs, io};

// 读取文件
fn read_file(path: &str) -> Result<String, io::Error> {
    let content = match fs::File::open(path) {
        Ok(data) => data,
        Err(error) => {
            println!("{:?}", error);
            return Err(error);
        }
    };
    return Ok(content);
}

// 创建文件
fn create_file(path: &str) {
    match fs::File::create(path) {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(error) => {
            println!("{:?}", error)
        }
    }
}

// 在最外层处理封装函数的逻辑
let content = match read_file("./hello-world.txt") {
    Ok(data) => data,
    Err(error) => {
        return match error.kind() {
            ErrorKind::NotFound => {
                create_file("./hello-world.txt");
            }
            _ => {
                println!("其他错误");
            }
        };
    }
};
```

#### 9.2.5 优化处理错误处理(?)

即使我们做了函数的封装，让嵌套变少了，但是还是显得很麻烦，那还没有更简单的方法呢？当然有，我们有语法糖`?`，它会将存储在Ok内部的值返回给外部的变量。如果出现了错误，?就会提前结束整个函数的执行，并将任何可能的Err值返回给函数调用者，例如：

```rust
fn read_file() -> Result<String, io::Error> {
    let mut str: String = String::new();
    fs::File::open("./hello-world.txt")?.read_to_string(&mut str)?;
    println!("{}", str);
    Ok(str)
}
```

使用该方法，如果文件不存在，会自动把错误抛出，而不会panic崩溃。
所有返回`Result`或者`Option`枚举的，我们都可以使用该语法糖处理。

## 10 泛型、trait和生命周期

### 10.1 泛型

#### 10.1.1 什么是泛型

在讲泛型之前，我们先来抽象一个公用的求最大值的函数，例如：

```rust
fn max(arr: &[i32]) -> i32 {
    let mut largest: i32 = arr[0];
    for item in arr {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}
let largest = max(&[100, 2, 3, 4]);
println!("{}", largest);
```

通过上面的对于方法的一个抽象封装，我们可以同样来理解泛型。
泛型其实就是对于类型的抽象。接下来，我们先来讲讲泛型的命名并对上面的函数再次进行泛型的封装。

在Rust中，我们一般对泛型采用极为简洁的命名，**一个大写的字母**，比如:`T`,`U`等等，当然如果你的命名比较长，那就使用**大驼峰命名**。

那现在我们就来实现对上面函数的类型使用泛型，例如：

```rust
fn max<T>(arr: &[T]) -> T {
    let mut largest: T = arr[0];
    for item in arr {
        if *item > largest { // 比较会报错 consider restricting type parameter `T`: `:std::cmp::PartialOrd`
            largest = *item;
        }
    }
    largest
}
```

**注意**
使用泛型之后，我们上面的比较是会报错的，这是为什么呢？
因为我们传入的泛型可以理解为，**可以传入任何类型**，但是不是所有类型都具有**比较**这个行为的，所以编译的时候，Rust就会给我们提示出来。

这个问题，我们会在后面的**trait 特征**章节来解决。

#### 10.1.2 函数 && 泛型

在上面的列子中我们也看到了，怎么在一个函数中定义泛型。那就是在函数名之后使用`<>`尖括号将我们的泛型传入，这个时候我们就可以在函数中使用它了。例如：

```rust
fn test<T, U>() {} //可以传入多个泛型
```

#### 10.1.3 结构体 && 泛型

在结构体里面使用泛型，我们需要在结构体名后面的使用`<>`尖括号将泛型传入，例如：

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

#### 10.1.4 枚举 && 泛型

对于枚举类型，我们可以在枚举名后面使用`<>`尖括号将泛型传入，例如：

```rust
enum Color<T, U, K> {
    Red(T),
    Green(U),
    Blue(K),
}
```

#### 10.1.5 方法 && 泛型

如果我们需要为一个方法添加泛型，我们需要在`impl`后使用`<>`尖括号将泛型传入，例如

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

let p: Point<f64> = Point::new(1.0, 2.0);
println!("{:#?}", p);
```

### 10.2 特征(trait)

#### 10.2.1 定义trait

对于特征，可以简单理解为：对于不同或者相同类型的方法的抽象。
我们使用`trait`来定义特征，例如：

```rust
trait Food {
    fn eat(&self) -> ();
    fn add() -> ();
}

struct Home;

impl Food for Home {
    fn eat(&self) -> () {
        println!("eat vegetable")
    }
    fn add() -> () {
        println!("add food")
    }
}

struct Company;
impl Food for Company {
    fn eat(&self) -> () {
        println!("eat buff")
    }
    fn add() -> () {
        println!("add drinkings")
    }
}

let home = Home {};
home.eat();
Home::add();

let company = Company {};
company.eat();
Company::add();
```

我们在上面定义了一个特征`Food`，然后我们使用`impl for`的语法为类型`Home`，`Company`添加了这个特征，并且自定义了方法的内容

#### 10.2.2 特征默认行为

我们其实也可以给给特征定义默认的方法，例如：

```rust
trait Food {
    // 定义方法
    fn eat(&self) {
        println!("eat anything")
    }
}
```

#### 10.2.3 特征约束

我们首先简单说一下什么是**特征约束**，就是我们可以在泛型后面使用`:`加上我们定义的特征或者第三方的特征，当只有我们传入了实现了我们传入特征的类型，才能通过编译。

我们定义的特征可以作为一个参数传入到函数或者方法中，例如：

```rust
trait Food {
    // 定义方法
    fn eat(&self) -> () {
        println!("hh")
    }
}

fn home_eat(item: impl Food) {
    item.eat();
}

struct Home;
// 当我们不为Home声明Food特征时，就会发生编译错误，因为我们对item做了行为约束
impl Food for Home { 
    fn eat(&self) -> () {
        println!("eat")
    }
}
home_eat(Home)
```

如果我们对多个参数都要做这种约束，那么上面的写法会相当冗杂，我们可以结合泛型来做约束，我们将上面的`home_eat`修改一下，加上泛型`T`并且使用特征`Food`去约束它，例如：

```rust
fn home_eat<T: Food>(item: T) {
    item.eat();
}
```

那如果我们需要对一个泛型实现多个特征的约束呢？我们可以直接使用`+`将多个特征加起来，例如：

```rust
trait Food {
    fn eat(&self) -> ();
}
trait Water {
    fn drink(&self) -> ();
}

fn home_eat<T: Food + Water>(item: T) {
    // 这个时候，我们传入的Item必须实现Food的eat方法和Water的drink方法
    item.eat();
    item.drink();
}

struct Home;
impl Food for Home {
    fn eat(&self) -> () {
        println!("eat")
    }
}
impl Water for Home {
    fn drink(&self) -> () {
        println!("drink")
    }
}
home_eat(Home);
```

上面的写法还可以使用`where`来优化，例如：

```rust
fn home_eat<T>(item: T)
where
    T: Food + Water,
{
    // 这个时候，我们传入的Item必须实现Food的eat方法和Water的drink方法
    item.eat();
    item.drink();
}
```

我们还可以用特征来约束一个返回值，继续上面的修改，当我想返回一个带有`Food`特征的返回值时，我们可以这样：

```rust
fn home_eat<T>(item: T) -> impl Food // 只需要在末尾加上impl + 特征
where
    T: Food + Water,
{
    // 这个时候，我们传入的Item必须实现Food的eat方法和Water的drink方法
    item.eat();
    item.drink();
    item // item是实现了Food特征的，所以返回是不会报错的
}
```

上面我们都是对一个函数做约束，拿要是需要对我们为结构体声明的方法进行约束，那应该怎么做呢？只需要在`impl`后面的泛型加上特征约束即可，例如：

```rust
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: Copy> Rectangle<T> {
    fn new(x: T, y: T) -> Rectangle<T> {
        Rectangle {
            width: x,
            height: y,
        }
    }
}

// 正确，因为整数实现了Copy特征，符合条件
let rect = Rectangle::new(1, 1);

// 报错，因为动态数组并没有实现Copy特征
let rect1 = Rectangle::new(vec![1], vec![1]);
```

最后我们来看看我们最开始定义的`largest`函数，我们来使用泛型约束它，让它能够正确的进行比较，例如：

 ```rust
fn max<T>(arr: &[T]) -> T
where
    T: Copy + PartialOrd,
{
    let mut largest: T = arr[0];
    for item in arr {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}
 ```

我们给它加上了可以`Copy`可复制和`PartialOrd`可以比较的特征，我们在传入参数的时候就会对参数进行这两个特征的限制，就不能随意传值了。

### 10.3 生命周期

Rust中的生命周期主要是用来解决悬垂引用的问题。首先我们来看看Rust中的悬垂引用问题怎么产生的。

#### 10.3.1 悬垂引用

以下这个例子，我们在模块外声明了一个变量`r`，然后在模块给他赋值`w`的引用，但是模块里面的内容执行完就会销毁掉里面的内容，造成`r`变量指向一个**空指针**，形成了**悬垂引用**，如下：

```rust
 let r;                             //----------'a
{                                   //        |
    let w = String::from("stephen");//-----'b |
    r = &w;                         //  |     |
}                                   //---     | 
println!("{}", r);                  //--------+
// 值比变量的声明的范围小，造成悬垂引用
```

通过上面例子来看，当我们声明的变量比值的生命周期更长时，这个时候就会触发**悬垂引用**。那么Rust是怎么检测出来的呢？

#### 10.3.2 生命周期标注规则

Rust有一个专门处理引用数据的检查器，叫**借用检查器**。它负责Rust整个生命周期的检查。

接下来我们就来了解一下生命周期的标注规则，我们一般使用`' + 小写字母`来表示一个生命周期，例如：
`'a`, `'b`等等。

当我们需要配合着`&`使用，我们就需要在后面再加一个空格，例如：

```rust
&str // 引用
&'a str // 带有生命周期标注的引用
&'a mut str  // 带有生命周期标注的可变引用
```

#### 10.3.3 省略规则

在实践之前，我们再来了解一下，借用检查机器内置的一些省略规则：
输入生命周期 类似 函数传参
输出生命周期 类似 函数返回值

**第一条：**
每个传入的参数都一个生命周期，传入一个参数有一个输入生命周期，传入两个参数则有两个生命周期

**第二条：**
如果只有一个参数，那么输出生命周期和输入生命周期相同

**第三条：**
在**结构体**和**枚举**的方法中，当拥有多个输入生命周期参数，而其中一个是`&self`或`&mut self`时，self的生命周期会被赋予给所有的输出生命周期参数。

#### 10.3.4 函数添加生命周期标注

接下来我们尝试写一个比较两段字符串长度的函数，例如：

```rust
 fn longest(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
```

但是因为这里传入的`str1`和`str2`的引用，我们并不知道他们的生命周期、什么时候会销毁，所以这里返回值，很有可能是一个悬垂引用，所以会编译失败。
我们来给他添加生命周期，保证引用的值比函数后面销毁，例如：

```rust
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
```

这我们将`str1`和`str2`、返回值的生命周期都设置为`'a`，然后就通过借用检查器的编译了。

#### 10.3.5 结构体添加生命周期标注

我们在之前的结构体章节里面定义一个了一个`User`结构体，如下：

```rust
struct User {
    name: String,
}
```

当时我们没有使用`&str`，因为我们还没有设计到生命周期的概念，现在我们使用`&str`来代替这个`name`字段，例如：

```rust
struct User<'a> {
    name: &'a str,
}
```

#### 10.3.7 静态生命周期

我们还可以在Rust中定义一个特别的生命周期`'static`，它只作用于字符串字面量，并且它作用于小程序从周期开始到结束，例如：

```rust
let str1: &'static str = "stephen";
println!("{}", str1);
```

**注意**
我们要慎用这个静态类型，它不仅会增加存储的时间周期、增加内存的消耗，作用于全局还会绕过**借用检查器**的规则。

#### 10.3.8 泛型，特征，生命周期结合使用

最后，让我们来写一个泛型，特征，生命周期结合的函数结束第10节的学习。我们来改造改造之前的`longest`函数，如下：

```rust
fn longest<'a, T>(str1: &'a str, str2: &'a str, str3: T) -> &'a str
where
    T: Display,
{
    println!("extra={}", str3);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
let s1 = String::from("stephen");
let s2 = String::from("james");
let long = longest(&s1, &s2, 2);
println!("{}", long);
```

## 11 编写检索字符串的工具

学完了，1-10节内容，我们接下来将他们结合起来，使用Rust做一个检测字符串的命令工具。

### 11.1 创建一个新项目

首先我们来创建一个新项目，使用`cargo new minigrep`命令。我们进入`src/main.rs`主入口开发我们命令行工具的开发

### 11.2 获取用户输入的参数

我们在来看看怎么获取用户输入的参数，我们使用rust内置的模块`std::env`里面的`args`方法，例如：

```rust
use std::env
let args = env::args();
```

此时我们执行命令`cargo run test filename.txt`，控制台打印出：

```shell
Args { inner: ["target/debug/hello-world", "test", "filename.txt"] }
```

我本来想使用`args.inner`去取里面的数据信息，但是`inner`是个私有属性无法获取。我再去看了看文档，知道返回值是一个迭代器(后面会详细讲到)，所以我们可以使用`collect`将它转为一个数组类型，例如：

```rust
let inputs: Vec<String> = args.collect();
```

这个时候重新值行刚才的命令，能正确打印出我们需要的值了，如下：

```shell
["target/debug/hello-world", "test", "filename.txt"]
```

我们可以看到第一个参数是**项目路径**，后面两个参数才是我们输入的参数，接下里我们就使用这两个参数来进行匹配。

### 11.1 使用获取的参数读取文件

#### 11.1.1 读取文件

我们建立在最外层建立一个文件`filename.txt`，并且随意输入内容，如下：

```
test1
test2
```

然后我们使用内置的文件系统模块`std::fs`来读取该文件，例如：

```rust
let check_file = fs::read_to_string("./filename.txt");
println!("{:?}", check_file);
```

之前在枚举章节有讲到，文件读取是会返回`Result`枚举，里面包含了`Ok`和`Err`变体的，这里成功我们会拿到一个`Ok`变体包裹起来的数据，失败会直接`panic!`，我们再这里`expect`快速做一下错误处理。如下：

```rust
let check_file = fs::read_to_string("./filename.txt").expect("读取文件失败");
println!("{:?}", check_file);
```

通过上述的处理，我们就可以拿到文件的内容了。

#### 11.1.2 处理文件数据

但是我们要匹配我们输入的字符串，这还远远不够，接下里我们将获取的文件数据，将每一行存储到动态数组里面，我们可以使用字符串的`lines`方法，例如：

```rust
check_file.lines();
```

`lines`方法会将文件内容以`\n`,`\r`来断开，并且生成一个迭代器，当然这个时候我们需要使用`collect`方法将其转为一个动态数组的

#### 11.1.3 进行内容匹配

接下来我们来完整的把整个流程写一遍，并且完成内容的匹配。如下：

```rust
let args: Vec<String> = env::args().collect();

let match_text = &args[1];
let file_name = &args[2];

let match_file = fs::read_to_string("./".to_string() + file_name).expect("读取失败");

let match_file_text_arr: Vec<&str> = match_file.lines().collect();

for value in match_file_text_arr.iter() {
    if value.contains(match_text) {
        println!("匹配成功:{}包含{}", value, match_text);
    } else {
        println!("匹配失败:{}不包含{}", value, match_text);
    }
}
```

在代码的尾部，我们使用字符串的`contains`方法来判断当前行是否包含我要匹配的字符串，完成了最终的匹配功能。但是这个代码看起来杂乱不堪，并且全部放到主线程里面，不方便各种错误处理。我们接下来尝试去优化这部分代码。

### 11.1 使用环境变量

接下来我们尝试使用环境变量来判断匹配的字符串是否需要对大小写敏感。
我们首先将刚刚创建文件中的第一行`test`改为`TeSt`。
我们在执行命令的时候，在命令行前面传入我们的环境变量`SENSITIVE=1`，如下：

```shell
SENSITIVE=1 cargo run test filename.txt
```

然后，我们使用`env::vars(key)`来获取我们需要的环境变量，该方法返回的也是一个`Result`类型，所以我们需要做一个错误处理才能拿到值，如下：

```rust
let env_param = env::var("SENSITIVE").expect("获取失败");
println!("env_param={}", env_param); // 1
```

接下里我们再完整地写一遍，加上环境变量，并且当`SENSITIVE=1`时，我们需要区分大小写，当`SENSITIVE=0`时，不需要区分。例如：

```rust
let args: Vec<String> = env::args().collect();

let match_text = &args[1];
let file_name = &args[2];

let match_file = fs::read_to_string("./".to_string() + file_name).expect("读取失败");

let match_file_text_arr: Vec<&str> = match_file.lines().collect();

let env_param: String = env::var("SENSITIVE").expect("获取失败");

for value in match_file_text_arr.iter() {
    if env_param == "1" { // 我们使用环境变量来判断是否敏感匹配
        if value.contains(match_text) {
            println!("大小写敏感匹配成功:{}包含{}", value, match_text);
        } else {
            println!("大小写敏感匹配失败:{}不包含{}", value, match_text);
        }
    } else {
        let n_value = value.to_lowercase(); // 都转为转为小写
        let n_match_text = match_text.to_lowercase(); // 都转为转为小写
        if n_value.contains(&n_match_text) {
            println!("大小写不敏感匹配成功:{}包含{}", value, match_text);
        } else {
            println!("大小写不敏感匹配失败:{}不包含{}", value, match_text);
        }
    }
}
```

### 11.1 优化：整合变量

接下来我们继续优化一下有关系变量，我们使用结构体将他们整合一下，如下：

```rust
let args: Vec<String> = env::args().collect();

/**
 * 命令行获取参数配置
 */
struct Config<T> {
    /**
     * 匹配文字
     */
    match_text: T,
    /**
     * 文件名
     */
    file_name: T,
}

let config: Config<&str> = Config {
    match_text: &args[1],
    file_name: &args[2],
};

// let match_text: &String =&args[1] ;
// let file_name = &args[2];

let match_file = fs::read_to_string("./".to_string() + config.file_name).expect("读取失败");

let match_file_text_arr: Vec<&str> = match_file.lines().collect();

let env_param: String = env::var("SENSITIVE").expect("获取失败");

for value in match_file_text_arr.iter() {
    if env_param == "1" {
        if value.contains(config.match_text) {
            println!("大小写敏感匹配成功:{}包含{}", value, config.match_text);
        } else {
            println!("大小写敏感匹配失败:{}不包含{}", value, config.match_text);
        }
    } else {
        let n_value = value.to_lowercase(); // 都转为转为小写
        let n_match_text = config.match_text.to_lowercase(); // 都转为转为小写
        if n_value.contains(&n_match_text) {
            println!("大小写不敏感匹配成功:{}包含{}", value, config.match_text);
        } else {
            println!("大小写不敏感匹配失败:{}不包含{}", value, config.match_text);
        }
    }
}
```

因为我们将输入的参数是有关系的，所以我们将它存储到了一个结构体中，然后在其他函数中使用。

### 11.1 优化：封装函数、方法

现在我们来将函数也单独抽离一下，我们首先来将获取命令行参数封装成一个函数，因为我们之前已经将变量整合到了一个结构体中，所以我们可以直接在这个结构体上面去定义方法，例如：

```rust
impl Config {
    /**
     * 直接为结构体定义一个new方法，之后可以直接获得一个config实例
     */
    fn new(args: &[String]) -> Config {
        let match_text = args[1].clone();
        let file_name = args[2].clone();
        Config {
            match_text,
            file_name,
        }
    }
}

let config = Config::new(&args);
// 其他使用到config的地方都需改为&config，否则会报错
```

我们将其他的逻辑都放到`run`函数里面，如下:

```rust
fn run(config: &Config) {
    let match_file = fs::read_to_string("./".to_string() + &config.file_name).expect("读取失败");

    let match_file_text_arr: Vec<&str> = match_file.lines().collect();

    let env_param: String = env::var("SENSITIVE").expect("获取失败");

    for value in match_file_text_arr.iter() {
        if env_param == "1" {
            if value.contains(&config.match_text) {
                println!("大小写敏感匹配成功:{}包含{}", value, &config.match_text);
            } else {
                println!("大小写敏感匹配失败:{}不包含{}", value, &config.match_text);
            }
        } else {
            let n_value = value.to_lowercase(); // 都转为转为小写
            let n_match_text = &config.match_text.to_lowercase(); // 都转为转为小写
            if n_value.contains(n_match_text) {
                println!("大小写不敏感匹配成功:{}包含{}", value, &config.match_text);
            } else {
                println!("大小写不敏感匹配失败:{}不包含{}", value, &config.match_text);
            }
        }
    }
}
```

然后我们在主函数`main`中只处理简单的逻辑，如下:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);
    run(&config);
}
```

这样看起来简单多了。当然，当我们抽离了函数之后，我们需要对其做错误处理。接下来，我们来优化错误处理。

### 11.1 优化：优化错误处理

首先我们来优化一下结构体里面的`new`方法，我们使用`Result`枚举处理`new`方法，如下：

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        /**
         * 判断命令行参数不能小于2
         */
        if args.len() < 3 {
            return Err("输入的参数不能小于2位");
        }
        let match_text = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config {
            match_text,
            file_name,
        })
    }
}
```

然后我们在`main`函数中使用`process`处理，当出现参数解析错误的时候，我们直接退出整个程序，如下：

```rust
let config: Config = Config::new(&args).unwrap_or_else(|err| {
    println!("参数解析错误：{}", err);
    process::exit(1)
});
```

对获取参数的方法进行错误优化之后，对用户更加友好了。

紧接着我们来，对我们的`run`函数进行错误处理优化，如下：

```rust
fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let match_file = fs::read_to_string("./".to_string() + &config.file_name)?;

    let match_file_text_arr: Vec<&str> = match_file.lines().collect();

    let env_param: String = env::var("SENSITIVE")?;

    for value in match_file_text_arr.iter() {
        if env_param == "1" {
            if value.contains(&config.match_text) {
                println!("大小写敏感匹配成功:{}包含{}", value, &config.match_text);
            } else {
                println!("大小写敏感匹配失败:{}不包含{}", value, &config.match_text);
            }
        } else {
            let n_value = value.to_lowercase(); // 都转为转为小写
            let n_match_text = &config.match_text.to_lowercase(); // 都转为转为小写
            if n_value.contains(n_match_text) {
                println!("大小写不敏感匹配成功:{}包含{}", value, &config.match_text);
            } else {
                println!("大小写不敏感匹配失败:{}不包含{}", value, &config.match_text);
            }
        }
    }
    Ok(())
}
```

我们这里的处理很简单，就是使用于语法糖`?`将错误向上传递。`Box<dyn Error>`这个类型是我们第一次看到，它表示的意思是`Error`的`trait`类型。我们会在十七节讲到。

然后我们在`main`函数中和上面处理保持一致，如下：

```rust
run(&config).unwrap_or_else(|err| {
    println!("文件或环境参数解析出错：{}", err);
    process::exit(1);
});
```

这样我们就完成了整个优化处理。

## 12. 闭包与迭代器

### 12.1闭包

#### 12.1.1 闭包的构成

闭包可以作为参数、返回值或者将其存储在一个变量中。

我们先来举个例子来讲讲闭包的构成：

```rust
let y: i32 = 1;
let get_sum = |x| x + y;
let result = get_sum(1);
println!("{}", result)
```

这里我们声明了一个闭包`get_sum`，它和函数的声明完全一样，只是我这是用`||`来作为闭包传参的入口，并且省略了`{}`，当然我们可以加上：

```rust
let get_sum = |x| { let sum = x + y; sum };
```

从上面的例子中我们会发现，闭包可以去**读取**环境中的变量，这是它和函数的区别

#### 12.1.2 闭包的类型

从上面的例子来看，其实我们并没有在闭包内部给它定义类型，其实Rust内部给我们推导了类型，例如：

```rust
let get_self = |x| x;
get_self(1);
get_self('1'); // error
```

当我们第一次传入了一个`i32`的数字类型，这个时候其实已经被默认推导成了`i32`的类型，当我们再传入一个不同类型的值，会编译出错。

在我们声明一个闭包时，它会自动被推导为三种特征，怎么决定使用哪个特征，是看闭包内怎么去使用读区环境量的参数：

- 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次
- **不对捕获变量改变**的闭包自动实现了 `Fn` 特征
- **改变了捕获变量的值但是没有改变捕获变量所有权**的闭包自动实现了 `FnMut` 特征

我们先来举一个`Fn`的列子：
我们可以看到我们并没有对`y`值进行修改，所以这里会被推导为`Fn`类型；

```rust
let y = 1;
let get_self = |x| x + y;
```

我们再来看一个`FnMut`的例子：
这里我们在函数里面使用了引用来去修改环境参数的字符串，但是没有修改所有权，所以会被推导为`FnMut`。

```rust
let mut str = String::from("hello");
let mut combine_str = || {
    &str.push_str("world");
};
combine_str();
println!("{:?}", str);
```

我们最后来看一个强制改变了捕获环境值所有权的用例，它会自动推导为`FnOnce`，这里我们使用了`move`关键字，他可以**将捕获的环境参数所有权强制移动到闭包内部**，例如：

```rust
let str = String::from("hello");
let combine_str = move || str;
combine_str();
```

刚刚我们举的例子都是闭包作为一个变量进行存储，我们接下里写一个将闭包作为参数传递的例子：

```rust
let s = String::from("hello");

let update_str = || println!("{}", s);

fn executeFn<T>(f: T)
where
    T: Fn() -> (),
{
    f();
}

executeFn(update_str);

fn executeFnOnce<T>(f: T)
where
    T: FnOnce() -> (),
{
    f();
}
executeFnOnce(update_str);

fn executeFnMut<T>(mut f: T)
where
    T: FnMut() -> (),
{
    f();
}
executeFnMut(update_str)
```

### 12.2 迭代器

#### 12.1.1 创建迭代器

首先我们先来创建一个迭代器，如下：

```rust
let arr =  vec![1,2,3];
let iter_arr = arr.iter();
```

我们这里使用`iter`方法，实际是对`arr`数组的**不可变引用**，当我们先获取可变引用时，我们需要使用`iter_mut`方法，或者当我们想直接获取其所有权时，我们可以使用`into_iter`方法。

#### 12.1.2 Iterator 和 next

所以可迭代的类型都是实现了特征`Iterator`，并且实现`next`方法，让我们来看看`iterator`特征：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

其中`Item`是一个关联类型，后续会详细讲到。这里我们实现的特征，需要实现`next`，执行`next`后，会消耗迭代器。

#### 12.1.3 消耗迭代器

接下里我们来讲讲怎么去消耗迭代器，我们首先创建一个迭代器，如：

```rust
let iter_arr = vec![1, 2, 3].iter();
```

然后我们来执行`sum`方法，它会去执行`next`方法，迭代每个元素，并且将其加在一起，并且返回得到一个总和，如：

```rust
let sum: i32 = iter_arr.sum();
println!("{}", sum);
```

接下来我们来看看怎么用其他的方法来创建迭代器。

#### 12.1.4 使用其他方法创建迭代器

我们来使用`map`方法来将数组中每个值`+1`，如下：

```rust
let arr = vec![1, 2, 3];
let iter_arr = arr.iter().map(|x| x + 1);
println!("{:?}", iter_arr)
```

我们来看看打印值，如：

```rust
// Map { iter: Iter([1, 2, 3]) }
```

我们可以看到，我们的值并没有加1。这是因为我们的迭代器并且执行消耗，所以值不会变化，这里我们执行`collect`方法去消耗这个迭代器之后，会将其转化为一个动态数组，如：

```rust
let arr = vec![1, 2, 3];
let iter_arr: Vec<i32> = arr.iter().map(|x| x + 1).collect();
println!("{:?}", iter_arr) // [2,3,4]
```

#### 12.1.5 使用环境参数

我们还可以去使用闭包的特性去使环境参数。比如，我们使用可迭代类型的`filter`方法，它会根据根据我们传入的值进行筛选，如果为`true`则返回，`false`不返回。例如：

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}
let arr = vec![
    User {
        name: String::from("stephen"),
        age: 18,
    },
    User {
        name: String::from("kyrie"),
        age: 34,
    },
    User {
        name: String::from("kris"),
        age: 22,
    },
];
let less_age: i32 = 30;
// 筛选出年纪小于less_age的用户
let iter_arr: Vec<&User> = arr.iter().filter(|user| user.age < less_age).collect();
println!("{:#?}", iter_arr);
```

这里我们写了一个简单的用例，根据环境参数`less_age`去使用`filter`方法去筛选，年纪小于`less_age`的用户。

#### 12.1.6 自定义迭代器

最后，我们来自定义一个迭代器，我们首先写一个结构体`Counter`，如：

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count + 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

let mut counter = Counter::new();
println!("{:?}", counter.next()); // 1
println!("{:?}", counter.next()); // 2
println!("{:?}", counter.next()); // 3
println!("{:?}", counter.next()); // 4
println!("{:?}", counter.next()); // 5
println!("{:?}", counter.next()); // None
```

我们定义了一个结构体，并且为它声明了一个`new`方法，并且实现`Iterator特征`，然后我们去执行`next`方法，就可以可以拿到`1-5`的值。

### 12.3 使用迭代器、闭包改进I/O项目

接下来，我们来使用学习的新内容来修改之前做的一个检索功能。

#### 12.3.1 修改入参

之前我们在声明`Config`的`new`方法时，我们使用到了一个叫`clone`克隆的方法，这是为了拿到传入参数的所有权方便后续操作，但是这个方法是很消耗性能的，所以我们这里我们使用迭代器去改造一下。我们先看看之前的这个方法：

```rust
impl Config {
fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("输入的参数不能小于2位");
    }
    let match_text = args[1].clone(); // 消耗性能
    let file_name = args[2].clone();
    Ok(Config {
        match_text,
        file_name,
    })
}
}
```

我们首先将我们的入参修改，我们之前传入的是一个动态字符串，现在我们直接传入一个迭代器：

```rust
Config::new(env::args())
```

并且我们需要在`new`方法中修改：

```rust
fn new(mut args: env::Args)
```

因为迭代器是要消耗每个元素的，所以我们需要加上`mut`。
因为`env::arg()`获取到的第一个参数是项目，所以我们需要执行一下`arg.next()`。然后第二次执行`next`方法时，我们可以拿到我们需要匹配的值，第三次则可以拿到文件名。因为`next`拿到的是`Option`枚举，所以我们需要使用`match`去拿到最终的值，如：

```rust
impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("输入的参数不能小于2位");
        // }
        // let match_text = args[1].clone();
        // let file_name = args[2].clone();
        args.next();
        let match_text = match args.next() {
            Some(text) => text,
            None => return Err("未获取到匹配内容"),
        };
        let file_name = match args.next() {
            Some(name) => name,
            None => return Err("未获取到文件名"),
        };
        Ok(Config {
            match_text,
            file_name,
        })
    }
}
```

#### 12.3.2 修改匹配

还有一处，之前我们进行匹配的操作时，我们使用的`for`循环，现在我们直接使用`filter`方法，去过滤，直接省去`for`循环遍历这一步，例如：

```rust
fs::read_to_string().unwrap().lines().filter(|line| line.contains(match_text)).collect();
```

我们直接使用`lines`方法获取文件的迭代器，并且执行`filter`去完成过滤匹配，为什么在`rust`中，我们使用的更多的是迭代器还不是`for`循环呢？

1. 迭代器是一种更高层次的抽象概念，避免我们在开发时疲于去声明、维护各个变量的状态以及处理循环中的逻辑
2. 迭代器编译出来的是一种零开销底层代码，性能要优于`for`循环遍历

所以在Rust中，我们经常看到迭代器的使用，并且我们也要习惯于这种写法。

## 13 进一步认识Cargo和crate.io

### 13.1 修改Cargo.toml默认配置

在我们平时执行`cargo build`或者`cargo build --release`时(第一个命令开发时使用，第二个用于生产环境)，其实`Cargo.toml`里面已经替我们使用了默认配置，比如我们执行`cargo build`时，`Cargo.toml`里面默认执行：

```rust
// Cargo.toml
[profile.dev]
opt-level=0
```

`opt-level`的值范围时0-3，值越大表示`Rust`为你做的优化越多，也越费时间，当然我们开发是不需要关注打包优化的，只需要编译越快越好，所以它的默认值是0，但是当我们使用`--release`时，因为是要构建线上产物，所以它会替我们做最高优化所以`opt-level`默认为3。其他的默认配置可以[通过这里查询](https://doc.rust-lang.org/cargo/reference/config.html)。

### 13.2 用更好的方式注释

在我们之前，我们对于文档的注释使用`//`，但是这样的注释也仅仅是对于包细节的描述，接下我们讲两种其他的注释方式。他们都可以以`markdown`语法的方式解析。

### 13.2.1 //

`//~`表示的是对于整个包模块的注释。我们可以在`src`下面建立一个`lib.rs`文件，然后在文件顶部，写入：

```rust
//! # My Crate
//! 仅供学习和参考
```

然后执行`cargo doc --open`我们会发现生成了包名以及这个包的注释

### 13.2.2 ///

当我们想对于模块中的某个函数或者块进行备注时，我们可以使用`///`，它同样可以使用`markdown`语法进行解析，并且最厉害的一点是，里面写的用例会被用作单元测试用，当你执行`cargo test`时，所有`///`里面的用例都会执行。在`lib.rs`中，我们来举个例子：

```rust
/// 传入值加一
///
/// # 用例
///
///  ```
/// use hello_world::add_one;
/// let y = 2;
/// let result = add_one(1);
/// assert_eq!(result, y)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

`markdown`里面含有代码块的地方，在执行`cargo test`后会被用作测试用例执行，并且执行通过。

### 13.3 使用pub use导出公共api

在我们之前的学习中，我们可以使用`pub`关键字去导出函数、枚举、结构体、模块，但是当别人想想引用包深处的一个函数、枚举、结构体时，需要`use crate::xx::xxx::xxx::xx::test`，在我们的文档中，用户很难去找到自己所需要的，这个时候，我们可以使用`pub use`将关键的函数等暴露在`root`处，用户可以直接看到并且使用，如：

```rust
//! # My Crate
//! 仅供学习和参考
pub use self::kinds::Color;

pub mod kinds {
    pub enum Color {
        Red,
        Green,
        Blue,
    }
}
```

在我们使用`pub use`后，导出的文档会在文档顶部显示。并且，我们在代码中使用时，可以直接`use crate::Color`就能使用了。但是这样导出也会有个问题就是，可能会出现同名的类型，这个时候就需要开发者自己控制这种问题的出现。

### 13.4 将包发布在crates.io

这一小节我们主要讲讲怎么讲我们写好的包发到crates.io上面分享给其他的开发者使用。

#### 13.4.1 在crates.io上面登录获取API Token

首先，我们需要在[crates.io](https://crates.io/)登录账号，目前它只支持github账号登录，登陆成功后，在头像处，我们进入，**Account Setting**，然后生成一个API TOKEN，接下来我们先创建一个包。

#### 13.4.2 配置一个发布包

我们先生成一个新的库包，使用命令`cargo new learn-rust-publish`，然后我们在github上面建立一个[仓库](https://github.com/1847016090/learn-rust-publish)，然后在我们的项目中执行`git init -y`，然后使用`git remote add origin 你的仓库地址`将项目和远程地址管理起来，将代码初始化到仓库中，后续在发布前也需要将代码更新到仓库中，再执行发布流程。

##### 13.4.2.1 为包添加元数据

打开项目，进入到`Cargo.toml`中，在`[package]`中配置元数据信息，如下：

```rust
[package]
# 创建项目默认添加
name = "learn-rust-publish"
version = "0.1.0"
edition = "2021"

# 需要添加
email = "your email"
description = "learn how to publish rust library"
license = "MIT OR Apache-2.0"
repository = "https://github.com/1847016090/learn-rust-publish"
```

其中前三项是创建项目时默认生成的，我们只需要添加后面几项即可。

#### 13.4.2.2 登录 & 发布

然后我们先来执行`cargo login`命令执行登录，然后将我们刚才生成的API TOKEN填入即可登录成功。我们接着再执行`cargo publish`，发现如下报错:

```rust
the remote server responded with an error: A verified email address is required to publish crates to crates.io. Visit https://crates.io/settings/profile to set and verify your email address.
```

我们需要跟随着链接去验证一下我们的邮箱即可。验证完后，再执行`cargo publish`就能成功发布我们的包了([我发布的包](https://crates.io/crates/learn-rust-publish))。

#### 13.4.2.3 撤销或重新发布新版本包

当我们发布了一个版本的包之后，我们就不能再次覆盖这个包了。但是我们使用`cargo yank --vers 0.1.0`撤销当前版本的包(只针对新包。如果有项目已经安装了当前版本的包，Cargo.lock文件已经存在当前版本，他依旧可以继续使用当前版本的包)。

当然，如果我们想取消撤销，我们执行`cargo yank --vers 0.1.0 --undo`就行。

如果我们想发布新的版本，我们只需要修改我们`Cargo.toml`文件中的`version`字段即可

### 13.5 使用工作空间管理多个包

如果我们有多个包需要管理，那我们应该怎么去管理呢？这就涉及到工作空间了。

#### 13.5.1 创建一个新的工作空间目录

我们首先心建立一个空文件夹`mkdir rust-workspace-library && cd mkdir rust-workspace-library`，然后我们按照刚才的步骤将它和远程仓库关联[地址](https://github.com/1847016090/rust-workspace-library)

#### 13.5.2 创建工作空间并且创建一个二进制包和一个库包

我们在空的文件夹中先创建一个`Cargo.toml`文件，并且在里面填写:

```rust
[workspace]
members=['adder', 'add_one']
```

这时候已经建立了工作空间了。然后我们使用`cargo new adder`和`cargo new add_one --lib`命令分别创建二进制包和库包。
这个时候我们的基本目录已经创建完成。

#### 13.5.3 改造代码

然后我们进入`add_one`中`lib.rs`文件将写一个简单函数以及他的测试函数：

```rust
pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

然后我们进入`adder`中`Cargo.toml`文件中将`add_one`引入进来：

```rust
[dependencies]
add_one = { path = '../add_one' }
```

然后再进入`main.rs`使用`add_one`里面的函数：

```rust
use add_one::add_one;

fn main() {
    let num = add_one(10);
    println!("result = {}", num);
}
```

然后使用`cargo run -p adder`执行，成功打印！这里`-p`的意思是，指定某个目录去单独执行。我们也可以但是测试某个包，比如:`cargo test -p add_one`，我们会发现我们刚刚添加的测试用例也跑过了。

**注意：**
使用工作空间建立包，某个包都需要单独发布，不能一起发布。

## 14 智能指针

### 14.1 什么是指针？什么是智能指针？

指针：

- 指针（pointer）是一个通用概念，它指代那些包含内存地址的变量
常用的就是：引用
智能指针：
它和指针的的区别在于
- 指的是一些数据结构，它们的行为类似于指针但拥有额外的元数据和附加功能
- 引用是只借用数据的指针；与之相反，大多数智能指针本身就拥有它们指向的数据

对于智能指针，我们通常会为它实现，`Deref Trait` 和 `Drop Trait` 。

- `Deref Trait`表示能够使你同时拥有引用和智能指针的代码
- `Drop Trait` 离开函数作用域时同时销毁指向指向堆上数据的指针以及存储在堆上的数据

接下来我们讲讲几个常见的智能指针：

- `Box<T>`，可用于在堆上分配值
- `Rc<T>`，允许多重所有权的引用计数类型
- `Ref<T>`和`RefMut<T>`，可以通过`RefCell<T>`访问，是一种可以在运行时而不是编译时执行借用规则的类型

### 14.2 使用 `Box<T>` 在堆上分配数据

装箱是一种简单直接的智能指针类型，它的类型被写作`Box<T>`。它使我们可以将数据存储在堆上，并在栈中保留一个指向堆数据的指针。装箱常常被用于以下的场景：

- 当你拥有一个无法在编译时确定大小的类型，但又想要在一个要求固定尺寸的上下文环境中使用这个类型的值时。
- 当你需要传递大量数据的所有权，但又不希望产生大量数据的复制行为时。
- 当你希望拥有一个实现了指定trait的类型值，但又不关心具体的类型时。

第一个场景我们接下来会讲到。
在第二种场景中，转移大量数据的所有权可能会花费较多的时间，因为这些数据需要在栈上进行逐一复制。为了提高性能，你可以借助装箱将这些数据存储到堆上。通过这种方式，我们只需要在转移所有权时复制指针本身即可，而不必复制它指向的全部堆数据。

#### 14.2.1 使用`Box<T>`存储数据

我们先来看看`Box`装箱的语法(代码无法通过编译)，如：

```rust
let box = Box::new(5);
println!("{}", box);
```

我们在堆上面存储了`5`这个值，然后将`box`作为一个指针指向它。实际使用中，我们存储在堆上的数据不会像`5`这么简单。接下来我们看一个比较复杂的例子。

#### 14.2.1 使用装箱存储递归类型

递归类型在编译时是无法确认编译大小的，因为他们本身存储着另外一个类型的值，然后嵌套的深度也不得而知，所以Rust无法去计算出其大小。这个时候我们就可以使用装箱来创建递归类型来规避这个问题，因为装箱有固定的大小。

接下来我们使用**链接列表**来时创建递归类型。链接列表的每一项都包含了两个元素：当前项的值及下一项。列表中的最后一项是一个被称作Nil且不包含下一项的特殊值。

我们先来尝试使用枚举来定义一个链接列表。

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

然后使用这个`List`类型来存储：

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

这样是无法通过编译的，会提示这个类型拥有无限大小。因为Rust无法计算出存储一个`List`需要花费多少的内存。

我们用装箱来重构它：

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(
    1,
    Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
);

println!("{:#?}", list);
```

新的变体`Cons`只需要一部分存储`i32`类型和一部分存储装箱指针的数据空间。因此是可以Rust计算出存储大小，然后正确编译。

### 14.3 通过将Deref将智能指针视为常规引用

实现`Deref`特征可以为我们自动实现**解引用运算符**的行为。然后我们可以将智能指针视作普通的引用来处理。

#### 14.3.1 使用解引用运算符跳转到指针指向的值

我们首先来看看使用引用和解引用的例子：

```rust
let y = 5;
let x = &y;
assert_eq!(5, x); // error 
assert_eq!(5, *x); // right
```

其中第一个比较，会发生编译报错，提示我们不能将`integer`和`&{integer}`进行比较。

接下来我们在来试试将`Box`装箱当做常规的引用，如下：

```rust
let y = Box::new(5);
assert_eq!(5, *y);
```

上述的代码依然能够通过编译，说明我们的解引用操作符也能够跟踪智能指针并且获取它指向的值。

接下来我们尝试着自定义一个我们自己的智能指针。
刚刚我们有提到，需要实现`Deref`才能够实现外部的解引用操作。
我们来定义一个`MyBox`结构体：

```rust
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(d: T) -> MyBox<T> {
        MyBox(d)
    }
}
```

`MyBox`和`Box`有着相同的方法，然后我们来为它实现`Deref`的行为：

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

let x = MyBox::new(5);
assert_eq!(5, *x);
```

我们在deref的方法体中填入了`&self.0`(因为MyBox是一个元祖结构体所以`.0`就能获取第一项的值)，这意味着deref会返回一个指向值的引用，进而允许调用者通过`*运算符`，(我们在上述的`*x`会被隐式转化为`*(x.deref())`)

**解引用转换**是Rust为函数和方法的参数提供的一种便捷特性。Rust通过实现解引用转换功能，使程序员在调用函数或方法时无须多次显式地使用&和*运算符来进行引用和解引用操作。例如：

```rust
let x = MyBox::new(String::from("world"));

fn hello(name: &str) {
    println!("{}", name);
}

hello(&x);
```

上面的自动转化原理是：Rust先调用x的`deref`方法将其转化为`&String::from('world')`，然后`String`内置的`deref`会将其转化为字符串切片`&str`，然后就能得到我们的`world`值。
如果没有自动转化，那我们就需要写下面复杂的代码去获取：

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

#### 14.3.2 解引用转换与可变性

Rust会在类型与trait满足下面3种情形时执行解引用转换：

- 当T: Deref<Target=U>时，允许&T转换为&U。
- 当T: DerefMut<Target=U>时，允许&mut T转换为&mut U。
- 当T: Deref<Target=U>时，允许&mut T转换为&U。

情形三则有些微妙：Rust会将一个可变引用自动地转换为一个不可变引用。但这个过程绝对不会逆转，也就是说不可变引用永远不可能转换为可变引用。因为按照借用规则，如果存在一个可变引用，那么它就必须是唯一的引用（否则程序将无法通过编译）。将一个可变引用转换为不可变引用肯定不会破坏借用规则，但将一个不可变引用转换为可变引用则要求这个引用必须是唯一的，而借用规则无法保证这一点。

### 14.4 使用Drop trait 在清理时运行代码

Drop Trait 允许我们在变量离开作用域时执行自定义的操作。它常常被用来释放诸如文件、网络连接等资源。我们来实现一个拥有Drop行为的结构体，Drop trait要求实现一个接收self可变引用作为参数的drop函数， 如：

```rust
struct MyCustomPointer {
    data: String,
}

impl Drop for MyCustomPointer {
    fn drop(&mut self) {
        println!("自定义操作")
    }
}

let m = MyCustomPointer {
    data: String::from("hello"),
};
println!("结束")
````

我们会发现执行的顺序是：

```rust
// 1.结束
// 2.自定义操作
```

因为是离开作用域时执行，所以`drop`里面的打印会晚一些。

但是我们可以通过单独使用`std::mem::drop`提前丢弃值，如下：

```rust
use std::ops::Deref;
struct MyCustomPointer {
    data: String,
}

impl Drop for MyCustomPointer {
    fn drop(&mut self) {
        println!("自定义操作")
    }
}

let m = MyCustomPointer {
    data: String::from("hello"),
};

drop(m);

println!("结束")
```

这个时候，我们自定义的操作就能提前执行了，因为它被提前释放了。

### 14.5 `RefCell<T>`和内部可变性模式

首先我们来了解一下什么是**内部可变性模式**
内部可变性是Rust的设计模式之一，它**允许你在只持有不可变引用的前提下对数据进行修改**

而`RefCell<T>`是内部可变性模式的实践。它代表了其持有数据的唯一所有权。

想想我们之前学习的借用规则：

- 在任何给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用
- 引用总是有效的

对于使用`一般引用和Box<T>`的代码，Rust会在编译阶段强制代码遵守这些借用规则。
而对于使用`RefCell<T>`的代码，Rust则只会在运行时检查这些规则，并在出现违反借用规则的情况下触发panic来提前中止程序。

下面是对于`Rc<T>`,`Box<T>`以及`RefCell<T>`使用场景：

- `Rc<T>`允许一份数据有多个所有者，而`Box<T>`和`RefCell<T>`都只有一个所有者。
- `Box<T>`允许在编译时检查的可变或不可变借用，`Rc<T>`仅允许编译时检查的不可变借用，`RefCell<T>`允许运行时检查的可变或不可变借用。
- 由于`RefCell<T>`允许我们在运行时检查可变借用，所以即便`RefCell<T>`本身是不可变的，我们仍然能够更改其中存储的值。

#### 14.5.1 内部可变性：可变地借用一个不可变的值

借用规则的一个推论是，你无法可变地借用一个不可变的值。
我们来写个例子看看：

```rust
let x = vec![1, 2, 3];
x.push(1); // error cannot borrow `x` as mutable, as it is not declared as mutable
x.push(1);
```

上面这段代码明显就违背了我们的借用规则，不能可变借用一个不可变的值。但是在某些特定情况下，我们也会需要一个值在对外保持不可变性的同时能够在方法内部修改自身。我们来写一个使用`RefCell<T>`修改不可变值的例子：

```rust
use std::{cell::RefCell, mem::drop};

let x = RefCell::new(vec![1, 2, 3]);
x.borrow_mut().push(1);
println!("{:?}", x)
```

我们使用`RefCell<T>`包裹一层我们初始化的值，然后使用`borrow_mut`方法去修改原数据的值。这种场景项目中不多，但是遇到的话，我们可以保证安全性的情况下考虑使用它来解决。

#### 14.5.2 将`Rc<T>`和`RefCell<T>`结合使用来实现一个拥有多重所有权的可变数据

将`RefCell<T>`和`Rc<T>`结合使用是一种很常见的用法。
`Rc<T>`允许多个所有者持有同一数据，但只能提供针对数据的不可变访问。如果我们在`Rc<T>`内存储了`RefCell<T>`，那么就可以定义出拥有多个所有者且能够进行修改的值了

例如：

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

打印之后，我们会发现，这三个值都发生了变化：

```rust
// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

## 15 无谓并发

讲无畏并发之前，我们首先来看看并发编程和并行编程的区别：
前者允许程序中的不同部分相互独立地运行，而后者则允许程序中的不同部分同时执行
但是从以往的经验来看，这类场景的编程往往是容易出错的。

Rust为我们提供了所有权和类型系统，相比于在运行时遭遇并发缺陷后花费大量时间来重现特定的问题场景，Rust编译器会直接拒绝不正确的代码并给出解释问题的错误提示信息。我们将这一特性称为**无畏并发**

本章讨论的内容讨论:

- 如何创建线程来同时运行多段代码。
- 使用通道在线程间发送消息的消息传递式并发。
- 允许多个线程访问同一片数据的共享状态式并发。
- Sync trait与Send trait，能够将Rust的并发保证从标准库中提供的类型扩展至用户自定义类型。

### 15.1 使用线程同时运行代码

首先我们来看看线程和进程的区别：
在大部分现代操作系统中，执行程序的代码会运行在**进程（process）**中，操作系统会同时管理多个进程。类似地，程序内部也可以拥有多个同时运行的独立部分，用来运行这些独立部分的就叫作**线程（thread）**。

通过使用线程，我们可以将计算机操作拆分成多个线程同时运行提高性能。但是这样会导致一些问题，例如：

- 当多个线程以不一致的顺序访问数据或资源时产生的**竞争状态**（race condition）。
- 当两个线程同时尝试获取对方持有的资源时产生的**死锁（deadlock）**，它会导致这两个线程无法继续运行。
- 只会出现在特定情形下且难以稳定重现和修复的bug。

#### 15.1.1 使用Spawn创建线程

我们可以使用`thread::spawn`来创建线程，它接受一个闭包，如：

```rust
use std::thread;
use std::time::Duration;

thread::spawn(|| {
    for i in 1..10 {
        println!("【新】线程中的数据：{}", i);
        thread::sleep(Duration::from_millis(1))
    }
});

for i in 1..5 {
    println!("【主】线程中的数据：{}", i);
    thread::sleep(Duration::from_millis(1))
}

// 【主】线程中的数据：1
// 【新】线程中的数据：1
// 【主】线程中的数据：2
// 【新】线程中的数据：2
// 【主】线程中的数据：3
// 【新】线程中的数据：3
// 【主】线程中的数据：4
// 【新】线程中的数据：4
// 【新】线程中的数据：5
```

执行完之后，我们会发现新线程中循环并没有执行完，这是因为执行到`【新】线程中的数据：5`时这个时候，主线程就结束了，所以新线程也不会继续执行。那么我们应该怎么解决这个问题呢？

#### 15.1.2 使用join句柄等待所有线程结束

我们可以使用`thread::spawn`返回的句柄，它是一个自持有所有权的JoinHandle，调用它的join方法可以阻塞当前线程直到对应的新线程运行结束。如下：

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("【新】线程中的数据：{}", i);
        thread::sleep(Duration::from_millis(1))
    }
});

for i in 1..5 {
    println!("【主】线程中的数据：{}", i);
    thread::sleep(Duration::from_millis(1))
}

handle.join().unwrap();

// 【主】线程中的数据：1
// 【新】线程中的数据：1
// 【主】线程中的数据：2
// 【新】线程中的数据：2
// 【新】线程中的数据：3
// 【主】线程中的数据：3
// 【主】线程中的数据：4
// 【新】线程中的数据：4
// 【新】线程中的数据：5
// 【新】线程中的数据：6
// 【新】线程中的数据：7
// 【新】线程中的数据：8
// 【新】线程中的数据：9
```

我们可以看这里顺序，显示主、新线程的循环同时执行，主线程内容打印完之后，新线程继续执行直到打印结束。仔细想想，这样之后，是不是提高了性能？那如果将**句柄**执行的位置修改一下呢？如下：

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("【新】线程中的数据：{}", i);
        thread::sleep(Duration::from_millis(1))
    }
});
handle.join().unwrap();
for i in 1..5 {
    println!("【主】线程中的数据：{}", i);
    thread::sleep(Duration::from_millis(1))
}
// 【新】线程中的数据：1
// 【新】线程中的数据：2
// 【新】线程中的数据：3
// 【新】线程中的数据：4
// 【新】线程中的数据：5
// 【新】线程中的数据：6
// 【新】线程中的数据：7
// 【新】线程中的数据：8
// 【新】线程中的数据：9

// 【主】线程中的数据：1
// 【主】线程中的数据：2
// 【主】线程中的数据：3
// 【主】线程中的数据：4
```

我们会发现，我们的句柄阻塞了主线程的执行，所以我们在使用线程时一定得想清楚了。

#### 15.1.3 在线程中使用 move 闭包

move闭包常常被用来与`thread::spawn`函数配合使用，它允许你在某个线程中使用来自另一个线程的数据。
我们先来写一个获取环境参数的例子:

```rust
use std::thread;
use std::time::Duration;

let v = vec![1, 2, 3];

let handle = thread::spawn(|| {
    println!("{:?}", v);
});

handle.join().unwrap();
```

上面的例子编译会失败。由于Rust不知道新线程会运行多久，所以它无法确定v的引用是否一直有效。
如果我们在途中使用`drop(v)`，将数据`v`给清除掉，那么程序就会报错。

这个时候，我们可以通通过在闭包前添加`move`关键字，我们会强制闭包获得它所需值的所有权，而不仅仅是基于Rust的推导来获得值的借用。

```rust
use std::thread;
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("{:?}", v);
});

handle.join().unwrap();
```

这个时候能成功编译，并且`v`的所有权已经移交到闭包内。

### 15.2 使用消息传递在线程间转移数据

Rust在标准库中实现了一个名为通道（channel）的编程概念，它可以被用来实现基于消息传递的并发机制。

通道由**发送者**和**接受者**组成。某一处代码可以通过调用发送者的方法来传送数据，而另一处代码则可以通过检查接收者来获取数据。当你丢弃了发送者或接收者的任何一端时，我们就称相应的通道被关闭（closed）了

接下来我们编写两个线程，一个用于发送消息，另外一个用于接受消息。

```rust
use std::{sync::mpsc, thread};

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hello world");
    tx.send(val).unwrap();
});

let receive_val = rx.recv().unwrap();
println!("{}", receive_val);
```

上面代码使用`mpsc::channel()`创建消息通道，其中`mpsc`表示`multiple producer，single consumer`。它会返回一个元祖类型，第一项表示**消息发送者**，第二项表示消息接受者。

通道的接收端有两个可用于获取消息的方法：`recv`和`try_recv`。我们使用的recv（也就是receive的缩写）会阻塞主线程的执行直到有值被传入通道。`try_recv`方法不会阻塞线程，它会立即返回`Result<T,E>`，当线程需要一边接受消息一边完成其他工作时我们可以使用`try_recv`。我们可以编写出一个不断调用try_recv方法的循环，并在有消息到来时对其进行处理，而在没有消息时执行。

### 15.2.1 通道和所有权转

所有权规则在消息传递的过程中扮演了至关重要的角色，因为它可以帮助你写出安全的并发代码。接着上面的例子，我们在线程里面去打印`val`。

```rust
use std::{sync::mpsc, thread};

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hello world");
    tx.send(val).unwrap();
    println!("{}", val); // 报错，这里会发生所有权的转移
});

let receive_val = rx.recv().unwrap();
println!("{}", receive_val);
```

这里`send`函数会获取`val`的所有权，并且在参数传递时将它转移给接受者。
所有权帮我们规避了一个大问题：
一旦这个值被发送到了另外一个线程中，那个线程就可以在我们尝试重新使用这个值之前修改或丢弃它。这些修改极有可能造成不一致或产生原本不存在的数据，最终导致错误或出乎意料的结果。

### 15.2.2 发送多个值并观察接收者的等待过程

我们再来写一个发送多个值的用例：

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let arr = vec![String::from("hello"), String::from("world")];
    for val in arr {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for receive_val in rx {
    println!("接受到：{}", receive_val);
}
```

在主线程中，我们会将rx视作迭代器，遍历拿到接受的值。我们并没有在主线程的for循环中执行暂停或延迟指令，这也就表明主线程确实是在等待接收新线程中传递过来的值。

### 15.2.3 通过克隆发送者创建多个生产者

上面的例子都是一个生产者发送消息，接下来我们试着创建多个生产者来发送消息。

```rust
use std::time::Duration;
use std::{sync::mpsc, thread};
let (tx, rx) = mpsc::channel();

// 第二个生产者
let tx1 = mpsc::Sender::clone(&tx);

// 线程1
thread::spawn(move || {
    let arr = vec![String::from("hi"), String::from("ni hao")];
    for val in arr {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

// 线程2
thread::spawn(move || {
    let arr = vec![String::from("hello"), String::from("world")];
    for val in arr {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for receive_val in rx {
    println!("接受到：{}", receive_val);
}

// 打印
// 接受到：hello
// 接受到：hi
// 接受到：world
// 接受到：ni hao
```

如果你在实验时为不同的线程调用了含有不同参数的thread::sleep函数，那么输出结果的差异有可能更为显著且难以确定。

## 15.3 共享状态的并发

消息传递确实是一种不错的并发通信机制，但它并不是唯一的解决方案。接下来，我们会先来讨论共享内存领域中一个较为常见的并发原语：**互斥体（mutex）**

### 15.3.1 互斥体一次只允许一个线程访问数据

访问互斥体中的数据，线程必须首先发出信号来获取互斥体的锁（lock）。锁是互斥体的一部分，这种数据结构被用来记录当前谁拥有数据的唯一访问权。通过锁机制，互斥体守护（guarding）了它所持有的数据。
互斥体是出了名的难用，因为你必须牢记下面**两条规则**：

- 必须在使用数据前尝试获取锁。
- 必须在使用完互斥体守护的数据后释放锁，这样其他线程才能继续完成获取锁的操
  
接下来我们来演示一个单线程环境里面使用互斥体：

```rust
let m = Mutex::new(1);
{
    let mut num = m.lock().unwrap();
    *num += 1;
}

println!("{:?}", m); // Mutex { data: 6 }
```

当我们获取到锁，我们可以将`num`视为一个指向内部数据的可变引用，从而去修改他的值。

#### 多个线程间共享`Mutex<T>`

现在，让我们试着在多线程环境中使用`Mutex<T>`:

```rust
use std::{sync::Mutex, thread};

let counter = Mutex::new(0);
let mut handles = vec![];
for _ in 0..10 {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle)
}

for h in handles {
    h.join().unwrap();
}

println!("{:?}", *counter.lock().unwrap())
```

在我们执行后，我们会发现counter被移动到了handle指代的线程后，阻止了我们在第二个线程中调用lock来再次捕获counter。我们不应该将counter的所有权移动到到多个线程中。这个时候我们是不是可以使用用`Rc`来创建多重所有权去解决呢？

#### 多线程与多重所有权

我们来试试用`Rc`来解决这个问题：

```rust
let counter = Rc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..10 {
    let handle = thread::spawn(move || {
        let clone_counter = Rc::clone(&counter);
        let mut num = clone_counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle)
}

for h in handles {
    h.join().unwrap();
}

println!("{:?}", *counter.lock().unwrap())
```

我们运行后发现报错：

```rust
`Rc<Mutex<i32>>` cannot be sent between threads safely
within `{closure@src/main.rs:117:36: 117:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
```

这段话告诉我`Mutex<i32>`类型无法安全地在线程间传递，该类型不满足trait约束Send。好家伙，又引入新的`trait`。我们会在后面章节详细讲到该特征。它确保了我们在线程中使用的类型能够在并发环境下正常工作。但是不幸的是，`Rc<T>`并未实现该特征。

`Rc<T>`在跨线程使用时并不安全。当`Rc<T>`管理引用计数时，它会在每次调用clone的过程中增加引用计数，并在克隆出的实例被丢弃时减少引用计数，但它并没有使用任何并发原语来保证修改计数的过程不会被另一个线程所打断

#### 原子引用计数`Arc<T>`

`Arc<T>`的类型，它既拥有类似于`Rc<T>`的行为，又保证了自己可以被安全地用于并发场景。原子是一种新的并发原语，我们可以参考标准库文档中的`std::sync::atomic`部分来获得更多相关信息。你现在只需要知道：原子和原生类型的用法十分相似，并且可以安全地在多个线程间共享。

那么标准库的类型为什么不默认使用`Arc<T>`来实现呢？
这是因为我们需要付出一定的性能开销才能够实现线程安全，而我们只应该在必要时为这种开销买单。

```rust
use std::sync::Arc;
use std::{sync::Mutex, thread};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..10 {
    let handle = thread::spawn(move || {
        let clone_counter = Arc::clone(&counter);
        let mut num = clone_counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle)
}

for h in handles {
    h.join().unwrap();
}

println!("{}", *counter.lock().unwrap()) // 10
```

你可以使用本节中的程序结构去完成比计数更为复杂的工作。基于这个策略，你可以将计算分割为多个独立的部分，并将它们分配至不同的线程中，然后使用`Mutex<T>`来允许不同的线程更新计算结果中与自己有关的那一部分

### 15.3.2 `RefCell<T>/Rc<T>`和`Mutex<T>/Arc<T>`之间的相似性

`Mutex<T>`与Cell系列类型有着相似的功能，它同样提供了内部可变性。我们在第15章使用了`RefCell<T>`来改变`Rc<T>`中的内容，而本节按照同样的方式使用`Mutex<T>`来改变`Arc<T>`中的内容。

另外还有一个值得注意的细节是，Rust并不能使你完全避免使用`Mutex<T>`过程中所有的逻辑错误。回顾第15章中讨论的内容，使用`Rc<T>`会有产生循环引用的风险。两个`Rc<T>`值在互相指向对方时会造成内存泄漏。与之类似，使用`Mutex<T>`也会有产生死锁（deadlock）的风险。当某个操作需要同时锁住两个资源，而两个线程分别持有其中一个锁并相互请求另外一个锁时，这两个线程就会陷入无穷尽的等待过程。

如果你对死锁感兴趣，不妨试着编写一个可能导致死锁的Rust程序。然后，你还可以借鉴其他语言中规避互斥体死锁的策略，并在Rust中实现它们。标准库API文档的`Mutex<T>`和`MutexGuard`页面为此提供了许多有用的信息。

## 15.4 使用Sync trait和Send trait对并发进行扩展

### 15.4.1 允许线程间转移所有权的Send trait

只有实现了Send trait的类型才可以安全地在线程间转移所有权。除了Rc<T>等极少数的类型，几乎所有的Rust类型都实现了Send trait：如果你将克隆后的Rc<T>值的所有权转移到了另外一个线程中，那么两个线程就有可能同时更新引用计数值并进而导致计数错误。因此，Rc<T>只被设计在单线程场景中使用，它

### 15.4.2 允许多线程同时访问的Sync trait

只有实现了Sync trait的类型才可以安全地被多个线程引用。智能指针`Rc<T>`同样不满足Sync约束，其原因与它不满足Send约束类似。而正如“在多个线程间共享`Mutex<T>`”一节中讨论的那样，智能指针`Mutex<T>`是Sync的，可以被多个线程共享访问。

### 15.4.3 手动实现Send和Sync是不安全的

手动实现这些trait涉及使用特殊的不安全Rust代码。我们将在第19章讨论这一概念，目前你需要注意的是，当你构建的自定义并发类型包含了没有实现Send或Sync的类型时，你必须要非常谨慎地确保设计能够满足线程间的安全性要求。Rust官方网站中的The Rustonomicon文档详细地讨论了此类安全性保证及如何满足安全性要求的具体技术。
