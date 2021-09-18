use proconio::input;
fn main() {
    input! {
        a: [u16; 4],
    };
    let mut min = 100;
    for &i in &a {
        if min > i {
            min = i
        }
    }
    println!("{}", min)
}
