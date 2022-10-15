use std::fmt::Display;


pub fn about_array(){
    // let i64arr_1: [i64] = [1, 2, 3, 4]; -> error
    // - the trait `Sized` is not implemented for `[i64]`
    // - https://stackoverflow.com/a/28175750
    // - `[i64]` 타입에 대해 얼마나 많은 공간을 할당해야 하는지 컴파일러가 모른다는 의미
    let i64arr_1: [i64; 4] = [1, 2, 3, 4];
    println!("{}", iter_to_string(i64arr_1.iter(), ","));

    // 반면, `[i32]`의 경우에는, 얼마나 많은 공간을 할당해야 하는지 컴파일러가 안다
    let i32arr_1 = [1, 2, 3, 4];
    println!("{}", iter_to_string(i32arr_1.iter(), ","));
}

// https://users.rust-lang.org/t/how-do-i-convert-vec-of-i32-to-string/18669/9
fn iter_to_string<I, T>(iter: I, seperator: &str) -> String where
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
