use proconio::input;
fn main() {
    input! {
        x: i8
    }
    println!("{}", if x >= 30 { "Yse" } else { "No" })
}
