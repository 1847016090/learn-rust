//! # My Crate
//! 仅供学习和参考

pub use self::kinds::Color;

pub mod kinds {
    pub enum Color {
        Red,
        Green,
        Blue,
    }
}

// /// 传入值加一
// ///
// /// # 用例
// ///
// ///  ```
// /// use hello_world::add_one;
// /// let y = 2;
// /// let result = add_one(1);
// /// assert_eq!(result, y)
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }
