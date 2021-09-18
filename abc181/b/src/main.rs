use proconio::input;
fn main() {
    input! {
        n: u64,
        ab: [(u64, u64); n]
    }
    let mut ans = 0;
    for &i in &ab {
        ans +=  (i.1 + i.0) * (i.1 - i.0 + 1) / 2
    }
    println!("{}", ans)
}
