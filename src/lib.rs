pub fn sort(pairs: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    let mut pairs = pairs.clone();
    let len = pairs.len();
    let min = pairs.iter().min_by_key(|p| p[0]).unwrap()[0];
    // let max = pairs.iter().max_by_key(|p| p[0]).unwrap()[0];
    let mut hist = hist(&pairs);
    let hist_copy = hist.clone();
    let start = cum(&hist);
    let mut objects = Vec::new();
    objects.resize(len, 0);
    for i in 0..len {
        let pos = start[(pairs[i][0] - min) as usize];
        let remaining = hist[(pairs[i][0] - min) as usize];
        hist[(pairs[i][0] - min) as usize] -= 1;
        objects[(pos + remaining - 1) as usize] = pairs[i][1];
    }
    for i in 0..(len - 1) {
        sort_with_index(&mut objects, start[i], start[i + 1]);
    }
    let mut j = 0;
    let mut l = 0;
    let mut last = -1;
    for i in 0..len {
        let val = hist_copy[i];
        let s = min + i as i64;
        for k in 0..val {
            let o = objects[l];
            l += 1;
            if k == 0 || o != last {
                pairs[j] = [s, o];
                j += 1;
            }
            last = o;
        }
    }
    pairs.resize(j as usize, [0, 0]);
    pairs
}

fn sort_with_index(v: &mut Vec<i64>, from: i64, to: i64) {
    for i in from..to {
        let mut j = i;
        let tmp = v[i as usize];
        while j > from && v[(j - 1) as usize] > tmp {
            v[j as usize] = v[(j - 1) as usize];
            j -= 1;
        }
        v[j as usize] = tmp;
    }
}

fn hist(pairs: &Vec<[i64; 2]>) -> Vec<i64> {
    let mut hist = Vec::new();
    hist.resize(pairs.len(), 0);
    for pair in pairs {
        hist[pair[0] as usize - 1] += 1;
    }
    hist
}

fn cum(hist: &Vec<i64>) -> Vec<i64> {
    let mut cum = Vec::new();
    cum.resize(hist.len(), 0);
    for (i, _e) in hist.iter().enumerate() {
        if i != 0 {
            cum[i] = cum[i - 1] + hist[i - 1];
        }
    }
    cum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![[4, 1], [2, 3], [1, 2], [5, 3], [4, 4]];
        let output = vec![[1, 2], [2, 3], [4, 1], [4, 4], [5, 3]];
        assert_eq!(sort(input), output);
    }

    #[test]
    fn dupplicate() {
        let input = vec![[4, 4], [2, 3], [1, 2], [5, 3], [4, 4]];
        let output = vec![[1, 2], [2, 3], [4, 4], [5, 3]];
        assert_eq!(sort(input), output);
    }

    #[test]
    fn already_sorted() {
        let input = vec![[1, 2], [2, 3], [4, 1], [4, 4], [5, 3]];
        let output = vec![[1, 2], [2, 3], [4, 1], [4, 4], [5, 3]];
        assert_eq!(sort(input), output);
    }

    #[test]
    fn test_hist() {
        assert_eq!(
            hist(&vec![[4, 1], [2, 3], [1, 2], [5, 3], [4, 4]]),
            vec![1, 1, 0, 2, 1]
        )
    }

    #[test]
    fn test_cum() {
        assert_eq!(cum(&vec![1, 1, 0, 2, 1]), vec![0, 1, 2, 2, 4])
    }
}
