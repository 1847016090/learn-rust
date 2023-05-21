fn main() {
    let x: i32 = 6;
    let y: &str = if x % 2 == 1 {
        "odd"
    } else if x % 3 == 0 {
        "ddddd"
    } else {
        "add"
    };
    print!("{}", y)
}
