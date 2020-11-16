pub trait Sorter<T>{
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}
pub trait CloneSorter {
    fn clone_sort<T>(&self, slice: &mut [T])
    where
        T: Ord+Clone;
}

mod bubblesort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use mergesort::MergeSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
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
}
