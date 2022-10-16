use crate::util::strings::iter_to_string;

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
