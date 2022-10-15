
pub fn caller() {
    let i64_num = 922337203685;
    let i64_num_ref = &i64_num;
    callee1(i64_num);
    callee2(i64_num_ref);
}

fn callee1(param1 : i64) {
    println!("param1 : i64: {}", param1.to_string());
}

fn callee2(param1 : &i64) {
    println!("param1 : &i64: {}", param1.to_string());
}