mod util;
mod sort;
mod learn_rust;

use crate::learn_rust::*;
use crate::util::{ArgsMap, iter_to_string};
use crate::sort::heap;


fn main() {
    // Note that the first argument `&self` is implicitly passed,
    let _args = util::Config.parse();
    // https://docs.rs/serde_json/latest/serde_json/fn.to_string.html
    println!("{}", serde_json::to_string(&_args).unwrap());
    // print_args_with_delimeter(&env::args().collect::<Vec<String>>(), Some(","));

    about_borrows();
    about_usize();
    about_array();
    about_i32();
    about_i64();
    caller();
    refs();

    println!("=================== heap sort ===================");
    let mut vec_to_sort: Vec<i64> = vec![7, 9, 4, 8, 6, 3];
    let len = vec_to_sort.len();
    heap::heap_sort(&mut vec_to_sort, len);
}


