use proconio::input;
fn main() {
    input! {
        a: u16
    }
    println!("{}", a + u16::pow(a, 2) + u16::pow(a, 3))
}
