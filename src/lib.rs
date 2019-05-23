use num_integer::binomial;

pub fn f(s: usize, k: usize, n: usize) -> Vec<usize> {
    let current = binomial(s, k);
    if current == 1 {
        return (0..k).collect::<Vec<_>>();
    }

    let half = binomial(s - 1, k - 1);
    if n < half {
        // print!("<");

        let mut res = f(s - 1, k - 1, n);

        for x in res.iter_mut() {
            *x += 1
        }

        res.insert(0, 0);
        res
    } else {
        let mut res = f(s - 1, k, n - half);

        for x in res.iter_mut() {
            *x += 1
        }
        res
    }
}

pub fn f_iter(mut s: usize, mut k: usize, mut n: usize) -> Vec<usize> {
    let len = k;
    let mut res = vec![0; k];
    let mut start_number = 0;

    let mut current = binomial(s, k);
    while current != 1 {
        let half = binomial(s - 1, k - 1);

        if n < half {
            res[len - k] = start_number;
            start_number += 1;

            s -= 1;
            k -= 1;
            current = half;
        } else {
            start_number += 1;

            s -= 1;
            n -= half;
            current = binomial(s, k)
        }
    }

    for a in 0..k {
        res[len - k + a] = start_number + a;
    }

    res
}
