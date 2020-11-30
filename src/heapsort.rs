use super::Sorter;

/// This is an in-place heapsort implementation. The code here is heavily
/// inspired by the Geeks for Geeks article on the topic, which can be found
/// [here](https://www.geeksforgeeks.org/heap-sort/).
/// This implementation uses a max-heap to sort the provided slice in ascending
/// order.
pub struct HeapSort;
impl<T> Sorter<T> for HeapSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // turn `slice` into a heap
        for i in (0..(slice.len() / 2)).rev() {
            heapify(slice, i);
        }

        // perform heapsort
        for unsorted in (0..slice.len()).rev() {
            // we have a valid max heap here, so remove the top
            slice.swap(0, unsorted);

            // now we want to make sure that the rest is also sorted
            heapify(&mut slice[..unsorted], 0);
        }
    }
}

fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    let n = slice.len();

    if left < n && slice[left] > slice[largest] {
        largest = left;
    }
    if right < n && slice[right] > slice[largest] {
        largest = right;
    }
    // at this point, `largest` points at the largest of root and its children

    if largest != root {
        slice.swap(largest, root);
        heapify(slice, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_array() {
        let mut slice = [1, 5, 4, 2, 3];
        HeapSort.sort(&mut slice);
        assert_eq!(slice, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorted_array() {
        let mut slice = (1..10).into_iter().collect::<Vec<_>>();
        HeapSort.sort(&mut slice);
        assert_eq!(slice, (1..10).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn very_unsorted() {
        let mut slice = (1..10000).into_iter().rev().collect::<Vec<_>>();
        HeapSort.sort(&mut slice);
        assert_eq!(slice, (1..10000).into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn simple_edge_cases() {
        let mut one = vec![1];
        HeapSort.sort(&mut one);
        assert_eq!(one, vec![1]);

        let mut two = vec![1, 2];
        HeapSort.sort(&mut two);
        assert_eq!(two, vec![1, 2]);

        let mut two = vec![2, 1];
        HeapSort.sort(&mut two);
        assert_eq!(two, vec![1, 2]);

        let mut three = vec![3, 1, 2];
        HeapSort.sort(&mut three);
        assert_eq!(three, vec![1, 2, 3]);
    }
}
