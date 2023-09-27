/*!
 * # 注释和文档
 *
 * 注释和文档学习笔记
 *
 * 这一条是模块注释
 *
 */

/// `add_one` 将指定值加一
///
/// # Example
///
/// ```rust
/// let arg = 5;
/// let anwser = crate::practic::prac_2_13::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(a: i32) -> i32 {
    a + 1
}

/// # Panic doc
/// 
/// The function div while panic when b == 0
/// 
/// ```rust, should_panic
/// book_study1::practic::prac_2_13::div(2, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}


pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    //! 定义颜色的类型

    /// 主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 副色
    #[derive(Debug,PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! 实用工具，目前只实现了调色板
    use super::kinds::*;

    /// 将两种主色调成副色
    /// ```rust
    /// use utils::mix;
    /// use super::kinds::{PrimaryColor,SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}



/// ```
/// # fn try_main() -> Result<(), String> {
/// let res = crate::practic::prac_2_13::try_div(10, 0)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { 
/// #    try_main().unwrap();
/// #
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}


/// Add one to the given value and return a [`Option`] type
pub fn add_three(x: i32) -> Option<i32> {
    Some(x + 3)
}

#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::practic::prac_2_10::prac_2_10;
