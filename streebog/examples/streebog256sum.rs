use std::env;
use std::fs;
use std::io::{self, Read};
use streebog::{Digest, Streebog256};

const BUFFER_SIZE: usize = 1024;

/// Print digest result as hex string and name pair
fn print_result(sum: &[u8], name: &str) {
    for byte in sum {
        print!("{:02x}", byte);
    }
    println!("\t{}", name);
}

/// Compute digest value for given `Reader` and print it
/// On any error simply return without doing anything
fn process<D: Digest + Default, R: Read>(reader: &mut R, name: &str) {
    let mut sh = D::default();
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return,
        };
        sh.update(&buffer[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }
    print_result(&sh.result(), name);
}

fn main() {
    let args = env::args();
    // Process files listed in command line arguments one by one
    // If no files provided process input from stdin
    if args.len() > 1 {
        for path in args.skip(1) {
            if let Ok(mut file) = fs::File::open(&path) {
                process::<Streebog256, _>(&mut file, &path);
            }
        }
    } else {
        process::<Streebog256, _>(&mut io::stdin(), "-");
    }
}
