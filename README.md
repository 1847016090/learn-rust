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
