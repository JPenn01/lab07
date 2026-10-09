const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
}

fn average_temp(log: &Vec<i32>) -> f64 {
	if log.is_empty() {
	return 0.0;
}
	let sum: i32 = log.iter().sum();
	sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<i32>) -> usize {
	let mut best = 0;
	for 1 in 1..log.len() {
	if log[i] > log [best] {
	best = i;
	}
}
best
}

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
	let mut count = 0;
	for $temp in log.iter() {
	if temp > threshold {
	count += 1;
	}
}
count
}

use rand::Rng;
let mut rng = rand::rng();
for _ in 0..2 {
highs.push(rng.random_range(60..=100))
}
println!("\n--- After adding two days ---");
print_log(&highs);


