use rand::Rng;
use std::io::stdin;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("随机生成的数字={}", secret_num);
    let mut str: String = String::new();
    stdin().read_line(&mut str).expect("报错啦");
    print!("输出={}", str)
}
