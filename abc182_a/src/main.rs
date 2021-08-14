use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32
    }
    println!("{}", a * 2 + 100 - b)
}
