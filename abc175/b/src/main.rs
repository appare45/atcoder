use proconio::input;
fn main() {
    input! {
         n: usize,
         mut l: [i32; n],
    }
    l.sort();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                if l[i] + l[j] > l [k] {
                    ans += 1;
                }
            }
        }
    };
    println!("{}", ans);
}
