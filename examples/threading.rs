extern crate ndarray;
extern crate ndarray_linalg;
extern crate rand;

use std::thread;

use ndarray::{Array1, arr1};
use ndarray::{Array2, arr2};
use rand::Rng;

use ndarray_linalg::Solve;

fn main() {
    println!("Hello, world!");

    let mut handles = vec![];

    let n: usize = 10_000_000_000usize;

    for idx in 0..4 {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut x: f64 = 0.;

            for _ in 0..n {
                x += rng.gen::<f64>();
            }

            println!(
                "Thread no {} finished averaging {} random numbers. \
                 Result: {}",
                idx,
                n,
                x / (n as f64)
            );

            let matrix: Array2<f64> = arr2(&[[1., 1., 0.], [0., 2., 0.], [0., 0., 3.]]);
            let vector: Array1<f64> = arr1(&[rng.gen(), rng.gen(), rng.gen()]);

            let result = matrix.solve_into(vector).unwrap();

            println!("Thread no {} says: result = {}", idx, result);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
