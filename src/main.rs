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

fn main() {
    let arr: Vec<i8> = Vec::new();
    arr.push(1);
    arr.push(2);
    println!("{:?}", arr)
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
