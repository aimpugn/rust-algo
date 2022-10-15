pub fn about_usize(){
    println!("usize::MIN: {}", usize::MIN.to_string());
    println!("usize::MAX: {}", usize::MAX.to_string());
}

pub fn about_i32(){
    println!("i32::MIN => {}", i32::MIN.to_string());
    println!("i32::MAX => {}", i32::MAX.to_string());
}

pub fn about_i64(){
    println!("i64::MIN => {}", i64::MIN.to_string());
    println!("i64::MAX => {}", i64::MAX.to_string());
    let num1: i64 = 10;
    // num1_tmp1 계산 시 `num1`을 사용하고
    // 다시 num1_tmp2 계산 시 `num1`을 사용하려고 하면,
    // `Use of moved value` 발생
    //  - let num1_tmp1 = num1 * 2;
    //  - let num1_tmp2 = num1 * 2; <- `Use of moved value`

    // `&num1`으로 해당 값만 사용한다
    let num1_tmp1 = &num1 * 2;
    let num1_tmp2 = &num1 * 2;
    println!("&num1 * 2 => {}", num1_tmp1); // 20
    println!("and then &num1 * 2 again => {}", num1_tmp2); // 20

    let num2: i64 = 20;
    let num2_tmp1 = num2 * 2;
    let num2_tmp2 = &num2 * 2;
    println!("num2 * 2 => {}", num2_tmp1); // 40
    println!("and then &num2 * 2 => {}", num2_tmp2); // 40

    let num3: i64 = 30;
    let num3_tmp1 = &num3 * 2;
    let num3_tmp2 = num3 * 2;
    println!("&num3 * 2 => {}", num3_tmp1); // 60
    println!("and then num3 * 2 => {}", num3_tmp2); // 60

}