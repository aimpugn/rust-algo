pub fn about_borrows() {
    // What are non-lexical lifetimes?
    // - https://stackoverflow.com/a/50253558
    let mut scores = vec![1, 2, 3];
    let score = &scores[0]; // immutable borrow occurs here
    scores.push(4); // cannot borrow `scores` as mutable because it is also borrowed as immutable
                            //  <-- score stops borrowing here
    // println!("{}", &score); // 사용하려고 할 때 에러 발생

    let mut items = vec![1];
    let item = items.last();
    items.push(2);
}