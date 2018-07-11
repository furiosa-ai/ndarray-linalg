extern crate ndarray;
extern crate ndarray_linalg;

use std::thread;

use ndarray::*;
use ndarray_linalg::*;

fn main() {
    let n = 10000;

    let mut handles = Vec::new();
    for idx in 0..4 {
        let handle = thread::spawn(move || {
            let a: Array2<f64> = random((n, n));
            let b: Array1<f64> = random(n);
            let x = a.solve_into(b).unwrap();
            println!("Done in thread {}", idx);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
