use proconio::input;
fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n]
    }
    
    let mut ans = false;
    for i in 0..n {
        if a[i] == x {
            ans = true;
        }
    }
    
    if ans {
        println!("Yes")
    } else {
        println!("No")
    }
}
