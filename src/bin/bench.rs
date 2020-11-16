use orst::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::{max, Ordering};
use std::rc::Rc;

//only the first digit of MIN_ELEMENT and MAX_ELEMENT is considered
const MIN_ELEMENT: usize = 0;
const MAX_ELEMENT: u32 = 6000;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    assert!(
        MAX_ELEMENT >= (MIN_ELEMENT + 10_usize.pow((MIN_ELEMENT as f64).log10() as u32)) as u32,
        "MAX_ELEMENT should be greater than (MIN_ELEMENT + 10^log10(MIN_ELEMENT))"
    );
    let digits_max: u32 = (MAX_ELEMENT as f64).log10() as u32;
    let digits_min: u32 = max(0, (MIN_ELEMENT as f64).log10() as u32);
    let n_max: usize = (digits_max * 9 + MAX_ELEMENT / (10_u32.pow(digits_max))) as usize;
    let n_min: usize = (digits_min * 9 + MIN_ELEMENT as u32 / (10_u32.pow(digits_min))) as usize;
    let index = n_max - n_min + 1;
    let mut items = Vec::with_capacity(index);
    let rounded_min_element = (n_min - digits_min as usize * 9) * (10_u32.pow(digits_min)) as usize;
    items.push(rounded_min_element);
    for _ in 1..index {
        let items_clone = items.clone();
        let la = items_clone.last().unwrap();
        items.push(la + 10_i32.pow((*la as f64).log10() as u32) as usize);
    }

    println!("algorithm n comparisons time");
    for &n in &items {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {} {}", "bubble", n, took.0, took.1);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("{} {} {} {}", "insertion-dumb", n, took.0, took.1);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {} {}", "selection", n, took.0, took.1);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {} {}", "quick", n, took.0, took.1);
            let took = bench(StdSorter, &values, &counter);
            println!("{} {} {} {}", "stdstable", n, took.0, took.1);
            let took = bench(StdUnstableSorter, &values, &counter);
            println!("{} {} {} {}", "stdunstable", n, took.0, took.1);
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter<T>>(
    sorter: S,
    values: &[T],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}
