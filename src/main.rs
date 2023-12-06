mod enter_01;
// use enter_01::print_hello_world;

mod guest_game_02;
// use guest_game_02::{classes, homework};
// use std::{fs, io::ErrorKind};

fn main() {
    let str: String = String::from("hello");
    let x = &str[1..3];
    println!("{}", x)
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
