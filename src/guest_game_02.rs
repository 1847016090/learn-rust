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
