mod util;
mod sort;
mod learn_rust;
mod search;

use crate::learn_rust::*;
use crate::util::{ArgsMap, iter_to_string};
use crate::sort::heap;
use crate::sort::selection;

//           <delimiter> <args parts>
// cargo run    --       --mod=learn_rust
// `--`: unix convention(https://unix.stackexchange.com/a/187548). 옵션이 끝났음을 알린다.
fn main() {
    delegator();
}

fn delegator() {
    // Note that the first argument `&self` is implicitly passed,
    let _args = util::Config.parse();
    // https://docs.rs/serde_json/latest/serde_json/fn.to_string.html
    // println!("{}", serde_json::to_string(&_args).unwrap()); // {"--mod":"learn_rust"}

    // early return: https://docs.rs/early_returns/latest/early_returns/
    if ! _args.contains_key("--mod") {
        return;
    }

    match _args.get("--mod").unwrap().as_str() {
        "learn_rust" => learn_rust(),
        "heap_sort" => heap_sort(),
        "selection_sort" => selection_sort(),
        _ => {
            // `()` 의미: https://stackoverflow.com/q/24842271
            ()
        }
    }
}

fn learn_rust() {
    about_borrows();
    about_usize();
    about_array();
    for_loops();
    about_i32();
    about_i64();
    about_divide();
    caller();
    refs();
}

fn heap_sort() {
    println_with_padding("=================== heap sort ===================");
    let mut vec_to_sort: Vec<i64> = vec![7, 9, 4, 8, 6, 3];
    println!("{}", iter_to_string(vec_to_sort.iter(),","));
    let len = vec_to_sort.len();
    let sorted = heap::sort(&mut vec_to_sort, len).unwrap();
    println!("ㄴ{}", iter_to_string(sorted.iter(),","));
}

fn selection_sort() {
    println_with_padding("=================== selection sort ===================");
    let mut vec_to_sort: Vec<i32> = vec![8, 31, 48, 73, 3, 65, 20, 29, 11, 15];
    println!("{}", iter_to_string(vec_to_sort.iter(),","));
    // selection::sort(&mut vec_to_sort, vec_to_sort.len());
    // ㄴ 이렇게 쓸 수는 없다. 이미 `&mut vec_to_sort` 통해서 mutable로 borrowed 됐기 때문에, immutable로 다시 borrowed 될 수 없다
    // ㄴ 따라서 size라는 새로운 변수에 담아서 전달한다
    let size = vec_to_sort.len();
    let sorted = selection::sort(&mut vec_to_sort, size);
    println!("ㄴ{}", iter_to_string(sorted.iter(), ","));
}

fn println_with_padding(msg: &str) {
    println!("=================== {} ===================", msg);
}




