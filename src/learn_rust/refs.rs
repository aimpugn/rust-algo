
pub fn refs() {
    let tmp: i32 = 1234;
    let tmp_ref = &tmp;
    // tmp is 1234, address: 0x16fbcac34
    println!("tmp is {}, address: {:p}", tmp.to_string(), tmp_ref);

    let tmp2 = tmp * 2;
    let tmp2_ref = &tmp2;
    // tmp2 is 2468, address: 0x16fbcacac
    println!("tmp2 is {}, address: {:p}", tmp2.to_string(), tmp2_ref);

    // let tmp3 = tmp * 3; -> Use of moved value
    let tmp3 = &tmp * 3;
    let tmp3_ref = &tmp3;
    // tmp3 is 3702, address: 0x16fbcad24
    println!("tmp3 is {}, address: {:p}", tmp3.to_string(), tmp3_ref);

    let tmp4 = &tmp3_ref;
    // address of &&tmp3: 0x16fbcad28
    println!("address of &&tmp3: {:p}", tmp4);

    caller1();
}

fn caller1() {
    let mut p1: usize = 12345;
    callee1(&mut p1);
}

fn callee1(p1: &mut usize) {
    println!("&p1.to_string() of &usize => {}", &p1.to_string());
    println!("p1.to_string() of &usize => {}", p1.to_string());
    println!("(*p1).to_string() of &usize => {}", (*p1).to_string());

    let mut tmp = (*p1).to_owned();
    tmp += tmp;
    println!("assign (*p1).to_owned() to var again and add => {}", tmp.to_string());

    // tmp += tmp; <- Use of moved value
}
