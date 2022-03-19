use std::fs;
use std::time::Instant;
use std::vec;

fn main() {
    let mut primes: Vec<u64> = vec![2];
    let start = Instant::now();
    find_primes(&mut primes, 200000000);
    let time_taken = start.elapsed();
    println!("It took {time_taken:.3?}");
    let mut prime_txt = String::new();
    for prime in primes {
        prime_txt.push_str(&prime.to_string());
        prime_txt.push('\n');
    }
    fs::write("primes.txt", prime_txt).expect("Writing to primes.txt");
}

fn find_primes(primes: &mut Vec<u64>, num: u64) {
    for n in 3..num {
        if is_prime(n, primes) {
            primes.push(n);
        }
    }
}

fn is_prime(x: u64, primes: &Vec<u64>) -> bool {
    let top = (x as f64).sqrt().floor() as u64;
    for n in primes {
        if (x % n) == 0 {
            return false;
        }
        if n > &top {
            return true;
        }
    }
    return true;
}
