use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is non-empty");

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
