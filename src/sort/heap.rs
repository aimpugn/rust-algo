use crate::util::strings::iter_to_string;

// T 타입의 길이를 구하는 방법은 다양할 수 있으므로,
// `len`은 인자로 받도록 한다
pub fn sort(list: &mut Vec<i64>, len: usize) -> Option<&Vec<i64>>
{
    // 1. 힙으로 만들어서 전체 노드가 힙성질을 갖게 만든다
    build_heap(list, &len);
    // 2. 정렬: 루트 노드가 가장 작으므로,
    //  a. 루트 노드부터 순서대로 가장 뒤로 빼내면 내림차순으로 정렬할 수 있고,
    //  b. 힙 성질이 깨진 나머지 배열을 다시 힙으로 만들어 주고
    //  c. 다시 루트 노드를 가장 뒤로 빼내는 것을 반복하면
    // 내림차순으로 정렬이 된다
    for i in (1..len.clone()).rev() {
        // a & c. 루트 노드를 끝에서부터 다시 채워나간다
        list.swap(0, i.clone());
        // b. 힙 성질을 유지시킨다
        heapify(list, 0, &i); // `i`: decrement 된 사이즈와 같은 값인 `i`를 그대로 전달한다
    }
    Some(list)
}

fn build_heap(list: &mut Vec<i64>, len: &usize) {
    // (0..10).rev(): 9부터 역순으로 차감해 간다
    // ㄴ https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
    for i in ((0 as usize)..(len / 2)).rev() { // `(len / 2)`: 리프 노드의 부모 노드부터 시작
        heapify(list, i, len);
    }
}

// 힙 성질:
// - 이진 트리
// - 맨 아래 층을 제외하고는 완전히 채워져 있음
// - 맨 아래층은 왼쪽부터 꽉 채워져 있음
// - 각 노드의 값은 자식 노드의 값보다 작거나 같다
//
//            0  1  2  3  4  5
// ex) list: [1, 3, 6, 4, 5, 7], len: 6
//           1
//       3      6
//     4  5    7
fn heapify(list: &mut Vec<i64>, i: usize, len: &usize) {
    // 이때 list[i]의 두 자식 노드 `list[(2 * i) + 1]` 및 `list[(2 * i) + 2]`의 서브 트리는 힙성질을 만족하고 있다
    // 좌측 자식 노드의 인덱스 예상: `list[(2 * i) + 1]`
    let left: usize = (&i * 2) + 1;
    // 우측 자식 노드의 인덱스 예상: `list[(2 * i) + 2]`
    let right: usize = (&i + 1) * 2;

    // 부모 노드의 값과 비교하기 위해,
    // 자식 노드 중에 더 작은 값은 갖는 노드의 인덱스를 찾는다
    let smaller_child = if &right < &len { // 두 자식 노드를 갖는다
        if &list[left.clone()] < &list[right.clone()] {
            Some(left)
        } else {
            Some(right)
        }
    } else if &left < &len { // 한 자식 노드를 갖는다
        Some(left)
    } else {
        // `len <= left && len <= right`
        // 더이상 자식 노드 없으며 끝에 도달함
        None
    };

    if smaller_child == None {
        return;
    }

    let smaller: usize = smaller_child.unwrap();
    // 부모노드의 값과 비교하여, 자식 노드의 값이 더 작다면 스왑
    if &list[smaller.clone()] < &list[i] {
        list.swap(smaller.clone(), i.clone());
        // println!("when recursive heapify, list => {}", iter_to_string(list.iter().clone(), ","));
        heapify(list, smaller, len);
    }
}