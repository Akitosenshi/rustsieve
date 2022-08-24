use std::{collections::HashMap, time::Instant};

fn main() {
	let historical_data: HashMap<usize, usize> = [
		(10, 4),
		(100, 25),
		(1000, 168),
		(10000, 1229),
		(100000, 9592),
		(1000000, 78498),
		(10000000, 664579),
		(100000000, 5761455),
		(1000000000, 50847534),
		(10000000000, 455052511),
	]
	.iter()
	.cloned()
	.collect();

	let max: usize = 1000000;
	let sqr: usize = (max as f64).sqrt() as usize;
	let primes: usize = historical_data[&max];
	let mut sieve = vec![true; max];
	sieve[0] = false;
	let mut digits: usize = 0;
	{
		let mut i = max as i64;
		while !i == 0 {
			digits += 1;
			i /= 10;
		}
	}
	// setup done
	let start = Instant::now();
	let mut i: usize = 1;
	while i < sqr {
		if sieve[i] {
			let mut j: usize = (i + 1) * (i + 1) - 1;
			while j < max {
				sieve[j] = false;
				j += i + 1;
			}
		}
		i += 1;
	}
	let t = Instant::now().duration_since(start);
	// sieve done
	let start_format = Instant::now();
	let mut res: String = "".to_string();
	let mut counted = 0;
	for i in 0..max {
		if sieve[i] {
			res.push_str(&(format!("{1:0$} {2}\n", &digits, &(i + 1), "is prime")));
			counted += 1;
		} else {
			res.push_str(&(format!("{1:0$} {2}\n", &digits, &(i + 1), "is not prime")));
		}
	}
	let t_format = Instant::now().duration_since(start_format);
	println!(
		"{}counted: {}\nactual:  {}\n\ncalculated in {:?}\nformatting this output took {:?}",
		res, counted, primes, t, t_format
	);
}
