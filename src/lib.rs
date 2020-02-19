fn sort(pairs: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    vec![[1, 2], [2, 3], [4, 1], [4, 4], [5, 3]]
}

fn hist(pairs: Vec<[i64; 2]>) -> Vec<i64> {
    let mut hist = Vec::new();
    hist.resize(pairs.len(), 0);
    for pair in pairs {
        hist[pair[0] as usize - 1] += 1;
    }
    hist
}

fn cum(hist: Vec<i64>) -> Vec<i64> {
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
    fn test_hist() {
        assert_eq!(
            hist(vec![[4, 1], [2, 3], [1, 2], [5, 3], [4, 4]]),
            vec![1, 1, 0, 2, 1]
        )
    }

    #[test]
    fn test_cum() {
        assert_eq!(cum(vec![1, 1, 0, 2, 1]), vec![0, 1, 2, 2, 4])
    }
}
