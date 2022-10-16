
// `mod`: here's the new module (from file or in curly brackets), store it for future use
// `use`: there's already created module, pull it from this path and let me use it here
// `pub` || `pub(crate)`: Make config public
pub mod config;
pub mod strings;

pub use self::config::*;

// `main.rs`에서 use Config 하지 않아도 바로 사용
pub use self::strings::*;