#[macro_use]
extern crate criterion;
extern crate advent_of_code2015;
extern crate aoc_runner;

use advent_of_code2015::*;
use aoc_runner::ArcStr;
use criterion::Criterion;
use std::fmt::Display;

#[inline]
fn black_box(t: &dyn Display) {
    criterion::black_box(t);
}

fn aoc_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2015/day1.txt"));


    
    let mut group = c.benchmark_group("Day1 - Part1");

    
    {
        let runner = Factory::day1_part1(input_day1.clone())
            .expect("failed to generate input for (default)");
        group.bench_function("(default)", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day1 - Part2");

    
    {
        let runner = Factory::day1_part2(input_day1.clone())
            .expect("failed to generate input for (default)");
        group.bench_function("(default)", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn input_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2015/day1.txt"));


    
}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
