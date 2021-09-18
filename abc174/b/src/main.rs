use proconio::input;
fn main() {
    input! {
        n: i64,
        d: f64,
        xy: [(i64, i64); n]
    }
    let mut count = 0;
    for &i in &xy {
        if ((&i.0.pow(2) + &i.1.pow(2)) as f64).sqrt() <= d {
            count += 1;
        }
    }
    println!("{}", count);
}
