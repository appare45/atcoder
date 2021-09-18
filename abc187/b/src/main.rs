use proconio::input;
fn main() {
    input! {
        n: usize,
        ij: [(i64, i64); n]
    }
    let mut count = 0;
    for k in 0..n {
        for l in k..n {
            if ij[l].0 != ij[k].0 {
                let c = (ij[l].1 - ij[k].1) / (ij[l].0 - ij[k].0);
                if c >= -1 && c <= 1 { count  += 1};
            }
        }
    }
    println!("{}", count)
}
