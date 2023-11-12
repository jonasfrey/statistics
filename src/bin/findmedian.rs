use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::env;
use std::time::{Instant, Duration};

fn read_file_and_find_median<P: AsRef<Path>>(path: P) -> io::Result<f32> {
    // Read the file
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Interpret the buffer as uint16 values
    let mut u16_values = Vec::new();
    for chunk in buffer.chunks_exact(2) {
        u16_values.push(u16::from_le_bytes([chunk[0], chunk[1]]));
    }

    // Check if the vector is empty
    if u16_values.is_empty() {
        return Ok(0.0);
    }

    // Sort the values to find the median
    u16_values.sort_unstable();
    let mid = u16_values.len() / 2;
    let median = if u16_values.len() % 2 == 0 {
        (u16_values[mid - 1] as f32 + u16_values[mid] as f32) / 2.0
    } else {
        u16_values[mid] as f32
    };

    Ok(median)
}

fn main() -> io::Result<()> {
    println!("Current directory: {:?}", env::current_dir()?);

    let start = Instant::now();

    let path = "./a_n_u16__random";
    match read_file_and_find_median(path) {
        Ok(median) => println!("Median: {}", median),
        Err(e) => println!("Error: {}", e),
    }

    // Stop the timer and calculate the duration
    let duration = start.elapsed();

    println!("Time taken: {} milliseconds", duration.as_millis());

        
    Ok(())
}