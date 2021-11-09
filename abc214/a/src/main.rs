use proconio::input;
fn main() {
    input! {
        n: i32
    }
    match &(n) {
       &(1..=125)  => println!("4"),
       &(126..=211) => println!("6"),
       &_ =>println!("8")
    }
}
