// 归并排序是一种复杂度为O(n log n)的排序算法。在Rust中，可以利用其数据结构高效地实现该算法。
fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp[..]);
    arr.copy_from_slice(&temp);
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    if i < left.len() {
        result[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        result[k..].copy_from_slice(&right[j..]);
    }
}
