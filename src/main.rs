use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = File::open("test.txt").expect("can't open test file.");
    let mut file_out = BufWriter::new(File::create("res.txt").expect("can't open test file."));

    let reader = BufReader::new(file);
  
    reader
        .lines()
        .for_each(|x| {
            let result = calc_prime(x.unwrap().parse::<u32>().unwrap());
            file_out
                .write(format!("{} {}\n", result.0, result.1).as_bytes())
                .expect("can not write");
        });
    let end = Instant::now();
    let elapsed = end - start;
    println!("Execution time: {:.2?}", elapsed);    
    print!("done");
}
    
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    if num < 4 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let sqrt_num = (num as f64).sqrt() as u32 + 1;
    for i in (3..sqrt_num).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn calc_prime(num: u32) -> (u32, u32) {
	for i in 2..num {
		if is_prime(i) && num%i == 0 {
			return(i, num / i);
		}
	}
	return(1, num);
}
