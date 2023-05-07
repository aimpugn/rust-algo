// T 타입의 길이를 구하는 방법은 다양할 수 있으므로,
// `len`은 인자로 받도록 한다
pub fn sort(input_list: &mut Vec<i64>, input_list_length: usize, desc: bool) -> Option<&Vec<i64>> {
    // 1. 힙으로 만들어서 전체 노드가 힙성질을 갖게 만든다
    build_heap(input_list, input_list_length, desc);

    // 2. 최소 힙 사용한 정렬:
    //  a. 최소값인 루트 노드부터 빼내면 내림차순으로 정렬할 수 있고,
    //  b. 힙 성질이 깨진 나머지 배열을 다시 힙으로 만들어 주고
    //  c. 다시 루트 노드를 가장 뒤로 빼내는 것을 반복하면
    //  d. 내림차순으로 정렬이 된다
    for unsorted_last_index in (1..input_list_length).rev() {
        // a & c. `pop` root node and property of heap might be broken
        input_list.swap(0, unsorted_last_index);
        // b. Modify the rest of the array to become a heap.
        heapify(input_list, 0, unsorted_last_index, desc); // `i`: decrement 된 사이즈와 같은 값인 `i`를 그대로 전달한다
    }
    Some(input_list)
}

fn build_heap(input_list: &mut Vec<i64>, unsorted_last_index: usize, desc: bool) {
    // `(last_length / 2)`: 리프 노드의 부모를 찾는다
    // - you start heapifying from the parent of the last leaf node and work your way up to the root.
    // - 리프 노드는 자식 노드가 없으므로 그 자체로 heap 성질 만족
    // - 따라서 리프 노드의 부모 노드부터 시작해서 루트까지 타고 올라가며 heap 성질을 복구
    for current_index in (0..(unsorted_last_index / 2)).rev() {
        heapify(input_list, current_index, unsorted_last_index, desc);
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
fn heapify(list: &mut Vec<i64>, current_index: usize, unsorted_last_index: usize, desc: bool) {
    // 이때 list[i]의 두 자식 노드 `list[(2 * i) + 1]` 및 `list[(2 * i) + 2]`의 서브 트리는 힙성질을 만족하고 있다
    // 좌측 자식 노드의 인덱스 예상: `list[(2 * i) + 1]`
    let left_child_index: usize = (&current_index * 2) + 1;
    // 우측 자식 노드의 인덱스 예상: `list[(2 * i) + 2]`
    let right_child_index: usize = (&current_index + 1) * 2;

    let mut smallest = current_index;

    // Check if the left child is smaller than the current smallest
    if left_child_index < unsorted_last_index && list[left_child_index] < list[smallest] {
        smallest = left_child_index;
    }

    // Check if the right child is smaller than the current smallest
    if right_child_index < unsorted_last_index && list[right_child_index] < list[smallest] {
        smallest = right_child_index;
    }

    // If a smaller child is found, swap it with the current node and continue heapifying
    if smallest != current_index {
        list.swap(smallest, current_index);
        heapify(list, smallest, unsorted_last_index, desc);
    }
}
