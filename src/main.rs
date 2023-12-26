// mod enter_01;
// use enter_01::print_hello_world;

// mod guest_game_02;
// use guest_game_02::{classes, homework};
// use std::{fs, io::ErrorKind};

// mod public_module {
//     pub fn test() {
//         println!("{}", 1)
//     }
// }
// fn test(){
//     self::
// }
// mod restaurant {
//     pub mod free {
//         pub fn add_rice() {}
//         mod soup {
//             fn add_soup() {
//                 crate::restaurant::money::add_vegetable()
//             }
//         }
//     }
//     mod money {
//         pub fn add_vegetable() {}
//         fn add_fruits() {}
//     }
// }

// use std::collections::HashMap;

// use std::fmt::Display;

use std::fmt::Display;

// use std::io;
// use std::io::{ErrorKind, Read};
// use std::{fs, io};
fn main() {
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
    // let str1: &'static str = "stephen";
    // println!("{}", str1);
    // #[derive(Debug)]
    // struct User<'a> {
    //     name: &'a str,
    // }

    // let user: User = User { name: "2222" };
    // println!("{:#?}", user);
    // let r;
    // {
    //     let w = String::from("value");
    //     r = &w;
    // }
    // println!("{}", r);

    // fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    //     if str1.len() > str2.len() {
    //         str1
    //     } else {
    //         str2
    //     }
    // }
    // fn longest(str1: &str, str2: &str) -> &str {
    //     if str1.len() > str2.len() {
    //         str1
    //     } else {
    //         str2
    //     }
    // }

    // let s1 = String::from("stephen");
    // let s2 = String::from("james");

    // let longest_str = longest(&s1, &s2);
    // println!("longest_str={}", longest_str)

    // fn max<T>(arr: &[T]) -> T
    // where
    //     T: Copy + PartialOrd,
    // {
    //     let mut largest: T = arr[0];
    //     for item in arr {
    //         if *item > largest {
    //             // 比较会报错 consider restricting type parameter `T`: `:std::cmp::PartialOrd`
    //             largest = *item;
    //         }
    //     }
    //     largest
    // }
    // struct Rectangle<T> {
    //     width: T,
    //     height: T,
    // }

    // impl<T: Copy> Rectangle<T> {
    //     fn new(x: T, y: T) -> Rectangle<T> {
    //         Rectangle {
    //             width: x,
    //             height: y,
    //         }
    //     }
    // }

    // let rect = Rectangle::new(1, 1);
    // //the trait bound `Vec<{integer}>: Copy` is not satisfied the trait `Copy` is not implemented for `Vec<{integer}>`
    // let rect1 = Rectangle::new(vec![1], vec![1]);

    // trait Food {
    //     fn eat(&self) -> ();
    // }

    // struct Home;
    // impl<T: Food> Home<T> {
    //     fn () {

    //     }
    // }

    // trait Water {
    //     fn drink(&self) -> ();
    // }

    // fn home_eat<T>(item: T) -> impl Food
    // where
    //     T: Food + Water,
    // {
    //     // 这个时候，我们传入的Item必须实现Food的eat方法和Water的drink方法
    //     item.eat();
    //     item.drink();
    //     item
    // }

    // struct Home;
    // impl Food for Home {
    //     fn eat(&self) -> () {
    //         println!("eat")
    //     }
    // }
    // impl Water for Home {
    //     fn drink(&self) -> () {
    //         println!("drink")
    //     }
    // }
    // home_eat(Home);

    // trait Food {
    //     // 定义方法
    //     fn eat(&self) -> () {
    //         println!("hh")
    //     }
    // }

    // fn home_eat<T: Food>(item: T) {
    //     item.eat();
    // }

    // struct Home;
    // impl Food for Home {
    //     fn eat(&self) -> () {
    //         println!("eat")
    //     }
    // }
    // home_eat(Home)

    // trait Food {
    //     // 定义方法
    //     fn eat(&self) -> () {
    //         println!("hh")
    //     }
    //     // 定义函数
    //     fn add() -> ();
    // }

    // struct Home;

    // impl Food for Home {
    //     fn eat(&self) -> () {
    //         println!("eat vegetable")
    //     }
    //     fn add() -> () {
    //         println!("add food")
    //     }
    // }

    // struct Company;
    // impl Food for Company {
    //     // fn eat(&self) -> () {
    //     //     println!("eat buff");
    //     //     self.eat()
    //     // }
    //     fn add() -> () {
    //         println!("add drinkings")
    //     }
    // }
    // impl Company {
    //     fn out(&self) {
    //         self.eat();
    //     }
    // }
    // let home = Home {};
    // home.eat();
    // Home::add();

    // let company = Company {};
    // company.eat();
    // company.out();
    // Company::add();
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // impl<T> Point<T> {
    //     fn new(x: T, y: T) -> Point<T> {
    //         Point { x, y }
    //     }
    // }

    // let p: Point<f64> = Point::new(1.0, 2.0);
    // println!("{:#?}", p);
    // enum Color<T, U, K> {
    //     Red(T),
    //     Green(U),
    //     Blue(K),
    // }

    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }
    // fn test<T, U>() {}
    // fn max<T>(arr: &[T]) -> T {
    //     let mut largest: T = arr[0];
    //     for item in arr {
    //         if *item > largest {
    //             largest = *item;
    //         }
    //     }
    //     largest
    // }
    // let largest = max(&[100, 2, 3, 4]);
    // println!("{}", largest);
    // fn read_file() -> Result<String, io::Error> {
    //     let mut str: String = String::new();
    //     fs::File::open("./hello-world.txt")?.read_to_string(&mut str)?;
    //     println!("{}", str);
    //     Ok(str)
    // }
    // let content = read_file();
    // println!("{:?}", content);
    // fn read_file(path: &str) -> Result<String, io::Error> {
    //     let content = match fs::read_to_string(path) {
    //         Ok(data) => data,
    //         Err(error) => {
    //             println!("{:?}", error);
    //             return Err(error);
    //         }
    //     };
    //     return Ok(content);
    // }

    // fn create_file(path: &str) {
    //     match fs::File::create(path) {
    //         Ok(data) => {
    //             println!("{:?}", data);
    //         }
    //         Err(error) => {
    //             println!("{:?}", error)
    //         }
    //     }
    // }

    // let content = match read_file("./hello-world.txt") {
    //     Ok(data) => data,
    //     Err(error) => {
    //         return match error.kind() {
    //             ErrorKind::NotFound => {
    //                 create_file("./hello-world.txt");
    //             }
    //             _ => {
    //                 println!("其他错误");
    //             }
    //         };
    //     }
    // };

    // fs::read("./hello-world.txt").expect("创建文件失败");
    // let f = fs::read("./hello-world.txt").unwrap();
    // let f = fs::read("./hello-world.txt");
    // match f {
    //     Ok(data) => {
    //         println!("{:?}", data);
    //     }
    //     Err(error) => {
    //         return match error.kind() {
    //             ErrorKind::NotFound => {
    //                 fs::File::create("./hello-world.txt")?;
    //             }
    //             _ => {
    //                 println!("{}", error);
    //             }
    //         };
    //     }
    // }
    // println!("{:?}", f)
    // use std::io::Result;
    // let arr = vec![1, 2, 3];
    // println!("{}", arr[100]);
    // let arr = Result<i32, io::Error > {
    //     Ok(32),
    //     Err(V)
    // }
    // panic!("2222");
    // let arr = vec![1, 2, 3];
    // let arr2 = vec![5, 6, 7];
    // let mut iteration = arr.iter();
    // println!("{:#?}", &iteration.next().zip(Some(3)));
    // println!("{:#?}", &iteration.next());
    // println!("{:#?}", &iteration.next());
    // println!("{:#?}", &iteration.next());
    // println!("{:#?}", &iteration.next());
    // let mut arr: Vec<i8> = Vec::new();
    // arr.push(1);
    // arr.push(2);
    // println!("{:?}", arr)
    // use std::collections::HashMap;
    // let mut hm = HashMap::new();
    // hm.insert(String::from("stephen"), 20);
    // hm.insert(String::from("james"), 30);
    // hm.entry(String::from("kyrie")).or_insert(20);
    // println!("{:?}", hm)
    // for (key, value) in hm {
    //     println!("{}-{}", key, value);
    // }

    // let mut hm = HashMap::new();
    // hm.insert(String::from("stephen"), 20);
    // hm.insert(String::from("james"), 30);
    // println!("{:?}", hm);
    // let str = String::from("Здравствуйте");
    // for i in str.bytes() {
    //     println!("{}", i);
    // }
    // let str_value = String::from("Здравствуйте");
    // let str_slice = &str_value[0..1];
    // println!("{}", str_slice)
    // let mut str1 = String::new();
    // str1.push_str("hello");
    // let mut str2 = String::new();
    // str2.push_str("world");
    // let str3 = format!("{}{}", str1, str2);
    // println!("{}", str3);
    // println!("{}", str1);

    // let str_value = String::from("hello, world");
    // let new_str = &str_value[0..1];
    // println!("{}", new_str);
    // let new_str: String = "hello, world".to_string();
    // println!("{:?}", new_str);
    // let mut str = String::new();
    // str.push('h');
    // str.push('e');
    // str.push('l');
    // str.push('l');
    // str.push('o');
    // str.push_str(", world");
    // println!("{}", str);
    // let mut arr = vec![1, 2, 3];
    // for i in &mut arr {
    //     *i *= 2;
    //     println!("{}", i);
    // }

    // match arr.get(5) {
    //     Option::Some(i) => {
    //         println!("{}", i);
    //     }
    //     Option::None => {
    //         println!("出现数组越界的情况"); // match匹配会达到这里
    //     }
    // }

    // #[derive(Debug)]
    // enum StoreData {
    //     Int(i32),
    //     Float(f64),
    //     Str(String),
    //     Boolean(bool),
    // }

    // let mut arr: Vec<StoreData> = Vec::new();

    // arr.push(StoreData::Float(1.0));
    // arr.push(StoreData::Int(2));
    // arr.push(StoreData::Str(String::from("这是一串字符")));
    // arr.push(StoreData::Boolean(true));
    // println!("{:?}", arr);

    // let arr: Vec<i8> = vec![1, 2, 3];
    // for i in arr {
    //     println!("{}", i)
    // }
    // let mut arr: Vec<i32> = Vec::new();
    // arr.push(1);
    // arr.push(2);

    // println!("{:?}", arr)

    // enum  {

    // }

    // mod my_module {
    //     fn test() {}
    //     struct User {
    //         age: i8,
    //         name: String,
    //         email: String,
    //     }
    //     enum Color {
    //         Red,
    //         Green,
    //         Blue,
    //     }
    // }

    // self::public_module::test();

    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }
    // let color: Color = Color::Blue;
    // if let Color::Blue = color {
    //     println!("蓝色")
    // }
    // fn add_one(value: Option<i16>) -> Option<i8> {
    //     match value {
    //         Option::Some(i:i8) => Some(i + 1),
    //         Option::None => None,
    //     }
    // }
    // let num: Option<i16> = Some(5);
    // let num: Option<i8> = add_one(num);
    // println!("{:#?}", num) // Some(6)
    // #[derive(Debug)]
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }
    // fn match_color(color: Color) {
    //     match color {
    //         Color::Blue => {
    //             println!("{}", "蓝色")
    //         }
    //         _ => println!("其余颜色"),
    //     }
    // }

    // match_color(Color::Red);
    // #[derive(Debug)]
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    // }

    // fn match_color(color: Color) {
    //     match color {
    //         Color::Blue => {
    //             println!("{}", "蓝色")
    //         }
    //         Color::Green => {
    //             println!("{}", "绿色")
    //         }
    //         Color::Red => {
    //             println!("{}", "红色")
    //         }
    //     }
    // }
    // match_color(Color::Blue);

    // impl Color {
    //     fn instance_fn(&self) {
    //         if let Color::Blue = self {
    //             println!("{:#?}", "蓝色");
    //         }
    //     }
    //     fn static_fn() {
    //         println!("{:#?}", Color::Red)
    //     }
    // }

    // Color::static_fn(); // Red
    // let color = Color::Blue;
    // color.instance_fn();
    //Blue
    // Blue
    // let _not_confirmed_value: Option<i8> = Some(1);
    // let _not_confirmed_value: Option<i8> = None;
    // #[derive(Debug)]
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    //     RGB(u16, u16, u16),
    //     Hex { input: String },
    // }
    // println!(
    //     "{:#?}",
    //     Color::Hex {
    //         input: String::from("#000")
    //     }
    // );
    // println!("{:#?}", Color::RGB(255, 255, 255));

    // struct Position(i16, i16, i16);
    // #[derive(Debug)]
    // enum Color {
    //     // 红色
    //     Red,
    //     Green,
    //     Blue,
    // };
    // println!("{:#?}", Color::Blue)
    // let pos = Position(1, 1, 1);
    // println!("{}", pos.0)
    // #[derive(Debug)]
    // struct User {
    //     name: String,
    //     age: i8,
    //     height: i32,
    // }
    // impl User {
    //     fn compare_age(&self, other: &User) -> bool {
    //         // 为User定义一个判断是否是成人的方法
    //         self.age >= other.age
    //     }
    // }
    // let user = User {
    //     name: String::from("james"),
    //     age: 17,
    //     height: 203,
    // };
    // let user1 = User {
    //     name: String::from("stephen"),
    //     age: 18,
    //     height: 176,
    // };
    // println!("我的年纪更大:{}", user1.compare_age(&user))
    // let str: String = String::from("hello");
    // let x = &str[1..3];
    // println!("{}", x)
    // str.push_str("hello");
    // fn calc_len(mut x: String) -> String {
    //     x.push_str(", world");
    //     return x;
    //     // }
    //    let  calc_len(str);
    //     println!("{}", str)
    // let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    // for i in arr {
    //     if i == 3 {
    //         continue;
    //     }
    //     println!("{}", i)
    // }
    // println!("========:========");
    // for i in 1..6 {
    //     if i == 3 {
    //         continue;
    //     }
    //     println!("{}", i)
    // }

    // let mut arr: [i8; 2] = [1, 1];
    // arr.push(1);
    // let mut arr1 = vec![];
    // arr1.push(1);
    // let arr: [i8; 2] = [1, 2];
    // let tuple: (i32, &str) = (1, "stephen");
    // let first: i32 = tuple.0; // 1
    // let second: &str = tuple.1; // "stephen"
    // let return_str = if true { "right" } else { 1 };

    // print_hello_world();
    // homework::transfer_temperature_to_hua(1.0);
    // homework::transfer_temperature_to_wen(33.8);
    // classes::guest_game();
    // println!("斐波那契数列。");
    // let input: i8 = 6;
    // let value = homework::fib(input);
    // println!("fib({})={}", input, value);

    // println!("读取歌词循环打出歌词");
    // let mut content: String = match homework::deal_song() {
    //     Ok(val) => val,
    //     Err(err) => {
    //         return match err.kind() {
    //             ErrorKind::NotFound => {
    //                 fs::File::create("./static/song.txt").unwrap();
    //             }
    //             _ => {
    //                 println!("err={}", err);
    //             }
    //         };
    //     }
    // };
    // println!("歌词={:?}", &content.split("\n\n"))
}
