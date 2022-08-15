use rand::{thread_rng, Rng};
use std::sync::mpsc::*;
use std::thread;

// The area A between y=lnx & y=(lnx)^2 equals 3-e
// lnx=(lnx)^2 (solving for x) is x=1, x=e
// A is inscribed in the rectangle b = (e-1) & h = 1
// Here's a proof using good ol' Monte Carlo

fn main() {
    let cpu_count = num_cpus::get();

    // Increase for a better approximation
    let iter = 2_000_000;

    let total = iter*cpu_count;
    println!("Total number of iterations: {:e}", total);

    let e = std::f64::consts::E;

    // the random number generated falls between [0,1)
    // excluding 1, but [0, 1+ε) includes 1
    // pretty unnecessary, yes
    let one1 = 1_f64+std::f64::EPSILON;
    let e1 = e + std::f64::EPSILON - 1_f64;

    println!("{}, {}", one1, e1);

    let (tx, rx) = channel();

    for _ in 0..cpu_count {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut rng = thread_rng();
            let mut hit_count = 0;
            for _ in 0..iter {
                // 1<=x<=e
                let x = rng.gen::<f64>() * e1 + 1_f64;
                // 0<=y<=1
                let y = rng.gen::<f64>()*one1;
                let lnx = x.ln();
                if y <= lnx && y >= lnx.powi(2) {
                    hit_count += 1;
                }
            }
            tx.send(hit_count).unwrap();
        });
    }

    println!("Thread count: {}", cpu_count);

    let mut hit_count = 0;
    for _ in 0..cpu_count {
        hit_count += rx.recv().unwrap();
    }

    println!(
        "{:e} / {:e} * ({}-1) = {} ≅ {} ≅ 3-𝑒",
        hit_count,
        total,
        e,
        (hit_count as f64) / (total as f64) * (e - 1_f64),
        3_f64 - e
    );
}
