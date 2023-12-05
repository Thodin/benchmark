use std::{time::{Instant, Duration}, cmp::min};
use volatile::VolatileRef;

fn main() {
    // Benchmark range-based for, indexed for and while loop.
    // Run in --release mode, otherwise it might take some time...
    let mut nums = [1_i32; 100_000];
    for n in &mut nums {
        *n = rand::random();
    }
    let mut sum = 0_i32;
    let reps = 100_000;
    #[allow(unused_assignments)]
    let mut start = Instant::now();
    let mut vol_sum = VolatileRef::from_mut_ref(&mut sum);
    let mut avg_duration_rb_no_access = Duration::from_secs(0);
    let mut avg_duration_indexed_no_access = Duration::from_secs(0);
    let mut avg_duration_while_no_access = Duration::from_secs(0);
    let mut avg_duration_rb = Duration::from_secs(0);
    let mut avg_duration_indexed = Duration::from_secs(0);
    let mut avg_duration_while = Duration::from_secs(0);
    let mut sum_of_sums = 0;


    for _ in 0..reps {
        start = Instant::now();
        for _ in nums {
            vol_sum.as_mut_ptr().write(1);
        }
        avg_duration_rb_no_access += start.elapsed();

        start = Instant::now();
        for _ in 0..nums.len() {
            vol_sum.as_mut_ptr().write(1);
        }
        avg_duration_indexed_no_access += start.elapsed();

        let mut i = 0_usize;
        start = Instant::now();
        while i < nums.len() {
            vol_sum.as_mut_ptr().write(1);
            i += 1;
        }
        avg_duration_while_no_access += start.elapsed();

        // With accessing nums
        // NOTE: if we do `for n in nums` (without the reference),
        // we copy n from the nums vector, thus the performance will be worse.
        // If we had an array of non-Copy elements, we would consume it.
        let mut sum = 0;
        start = Instant::now();
        for n in &nums {
            sum += n;
        }
        avg_duration_rb += start.elapsed();
        sum_of_sums += sum;

        sum = 0;
        start = Instant::now();
        for i in 0..nums.len() {
            sum += nums[i];
        }
        avg_duration_indexed += start.elapsed();
        sum_of_sums += sum;

        sum = 0;
        let mut i = 0_usize;
        start = Instant::now();
        while i < nums.len() {
            sum += nums[i];
            i += 1;
        }
        avg_duration_while += start.elapsed();
        sum_of_sums += sum;

        //start = Instant::now();
        //sum = nums.iter().fold(0, |acc, n| acc + n);
        //elapsed = start.elapsed();
        //println!("Duration fold is            {:?}, checksum: {}", elapsed, sum);

        //start = Instant::now();
        //sum = nums.iter().sum();
        //elapsed = start.elapsed();
        //println!("Duration sum is             {:?}, checksum: {}", elapsed, sum);
    }

    // normalization
    let min_no_access = min(min(avg_duration_rb_no_access, avg_duration_indexed_no_access), avg_duration_while_no_access).as_millis();
    let avg_duration_rb_no_access_ms = avg_duration_rb_no_access.as_millis() as f64 / min_no_access as f64;
    let avg_duration_indexed_no_access_ms = avg_duration_indexed_no_access.as_millis() as f64 / min_no_access as f64;
    let avg_duration_while_no_access_ms = avg_duration_while_no_access.as_millis() as f64 / min_no_access as f64;
    let min = min(min(avg_duration_rb, avg_duration_indexed), avg_duration_while).as_millis();
    let avg_duration_rb_ms = avg_duration_rb.as_millis() as f64 / min as f64;
    let avg_duration_indexed_ms = avg_duration_indexed.as_millis() as f64 / min as f64;
    let avg_duration_while_ms = avg_duration_while.as_millis() as f64 / min as f64;
    println!("Relative durations:\n\tRange-based for (no access): {}\n\tIndex-based for (no access): {}\n\tWhile (no access): {}",
             avg_duration_rb_no_access_ms, avg_duration_indexed_no_access_ms, avg_duration_while_no_access_ms);
    println!("Relative Durations:\n\tRange-based for: {}\n\tIndex-based for: {}\n\tWhile: {}",
             avg_duration_rb_ms, avg_duration_indexed_ms, avg_duration_while_ms);
    println!("Sum of sums = {}", sum_of_sums);
}
