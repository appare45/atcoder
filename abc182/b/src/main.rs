use proconio::input;
fn main() {
    input! {
        a: [i32]
    }
    let mut gcd: i32 = 0;
    let mut max_count: i32 = 0;
    for i in 2..1000 {
        let mut count = 0;
        for &j in &a {
            if &j % &i==0 {
                count += 1;
            }
        }
        if max_count < count {
            gcd = i;
            max_count = count
        }
    }
    println!("{}", gcd)
}
