fn owner() {
    let basic_value = "basic_value";
    // 基本类型自动拷贝
    let copy_basic_value = makes_copy(basic_value);
    println!("basic_value = {}", basic_value);
    println!("copy_basic_value = {}", copy_basic_value);

    let transfer_owner = String::from("transfer_owner");
    // “hello” 的所有权已经移动给了y变量
    let transferred_owner = transfer_ownership(transfer_owner);
    // error: move occurs because `x` has type `String`, which does not implement the `Copy` trait
    // println!("x={}", x);
    println!("transferred_owner = {}", transferred_owner)
}

// 当值作为参数传入函数中时，此时它绑定的值已经移动到了并且绑定到str上
fn transfer_ownership(str: String) -> String {
    let transfer_owner = str;
    transfer_owner
}

// 基本类型存储于栈中将会自动进行拷贝
fn makes_copy(str: &str) -> &str {
    let copy_basic_value = str;
    copy_basic_value
}
