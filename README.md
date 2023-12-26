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
