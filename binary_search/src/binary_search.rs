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


// pub fn last_true(mut lo :i32, mut hi :i32, f :fn(i32) -> bool) -> i32 {
//     lo -= 1;
//     while lo < hi {
//         let mid :i32 = lo + (hi - lo + 1) / 2;
//         if f(mid) {
//             lo = mid;
//         } else {
//             hi = mid - 1;
//         }
//     }
//     lo
// }



pub fn last_true<F>(mut lo :i64, mut hi :i64, f :F) -> i64
where 
    F: Fn(i64) -> bool,
{
    lo -= 1;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if f(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

pub fn always_true(_ :i32) -> bool {
    true
}

pub fn always_false(_ :i32) -> bool {
    false
}

pub fn check(x :i32) -> bool {
    x * x <= 30
}
