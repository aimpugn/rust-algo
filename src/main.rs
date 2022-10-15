mod util;
mod sort;
mod learn_rust;

use crate::learn_rust::*;
use crate::util::ArgsConfig;


fn main() {
    // Note that the first argument `&self` is implicitly passed,
    let _args = util::Config.parse();
    // https://docs.rs/serde_json/latest/serde_json/fn.to_string.html
    println!("{}", serde_json::to_string(&_args).unwrap());
    // print_args_with_delimeter(&env::args().collect::<Vec<String>>(), Some(","));

    about_usize();
    about_array();
    about_i32();
    about_i64();
    caller();
    refs();
}


