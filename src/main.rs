use num_integer::binomial;

use nsubset::{f, f_iter};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let s: usize = args[1].parse().unwrap();
    let k: usize = args[2].parse().unwrap();
    let len = binomial(s, k);

    println!("s = {}, k = {}", s, k);
    for n in 0..len {
        println!("n: {:2} = {:?} == {:?}", n, f(s, k, n), f_iter(s, k, n));
    }
}
