use rand::prelude::*;
use rayon::prelude::*;
use std::thread;

fn sum_single_thread(x: Vec<i32>) -> i32 {
    if x.is_empty() {
        eprintln!("Input vector is empty. Returning 0.");
        return 0;
    }
    x.iter().sum()
}

fn sum_with_threads(x: Vec<i32>, n: i32) -> i32 {
    let n_usize: usize = n as usize;
    let out = sum_with_threads_impl(x, n_usize);
    out
}

fn sum_with_threads_impl(x: Vec<i32>, n: usize) -> i32 {
    if x.is_empty() {
        eprintln!("Input vector is empty. Returning 0.");
        return 0;
    }

    let n = n.min(x.len());
    let chunk_size = (x.len() + n - 1) / n;

    let mut handles = Vec::new();
    for i in 0..n {
        let chunk = x[i * chunk_size..((i + 1) * chunk_size).min(x.len())].to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<i32>()));
    }

    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().expect("Thread panicked");
    }

    total_sum
}

fn sum_with_rayon(x: Vec<i32>, n: i32) -> i32 {
    if x.is_empty() {
        eprintln!("Input vector is empty. Returning 0.");
        return 0;
    }

    // Configure Rayon to use a specific number of threads
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n as usize)
        .build()
        .expect("Failed to set up thread pool");

    let res = thread_pool.install(|| x.par_iter().sum());
    res
}

fn main() {
    use std::time::Instant;

    let v: Vec<i32> = (0..50_000).map(|_| rand::thread_rng().gen()).collect();
    let v_orig = v.clone();

    // sum the vector with just `sum()`
    let now = Instant::now();
    {
        println!("{:?}", sum_single_thread(v));
    }
    let elapsed = now.elapsed();
    println!("Sum with single thread: {:.2?}", elapsed);

    for n_threads in [2, 4, 8] {
        let v2 = v_orig.clone();
        let v3 = v_orig.clone();

        let now = Instant::now();
        {
            println!("{:?}", sum_with_threads(v2, n_threads));
        }
        let elapsed = now.elapsed();
        println!("Sum with {} threads: {:.2?}", n_threads, elapsed);

        let now = Instant::now();
        {
            println!("{:?}", sum_with_rayon(v3, n_threads));
        }
        let elapsed = now.elapsed();
        println!("Sum with rayon ({} threads): {:.2?}", n_threads, elapsed);
    }
}
