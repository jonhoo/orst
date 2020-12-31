pub trait Sorter<T>{
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;
mod radixsort;
mod heapsort;
mod mergesort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;
pub use radixsort::RadixSort;
pub use radixsort::Bytify;
pub use heapsort::HeapSort;
pub use mergesort::MergeSort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    #[test]
    fn stdunstable_works() {
        let mut things = vec![4, 2, 3, 1];
        StdUnstableSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
