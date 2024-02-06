// This is the documentation that will be created in an html file and this comments supports the
// markdown
//
// we can open it by running the command - cargo doc --open
//
// there are some common section, that can be helpful by writing the documentation
//
// to run the test in example section we can call the doc-tests command
//
// this is the description of the crate
//! # Profiles crate
//!
//! this crate should show you how to write cargo documentation and how can we overwriting the
//! profiles settings
/// Adds one to the number given
///
/// # Panic
/// scenarias in which the code will panic
///
/// # Errors
/// if the fn returns the result it will be helpfull to write which errors that can returns
///
/// # Safety
/// in case we use unsafe, that should be explained
///
/// # Examples
///
/// ```
/// use profiles::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
// is we want to reimport the modules, so that the users can import them like: use
// crate::some_module
// we should use the pub use syntax
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
pub fn add_one(x: i32) -> i32 {
    x + 1
}
