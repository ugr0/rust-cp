use proconio::input;
fn main() {
    input! {
        n: i32,
        k: i32
    }

    let mut ans = 0;
    for i in 1..(n + 1) {
        for j in 1..(n + 1) {
            let tmp: i32 = k - i - j;
            if tmp > 0 && tmp <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
