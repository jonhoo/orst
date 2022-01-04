use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    // if right <= 0, it's done
    while left <= right && right > 0 {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot && right > 0 {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            right -= 1;
        } else {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // place the pivot at its final location
    slice.swap(0, left);

    // left is the pivot index
    // less_pivot = [0, left)     greater_pivot = (left, slice.len() - 1]
    let (less_pivot, greater_pivot) = slice.split_at_mut(left);
    assert!(less_pivot.last() <= greater_pivot.first());
    quicksort(less_pivot);
    quicksort(&mut greater_pivot[1..]);
}

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
