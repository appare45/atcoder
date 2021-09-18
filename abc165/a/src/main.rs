use proconio::input;
fn main() {
    input! {
        k: u32,
        a: u32,
        b: u32
    }
    println!("{}", if b - b % k >= a { "OK" } else { "NG" })
}
