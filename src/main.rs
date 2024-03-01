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

// use std::fmt::Display;

// use std::io;
// use std::io::{ErrorKind, Read};
// use std::{fs, io};

// use std::env;
// use std::error::Error;
// use std::fs;
// use std::process;

// use std::env;

// /**
//  * 命令行获取参数配置
//  */
// struct Config {
//     /**
//      * 匹配文字
//      */
//     match_text: String,
//     /**
//      * 文件名
//      */
//     file_name: String,
// }

// impl Config {
//     fn new(mut args: env::Args) -> Result<Config, &'static str> {
//         // if args.len() < 3 {
//         //     return Err("输入的参数不能小于2位");
//         // }
//         // let match_text = args[1].clone();
//         // let file_name = args[2].clone();
//         args.next();
//         let match_text = match args.next() {
//             Some(text) => text,
//             None => return Err("未获取到匹配内容"),
//         };
//         let file_name = match args.next() {
//             Some(name) => name,
//             None => return Err("未获取到文件名"),
//         };
//         Ok(Config {
//             match_text,
//             file_name,
//         })
//     }
// }

// fn run(config: &Config) -> Result<(), Box<dyn Error>> {
//     let match_file = fs::read_to_string("./".to_string() + &config.file_name)?;
//     let match_file_text_arr: Vec<&str> = match_file.lines().collect();
//     let env_param: String = env::var("SENSITIVE")?;

//     for value in match_file_text_arr.iter() {
//         if env_param == "1" {
//             if value.contains(&config.match_text) {
//                 println!("大小写敏感匹配成功:{}包含{}", value, &config.match_text);
//             } else {
//                 println!("大小写敏感匹配失败:{}不包含{}", value, &config.match_text);
//             }
//         } else {
//             let n_value = value.to_lowercase(); // 都转为转为小写
//             let n_match_text = &config.match_text.to_lowercase(); // 都转为转为小写
//             if n_value.contains(n_match_text) {
//                 println!("大小写不敏感匹配成功:{}包含{}", value, &config.match_text);
//             } else {
//                 println!("大小写不敏感匹配失败:{}不包含{}", value, &config.match_text);
//             }
//         }
//     }
//     Ok(())
// }

fn main() {
    use std::time::Duration;
    use std::{sync::mpsc, thread};
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let arr = vec![String::from("hi"), String::from("ni hao")];
        for val in arr {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

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

    // use std::time::Duration;
    // use std::{sync::mpsc, thread};

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let arr = vec![String::from("hello"), String::from("world")];
    //     for val in arr {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for receive_val in rx {
    //     println!("接受到：{}", receive_val);
    // }

    // use std::{sync::mpsc, thread};

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hello world");
    //     tx.send(val).unwrap();
    //     println!("{}", val);
    // });

    // let receive_val = rx.recv().unwrap();
    // println!("{}", receive_val);

    // use std::thread;
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    // drop(v);

    // handle.join().unwrap();

    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("【新】线程中的数据：{}", i);
    //         thread::sleep(Duration::from_millis(1))
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("【主】线程中的数据：{}", i);
    //     thread::sleep(Duration::from_millis(1))
    // }

    // use std::{cell::RefCell, mem::drop};

    // let x = RefCell::new(vec![1, 2, 3]);
    // x.borrow_mut().push(1);
    // println!("{:?}", x)
    // use std::ops::Deref;
    // struct MyCustomPointer {
    //     data: String,
    // }

    // impl Drop for MyCustomPointer {
    //     fn drop(&mut self) {
    //         println!("自定义操作")
    //     }
    // }

    // let m = MyCustomPointer {
    //     data: String::from("hello"),
    // };

    // drop(m);

    // println!("结束")
    // struct MyBox<T>(T);
    // impl<T> MyBox<T> {
    //     fn new(d: T) -> MyBox<T> {
    //         MyBox(d)
    //     }
    // }

    // impl<T> Deref for MyBox<T> {
    //     type Target = T;
    //     fn deref(&self) -> &T {
    //         &self.0
    //     }
    // }

    // let x = MyBox::new(String::from("world"));

    // fn hello(name: &str) {
    //     println!("{}", name);
    // }

    // hello(&x);

    // let y = Box::new(5);
    // assert_eq!(5, *y);
    // let y = 5;
    // let x = &y;
    // assert_eq!(5, x);
    // assert_eq!(5, *x);
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // let list = List::Cons(
    //     1,
    //     Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    // );

    // println!("{:#?}", list);

    // let box = Box::new(5);

    // let box = Box::new(3);
    // println!("{}", box)

    // let args: Vec<String> = env::args().collect();

    // let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
    //     println!("参数解析错误：{}", err);
    //     println!("参数解析错误：{}", err);
    //     process::exit(1)
    // });

    // run(&config).unwrap_or_else(|err| {
    //     println!("文件或环境参数解析出错：{}", err);
    //     println!("文件或环境参数解析出错：{}", err);
    //     process::exit(1);
    // });

    // struct Counter {
    //     count: i32,
    // }

    // impl Counter {
    //     fn new() -> Counter {
    //         Counter { count: 0 }
    //     }
    // }

    // impl Iterator for Counter {
    //     type Item = i32;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         self.count = self.count + 1;
    //         if self.count < 6 {
    //             Some(self.count)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // let mut counter = Counter::new();
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());

    // #[derive(Debug)]
    // struct User {
    //     name: String,
    //     age: i32,
    // }
    // let arr = vec![
    //     User {
    //         name: String::from("stephen"),
    //         age: 18,
    //     },
    //     User {
    //         name: String::from("kyrie"),
    //         age: 34,
    //     },
    //     User {
    //         name: String::from("kris"),
    //         age: 22,
    //     },
    // ];
    // let less_age: i32 = 30;
    // // 筛选出年纪小于less_age的用户
    // let iter_arr: Vec<&User> = arr.iter().filter(|user| user.age < less_age).collect();
    // println!("{:#?}", iter_arr);

    // let arr = vec![1, 2, 3];
    // let iter_arr: Vec<i32> = arr.iter().map(|x| x + 1).collect();
    // println!("{:?}", iter_arr)
    // let sum: i32 = arr.iter().sum();
    // println!("{}", sum)
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }
    // let arr = vec![1, 2, 3];
    // let iter_arr = arr.iter();
    // println!("{:?}", arr);
    // struct Counter {
    //     num: i32,
    // }

    // impl Iterator for Counter {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         let num = self.num + 1;
    //         Counter { num }
    //     }
    // }

    // let s = String::from("hello");

    // let update_str = || println!("{}", s);

    // fn executeFn<T>(f: T)
    // where
    //     T: Fn() -> (),
    // {
    //     f();
    // }

    // executeFn(update_str);

    // fn executeFnOnce<T>(f: T)
    // where
    //     T: FnOnce() -> (),
    // {
    //     f();
    // }
    // executeFnOnce(update_str);

    // fn executeFnMut<T>(mut f: T)
    // where
    //     T: FnMut() -> (),
    // {
    //     f();
    // }
    // executeFnMut(update_str)

    // let mut str = String::from("hello");
    // let mut combine_str = || {
    //     &str.push_str("world");
    // };
    // combine_str();
    // println!("{:?}", str);
    // let y = 1;
    // let mut get_self = |x| {
    //     y = y + 1;
    //     x + y
    // };
    // let x = get_self(1);
    // let z = get_self(2);
    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", z)
    // let str1: String = String::from("hello ");
    // let combine_str = |x| str1.push_str(x);
    // combine_str("world");

    // println!("{:?}", str1);
    // let get_self = |x| x;
    // get_self(1);
    // get_self(2);
    // get_self('1');
    // let y: i32 = 1;
    // let get_sum = |x| x + y;
    // let result = get_sum(1);
    // println!("{}", result)
    // fn get_self() {
    //     let get_cache = |x| x;
    //     let z = get_cache(2);
    //     println!("{}", z);
    // }

    // get_self();

    // let match_text: &String =&args[1] ;
    // let file_name = &args[2];

    // let args: env::Args = env::args();
    // let inputs: Vec<String> = args.collect();
    // println!("{:?}", inputs);

    // let check_file = fs::read_to_string("./filename.txt").expect("读取文件失败");
    // // check_file
    // let str: Vec<&str> = check_file.lines().collect();
    // println!("{:?}", str);

    // for argument in env::args() {
    //     println!("{argument}");
    // }

    // fn longest<'a, T>(str1: &'a str, str2: &'a str, str3: T) -> &'a str
    // where
    //     T: Display,
    // {
    //     println!("extra={}", str3);
    //     if str1.len() > str2.len() {
    //         str1
    //     } else {
    //         str2
    //     }
    // }
    // let s1 = String::from("stephen");
    // let s2 = String::from("james");
    // let long = longest(&s1, &s2, 2);
    // println!("{}", long);
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
