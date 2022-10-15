
// T 타입의 길이를 구하는 방법은 다양할 수 있으므로,
// `len`은 인자로 받도록 한다
pub fn heap_sort(list: Vec<i64>, len: usize) -> Option<Vec<i64>>
{
    // 힙으로 만든다

    // 정렬한다

    Some(list)
}

fn build_heap(list: Vec<i64>, len: usize) {
    for i in (len / 2) .. 0 {
        heapify(&list, i as usize, &len);
    }
}

fn heapify(list: &Vec<i64>, i: usize, len: &usize) {
    // list[i]의 두 자식 노드
    // 이때 list[2 * i] 및 list[2 * i + 1]의 서브 트리는 힙성질을 만족하고 있다
    let left: usize = &i * 2;
    let right: usize = (&i * 2) + 1;

    let smaller = if right < *len {
        if &list[left] < &list[right] {
            &left
        } else {
            &right
        }
    } else if left < *len {
        &left
    } else {
        return;
    };

}