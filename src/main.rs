use rand::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::thread;
use std::time::Instant;

// this fn calcultes the total number of points that are inside the circle for given N points.
fn simulate_and_return_n(n: u32, radius: f64) -> u32 {
    // equation of a circle: returns if a point is inside a circle or not.
    let inside_circle = |x: f64, y: f64, r: f64| -> bool { x * x + y * y - r * r <= 0 as f64 };

    let count = (1..=n)
        .into_par_iter()
        .filter(move |_| {
            let mut rng = rand::thread_rng();
            let y: f64 = rng.gen::<f64>() * radius;
            let x: f64 = rng.gen::<f64>() * radius;

            inside_circle(x, y, radius)
        })
        .count();
    count as u32
}

fn run_simulation(n: u32, radius: f64) {
    let pi = |n, big_n| -> f64 { (4 * n) as f64 / big_n as f64 };
    let success = simulate_and_return_n(n, radius);
    println!("pi = {} for n = {} and N = {}", pi(success, n), success, n);
}

fn main() {
    let now = Instant::now();
    let total_test: u32 = 8;
    let mut vec_handle = Vec::new();

    for i in 1..=total_test {
        // spawn threads equal to the number of tests.
        let handle = thread::spawn(move || run_simulation(10u32.pow(i), 4.0));
        vec_handle.push(handle);
    }

    for handle in vec_handle {
        handle.join().unwrap();
    }
    println!("Finished in {} secs", now.elapsed().as_secs());
}
