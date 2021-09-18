use proconio::input;
fn main() {
    input! {
        n: i32,
    }
    let mut ab_count = 0;
    for i in 1..n {
        ab_count += (n-1) / i;
    }
    println!("{}", ab_count);
}
