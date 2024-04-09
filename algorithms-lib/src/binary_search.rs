fn binary_search(arr: &[i32; 20], needle: i32) -> i32 {
    let mut lo: usize = 0;
    let mut hi = arr.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let v = arr[m];

        if v == needle {
            println!("{}", "found the needle");
            return v;
        } else if v < needle {
            lo = m + 1;
        } else if v > needle {
            hi = m;
        }
        println!(
            "values after loop: lo: {}, hi: {}, m: {}, v: {}",
            lo, hi, m, v
        );
    }
    println!("{}", "didn't find the needle");
    return -1;
}

#[cfg(test)]
#[test]
pub fn binary_search_works() {
    let pre_sorted_arr: [i32; 20] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    let needle: i32 = 12;
    let result: i32 = binary_search(&pre_sorted_arr, needle);
    assert_eq!(needle, result);
}