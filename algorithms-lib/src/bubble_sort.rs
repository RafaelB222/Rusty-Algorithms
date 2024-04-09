fn bubble_sort(mut arr: [i32; 10]) -> [i32; 10] {
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    return arr;
}

#[cfg(test)]
#[test]
pub fn bubble_sort_works() {
    let arr: [i32; 10] = [1, 9, 2, 8, 3, 7, 4, 6, 5, 2];

    let sorted_array = bubble_sort(arr);

    assert_eq!([1, 2, 2, 3, 4, 5, 6, 7, 8, 9], sorted_array);
}