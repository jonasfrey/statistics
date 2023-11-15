use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Specify the path to your file with random bytes
    let file_path = "./../random_test_data//5120000_random_bytes";

    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file's contents into a byte array
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Find the minimum value in the byte array
    if let Some(&min_value) = buffer.iter().min() {
        println!("The minimum byte value is: {}", min_value);
    } else {
        println!("The file is empty");
    }

    Ok(())
}
