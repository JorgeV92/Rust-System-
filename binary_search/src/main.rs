
mod binary_search;

fn last_true(mut lo :i32, mut hi :i32, f :fn(i32) -> bool) -> i32 {
    lo -= 1;
    while lo < hi {
        let mid :i32 = lo + (hi - lo + 1) / 2;
        if f(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn always_true(_ :i32) -> bool {
    true
}

fn always_false(_ :i32) -> bool {
    false
}

fn check(x :i32) -> bool {
    x * x <= 30
}

fn main() {
    println!("{}", last_true(2, 10, always_true));
    println!("{}", last_true(2, 10, check));
    println!("{}", last_true(2, 10, always_false));

    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let result = binary_search::binary_search(&mut arr, target);
    println!("Element found at index: {}", result);
}
