#![feature(generators, generator_trait, conservative_impl_trait)]
use std::ops::{Generator, GeneratorState};

fn main() {
    let mut generator = || {
        yield 1;
        return "foo";
    };

    match generator.resume() {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }
    match generator.resume() {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected value from resume"),
    }

    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);
}

fn fibonacci() -> impl Iterator<Item = u32> {
    || {
        let mut curr = 0;
        let mut next = 1;
        loop {
            yield curr;
            let old = curr;
            curr = next;
            next += old;
        }
    }
}
