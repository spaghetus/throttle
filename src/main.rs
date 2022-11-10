use std::{
	io::{stdin, stdout, Read, Write},
	time::{Duration, Instant},
};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
	/// Bytes per second
	#[structopt(short)]
	rate: u128,
	/// Whether to flush after every byte or buffer lines
	#[structopt(short)]
	flush: bool,
}

fn main() {
	let mut i = stdin().lock();
	let mut o = stdout().lock();
	let args = Opts::from_args();
	let rate = args.rate;
	let flush = args.flush;
	let mut last = Instant::now();
	let mut buf = [0u8; 1024];
	let mut stacked = Duration::ZERO;
	loop {
		let now = Instant::now();
		let delta = now - last;
		last = now;
		stacked += delta;
		let bytes = stacked.as_nanos() * rate / 1_000_000_000;
		if bytes > 0 {
			stacked = Duration::ZERO;
		} else {
			std::thread::sleep(Duration::from_nanos(
				(1_000_000_000 / rate as u64) - (stacked.as_nanos() as u64),
			));
			continue;
		}
		let bytes = bytes.min(1024) as usize;
		let read = i.read(&mut buf[..bytes]).unwrap();
		o.write_all(&buf[..read]).unwrap();
		if flush {
			o.flush().unwrap();
		}
	}
}
