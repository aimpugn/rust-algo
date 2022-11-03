// use crate::util::strings::iter_to_string;

pub fn sort(list: &mut Vec<i32>, size: usize) -> &mut Vec<i32>{
    // 가장 큰 값을 뒤로 보내기 위해 마지막 인덱스부터 차감해 간다
    for idx in (1..size).rev() {
        // https://doc.rust-lang.org/error_codes/E0502.html
        // list.swap(idx, largest(list)); 이렇게 바로 사용할 수는 없다
        // list.swap에서 이미 borrowed 됐기 때문
        let largest = largest(list, idx);
        list.swap(idx, largest);
    }
    list
}

// 가장 큰 수의 인덱스를 반환한다
// `last`: 다음 largest 호출 시 `last(=idx)`는 계속 차감되며 배열의 범위를 좁혀 간다
fn largest(list: &Vec<i32>, last: usize) -> usize {
    let mut largest: usize = 0;
    // `min..=max`: max 포함
    //  ㄴ `1..1` -> 1 무시된다
    //  ㄴ `1..=1` -> 1까지 처리
    for idx in 1..=last {
        if &list[largest] < &list[idx]{
            largest = idx
        }
    }
    largest
}