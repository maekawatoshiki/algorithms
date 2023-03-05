use proconio::input;

struct Context {
    l: i32,
    k: i32,
    a: Vec<i32>,
}

impl Context {
    fn is_score_feasible(&self, score: i32) -> bool {
        let mut count = 0;
        let mut last_cut_pos = 0;
        for &cut_pos in &self.a {
            if cut_pos - last_cut_pos >= score && self.l - cut_pos >= score {
                count += 1;
                last_cut_pos = cut_pos;
            }
        }
        count >= self.k
    }
}

fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        a: [i32; n],
    }
    assert!(1 <= k && k <= n && n <= 100000);

    let context = Context { l, k, a };

    let mut min = 1;
    let mut max = l;

    while max - min > 1 {
        let mid = min + (max - min) / 2;
        if context.is_score_feasible(mid) {
            min = mid;
        } else {
            max = mid;
        }
    }
    assert!(min + 1 == max);

    let max_score = min;
    println!("{}", max_score);
}
