use rand::Rng;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
	day: String,
	high: i32,
}

struct Summary {
	average: f64,
	hottest_day: String,
	days_above: usize,
}

fn average_temp(log: &Vec<Reading>) -> f64 {
	if log.is_empty() {
	return 0.0;
}
	let mut sum = 0;
	for r in log.iter() {
	sum += r.high;
}
	sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
	let mut best = 0;
	for i in 1..log.len() {
	if log[i].high > log[best].high {
	best = i;
	}
}
best
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
	let mut count = 0;
	for r in log.iter() {
	if r.high > threshold {
	count += 1;
	}
}
count
}

fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
	let hottest = if log.is_empty() {
	String::new()
}else{
	log[hottest_day(log)].day.clone()
};
Summary {
	average: average_temp(log),
	hottest_day: hottest,
	days_above: count_above(log, threshold),
	}
}

fn print_log(log: &Vec<Reading>) {
	for r in log.iter() {
	println!("{}: {}", r.day, r.high);
	}
}

fn main() {
	let starting = [72, 68, 75, 81, 79];
	let mut log: Vec<Reading> = Vec::new();
	for i in 0..starting.len() {
	  log.push(Reading {
	    day: DAYS[i].to_string(),
	    high: starting[i],
	});
}

println!("--- Initial log ---");
print_log(&log);

let mut rng = rand::rng();
for _ in 0..2 {
	let next_day = DAYS[log.len()].to_string();
	log.push(Reading {
	   day: next_day,
	   high: rng.random_range(60..=100),
	});
}

println!("\n--- After adding two days ---");
print_log(&log);

	let summary = summarize(&log, 75);
	println!("\nAverage: {:.1}", summary.average);
	println!("Hottest day: {}", summary.hottest_day);
	println!("Days above 75: {}", summary.days_above);
}
