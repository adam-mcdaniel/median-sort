use sorting::*;

use clap::Parser;
use rand::prelude::*;
use log::{error, info};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// The file to read the array from. If not provided, a random array will be generated
    #[clap(value_parser)]
    file: Option<String>,
    /// The number of elements to use for median-of-median selection
    #[clap(short, long, default_value = "5")]
    r: usize,
    /// Cutoff for insertion sort
    #[clap(short, long, default_value = "12")]
    cutoff: usize,
    /// Reverse the array after sorting
    #[clap(short, long)]
    reverse: bool,
    /// The separator to use when writing the array to a file
    #[clap(short, long, default_value = "\n")]
    separator: String,
    /// Run the tests
    #[clap(short, long)]
    test: bool,
}

#[allow(dead_code)]
fn random_char_arrays<const LEN: usize>(n: usize) -> Vec<[char; LEN]> {
    let mut rng: ThreadRng = thread_rng();
    (0..n).map(|_| {
        let mut arr = ['\0'; LEN];
        for c in arr.iter_mut() {
            *c = rng.gen();
        }
        arr
    }).collect()
}

#[allow(dead_code)]
pub fn time_it<F: FnOnce() -> T, T>(f: F) -> std::time::Duration {
    let start = std::time::Instant::now();
    f();
    start.elapsed()
}

#[allow(dead_code)]
fn test_sort<T>(arr: Vec<T>) where T: Ord + Copy {
    use std::io::Write;

    let trials = 5;
    let mut csv_file = std::fs::File::create("quicksort.csv").unwrap();
    csv_file.write_all(b"r,cutoff,avg_time (seconds),median_time (seconds)\n").unwrap();

    for r in vec![3, 5, 7, 11, 13, 15, 17, 23, 29] {
        for cutoff in 3..=200 {
            let mut times = (0..trials).map(|_| {
                let mut temp = arr.clone();
                let duration = time_it(|| quicksort(&mut temp, r, cutoff));
                assert!(temp.windows(2).all(|w| w[0] <= w[1]), "Failed to sort");
                println!("Trial took {:?}", duration);
                duration
            }).collect::<Vec<_>>();
            
    
            times.sort();
    
            let best_times = &times[0..times.len() / 2];
    
            let avg_time = best_times.iter().sum::<std::time::Duration>() / best_times.len() as u32;
            let median_time = best_times[best_times.len() / 2];
    
            // Write to a CSV file
            writeln!(csv_file, "{},{},{},{}", r, cutoff, avg_time.as_secs_f64(), median_time.as_secs_f64()).unwrap();
            info!("Done with r={r}, cutoff={cutoff}: Avg time={avg_time:?} Median time={median_time:?} over {trials} trials");
        }
	
    }
}

/*
fn main() {
    let arr = random_array(5_000_000, 0..250);
    test_sort(arr);
}
*/

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Show info and error logs
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut arr = match args.file {
        Some(file) => {
            let contents = std::fs::read_to_string(file).unwrap_or_else(|e| {
                error!("Failed to read file: {}", e);
                std::process::exit(1);
            });
            contents.split_whitespace().map(|s| s.parse::<i32>().unwrap_or_else(|e| {
                error!("Failed to parse number: {}", e);
                std::process::exit(1);
            })).collect::<Vec<_>>()
        },
        None => random_array(1_000_000, 0..1_000_000),
    };
    
    if args.test {
        test_sort(arr);
        return;
    }

    // let arr = random_array(5_000_000, 0..250);
    // let arr = (0..5_000_000).rev().collect::<Vec<_>>();
    // let arr = random_char_arrays::<10>(5_000_000);
    // println!("Sorting array of length {}", arr.len());
    quicksort(&mut arr, args.r, args.cutoff);

    if args.reverse {
        arr.reverse();
    }

    let output = arr.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(&args.separator);
    println!("{output}");
}
