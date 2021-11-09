use proconio::input;
fn main() {
    input! {
        a: char
    }
    match a {
        'A'..='Z' => println!("A"),
        'a'..='z' => println!("a"),
        _ => ()
    }
}
