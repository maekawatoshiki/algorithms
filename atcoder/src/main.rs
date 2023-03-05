use proconio::input;

struct Context {
    n: i32,
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

    let context = Context { n, l, k, a };
    for score in 1..l {
        if !context.is_score_feasible(score) {
            println!("{}", score - 1);
            return;
        }
    }
    // println!("{}", context.is_score_feasible(2));
}
