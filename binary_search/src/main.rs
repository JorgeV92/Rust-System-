
use std::io;
mod binary_search;


fn main() {
    // println!("{}", binary_search::last_true(2, 10, binary_search::always_true));
    // println!("{}", binary_search::last_true(2, 10, binary_search::check));
    // println!("{}", binary_search::last_true(2, 10, binary_search::always_false));

    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let result = binary_search::binary_search(&mut arr, target);
    println!("Element found at index: {}", result);

    // Maximum Medium problem
    // https://codeforces.com/contest/1201/problem/C
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("Failed to read line");

    let mut iter = first.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i64 = iter.next().unwrap().parse().expect("Failed to parse k");

    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Failed to read line");

    let mut vec: Vec<i32> = second
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    println!("n = {}, k = {}", n, k);
    println!("Vector: {:?}", vec);

    vec.sort();
    println!("Vector: {:?}", vec);

    let res = binary_search::last_true(1, 2_000_000_000, |x| {
        let mut ops_needed: i64 = 0;
        for i in (n - 1) / 2..n {
            ops_needed += (x - vec[i] as i64).max(0);
        }
        ops_needed <= k
    });
    println!("{}",  res);
}
