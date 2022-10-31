pub fn for_loops() {
    for i in 2..10 {
        print!("{}, ", i.to_string())
    }
    println!();
    for i in 10..1 {
        print!("{}, ", i.to_string())
    }
    println!();
    for i in (1..10).rev() {
        print!("{}, ", i.to_string())
    }
    println!();

    let arr1 = vec![1, 2, 3, 4];
    for el in arr1.clone() {
        print!("{},", el.to_string());
    }
    println!();

    for (i, el) in arr1.clone().iter().enumerate() {
        print!("[{}]{},", i.to_string(), el.to_string());
    }
    println!();

    // https://users.rust-lang.org/t/for-loops-decrement-by-more-than-1-value/18389/2
    for el in arr1.clone().into_iter().step_by(2) {
        print!("{},", el.to_string());
    }
    println!();
}