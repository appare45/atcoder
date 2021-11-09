use proconio::input;
fn main() {
    input! {
        s: String
    };
    println!("{}", solve(s))
}


fn solve(s: String) -> f32 {
let mut count: i32 = 0;
    let mut next = '0';
    for i in s.chars() {
        if i == next {
            count+=1;
        }
        next = if next == '0' {'1'} else {'0'}
    }
    return if (s.len() - count as usize) > count as usize {count as f32} else {(s.len() - count as usize) as f32};
}

#[cfg(test)]
#[test]
fn ex1() {
    assert_eq!(solve(String::from("000")), 1_f32);
}
#[test]
fn ex2() {
    assert_eq!(solve(String::from("10010010")), 3_f32);
}
#[test]
fn ex3() {
    assert_eq!(solve(String::from("0")), 0_f32);
}
#[test]
fn ex4() {
    assert_eq!(solve(String::from("1")), 0_f32)
}