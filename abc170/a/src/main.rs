use proconio::input;
fn main() {
    input! {
        x: [u8; 5]
    }
    let mut ans: u8 = 0;
    for i in 0..x.len() {
        if x[i] == 0 && i as u8 != 0 {
            ans = if i+1 == x.len() {x[i - 1 as usize] + 1} else {x[i + 1 as usize] - 1}
        }
    }
    println!("{}", if ans == 0 { x[1] - 1} else {ans})
}
