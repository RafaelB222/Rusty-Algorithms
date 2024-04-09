fn quick_sort(mut arr: [i32; 10]) -> [i32; 10] {
    let hi = arr.len() - 1;
    qs(&mut arr, 0, hi);

    return arr;
}

fn qs(arr: &mut [i32; 10], lo: usize, hi: usize) {
    if lo >= hi {
        println!("{}", "i'm done");
        return;
    }

    let pivot_index = partition(arr, lo, hi);

    qs(arr, lo, pivot_index - 1);

    qs(arr, pivot_index + 1, hi);
}

fn partition(arr: &mut [i32; 10], lo: usize, hi: usize) -> usize {
    let pivot: i32 = arr[hi];

    let mut idx = lo;

    for i in lo..hi {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }
    arr.swap(idx, hi);

    return idx;
}

#[cfg(test)]
#[test]
pub fn quick_sort_works() {
    let arr: [i32; 10] = [1, 9, 2, 8, 3, 7, 4, 6, 5, 2];

    let sorted_array = quick_sort(arr);

    assert_eq!([1, 2, 2, 3, 4, 5, 6, 7, 8, 9], sorted_array);
}