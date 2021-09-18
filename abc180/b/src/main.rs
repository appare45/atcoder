use proconio::input;
fn main() {
    input! {        
        x: [i64]
    }
    let mut man = 0;
    let mut yu: f64 = 0_f64;
    let mut che= Vec::<i64>::new();
    for &i in &x {
        man += &i.abs();
        yu += i.pow(2) as f64;
        che.push(i.abs());
    }
    println!("{}" ,man);
    println!("{}", yu.sqrt());
    println!("{}", che.iter().max().unwrap())
}
