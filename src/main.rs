extern crate ferris_says;
extern crate motivations;
extern crate rand;

use ferris_says::say;
use rand::Rng;
use std::cmp;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = rand::thread_rng()
        .choose(&motivations::MOTIVATIONS)
        .unwrap()
        .as_bytes();
    let width = cmp::min(50, out.len());

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
