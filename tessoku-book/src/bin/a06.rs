use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        range: [(usize,usize); q]
    }

    let range = range
        .iter()
        .map(|(l, r)| (l - 1, r - 1))
        .collect::<Vec<(usize, usize)>>();

    let mut sum = vec![0; n];
    for i in 0..n {
        sum[i] = a[i];
        if i > 0 {
            sum[i] += sum[i - 1];
        }
    }

    for (l, r) in range {
        let ans = if l == 0 { sum[r] } else { sum[r] - sum[l - 1] };
        println!("{}", ans);
    }
}
