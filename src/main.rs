mod matrix;
mod naive;
mod power;
mod simple;

use std::time::{Duration, Instant};

use num::BigUint;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub trait FibonnaciCalculator: Sync + Send {
    fn calculate(&self, index: u64) -> BigUint;
}

impl FibonnaciCalculator for Box<dyn FibonnaciCalculator> {
    fn calculate(&self, index: u64) -> BigUint {
        self.as_ref().calculate(index)
    }
}

fn bench_fibonnaci<T>(calculator: T, time: Duration) -> u64
where
    T: FibonnaciCalculator,
{
    let mut index = 1;
    let start_time = Instant::now();
    loop {
        calculator.calculate(index);
        if start_time.elapsed() >= time {
            break;
        }
        index *= 2;
    }
    index
}

fn main() {
    let time = Duration::from_secs(1);

    let calculators: Vec<(&'static str, Box<dyn FibonnaciCalculator>)> = vec![
        //("Naive", Box::new(naive::Naive)),
        ("Simple", Box::new(simple::Simple)),
        ("Power", Box::new(power::Power)),
    ];

    let results: Vec<_> = calculators
        .into_par_iter()
        .map(|(name, calculator)| {
            let index = bench_fibonnaci(calculator, time);
            (name, index)
        })
        .collect();

    for (name, result) in results {
        println!("{}: {}", name, result);
    }
}
