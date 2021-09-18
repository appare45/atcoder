use proconio::input;
fn main() {
    input! {
        n: i32,
        k: usize,
        mut p: [i32; n]
    }
    p.sort();
    let mut ans = 0;
    for i in &p[0..k] {
        ans += i;
    }
    println!("{}", ans)
}
