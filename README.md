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
let arr: Vec<i8> = Vec::new();
arr.push(1)
arr.push(2)
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

接下来我们讲讲，如何创建一个动态字符串，它和动态数组有点类型，我们可以直接使用`String`上面的`new`方法，例如：

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

但是这里有个问题，因为rust是使用UTF-8作为编码格式，每一个字符占一个字节，但是如果Unicode标量的话，需要占两个字节，如果我们使用字符串切片不当就会报错，例如：

```rust
let str_value = String::from("дравствуйте");
let str_slice = &str_value[0..1]; // 因为д占两个字节，这里我只取一个字节，就会发生panic，所以对字符串使用切片需要小心谨慎
println!("{}", str_slice)
```

我们还可以批量遍历每个标量值，例如：

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

在rust中，我们创建`HashMap`同样使用其中的`new`方法，因为该集合类型用的比较少，所以没有被rust预引入到作用域中，我们需要使用`use std::collections::HashMap;`手动引入，例如：

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
hm.get(&String::from("stephen")) // 20，这里我们需要使用值的引用
```

遍历哈希映射也是使用`for`， 例如：

```rust
for (key, value) in &hm {
    println!("{}-{}", key, value);
}
// james-30
// stephen-20
```

那我们应该怎么去覆盖之前的值呢？首先我们得先判断哈希映射里面有没有当前这个字段，这时候我们需要使用`entry`来判断，然后如果不存在的话，可以使用`or_insert`来插入值，有得话就会直接更新，例如：

```rust
use std::collections::HashMap;
let mut hm = HashMap::new();
hm.insert(String::from("stephen"), 20);
hm.insert(String::from("james"), 30);
hm.entry(String::from("kyrie")).or_insert(20);
println!("{:?}", hm)
```

## 9 错误处理

## 10 特征和声明周期
