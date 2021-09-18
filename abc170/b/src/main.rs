use proconio::input;
fn main() {
    input! {
        x: f64,
        y: f64
    }
    let b = (y - 2f64 * x) / 2f64;
    println!("{}", if b % 1f64 == 0f64 && b <= x && y >= 2f64 * x { "Yes" } else { "No" })
}
