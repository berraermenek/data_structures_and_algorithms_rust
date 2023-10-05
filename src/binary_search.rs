fn main() {
    let a = vec![13, 16, 23, 45, 54, 58, 76, 91];
    println!("{}", binary_search(&a, 73));
}

fn binary_search(a: &[usize], b: usize) -> bool {
    let n = a.len();
    if n == 0 {
        false
    } else if a[n / 2] == b {
        true
    } else if a[n / 2] > b {
        binary_search(&a[0..n / 2], b)
    } else {
        binary_search(&a[n / 2 + 1..], b)
    }
}
