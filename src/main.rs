use std::time::Instant;
use rand::{distributions::Standard, Rng};

const ITERATIONS: usize = 10000;

fn main() {

    let mut stable_sort_speed: [u64; ITERATIONS as usize] = [0; ITERATIONS];
    let mut unstable_sort_speed: [u64; ITERATIONS as usize] = [0; ITERATIONS];
    let mut fung_sort_speed: [u64; ITERATIONS as usize] = [0; ITERATIONS];
    let mut select_sort_speed: [u64; ITERATIONS as usize] = [0; ITERATIONS];

    for i in 0..ITERATIONS {
        let mut stable_vec: Vec<u64> = rand::thread_rng().sample_iter(Standard).take(100).collect();
        let mut unstable_vec = stable_vec.clone();
        let mut fung_vec = stable_vec.clone();
        let mut select_vec = stable_vec.clone();

        let now = Instant::now();
        stable_vec.sort();
        stable_sort_speed[i] = now.elapsed().as_nanos() as u64;

        let now = Instant::now();
        unstable_vec.sort_unstable();
        unstable_sort_speed[i] = now.elapsed().as_nanos() as u64;
        assert_eq!(stable_vec, unstable_vec);

        let now = Instant::now();
        fung_sort(&mut fung_vec);
        fung_sort_speed[i] = now.elapsed().as_nanos() as u64;
        assert_eq!(unstable_vec, fung_vec);

        let now = Instant::now();
        select_sort(&mut select_vec);
        select_sort_speed[i] = now.elapsed().as_nanos() as u64;
        assert_eq!(fung_vec, select_vec);
    }

    println!("Stable sort completed {} iterations in {} an average of nanoseconds", ITERATIONS, mean(&stable_sort_speed));
    println!("Unstable sort completed {} iterations in {} an average of nanoseconds", ITERATIONS, mean(&unstable_sort_speed));
    println!("Fung sort completed {} iterations in {} an average of nanoseconds", ITERATIONS, mean(&fung_sort_speed));
    println!("Select sort completed {} iterations in {} an average of nanoseconds", ITERATIONS, mean(&select_sort_speed));

}

fn fung_sort(numbers: &mut Vec<u64>) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if numbers[i] < numbers[j] {
                numbers.swap(i, j);
            }
        }
    }
}

fn select_sort(numbers: &mut Vec<u64>) {
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
            }
        }
    }
}


fn mean(numbers: &[u64]) -> f64 {
    let sum : u64 = numbers.iter().sum();
    let len = numbers.len() as f64;
    sum as f64 / len
}