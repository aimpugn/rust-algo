use std::fmt::Display;

// https://users.rust-lang.org/t/how-do-i-convert-vec-of-i32-to-string/18669/9
pub fn iter_to_string<I, T>(iter: I, seperator: &str) -> String where
// https://stackoverflow.com/a/34969944
    I: Iterator<Item = T>,
// Note: the following trait bounds were not satisfied: `T: std::fmt::Display
// ToString trait은 Display trait을 구현하는 경우 자동으로 구현되며,
// ToString을 직접 구현하지 않고, Display을 구현해야 한다
// - https://doc.rust-lang.org/std/string/trait.ToString.html
    T: Display
{
    iter
        // Display 타입을 구현한 element를 String으로 변환하고
        .map(|el|
            el.to_string()
        )
        // join을 위해 Vec 타입으로 collect
        .collect::<Vec<String>>()
        .join(seperator)
}