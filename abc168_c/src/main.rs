use proconio::input;
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }
    let deg = (h * 60_f64 + m) * 0.5 - m * 6_f64;
    println!("{}", (a * a + b * b - 2_f64 * a * b * ((deg)/180_f64*std::f64::consts::PI).cos()).sqrt())
}
