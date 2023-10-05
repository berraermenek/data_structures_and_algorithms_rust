fn main() {
    println!("{}", euclid(1633, 355))
}

fn euclid(m: usize, n: usize) -> usize {
    let r = m % n;
    if r == 0 {
        n
    } else {
        euclid(n, r)
    }
}
