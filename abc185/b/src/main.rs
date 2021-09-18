use proconio::input;
fn main() {
    input! {
        n: i64,
        m: u64,
        t: i64,
        ab: [(i64, i64); m]
    }
    let mut battery = n;
    let mut time = 0;
    for &i in &ab {
        battery -= &i.0 - time;
        if battery <= 0 {
            break;
        }
        battery += if battery + i.1 - i.0 > n { n - battery } else { i.1 - i.0 };
        time = i.1
    }
    battery -= t - time;
    if battery <= 0{
        println!("No")
    } else {
        println!("Yes")
    }
}