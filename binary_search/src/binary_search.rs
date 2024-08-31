pub fn binary_search(a: &mut [i32], t :i32) -> i32 {
    let n = a.len();
    let mut l = 0;
    let mut r = n as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        if a[m as usize] == t {
            return m 
        } else if a[m as usize] < t {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    -1
}

