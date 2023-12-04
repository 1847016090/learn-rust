// 完成一个字谜游戏
pub mod classes {
    pub fn guest_game() {
        use rand::Rng;
        use std::cmp::Ordering;
        use std::io::stdin;

        let secret_num: u32 = rand::thread_rng().gen_range(1..101);
        println!("The secret num is {}", secret_num);

        loop {
            println!("Please input your number:");
            let mut guest = String::new();
            stdin().read_line(&mut guest).expect("Input failed");
            let guest: u32 = match guest.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guest.cmp(&secret_num) {
                Ordering::Equal => {
                    println!("You win!!!!");
                    break;
                }
                Ordering::Greater => println!("Too big!!!!"),
                Ordering::Less => println!("Too small!!!!"),
            }
        }
    }
}

pub mod homework {
    // 摄氏温度与华氏温度的相互转换。 1摄氏度 = 33.8 华氏温度
    pub fn transfer_temperature_to_hua(tem: f32) {
        let transferred_value = tem * 33.8;
        println!(
            "摄氏温度{a}度转为华氏摄氏度为{b}",
            a = tem,
            b = transferred_value
        )
    }
    pub fn transfer_temperature_to_wen(tem: f32) {
        let transferred_value = tem / 33.8;
        println!(
            "华氏摄氏度{a}度转为摄氏温度为{b}",
            a = tem,
            b = transferred_value
        )
    }
    // 生成一个n阶的斐波那契数列。
    pub fn fib(n: i8) -> i8 {
        if n == 1 || n == 2 {
            return 1;
        }
        return fib(n - 1) + fib(n - 2);
    }
    // 打印圣诞颂歌The Twelve Days of Christmas的歌词，并利用循环处理其中重复的内容。
    pub fn deal_song() -> Result<String, std::io::Error> {
        use std::fs;
        let result: Result<String, std::io::Error> = fs::read_to_string("./static/song.txt");
        let content = match result {
            Ok(data) => data,
            Err(err) => return Err(err),
        };
        return Ok(content);
    }
}
