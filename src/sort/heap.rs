use crate::util::strings::iter_to_string;

// T 타입의 길이를 구하는 방법은 다양할 수 있으므로,
// `len`은 인자로 받도록 한다
pub fn heap_sort(list: &mut Vec<i64>, len: usize) -> Option<&Vec<i64>>
{
    // 힙으로 만든다

    // 정렬한다

    build_heap(list, &len);
    Some(list)
}

fn build_heap(list: &mut Vec<i64>, len: &usize) {
    for i in (0..(len / 2)).rev() {
        println!("i is {}", i.to_string());
        heapify(list, i as usize, len);
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
//          1
//       3     6
//     4  5   7
fn heapify(list: &mut Vec<i64>, i: usize, len: &usize) {
// list[i]의 두 자식 노드
    // 이때 list[2 * i] 및 list[2 * i + 1]의 서브 트리는 힙성질을 만족하고 있다
    // 좌측 자식 노드의 인덱스 예상
    let left: usize = (&i * 2) + 1;
    // 우측 자식 노드의 인덱스 예상
    let right: usize = (&i + 1) * 2;

    // 부모 노드의 값과 비교하기 위해,
    // 자식 노드 중에 더 작은 값은 갖는 노드의 인덱스를 찾는다
    let smaller_opt = if &right < &len {
        // `right < len` 경우, `i`인덱스의 노드가 두 자식 노드를 갖는다
        if &list[left.clone()] < &list[right.clone()] {
            Some(left.clone())
        } else {
            Some(right.clone())
        }
    } else if &left < &len {
        // `right >= len && left < len`
        // `i`인덱스의 노드가 한 자식 노드를 갖는다
        Some(left.clone())
    } else {
        // `right >= len && left >= len`
        // 끝에 도달
        None
    };

    if smaller_opt == None {
        return;
    }

    let smaller: usize = smaller_opt.unwrap();
    // 부모노드의 값과 비교하여, 자식 노드의 값이 더 작다면 스왑
    if &list[smaller] < &list[i] {
        list.swap(smaller.clone(), i.clone());
        // println!("when recursive heapify, list => {}", iter_to_string(list.iter().clone(), ","));
        heapify(list, smaller.clone(), len);
    }
}