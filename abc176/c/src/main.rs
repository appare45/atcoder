use proconio::input;
fn main() {
    input! {
        a: [u64]
    }
    let mut max = a[0];
    let mut ans = 0;
    for &i in &a {
        if &i < &max {
            ans += &max - &i
        } else {
            max = i
        }
    }
    println!("{}", ans)
}
