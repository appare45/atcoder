use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
        s: String
    }

    println!("{} {}", n + a + b, s);
}
